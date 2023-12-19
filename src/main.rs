use std::io;
use std::collections::{HashMap,HashSet};

mod formulae_8;

// List of all characters allowed in a nerdle formula
pub static CHARLIST: [char; 14 ] = [ '0','1','2','3','4','5','6','7','8','9','+','-','*','/'];

// Length of the nerdle challenge
pub const FLENGTH: usize=8;


// A key part of the nerdle solver generates the list of all possible nerdle problems by first 
// generating a list of all strings that contain valid nerdle characters.
// CharWalker is the iterator that iterates over these strings. 
struct CharWalker {
    curr: u64,
    digits: usize,
    max: u64,
}

impl CharWalker {

    // return true if the resultnig string will have leading 0's or double signs. These are both
    // invalid in nerdle. e.g. 04*-7 is valid arithmetic but not not a valid nerdle guess,
    fn leading_zeros_double_signs(raw: &Vec<char>)->bool{
        let mut leading=true;
        for &c in raw{
            if leading && (c=='0' || c=='+' || c=='-' || c=='*' || c=='/') {
                return true;
            }else if c=='+' || c=='-' || c=='*' || c=='/'{
                leading=true;
            }
            else{
                leading=false;
            }
        }

        false
    }
}

// Iterate over possible nerdle strings
impl Iterator for CharWalker {

    type Item = Vec<char>;
   
    fn next(&mut self) -> Option<Self::Item>{
        if self.curr==self.max {
            None
        }else{
            let mut it=self.curr;
            let mut next_str=Vec::with_capacity(self.digits+5);
            for _ in 0..self.digits{
                next_str.push(CHARLIST[(it%14) as usize]);
                it/=14;
            }
            self.curr+=1;
            Some(next_str)
            }
        }
}

fn char_walker( d: usize) -> CharWalker {
    let fourteen:u64 = 14;
    CharWalker{ curr:0, digits: d,  max: fourteen.pow(d as u32) }
}

fn equations_size(length: usize)->HashSet<String>{

    let mut d = HashSet::<String>::new();

    // Generate all clues that have a single digit after the ='s sign
    let e=scan_size(length, 0.0_f64, 10.0_f64 );
    d.extend(e);

    // Generate all clues that have double digit after the ='s sign
    let e=scan_size(length, 10.0_f64, 100.0_f64 );
    d.extend(e);
    
    // triple digits
    let e=scan_size(length, 100.0_f64, 1000.0_f64 );
    d.extend(e);

    d

}

fn scan_size(length: usize, start_range: f64, end_range: f64)->Vec<String> {

    println!("Creating {}", length);

    let nerdle_string = format!("={}",start_range);
    let rhs_len=length-nerdle_string.len();

    char_walker(rhs_len)
        .filter(|cw| !CharWalker::leading_zeros_double_signs(cw))
        .map(|cw| cw.into_iter().collect::<String>())
        .map(|s| {let e=equation::evaluate(&s); (s, e)})
        .filter( |(_s,e)| e.is_ok())
        .map( |(s,e)| (s, e.unwrap()))
        .filter( |(_s,f)| f>=&start_range && f<&end_range && f==&f64::floor(*f))
        //.inspect( |(s,f)| println!("> {} {}",s,f))
        .map( |(s,f)| {let mut s2: String=s.clone(); s2.push_str(&format!("={}",f)); s2})
        .collect::<Vec<String>>()
}

// Compare two words, and return how good the guess is relative to the goal.
// Output is a string of five letters
// ' ' means the guess is not in the goal word
// 'Y' means the guess letter is in the goal word, but not in the right location.
// 'G' means the guess letter is in the right location in the goal word.
fn compare_words(goal: &str, guess: &str) -> String {
    let mut s: [char; FLENGTH] = [' '; FLENGTH];

    let mut goal_chars: Vec<char> = goal.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    // First pass. Mark the correct letters with a 'G'
    for i in 0..FLENGTH {
        if goal_chars[i] == guess_chars[i] {
            s[i] = 'G';
            goal_chars[i] = ' ' // Clear out this character so we don't match it again.
        }
    }

    // Second pass... Mark the guess letters that exist in the goal word but not in the right spot
    // as 'Y'
    for i in 0..FLENGTH {
        if s[i] == ' ' {
            let found = goal_chars.iter().enumerate().find_map(|(j, c)| {
                if *c == guess_chars[i] {
                    Some(j)
                } else {
                    None
                }
            });
            if let Some(j) = found {
                s[i] = 'Y';
                goal_chars[j] = ' ';
            }
        }
    }

    s.iter().collect()
}


   fn score(all_words: &HashSet<String>, word_set: &HashSet<String>) -> String {
        let word_set_count: f64 = word_set.len() as f64;
        println!("Word set count: {}", word_set_count);

        let mut max_score: f64 = 0.0;
        let mut max_fscore: f64 = 0.0;
        let mut max_word = String::from("");
        let min_of_max: usize = 10000;
        let great: f64 = 6.72;

        for possible_guess in all_words {
            // Calculate the clue sets and their size.
            let counted = &word_set
                .iter()
                .fold(HashMap::new(), |mut acc, possible_goal| {
                    *acc.entry(compare_words(possible_goal, possible_guess))
                        .or_insert(0) += 1;
                    acc
                });

            // Given the clue set, Calculate the Shannon entropy.
            let fscore: f64 = counted
                .iter()
                .map(|(_key, value)| {
                    let v_c: f64 = f64::from(*value);
                    let f = word_set_count / v_c;
                    v_c * f.ln()
                })
                .sum::<f64>()
                / word_set_count;

            // Given a clue set, calculate it's size
            let score: f64 = counted.len() as f64;

            // Given a cluse set, calculate the maximum clue size.
            let _max_clue_size = counted
                .iter()
                .map(|(_key, value)| value)
                .max()
                .ok_or(Some(0))
                .unwrap();

            if fscore == max_fscore && score > max_score {
                max_word = possible_guess.to_string();
                max_score = score;
                max_fscore = fscore;
                println!("Improving to {} {} {}", max_word, max_fscore, max_score);
            } else if fscore>max_fscore || 
                (fscore == max_fscore
                && score == max_score
                && word_set.contains(possible_guess))
            {
                // prefer possible solutions
                max_score = score;
                max_fscore = fscore;
                max_word = possible_guess.to_string();
                println!("Improving to {} {} {}", max_word, max_fscore, max_score);
            }

            if fscore >= great {
                println!("{} {} is also great", possible_guess, fscore);
            }

        }

        println!(
            "Guess... {}, {} {} {}",
            max_word, max_score, max_fscore, min_of_max
        );
        max_word
    }

    fn play_nerdle(all_word_set: &HashSet<String>) {

        let mut word_set = all_word_set.clone();

        //let mut recommend = score(all_word_set, &word_set);
        //println!("Guess: {}", recommend);
        println!("Guess 48-32=16");

        let mut recommend = String::from("48-32=16");

        while !word_set.is_empty() {

            let mut clue = String::new();

            println!("Enter clue...");
            io::stdin()
                .read_line(&mut clue)
                .expect("Failed to read line");

            remove(&mut word_set, &recommend, &clue);

            let count = word_set.len();
            println!("{} possible words", count);

            if count < 300 {
                for word in &word_set {
                    print!("{} ", word);
                }
                println!(" ");
            }
            
            recommend = score(all_word_set, &word_set);
            println!("Guess: {}", recommend);
        }
    }

    fn remove(word_set: &mut HashSet<String>, guess: &str, clue: &str) {
        let guess_chars: Vec<char> = guess.chars().collect();
        let clue_chars: Vec<char> = clue.chars().collect();
        let mut remove_set = HashSet::new();

        for word in word_set.iter() {
            let mut word_chars: Vec<char> = word.chars().collect();
            let mut remove = false;

            for i in 0..FLENGTH {
                if clue_chars[i] == 'G' {
                    if guess_chars[i] == word_chars[i] {
                        word_chars[i] = ' '; // Don't match this letter again
                    } else {
                        // Remove words where the clue is green, but the letters don't match
                        remove = true;
                    }
                }

                if remove {
                    break;
                }
            }
            if !remove {
                for i in 0..FLENGTH {
                    if clue_chars[i] == 'Y' {
                        if guess_chars[i] == word_chars[i] {
                            // This should have been a 'G'
                            remove = true;
                            break;
                        }

                        // If the clue is Y then search for that letter.
                        // For Y, valid matches only happen when the match is not in the same position.
                        let found = word_chars.iter().enumerate().find_map(|(j, c)| {
                            if *c == guess_chars[i] {
                                Some(j)
                            } else {
                                None
                            }
                        });

                        if let Some(j) = found {
                            if j != i {
                                word_chars[j] = ' '; // Don't match this letter again.
                            } else {
                                remove = true; // This clue should have been 'G'
                            }
                        } else {
                            remove = true; // Didn't find the matching letter.
                        }
                    }

                    if remove {
                        break;
                    }
                }
            }

            if !remove {
                for i in 0..FLENGTH {
                    if clue_chars[i] == ' ' {
                        // If the clue is ' ' then that guess letter must not exist in the target.
                        let found = word_chars.iter().enumerate().find_map(|(j, c)| {
                            if *c == guess_chars[i] {
                                Some(j)
                            } else {
                                None
                            }
                        });

                        if let Some(_j) = found {
                            remove = true;
                        }

                        if remove {
                            break;
                        }
                    }
                }
            }

            if remove {
                remove_set.insert(word.clone());
            }
        }

        for removeable in &remove_set {
            word_set.remove(removeable);
        }
    }

fn count_chars() {
    let counted = formulae_8::FORMULAE 
        .iter()
        .flat_map(|w| w.chars())
        .fold(HashMap::with_capacity(128), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let mut count_vec = counted.iter().collect::<Vec<(&char, &i32)>>();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    count_vec.iter().for_each(|(c, x)| println!("{}:{}", c, x));
}



fn main() {

    let args: Vec<String> = std::env::args().collect();


    if args[1] == *"calc"{
        let e = equation::evaluate(&args[2]).unwrap();

        println!("{} => {:?}", args[2], e);

    }
 
    if args[1] == *"count" {
        count_chars();
    }

    if args[1] == *"play" {
        //let d = equations_size(FLENGTH);
        
        let d:  HashSet::<String> = formulae_8::FORMULAE.iter().map(|&s| String::from(s)).collect();


        play_nerdle(&d);
    }

    if args[1] == *"rust" {
        let formulae = equations_size(FLENGTH);
        println!("//Set contains {} formaulae", formulae.len());
        println!("pub static FORMULAE: [&str;{}]=[", formulae.len());

        let s = formulae.iter()
                    .enumerate()
                    .map(|(c,s)| format!("\"{}\"{}",s, if c%8==7 {",\n    "} else {","}))
                    .collect::<Vec<String>>()
                    .join("");

        println!("{}",s);
        println!("];");
    }
}
