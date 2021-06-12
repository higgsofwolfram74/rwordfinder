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
#[derive(Debug, PartialEq)]
pub struct Dictionary {
    lexicon: HashSet<String>,
}

impl Dictionary {
    pub fn init(path: &str) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("File Not Found at {}, please recheck your path.", path));

        let reader = BufReader::new(file);

        let read = reader.lines().map(|x| x.unwrap());

        Self {
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

//Use hash set for O(1) contains method
#[derive(Debug, PartialEq)]
pub struct Letters {
    consonants: HashSet<char>,
    vowels: HashSet<char>,
}

impl Letters {
    pub fn init() -> Self {
        let cons: HashSet<char> = vec![
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'z',
        ]
        .into_iter()
        .collect();

        let vowel: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();

        Self {
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
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub struct Wordsearch {
    character_matrix: Vec<char>,
    width: usize,
}

impl Wordsearch {
    pub fn init(path_to_wordsearch: &str) -> Self {
        let mut wsearch: Vec<char> = Vec::new();
        let mut column: usize = 0;

        let file = File::open(path_to_wordsearch).unwrap_or_else(|_| {
            panic!(
                "File Not Found at {}, please recheck your path.",
                path_to_wordsearch
            )
        });

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line.unwrap();

            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }

            if column == 0 {
                column = line.len()
            } else {
                if !(column == line.len()) {
                    panic!("Wordsearch must be rectangular")
                }
            }

            for letter in line.chars() {
                wsearch.push(letter)
            }
        }

        Self {
            character_matrix: wsearch,
            width: column,
        }
    }

    pub fn get(&self, index: [usize; 2]) -> Option<char> {
        if index[1] < self.width {
            let local: usize = self.width * index[0] + index[1];

            if self.character_matrix.get(local).is_some() {
                Some(self.character_matrix[local])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn indexer(&self, index: usize) -> [usize; 2] {
        [index / self.width, index % self.width]
    }

    pub fn go(&self, location: [usize; 2], direction: &str) -> Option<[usize; 2]> {
        let point = match direction {
            "Up" if location[0] > 0 => [location[0] - 1, location[1]],
            "Upleft" if location[0] > 0 && location[1] > 0 => [location[0] - 1, location[1] - 1],
            "Left" if location[1] > 0 => [location[0], location[1] - 1],
            "Downleft" if location[1] > 0 => [location[0] + 1, location[1] - 1],
            "Down" => [location[0] + 1, location[1]],
            "Downright" => [location[0] + 1, location[1] + 1],
            "Right" => [location[0], location[1] + 1],
            "Upright" if location[0] > 0 => [location[0] - 1, location[1] + 1],

            "Up" | "Upleft" | "Left" | "Downleft" | "Upright" => return None,

            _ => panic!("Invalid Direction has been passed"),
        };

        if point[0] >= self.width {
            None
        } else {
            Some(point)
        }
    }

    pub fn len(&self) -> usize {
        self.character_matrix.len()
    }
}

struct AltWord {
    current_letter: char,
    letters: String,
    location: [usize; 2],
    final_word: usize,
    last_state: LastandSecondLast,
}

impl AltWord {
    fn new() -> Self {
        Self {
            current_letter: '_',
            letters: String::new(),
            location: [0, 0],
            final_word: 0,
            last_state: LastandSecondLast::None,
        }
    }
}

//struct to just hold everything together
#[derive(Debug, PartialEq)]
pub struct WordBlob {
    pub wordsearch: Wordsearch,
    dictionary: Dictionary,
    letters: Letters,
}

impl WordBlob {
    pub fn alloc(path_to_wordsearch: &str, path_to_dictionary: &str) -> Self {
        Self {
            wordsearch: Wordsearch::init(path_to_wordsearch),
            dictionary: Dictionary::init(path_to_dictionary),
            letters: Letters::init(),
        }
    }

    fn traverse(&self, direction: &str, location: [usize; 2]) -> Option<(String, [usize; 2])> {
        //big brain allocate mutable memory in loop so it drops out of scope
        let mut current_state: LastandSecondLast;
        let mut currentword = AltWord::new();

        currentword.location = location;

        match self.wordsearch.get(currentword.location) {
            Some(c) => {
                currentword.current_letter = c;
            }
            None => panic!("Invariant: Function called out of bounds."),
        }

        currentword.last_state = self.letters.letter_test(currentword.current_letter);

        if currentword.last_state == LastandSecondLast::None {
            return None;
        } else {
            currentword.letters.push(currentword.current_letter);
        }

        loop {
            let next: Option<[usize; 2]>;
            //move to next letter
            match self.wordsearch.go(currentword.location, direction) {
                Some(i) => next = Some(i),
                None => next = None,
            };

            //if go returns none, we have reached the bound
            if next.is_none() {
                if currentword.final_word != 0 {

                    break Some((
                        currentword.letters[..currentword.final_word].to_string(),
                        location,
                    ));
                } else {
                    break None;
                }

            } else {
                currentword.location = next.unwrap();
            }

            //get the next letter to add
            match self.wordsearch.get(currentword.location) {
                Some(c) => currentword.current_letter = c,
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
                            currentword.letters[..currentword.final_word].to_string(),
                            location,
                        ));
                    } else {
                        break None;
                    }
                }

                _ => {
                    currentword.letters.push(currentword.current_letter);

                    if currentword.letters.len() >= 3 {
                        //println!("Testing {}", currentword.letters);
                        if self.dictionary.word_check(&currentword.letters) {
                            currentword.final_word = currentword.letters.len();
                        }
                    }

                    if currentword.letters.len() == LONGEST_WORD {
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
            }
        }
    }

    pub fn start(&self, index: [usize; 2]) -> Option<Vec<(String, String, [usize; 2])>> {
        let mut words_found: Vec<(String, String, [usize; 2])> = Vec::new();

        for &direction in DIRECTIONS.iter() {
            match self.traverse(direction, index) {
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
    //#[test]
    //fn dictionary_checker() {
    //    let testvec: Wordsearch = Wordsearch{
    //        character_matrix: vec!('a', 'b', 'c', 'a', 'p', 't', 'i', 'h', 'g'),
    //        width: 3
    //    };
    //
    //    let testblob: WordBlob = WordBlob {
    //        wordsearch: testvec,
    //        dictionary: Dictionary::init("myDictsorted.txt"),
    //        letters: Letters::init(),
    //    };
    //
    //    assert_eq!(testblob, WordBlob::alloc("test.txt", "myDictsorted.txt"));
    //}

    #[test]
    fn where_are_we() {
        let testvec: Wordsearch = Wordsearch {
            character_matrix: vec!['a', 'b', 'c', 'a', 'p', 't', 'i', 'h', 'g'],
            width: 3,
        };

        assert_eq!(testvec.get([2, 0]).unwrap(), 'i');
        assert_eq!(testvec.get([1, 1]).unwrap(), 'p');
        assert_eq!(testvec.get([0, 5]), None);
        assert_eq!(testvec.get([22, 8]), None);
    }

    #[test]
    fn where_do_we_go() {
        let testvec: Wordsearch = Wordsearch {
            character_matrix: vec!['a', 'b', 'c', 'a', 'p', 't', 'i', 'h', 'g'],
            width: 3,
        };

        let up = "Up";
        let right = "Right";
        let downleft = "Downleft";

        assert_eq!(testvec.go([1, 0], up).unwrap(), [0, 0]);
        assert_eq!(testvec.go([1, 0], right).unwrap(), [1, 1]);
        assert_eq!(testvec.go([1, 0], downleft), None);
        assert_eq!(testvec.go([5, 7], up), None);
    }

    #[test]
    fn indexing() {
        let testvec: Wordsearch = Wordsearch {
            character_matrix: vec!['a', 'b', 'c', 'a', 'p', 't', 'i', 'h', 'g'],
            width: 3,
        };

        assert_eq!(testvec.indexer(7), [2, 1]);
        assert_eq!(testvec.indexer(5), [1, 2]);
    }

    #[test]
    fn cat() {
        let testvec: Wordsearch = Wordsearch {
            character_matrix: vec!['c', 'b', 'c', 'a', 'p', 't', 'r', 'h', 'g'],
            width: 3,
        };

        let mut word = String::new();
        let mut location = [0, 0];

        for _ in 0..2 {
            word.push(testvec.get(location).unwrap());
            location = testvec.go(location, "Down").unwrap();
        }

        word.push(testvec.get(location).unwrap());
        assert_eq!(word, "car");
    }
}

//row, column
