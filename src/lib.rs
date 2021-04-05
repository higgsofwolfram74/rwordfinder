use std::{collections::{HashMap, HashSet}, usize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use ndarray::Array2;

const LONGEST_WORD: usize = 31;
const ARRAY_HEIGHT: usize = 5;
const ARRAY_LENGTH: usize = 5;
//const HashMap<&str, fn> = ["up", "upright", "right", "downright", "down", "downleft", "left", "upleft"];


//pub struct Graph {
//    graph: [Node; ARRAY_HEIGHT * ARRAY_LENGTH]
//}
//
//impl Graph {
//    fn make(wordsearch: WordBlob) -> Graph {
//        
//        let mut reprgraph: Graph;
//        let mut arr: [Node; ARRAY_HEIGHT * ARRAY_LENGTH]; 
//        for ((y, x), point) in &wordsearch.wordsearch.indexed_iter() {
//            move_up(&wordsearch, x, y)
//        }
//        Graph
//    }
//}
//
//impl ArrayTraversal for Graph {
//    fn move_up(&self) -> Option<
//}
//pub struct Node {
//    node: char,
//    directions: [(&str, Option<Box<Node>>); 8],
//}

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



//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Vowel,
    Doublevowel,
    Y,
    None
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

trait DictLookup {
    fn word_check(&self) -> bool;
}


trait ArrayTraversal {
    fn traverse(&self, row: usize, column: usize, direction: &str) -> Option<char>;
}

pub struct WordBlob {
    wordsearch: Array2<char>,
    dictionary: HashSet<String>,
    letters: Letters
}

impl WordBlob {
    pub fn alloc() -> WordBlob {
        WordBlob {
            wordsearch: Array::from_elem((ARRAY_HEIGHT, ARRAY_LENGTH), '_')
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

    fn letter_append(letter: &char, &mut stack: String, letter_state: LastandSecondLast)

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

impl ArrayTraversal for WordBlob {
    fn traverse(&self, row: usize, column: usize, direction: &str) -> Option<Vec<String>> {
        let found = Vec::new();
        let stack = String::new();

        let currentrow = row;
        let currentcolumn = column;

        let mut current_letter = self.wordsearch.get(row, column).unwrap();
        let mut letter_state: LastandSecondLast = self.letters.letter_test(current_letter);
        let mut next_state: LastandSecondLast;

        if letter_state == LastandSecondLast::None {
            return None
        } else {
            stack.append(current_letter)
        }

        (currentrow, currentcolumn) = WordBlob::go((currentrow, currentcolumn), direction);

        match self.wordsearch.get((currentrow, currentcolumn)) {
            Some(c) => {
                current_letter = c;
                match self.letters.letter_test(current_letter) {
                    LastandSecondLast::Consonant => {
                        if letter_state != LastandSecondLast::Consonant && letter_state != LastandSecondLast::Doubleconsonant {
                            letter_state = LastandSecondLast::Consonant;
                            stack.append(current_letter)
                        } else if letter_state != LastandSecondLast::Doubleconsonant {
                            letter_state = LastandSecondLast::Doubleconsonant;
                            stack.append(current)
                        }
                    }
                    LastandSecondLast::Vowel =>
                    LastandSecondLast::Y =>
                    LastandSecondLast::Consonant =>
                }

                
                loop {
                    (currentrow, currentcolumn) = WordBlob::go((currentrow, currentcolumn), direction);
                    
                    match self.wordsearch.get(rowdex, column) {
                        Some(c) => {
                            stack.append(c);
                            
                            if stack.len() > 2 && stack.len() < LONGEST_WORD{
                                if self.word_check(stack) {
                                    found.append(stack);
                                }
                            } else if stack.len > 2 {
                                return found;                                
                            }                                                  
                        }

                        None => return None
                    }
                }
            }

            None => return None
        }
    }
}