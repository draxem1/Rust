//Pig latin takes the first letter of a word and puts it at the end with "ay" 
//Unless the word starts with a vowel then it puts "hay" at the end.
//Ex1: first = irst-fay
//Ex2: apple = apple-hay
//Dated Created: 3/14/2022 Time: 9:17 AM

use std::io;

fn main() {

    println!("\nEnter a string you would like converted to pig latin..");

    let mut user = String::new();

    io::stdin()
        .read_line(&mut user).expect("Failed to read line");

    let mut words = Vec::new();

    for i in user.split_whitespace(){
        let latin = convert_pig(i);
        words.push(latin);
    }

    println!("\nIn latin is");

    for i in words{
        print!("{} ", i);
    }
}

fn convert_pig(word: &str) -> String {
    let vowels = ["A","a","E","e","I","i","O","o","U","u"];
    let latin = ["ay", "hay"];

    let first = &word[0..1];
    let mut count = 0;

    for i in vowels{
        if i == first {
            count = 1;
        }
    }

    let converted;
    match count {
        1 => converted = format!("{}-{}", &word, &latin[1]),
        _ => converted = format!("{}-{}{}",&word[1..word.len()], first, &latin[0]),
    }
    converted
}

