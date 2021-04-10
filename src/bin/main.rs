use anyhow::{Result, Error};
use std::io::{Write, stdout, Stdout};
use crossterm::{ExecutableCommand, QueueableCommand, cursor::{self, MoveTo}, event::KeyCode, style::{self, Colorize, SetForegroundColor, Color}, terminal::{self, ClearType}};
use crossterm::event::{read, poll, Event};
use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};
use hangman_rs::cpu::HangmanCPU;
use std::process::exit;

pub struct HangmanGame {
	revealed_word: Vec<Option<char>>,
	misses: Vec<char>,
}
impl HangmanGame {
    pub fn new(word_len: usize) -> Self {
        Self {
            revealed_word: vec![None; word_len],
            misses: Vec::with_capacity(26),
        }
    }
    pub fn print_guess_response(&self, s: &mut Stdout) -> Result<()> {
        // Print revealed word
        s.execute(MoveTo(0, 11))?
        .execute(SetForegroundColor(Color::Green))?
        .execute(
            Print(
                self.revealed_word.iter()
                    .map(|c| c.unwrap_or(' '))
                    .collect::<String>()
            ))?;

        // Print misses
        s.execute(MoveTo(0, 15))?
        .execute(SetForegroundColor(Color::Red))?
        .execute(
            Print(
                self.misses.iter()
                    .map(|c| c.to_string() + " ")
                    .collect::<String>()
            ))?;
        Ok(())
    }
    pub fn handle_guess(&mut self, guess: char, cpu: &HangmanCPU){
       let response = cpu.respond_to_guess(guess);
       if response.len() == 0 {
           self.misses.push(guess);
           return;
       }
       for i in response.iter() {
           self.revealed_word[*i] = Some(guess);  
       }
    }
    pub fn receive_guess(&mut self, s: &mut Stdout) -> Result<char> {
        let mut guess : char = ' ';
        loop {
            match read()? {
                Event::Key(e) => { 
                    match e.code { 
                        KeyCode::Char(c) if c.is_alphabetic() && !(self.misses.contains(&c))=> {
                            guess = c;
                            s.execute(MoveTo(12, 17))?
                                .execute(SetForegroundColor(Color::White))?
                                .execute(Print(guess))?
                                .execute(MoveTo(0,18))?
                                .execute(SetForegroundColor(Color::DarkGrey))?
                                .execute(Print("Enter to Confirm"))?;
                        },
                        KeyCode::Enter if guess.is_alphabetic() => {
                            s.execute(MoveTo(12, 17))?
                                .execute(Print(" "))?
                                .execute(MoveTo(0,18))?
                                .execute(Print(" ".repeat(17)))?;
                            return Ok(guess);
                        },
                        KeyCode::Esc => exit(1),
                        _ => (),
                    }                   
                }
                _ => (),
            }
        }
    }
    pub fn has_won(&self) -> bool {
        self.revealed_word.iter().flatten().count() == self.revealed_word.len()
    }
	pub fn print_base(&self, s: &mut Stdout)-> Result<()> {
		s.queue(MoveTo(0, 0))?
			.queue(SetForegroundColor(Color::White))?
            .queue(Print("-----------"))?
            .queue(MoveTo(0, 1))?
            .queue(Print("| HANGMAN |"))?
            .queue(MoveTo(0, 2))?
            .queue(Print("-----------"))?
            .queue(MoveTo(0, 4))?
            .queue(SetForegroundColor(Color::Blue))?
            .queue(Print(" |------|"))?
            .queue(MoveTo(0, 5))?
            .queue(Print(" |      |"))?
            .queue(MoveTo(0, 6))?
            .queue(Print(" |"))?
            .queue(MoveTo(0, 7))?
            .queue(Print(" |"))?
            .queue(MoveTo(0, 8))?
            .queue(Print(" |"))?
            .queue(MoveTo(0, 9))?
            .queue(Print("_|__"))?
            .queue(MoveTo(0, 12))?
			.queue(SetForegroundColor(Color::White))?
            .queue(Print("-".repeat(self.revealed_word.len())))?
            .queue(MoveTo(0, 14))?
            .queue(Print("Wrong Letters:"))?
            .queue(MoveTo(0, 17))?
            .queue(Print("Next Guess:"))?
            .queue(MoveTo(11, 17))?
            ;
        s.flush()?;
        Ok(())
	}
}


fn run() -> Result<()> {
	let mut stdout = stdout();
	stdout.execute(terminal::EnterAlternateScreen)?;
	stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;
    let mut cpu = HangmanCPU::randomize();
    let mut game = HangmanGame::new(cpu.get_word_len());
    game.print_base(&mut stdout)?;
    while !game.has_won() { 
        let guess = game.receive_guess(&mut stdout)?;
        game.handle_guess(guess, &cpu);
        game.print_guess_response(&mut stdout)?;
    }
    stdout.execute(MoveTo(0, 19))?
        .execute(SetForegroundColor(Color::Green))?
        .execute(Print("WIN"))?;
    match read() {
        _ => ()
    };
	Ok(())
}

fn main() {
	match run() {
		Ok(_) => (),
		Err(e) => println!("Error: {}", e)	
	};
}
