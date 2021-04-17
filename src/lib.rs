use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use ndarray::Array2;

const LONGEST_WORD: usize = 31;
const ARRAY_HEIGHT: usize = 5;
const ARRAY_LENGTH: usize = 5;


pub trait DictLookup {
    fn word_check(&self, word: &String) -> bool;
}


pub trait ArrayTraversal {
    fn traverse(&self, row: &usize, column: &usize, direction: &str) -> Option<((usize, usize), String)>;
}

//holds the dictionary to use
pub struct Dictionary {
    lexicon: HashSet<String>
}


impl Dictionary {
    pub fn init(path: &str) -> Dictionary {
        

        let file = File::open(path).expect("File Not Found");

        let reader = BufReader::new(file);

        let read = reader.lines().map(|x| {x.unwrap()});
        
        Dictionary {
            lexicon: read.collect(),
        }
    }
}


pub struct Letters {
    consonants: HashSet<char>,
    vowels: HashSet<char>,
}

impl Letters {
    pub fn init() -> Letters {

        let cons: HashSet<char> = vec!('b', 'c', 'd', 'f', 'g', 'h',
                           'j', 'k', 'l', 'm', 'n', 'p',
                           'q', 'r', 's', 't', 'v', 'w', 
                           'x', 'z').into_iter().collect();


        let vowel: HashSet<char> = vec!('a', 'e', 'i', 'o', 'u').into_iter().collect();

        Letters {
            consonants: cons,
            vowels: vowel
        }
    }


    pub fn letter_test(&self, letter: &char) -> LastandSecondLast {
        if self.consonants.contains(letter) {
            return LastandSecondLast::Consonant;
        } else if self.vowels.contains(letter) {
            return LastandSecondLast::Vowel;
        } else if *letter == 'y' {
            return LastandSecondLast::Y;
        } else {
            return LastandSecondLast::None;
        }
    }
}

struct CurrentWord {
    current_letter: char,
    letters: String,
    location: (usize, usize),
    final_word: ((usize, usize), String),
    last_state: LastandSecondLast,
    current_state: LastandSecondLast,
}

impl CurrentWord {
    fn new() -> CurrentWord {
        CurrentWord {
            current_letters: '_',
            letters: String::new(),
            location: (0,0),
            final_word: ((0,0), String::new()),
            last_state: LastandSecondLast::None,
            current_state: LastandSecondLast::None
        }
    }
}

//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
//update: a select few words have 3 consonants together
#[derive(PartialEq, Clone)]
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Tripleconsonant,
    Vowel,
    Doublevowel,
    Y,
    None,
    Err(&'static str)
}

impl LastandSecondLast {
    //ugly code that takes our current letter state, compares it with the last code and returns the corresponding state
    fn last_letter(current_letter: &LastandSecondLast, last_letter: &LastandSecondLast) -> LastandSecondLast {
        match current_letter {
            LastandSecondLast::Consonant => {
                
                match last_letter {    
                    LastandSecondLast::Consonant => LastandSecondLast::Doubleconsonant,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Tripleconsonant,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::None,
                    LastandSecondLast::Vowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    _ => LastandSecondLast::Err("Invariant broken")

                
                }
            }

            
            LastandSecondLast::Vowel => {
                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Vowel => LastandSecondLast::Doublevowel,
                    LastandSecondLast::Doublevowel => LastandSecondLast::None,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    _ => LastandSecondLast::Err("Invariant broken")
                }    

            }
            LastandSecondLast::Y => {

                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Y,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Vowel => LastandSecondLast::Y,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Y,
                    LastandSecondLast::Y => LastandSecondLast::None,
                    _ => LastandSecondLast::Err("Invariant broken")
                }
            }

            LastandSecondLast::None => LastandSecondLast::None,

            _ => LastandSecondLast::Err("Data in corrupted state.")
        }
    }
}



pub struct WordBlob {
    wordsearch: Array2<char>,
    dictionary: Dictionary,
    letters: Letters
}

impl WordBlob {
    pub fn alloc(path_to_dictionary: &str) -> WordBlob {
        WordBlob {
            wordsearch: Array2::from_elem((ARRAY_HEIGHT, ARRAY_LENGTH), '_'),
            dictionary: Dictionary::init(path_to_dictionary),
            letters: Letters::init()
        }
    }

    fn go((row, column): (usize, usize), direction: &str) -> (usize, usize) {
        match direction {
            "up" => return (row - 1, column),
            "upright" => return (row - 1, column + 1),
            "right" => return (row, column + 1),
            "downright" => return (row + 1, column + 1),
            "down" => return (row + 1, column),
            "downleft" => return (row + 1, column - 1),
            "left" => return (row, column - 1),
            "upleft" => return (row - 1, column - 1),

            _ => panic!("Unexpected direction passed")
        }
    }

    

    pub fn get(&mut self, path: &str) {
        let file = File::open(path).expect("File not found");

        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let mut line = line.unwrap();

            if line.ends_with("\n") {
                if line.ends_with("\r") {
                    let _ = line.pop();
                }
                let _ = line.pop();
            }

            line.retain(|c| !c.is_whitespace());

            if line.len() > ARRAY_LENGTH {
                panic!("Wrong word search dimensions. Please set length to {}.", ARRAY_LENGTH);
            } else if index > ARRAY_HEIGHT {
                panic!("Input word search is too tall.");
            } else {
                for (jindex, character) in line.chars().enumerate() {
                    self.wordsearch[[index, jindex]] = character; 
                }
            }
        }
    }

    fn whatitdo(&self, CurrentWord, &str) {

    }

}

impl DictLookup for WordBlob {
    fn word_check(&self, word: &String) -> bool {
        self.dictionary.lexicon.contains(word)
    }
}

impl ArrayTraversal for WordBlob {
    fn traverse(&self, row: &usize, column: &usize, direction: &str) -> Option<((usize, usize), String)> {
        let mut gamertime = CurrentWord::new();

        gamertime.location = (*row, *column);
        gamertime.current_letter = *self.wordsearch.get(gamertime.location).unwrap();
        gamertime.last_state = self.letters.letter_test(&gamertime.current_letter);

        if gamertime.last_state == LastandSecondLast::None || gamertime.last_state == LastandSecondLast::Err {
            return None;
        } else {
            gamertime.letters.push(gamertime.current_letter);
        }
                
        loop {
            match self.wordsearch.get((currentrow, currentcolumn)) {
                Some(c) => {
                    current_letter = c;
                    
                    current_state = self.letters.letter_test(current_letter);

                    self.whatitdo()
    

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dictionary_checker() {
        let testdict = Dictionary::init("myDictsorted.txt");
        assert!(testdict.lexicon.contains("wantonly"));
        
    }
}