use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use ndarray::prelude::*;

const LONGEST_WORD = 31;
const ARRAY_HEIGHT = 5;
const ARRAY_LENGTH = 5;


//holds the dictionary to use
struct WordList {
    words: HashSet<String>
}

impl WordList {
    fn init(path: &str) -> WordList {
        let mut wordlist = WordList {
            words: HashSet::new(),
        };

        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);


        for line in reader.lines() {
            let line = line.unwrap();

            wordlist.words.insert(line);
        }

        wordlist
    }



//See what the last letter was. Most words don't have more than 2 of a letter type sequentially
enum LastandSecondLast {
    Consonant,
    Doubleconsonant,
    Vowel,
    Doublevowel,
    Y
}


struct Letters {
    cons: HashSet<char>,
    vows: HashSet<char>,
    //Y has characteristics of both so it will be used instead
    Y: 'y',
}

impl Letters {
    fn init() -> Letters {

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
        }
    }
}

trait DictLookup {
    fn word_check(&self) -> bool;
}

struct WordBlob {
    wordsearch: Array2<char>
}

impl WordBlob {
    fn new() -> WordBlob {
        WordBlob {
            wordsearch: Array::from_elem((ARRAY_HEIGHT, ARRAY_LENGTH), 'a')
        }
    }

    fn get(&self, path)
} 







fn main() {
    let wordlist = WordList::init();


}
