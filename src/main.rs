use eval::{eval };
use std::collections::HashSet;

pub static CHARLIST: [char; 14 ] = [ '0','1','2','3','4','5','6','7','8','9','+','-','*','/'];
pub const FLENGTH: usize=5;

struct CharWalker {
    curr: u64,
    digits: usize,
    max: u64,
}

impl CharWalker {

    fn leading_or_lone_zeros(next_str:&String)->bool{
        let mut at_lead=true;
        for c in next_str.chars(){
            if at_lead && c=='0' {
                return true;
            }else if c=='+' || c=='-' || c=='*' || c=='/'{
                at_lead=true;
            }
            else{
                at_lead=false;
            }
        }
        false
    }
}

impl Iterator for CharWalker {

    type Item = String;
   
    fn next(&mut self) -> Option<Self::Item>{
        loop{
            if self.curr==self.max {
                return None
            }else{
                let mut it=self.curr;
                let mut next_str=String::with_capacity(self.digits+5);
                for _ in 0..self.digits{
                    next_str.push(CHARLIST[(it%14) as usize]);
                    it=it/14;
                }
                self.curr+=1;
                if CharWalker::leading_or_lone_zeros(&next_str){
                    continue;
                }else{
                   return Some(next_str)
                }
            }
        }
    }
}

fn char_walker( d: usize) -> CharWalker {
    let fourteen:u64 = 14;
    CharWalker{ curr:0, digits: d,  max: fourteen.pow(d as u32) }
}

fn equations_size(length: usize)->HashSet<String>{

    let mut d = HashSet::<String>::new();

    for i in 0..10{
        let e=scan_size(length-2,&format!("={}",i));
        d.extend(e)
    }

    for i in 10..100{
        let e=scan_size(length-3,&format!("={}",i));
        d.extend(e)
    }

    for i in 100..1000{
        let e=scan_size(length-4,&format!("={}",i));
        d.extend(e)
    }

    d

}

fn scan_size(length: usize, equals: &str)->Vec<String> {

    println!("Creating {} {}", length, equals);

    let mut json_string = String::from('=');
    json_string.push_str(equals);

    let functions = char_walker(length)
        .map(|cw| {let mut s=String::from(cw.clone()); s.push_str(&json_string); (cw,s)})
        .map(|(cw,s)| {let e=eval(&s); (cw,s,e)})
        .filter(|(_cw, _s, e)| e.is_ok())
        .map(|(cw,s,e)| (cw,s, e.unwrap()))
        .filter(|(_cw,_s,e)| e.eq(&true))
        .map(|(cw,_s,_e)| cw)
        .map(|cw| {let mut s=String::from(cw); s.push_str(equals); s})
        .collect::<Vec<_>>();

    functions
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


fn main() {


    let d = equations_size(6);
    for s in d{
        println!("f..>{}<",s);
    }


    //let my_args: Vec<String> = env::args().collect();

    //dbg!(my_args);

    //let query = &my_args[1];
    //let e=eval(query);

    //match e {
    //    Ok(v) => println!("Value : {v:?}"),
    //    Err(e) => println!("Error: {e:?}"),

    //}


}
