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
    fn traverse(&self, row: usize, column: usize, direction: &str) -> Option<char>;
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
        
        let mut wordlist = Dictionary {
            lexicon: read.collect(),
        };
        
        wordlist
    }
}


pub struct Letters {
    consonants: HashSet<char>,
    vowels: HashSet<char>,
    //Y has characteristics of both so it will be used instead
    y: char,
}

impl Letters {
    pub fn init() -> Letters {

        let cons: HashSet<char> = vec!('b', 'c', 'd', 'f', 'g', 'h',
                           'j', 'k', 'l', 'm', 'n', 'p',
                           'q', 'r', 's', 't', 'v', 'w', 
                           'x', 'z').iter().collect();


        let vowel = vec!('a', 'e', 'i', 'o', 'u').iter().collect();

        for letter in letters {
            vo.insert(letter);
        }

        Letters {
            cons: co,
            vows: vo,
            y: char,
        }
    }

    pub fn letter_test(&self, letter: &char) -> LastandSecondLast {
        if self.consonant.contains(letter) {
            return LastandSecondLast::Consonant;
        } else if self.vowels.contains(letter) {
            return LastandSecondLast::Vowel;
        } else if letter == 'y' {
            return LastandSecondLast::Y;
        } else {
            return LastandSecondLast::None;
        }
    }
}


//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
//update: a select few words have 3 consonants together
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Tripleconsonant,
    Vowel,
    Doublevowel,
    Y,
    None
}

impl LastandSecondLast {
    //ugly code that takes our current letter state, compares it with the last code and returns the corresponding state
    fn last_letter(current_letter: LastandSecondLast, last_letter: LastandSecondLast) -> LastandSecondLast {
        match current_letter {
            LastandSecondLast::Consonant => {
                
                match last_letter {    
                    LastandSecondLast::Consonant => LastandSecondLast::Doubleconsonant,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Tripleconsonant,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::None,
                    LastandSecondLast::Vowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    LastandSecondLast::None => panic!("Invariant has not been upheld.")
                
                }
            }

            LastandSecondLast::Doubleconsonant => panic!("How?"),
            LastandSecondLast::Tripleconsonant => panic!("How?"),
            
            LastandSecondLast::Vowel => {
                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Vowel => LastandSecondLast::Doublevowel,
                    LastandSecondLast::Doublevowel => LastandSecondLast::None,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    LastandSecondLast::None => panic!("Invariant has not been upheld.")
                                       

            }
            LastandSecondLast::Doublevowel => panic!("How?"),
            LastandSecondLast::Y => {

                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Y,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Vowel => LastandSecondLast::Y,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Y,
                    LastandSecondLast::Y => LastandSecondLast::None,
                    LastandSecondLast::None => panic!("Invariant has not been upheld.")

            }
            LastandSecondLast::None => LastandSecondLast::None
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

        let reader = Bufreader::new(file);

        for (index, line) in reader.lines().enumerate() {
            if line.endswith("\n") {
                if line.endswith("\r") {
                    line.pop()
                }
                line.pop()
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

}

impl DictLookup for WordBlob {
    fn word_check(&self, word: &str) -> bool {
        self.dictionary.lexicon.contains(word)
    }
}

impl ArrayTraversal for WordBlob {
    fn traverse(&self, row: &usize, column: &usize, direction: &str) -> Option<Vec<String>> {
        let mut found = Vec::new();
        let mut stack = String::new();
        let mut current_state: LastandSecondLast;


        let mut currentrow = row;
        let mut currentcolumn = column;

        let mut current_letter = self.wordsearch.get(row, column).unwrap();
        let mut last_state: LastandSecondLast = self.letters.letter_test(current_letter);
        
        if letter_state == LastandSecondLast::None {
            return None
        } else {
            stack.append(c)
        }

        (currentrow, currentcolumn) = WordBlob::go((currentrow, currentcolumn), direction);
        
        loop {
            match self.wordsearch.get((currentrow, currentcolumn)) {
                Some(c) => {
                    current_letter = c;
                    
                    current_state = self.letters.letter_test(current_letter);
    
                    match LastandSecondLast::last_letter(current_state, last_state) {
                        LastandSecondLast::None => {
                            if !(found.is_empty()) {
                                return Some(found)
                            } else {
                                return None;
                            }
                        }

                        _ => {
                            stack.append(c);

                            if stack.len() >= 3 {
                                
                                if self.word_check(stack) {
                                    found.append(((row, column), stack));
                                }
                            }

                            if stack.len() > LONGEST_WORD {
                                if !(found.is_empty()) {
                                    return Some(found);
                                }
                            }
                            
                            (currentrow, currentcolumn) = WordBlob::go((currentrow, currentcolumn), direction);
                            last_state = current_state

                        }
                    }                   
                }
    
                None => None
            }            
        }
    }
}