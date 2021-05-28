use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//you can change this if the words you want have smaller words
const LONGEST_WORD: usize = 31;

pub trait DictLookup {
    fn word_check(&self, word: &String) -> bool;
}

//holds the dictionary to use
pub struct Dictionary {
    lexicon: HashSet<String>,
}

impl Dictionary {
    pub fn init(path: &str) -> Dictionary {
        let file = File::open(path).expect(&format!(
            "File Not Found at {}, please recheck your path.",
            path
        ));

        let reader = BufReader::new(file);

        let read = reader.lines().map(|x| x.unwrap());

        Dictionary {
            lexicon: read.collect(),
        }
    }
}

//just return true if word checked gets a match
impl DictLookup for Dictionary {
    fn word_check(&self, word: &String) -> bool {
        self.lexicon.contains(word)
    }
}

pub struct Letters {
    consonants: HashSet<char>,
    vowels: HashSet<char>,
}

impl Letters {
    pub fn init() -> Letters {
        let cons: HashSet<char> = vec![
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'z',
        ]
        .into_iter()
        .collect();

        let vowel: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();

        Letters {
            consonants: cons,
            vowels: vowel,
        }
    }

    pub fn letter_test(&self, letter: char) -> LastandSecondLast {
        if self.consonants.contains(&letter) {
            return LastandSecondLast::Consonant;
        } else if self.vowels.contains(&letter) {
            return LastandSecondLast::Vowel;
        } else if letter == 'y' {
            return LastandSecondLast::Y;
        } else {
            return LastandSecondLast::None;
        }
    }
}

//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
//update: a select few words have 3 consonants & vowels together
#[derive(PartialEq, Clone, Copy)]
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Tripleconsonant,
    Vowel,
    Doublevowel,
    Triplevowel,
    Y,
    None,
}

impl LastandSecondLast {
    //ugly code that takes our current letter state, compares it with the last code and returns the corresponding state
    //TODO: replace with match guards
    fn sequent_letter(
        current_letter: LastandSecondLast,
        last_letter: LastandSecondLast,
    ) -> LastandSecondLast {
        match current_letter {
            LastandSecondLast::Consonant if last_letter == LastandSecondLast::Consonant => {
                LastandSecondLast::Doubleconsonant
            }
            LastandSecondLast::Consonant if last_letter == LastandSecondLast::Doubleconsonant => {
                LastandSecondLast::Tripleconsonant
            }
            LastandSecondLast::Consonant if last_letter == LastandSecondLast::Tripleconsonant => {
                LastandSecondLast::None
            }
            LastandSecondLast::Consonant => LastandSecondLast::Consonant,

            LastandSecondLast::Vowel if last_letter == LastandSecondLast::Vowel => {
                LastandSecondLast::Doublevowel
            }
            LastandSecondLast::Vowel if last_letter == LastandSecondLast::Doublevowel => {
                LastandSecondLast::Triplevowel
            }
            LastandSecondLast::Vowel if last_letter == LastandSecondLast::Triplevowel => {
                LastandSecondLast::None
            }
            LastandSecondLast::Vowel => LastandSecondLast::Vowel,

            LastandSecondLast::Y if last_letter == LastandSecondLast::Y => LastandSecondLast::None,
            LastandSecondLast::Y => LastandSecondLast::Y,

            //if current letter isn't one of these 3 discriminants, we made a bad write somewhere
            _ => panic!("That's not how the current letter works."),
        }
    }
}

//holds our word while we iterate
struct AltWord {
    current_letter: char,
    letters: String,
    location: [usize; 2],
    final_word: usize,
    last_state: LastandSecondLast,
}

impl AltWord {
    fn new() -> AltWord {
        AltWord {
            current_letter: '_',
            letters: String::new(),
            location: [0, 0],
            final_word: 0,
            last_state: LastandSecondLast::None,
        }
    }
}

const DIRECTIONS: [&'static str; 8] = [
    "Up",
    "Upleft",
    "Left",
    "Downleft",
    "Down",
    "Downright",
    "Right",
    "Upright",
];

pub struct WordBlob {
    pub wordsearch: Vec<char>,
    dictionary: Dictionary,
    letters: Letters,
    //wordsearch uses vec so only do rectangular wordsearches
    width: usize,
}

impl WordBlob {
    pub fn alloc(path_to_wordsearch: &str, path_to_dictionary: &str) -> WordBlob {
        let found: (Vec<char>, usize) = WordBlob::get_wordsearch(path_to_wordsearch);

        WordBlob {
            wordsearch: found.0,
            dictionary: Dictionary::init(path_to_dictionary),
            letters: Letters::init(),
            width: found.1,
        }
    }

    pub fn get_wordsearch(path_to_wordsearch: &str) -> (Vec<char>, usize) {
        let mut wsearch: Vec<char> = Vec::new();
        let mut width: usize = 0;

        let file = File::open(path_to_wordsearch).expect(&format!(
            "File Not Found at {}, please recheck your path.",
            path_to_wordsearch
        ));

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line.unwrap();

            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }

            if width == 0 {
                width = line.len()
            } else {
                if !(width == line.len()) {
                    panic!("Wordsearch must be rectangular")
                }
            }

            for letter in line.chars() {
                wsearch.push(letter)
            }
        }

        (wsearch, width)
    }

    pub fn indexer(&self, index: usize) -> [usize; 2] {
        [index / self.width, index % self.width]
    }

    //length, width
    fn gather(&self, location: [usize; 2]) -> Option<&char> {
        let index: usize = self.width * location[0] + location[1];
        self.wordsearch.get(index)
    }

    fn go(location: [usize; 2], direction: &str) -> [usize; 2] {
        match direction {
            "up" => [location[0] - 1, location[1]],
            "upright" => [location[0] - 1, location[1] + 1],
            "right" => [location[0], location[1] + 1],
            "downright" => [location[0] + 1, location[1] + 1],
            "down" => [location[0] + 1, location[1]],
            "downleft" => [location[0] + 1, location[1] - 1],
            "left" => [location[0], location[1] - 1],
            "upleft" => [location[0] - 1, location[1] - 1],

            _ => panic!("Unexpected direction passed"),
        }
    }

    fn traverse(&self, direction: &str, location: [usize; 2]) -> Option<(String, [usize; 2])> {
        //big brain allocate mutable memory in loop so it drops out of scope
        let mut current_state: LastandSecondLast;
        let mut currentword = AltWord::new();

        currentword.location = location;

        match self.gather(currentword.location) {
            Some(c) => currentword.current_letter = *c,
            None => panic!("Invariant: Function called out of bounds."),
        }

        currentword.last_state = self.letters.letter_test(currentword.current_letter);

        if currentword.last_state == LastandSecondLast::None {
            return None;
        }

        loop {
            let next = WordBlob::go(currentword.location, direction);

            match self.gather(next) {
                Some(c) => currentword.current_letter = *c,
                None => {
                    if currentword.final_word != 0 {
                        break Some((
                            currentword.letters[..currentword.final_word].to_string(),
                            location,
                        ));
                    } else {
                        break None;
                    }
                }
            }

            current_state = self.letters.letter_test(currentword.current_letter);

            currentword.last_state =
                LastandSecondLast::sequent_letter(current_state, currentword.last_state);

            match currentword.last_state {
                LastandSecondLast::None => {
                    if currentword.final_word != 0 {
                        break Some((
                            currentword.letters[..currentword.final_word - 1].to_string(),
                            location,
                        ));
                    } else {
                        break None;
                    }
                }

                _ => {
                    currentword.letters.push(currentword.current_letter);

                    if currentword.letters.len() >= 3 {
                        if self.dictionary.word_check(&currentword.letters) {
                            currentword.final_word = currentword.letters.len();
                        }
                    }

                    if currentword.letters.len() == LONGEST_WORD {
                        if currentword.final_word != 0 {
                            break Some((
                                currentword.letters[..currentword.final_word - 1].to_string(),
                                location,
                            ));
                        } else {
                            break None;
                        }
                    }
                }
            }

            currentword.location = next;
        }
    }

    pub fn start(&self, index: usize) -> Option<Vec<(String, String, [usize; 2])>> {
        let mut words_found: Vec<(String, String, [usize; 2])> = Vec::new();
        let location = self.indexer(index);

        for direction in DIRECTIONS.iter() {
            match self.traverse(direction, location) {
                Some(r) => words_found.push((r.0, direction.to_string(), r.1)),
                None => {}
            }
        }

        if !(words_found.is_empty()) {
            Some(words_found)
        } else {
            None
        }
    }
}

impl DictLookup for WordBlob {
    fn word_check(&self, word: &String) -> bool {
        self.dictionary.lexicon.contains(word)
    }
}
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dictionary_checker() {
        let testdict = Dictionary::init("myDictsorted.txt");
        assert!(testdict.lexicon.contains("wantonly"));
    }
}
