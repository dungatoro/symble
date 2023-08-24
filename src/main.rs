use std::fs::File;
use std::io::{Write, Result, BufRead, BufReader};
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Symble {
    answer: String,
    guesses: Vec<String>,
    results: Vec<String>,
    key: (char, char, char),
    turn: usize,
}

impl Symble {
    fn new(answers: &Vec<String>) -> Self {
        let answer = answers.choose(&mut thread_rng()).unwrap().clone();
        let mut codes = ['!', '@', '#', '$',  '%', '^', '&', '*'];
        codes.shuffle(&mut thread_rng());

        Self {
            answer,
            guesses: Vec::new(),
            results: Vec::new(),
            key: (codes[0], codes[1], codes[2]),
            turn: 0
        }

    }

    fn take_turn(&mut self, valid: &Vec<String>) {
        let guess = get_user_input(valid);
        self.guesses.push(guess);
        self.encrypt();

        self.turn += 1;

    }

    fn encrypt(&mut self) {
        let ans: Vec<char> = self.answer.chars().collect();
        let guess: Vec<char> = self.guesses.last().unwrap().chars().collect();
        let mut marked = [false;5];
        let mut result = [self.key.0;5];

        for i in 0..5 {
            if ans[i] == guess[i] {
                marked[i] = true;
                result[i] = self.key.2;
            }
        }

        ans.iter()
            .enumerate()
            .for_each(|(idx, letter)| {
                if !marked[idx] {
                    let pos = guess.iter()
                        .position(|c| c == letter);
                    match pos {
                        None => (),
                        Some(_) => {
                            marked[idx] = true;
                            result[idx] = self.key.1;
                        }
                    }
                }
            }
            );

        self.results.push(result.iter().collect());
    }

    fn draw(&self) {
        for i in 0..self.turn {
            println!(" {} | {}", self.results[i], self.guesses[i]);
        }

        for _ in self.turn..6 {
            println!(" ----- | -----");
        }
    }
}

fn get_user_input(valid: &Vec<String>) -> String {
    let mut input: String;
    loop {
        print!("\n Enter guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input.pop();
        if valid.contains(&input) {
            break;
        } else {
            println!("Not a word");
        }
        println!();
    }

    input
}

fn main() -> Result<()> {
    let file = File::open("answers.txt")?;
    let reader = BufReader::new(file);
    let answers: Vec<String> = reader.lines().collect::<Result<_>>()?;

    let file = File::open("guesses.txt")?;
    let reader = BufReader::new(file);
    let guesses: Vec<String> = reader.lines().collect::<Result<_>>()?;


    let mut game = Symble::new(&answers);
    while game.turn < 6 {
        game.take_turn(&guesses);
        game.draw();
        if *game.guesses.last().unwrap() == game.answer {
            println!("Correct! {} is the answer.", game.answer);
            return Ok(())
        }
    }

    println!("Unlucky. The answer was {}.", game.answer);
    Ok(())

}
