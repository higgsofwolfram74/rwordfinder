use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


const LONGEST_WORD: usize = 31;



pub trait DictLookup {
    fn word_check(&self, word: &String) -> bool;
}


//holds the dictionary to use
pub struct Dictionary {
    lexicon: HashSet<String>
}


impl Dictionary {
    pub fn init(path: &str) -> Dictionary {
        

        let file = File::open(path).expect(format!("File Not Found at {}, please recheck your path.", path));

        let reader = BufReader::new(file);

        let read = reader.lines().map(|x| {x.unwrap()});
        
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
    final_word: String,
    last_state: LastandSecondLast,
}

impl CurrentWord {
    fn new() -> CurrentWord {
        CurrentWord {
            current_letter: '_',
            letters: String::new(),
            location: (0,0),
            final_word: (String::new()),
            last_state: LastandSecondLast::None,
        
        }
    }
}

//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
//update: a select few words have 3 consonants together
#[derive(PartialEq, Clone, Copy)]
pub enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Tripleconsonant,
    Vowel,
    Doublevowel,
    Triplevowel,
    Y,
    None
}

impl LastandSecondLast {
    //ugly code that takes our current letter state, compares it with the last code and returns the corresponding state
    //TODO: replace with match guards
    fn last_letter(current_letter: LastandSecondLast, last_letter: LastandSecondLast) -> LastandSecondLast {
        match current_letter {
            LastandSecondLast::Consonant => {
                
                match last_letter {    
                    LastandSecondLast::Consonant => LastandSecondLast::Doubleconsonant,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Tripleconsonant,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::None,
                    LastandSecondLast::Vowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Triplevowel => LastandSecondLast::Consonant,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    LastandSecondLast::None => panic!("We went too far we have to go back.")

                
                }
            }

            
            LastandSecondLast::Vowel => {
                
                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Vowel,
                    LastandSecondLast::Vowel => LastandSecondLast::Doublevowel,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Triplevowel,
                    LastandSecondLast::Triplevowel => LastandSecondLast::None,
                    LastandSecondLast::Y => LastandSecondLast::Consonant,
                    LastandSecondLast::None => panic!("We went too far we have to go back.")
                }    

            }
            LastandSecondLast::Y => {

                match last_letter {    

                    LastandSecondLast::Consonant => LastandSecondLast::Y,
                    LastandSecondLast::Doubleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Tripleconsonant => LastandSecondLast::Y,
                    LastandSecondLast::Vowel => LastandSecondLast::Y,
                    LastandSecondLast::Doublevowel => LastandSecondLast::Y,
                    LastandSecondLast::Triplevowel => LastandSecondLast::Y,
                    LastandSecondLast::Y => LastandSecondLast::None,
                    LastandSecondLast::None => panic!("We went too far we have to go back.")
                }
            }

            LastandSecondLast::None => LastandSecondLast::None,

            _ => panic!("That's not how the current letter works.")
        }
    }


}

const DIRECTIONS: [&'static str;  8] = ["Up", "Upleft", "Left", "Downleft", "Down", "Downright", "Right", "Upright"];


mod WordSearch {
    use super::*;
    
    pub struct WordBlob {
        wordsearch: Vec<char>,
        dictionary: Dictionary,
        letters: Letters
    }
    
    
    
    impl WordBlob {
        pub fn alloc(path_to_wordsearch: &str, path_to_dictionary: &str) -> WordBlob {
            WordBlob {
                wordsearch: WordBlob::get_wordsearch(path_to_wordsearch),
                dictionary: Dictionary::init(path_to_dictionary),
                letters: Letters::init()
            }
        }
    
        pub fn get_wordsearch(path_to_wordsearch: &str) -> Vec<char> {
            let wsearch: Vec<char> = Vec::new();
            let width: usize = 0;
    
            let file = File::open(path_to_wordsearch)
                .expect(format!("File Not Found at {}, please recheck your path.", path_to_wordsearch));
    
            let reader = BufReader::new(file);
    
            for line in reader.lines() {
                let line = line.unwrap();
    
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
    
            wsearch
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
    
    
        fn traverse(&self, word: &mut CurrentWord, direction: &str) -> Option<String> {
            loop {
                let ourword: String;
                let next = WordBlob::go(word.location, direction);
    
                word.current_letter = *self.wordsearch.get(next).unwrap();
    
                let current_state = self.letters.letter_test(&word.current_letter);
    
                word.last_state = LastandSecondLast::last_letter(current_state, word.last_state);
                
                
                match  word.last_state{
                    LastandSecondLast::None => {
                        if !(word.final_word.is_empty()) {
                            break Some(word.final_word.clone())
                        } else {
                            break None
                        }        
                    }
    
                    _ => {
                        word.letters.push(word.current_letter);
    
                        if word.letters.len() >= 3 {
                            if self.dictionary.word_check(&word.letters) {
                                word.final_word = word.letters.clone();
                            }
                        }
    
                        if word.letters.len() == LONGEST_WORD {
                            if !(word.final_word.is_empty()) {
                                break Some(word.final_word.clone())
                            } else {
                                break None
                            }
                        }
                    }
                }
    
                word.location = next;
    
            }
    
        }
    
        
    
        fn whatitdo(&self, row: usize, column: usize) -> Option<Vec<((usize, usize), String)>> {
            let mut gamertime = CurrentWord::new();
            let mut words: Vec<((usize, usize), String)> = Vec::new();
    
            gamertime.location = (row, column);
            gamertime.current_letter = *self.wordsearch.get(gamertime.location).unwrap();
            gamertime.last_state = self.letters.letter_test(&gamertime.current_letter);
    
            if gamertime.last_state == LastandSecondLast::None { 
                        return None;
            } else {
                gamertime.letters.push(gamertime.current_letter);
            }
    
            for direction in DIRECTIONS.into_iter() {
                self.traverse(&mut gamertime, direction);
    
                if !(gamertime.letters.is_empty()) {
                    words.push(((row, column), gamertime.final_word.clone()));
                }
            
            }
    
            if !(words.is_empty()) {
                Some(words)
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