use std::{collections::HashSet, usize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use ndarray::prelude::*;

const LONGEST_WORD: usize = 31;
const ARRAY_HEIGHT: usize = 5;
const ARRAY_LENGTH: usize = 5;


//holds the dictionary to use
pub struct WordList {
    words: HashSet<String>
}

impl WordList {
    pub fn init(path: &str) -> WordList {
        let mut wordlist = WordList {
            words: HashSet::new(),
        };

        let file = File::open(path).expect("File Not Found");

        let reader = BufReader::new(file);


        for line in reader.lines() {
            let line = line.unwrap();

            wordlist.words.insert(line);
        }

        wordlist
    }



//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Vowel,
    Doublevowel,
    Y
}

pub enum Direction {
    Up,
    Upright,
    Right,
    Downright,
    Down,
    Downleft,
    Left,
    Upleft
}


pub struct Letters {
    cons: HashSet<char>,
    vows: HashSet<char>,
    //Y has characteristics of both so it will be used instead
    y: char,
}

impl Letters {
    pub fn init() -> Letters {

        let consonant: HashSet<char> = vec!('b', 'c', 'd', 'f', 'g', 'h',
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
            y: char
        }
    }
}

trait DictLookup {
    fn word_check(&self) -> bool;
}

trait ArrayTraversal {
    fn move_up(&self) -> char;

    fn move_upright(&self) -> char;
    
    fn move_right(&self) -> char;

    fn move_downright(&self) -> char;

    fn move_down(&self) -> char;

    fn move_downleft(&self) -> char;

    fn move_left(&self) -> char;

    fn move_upleft(&self) -> char;
}

pub struct WordBlob {
    wordsearch: Array2<char>
}

impl WordBlob {
    pub fn new() -> WordBlob {
        WordBlob {
            wordsearch: Array::from_elem((ARRAY_HEIGHT, ARRAY_LENGTH), '_')
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