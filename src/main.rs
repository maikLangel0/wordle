mod words;
use words::WORDS;
use rand::Rng;

const WORD_SIZE: usize = 5;
const TOTAL_GUESSES: usize = 7;

fn main() {
    println!("Welcome to Wordle in the terminal!");

    let red:    &str = emojis::get("🟥").expect("Noooo my emoticon redd :(").as_str();   
    let orange: &str = emojis::get("🟧").expect("Noooo my emoticon appelsin :(").as_str();
    let green:  &str = emojis::get("🟩").expect("Noooo my emoticon grøn :(").as_str();
    let white:  &str = emojis::get("⬜").expect("Noooo my emoticon vit :(").as_str();

    let mut victory: bool = false;
    let mut curr_guess: usize = 0; 

    let word: &str = WORDS[rand::rng().random_range(0..WORDS.len() - 1)];   

    let mut guesses: Vec<String> = Vec::new();
    let mut guesses_score: Vec<Vec<String>> = vec![vec!["-".to_string() + white; WORD_SIZE]; TOTAL_GUESSES];
    
    
    while TOTAL_GUESSES != curr_guess {
        println!("Please guess a 5 letter word: ");
        
        let mut guess: String = String::new();
        std::io::stdin().read_line(&mut guess).expect( "Failed to read line");
        guess.retain(|c: char| !c.is_whitespace());

        //if guess len != 5 OR guess is in previous guesses OR guess is NOT in WORDS
        if guess.len() != WORD_SIZE                          {println!("--- Not a valid length."); continue;} 
        else if guesses.iter().any(|x| *x == guess) {println!("--- Same as a previous guess."); continue;}
        else if !WORDS.iter().any(|y| *y == &guess)   {println!("--- Not a valid 5 letter word."); continue;}
        else {
            println!("     --- You guessed: {} ---", guess);
            guesses.push(guess.clone());

            for (i, char_guess) in guess.chars().enumerate() {
                let mut found: bool = false;

                for (j, char_word) in word.chars().enumerate() {
                    if char_guess == char_word {
                        found = !found;
                        if i == j  {guesses_score[curr_guess][i] = format!("{}{}", char_guess, green);}
                        else       {guesses_score[curr_guess][i] = format!("{}{}", char_guess, orange);}
                    }
                    else if !found {guesses_score[curr_guess][i] = format!("{}{}", char_guess, red)}
                }
            }
            curr_guess += 1;
            for res in &guesses_score { println!("{:?}", res); }

            if guess == word {
                println!("     --- Congrats!!\n     --- The word was {}", guess);
                println!("     --- It took you {} tries!", curr_guess);
                curr_guess = TOTAL_GUESSES;
                victory = !victory;
                continue;
            }
        }
    }
    if !victory { println!(" ---Rip. The word was: {}---", word); }

}
