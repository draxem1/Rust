//Given a list of integers give the Median (middle integer(s) after being sorted)
//And give the Mode (The value that occurs most)
//Date Created: 3/10/22 at 7:35 PM

use std::io;
use std::collections::HashMap;

fn main() {

    println!("Enter a list of numbers");
    let mut v = get_numbers();

    v.sort();
    let mid = (v.len()/2) - 1;

    if v.len() % 2 == 0 {
        println!("Medians: {} and {}", &v[mid], &v[mid+1]);
    }
    else {
        println!("Median: {}", &v[mid + 1]);
    }
}

fn get_numbers() -> Vec<i32> {
    let mut numbers = String::new();
    let mut map = HashMap::new();
    let mut v: Vec<i32> = Vec::new();

    io::stdin()
    .read_line(&mut numbers).expect("Failed");

    for i in numbers.split_whitespace() {

       let val = match i.trim().parse() {       //checks for valid numbers
            Ok(num) => num,
            Err(_) => continue,
        };

        let count = map.entry(val).or_insert(0);        //organizes hashmap by value occurences
        *count += 1;
    }

    let mut most: i32 = 0;
    let mut tmp: i32 = 0;

    for (key, value) in &map {              //removes multiple occurences than pushes value to vector

        if tmp < *value && *value > 1{
            tmp = *value;
            most = *key;
        }
        v.push(*key);
    }

    if most <= 1 {
        println!("Mode: all values had eqaul occurences.");
    }
    else {
        println!("Mode: {}", most);
    }

    v
}
