//Generates a random pass word
//Date Created: 3/19/2022 Time: 10:12 AM
use std::io;
use rand::Rng;

struct Elements {
    alphabet: [char;52],
    numbers: [char;10],
    special: [char;7],
}

#[allow(unused_variables)]
fn main() {
    println!("\nThis is a password generator press (Enter) for new password!!");
    let user = get_input();

    loop{

        if user.contains("\n"){
            let password_length: u32 = rand::thread_rng().gen_range(5..36);
            let mut password = String::new();

            for n in 1..password_length{
                password.push(random_character());
            }

            println!("Password Created: {}", password);
        }
        else {
            continue;
        }

        println!("Would you like a different password?? Y/N?");
        let yon = get_input();

        match yon.as_str().trim(){
            "y" | "Y" => continue,
            _ => break,
        }
    }

    println!("Good bye!!!");
}

fn get_input() -> String{
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line).expect("Failed to read line");
    line
}

fn random_character() -> char {
    let selector: u32 = rand::thread_rng().gen_range(0..3);

    let elements = Elements {
        alphabet: ['a','b','c','d','e','f','g','h','i','j','k','l','m','n',
                    'o','p','q','r','s','t','u','v','w','x','y','z', 'A','B',
                    'C','D','E','F','G','H','I','J','K','L','O','M','N','P','Q','R'
                    ,'S','T','U','V','W','X','Y','Z'],

        numbers: ['1','2','3','4','5','6','7','8','9','0'],

        special: ['!','@','$','%','&','=','?'],
    };
    let end_range;

    match selector {
        0 => end_range = 52,
        1 => end_range = 10,
        2 => end_range = 7,
        _ => panic!("Index out of bounds!!!"),
    }

    let ind_of_char = rand::thread_rng().gen_range(0..end_range);
    let character;

    match selector {
        0 => character = &elements.alphabet[ind_of_char],
        1 => character = &elements.numbers[ind_of_char],
        2 => character = &elements.special[ind_of_char],
        _ => panic!("Out of characters"),
    }

    *character
}
