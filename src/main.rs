mod words;
use words::WORDS;
use rand::{self, Rng};
use std::io::{self};

fn main() {
    println!("Welcome to Wordle in the terminal!");

    let red:    &str = emojis::get("🟥").unwrap().as_str();   
    let orange: &str = emojis::get("🟧").unwrap().as_str();
    let green:  &str = emojis::get("🟩").unwrap().as_str();
    let white:  &str = emojis::get("⬜").unwrap().as_str();

    let mut victory: bool = false;
    let word_size: usize = 5;
    let total_guesses: usize = 6;
    let mut curr_guess: usize = 0; 

    let words_length: usize = WORDS.len() - 1;
    let word: &str = WORDS[rand::rng().random_range(0..words_length)];   

    let mut guesses: Vec<String> = Vec::new();
    let mut guesses_score: Vec<Vec<String>> = vec![vec!["-".to_string() + white; word_size]; total_guesses];
    
    
    while total_guesses != curr_guess {
        println!("Please guess a 5 letter word: ");
        
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect( "Failed to read line");
        guess.retain(|c: char| !c.is_whitespace());

        //if guess len != 5 OR guess is in previous guesses OR guess is NOT in WORDS
        if guess.len() != word_size                          {println!("--- Not a valid length."); continue;} 
        else if guesses.iter().any(|x: &String| x == &guess) {println!("--- Same as a previous guess."); continue;}
        else if !WORDS.iter().any(|y: &&str| y == &&guess)   {println!("--- Not a valid 5 letter word."); continue;}
        else {
            println!("     --- You guessed: {} ---", guess);
            guesses.push(guess.clone());

            for (i, char_guess) in guess.chars().enumerate() {
                let string_guess: String = char_guess.encode_utf8(&mut [0; 4]).to_string();
                let mut found: bool = false;

                for (j, char_word) in word.chars().enumerate() {
                    if char_guess == char_word {
                        found = !found;
                        if i == j  {guesses_score[curr_guess][i] = format!("{}{}", string_guess, green);}
                        else       {guesses_score[curr_guess][i] = format!("{}{}", string_guess, orange);}
                    }
                    else if !found {guesses_score[curr_guess][i] = format!("{}{}", string_guess, red)}
                }
            }
            curr_guess += 1;
            for res in guesses_score.clone() { println!("{:?}", res); }

            if guess == word {
                println!("     --- Congrats!!\n     --- The word was {}", guess);
                println!("     --- It took you {} tries!", curr_guess);
                curr_guess = total_guesses;
                victory = !victory;
                continue;
            }
        }
    }
    if !victory { println!(" ---Rip. The word was: {}---", word); }

}
