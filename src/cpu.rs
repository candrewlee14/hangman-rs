use std::fs::File;
use std::io::{prelude::*, BufReader};
use rand::seq::IteratorRandom;
use std::path::Path;
use anyhow::{Result, Error};
/// This struct handles guesses and passes information back to the Hangman game client.
/// It's implemented in this fashion so that multiplayer would be secure.
/// Client never sees opponent's word, only passes guesses and responses back and forth.
pub struct HangmanCPU {
    word: String,
}
impl HangmanCPU {
    /// Initialize with given starting word.
    pub fn with_word(word: String) -> Self {
        Self { word }
    }
    /// Initialize with random starting word.
    pub fn randomize() -> Self {
        Self { word: Self::random_word().unwrap() }
    }
    /// Returns length of word
    pub fn get_word_len(&self) -> usize {
        self.word.len()
    }
    /// Get random word from wordlist file. Downloads the wordlist file if it doesn't exist.
    fn random_word() -> Result<String> {
        if !Path::new("wordlist.txt").exists() {
            println!("Downloading wordlist file...");
            let mut res = reqwest::blocking::get("https://raw.githubusercontent.com/InnovativeInventor/dict4schools/master/safedict_full.txt")?;
            let mut file = File::create("wordlist.txt")?;
            res.copy_to(&mut file)?;
        }
        let file = File::open("wordlist.txt")?;
        let reader = BufReader::new(file);
        let mut rng = rand::thread_rng();
        reader.lines()
            .choose(&mut rng)
            .unwrap_or(
                Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData, "Wordlist contains invalid data")))
            .map_err(|e| Error::new(e))
    }
    /// Returns vector of indices where this letter is found.
    /// Empty vector if guess is not in word
    pub fn respond_to_guess(&self, guess: char) -> Vec<usize> {
        let mut hits : Vec<usize> = Vec::new();
        for (i, c) in self.word.chars().enumerate() {
            if c == guess {
                hits.push(i);
            }
        }
        hits
    }
}

