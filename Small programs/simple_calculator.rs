//Simple calculator built to practice rust structs and matching
//nothing special
//Date Created: 3/8/2022 11:22 AM
use std::io;

#[derive(Debug)]
struct Calculator {
    number1: f64,
    number2: f64,
    sign: char,
}

impl Calculator {
    fn add(&self) -> f64 {
        self.number2 + self.number1
    }

    fn subtract(&self) -> f64 {
        self.number1 - self.number2
    }

    fn divide(&self) -> f64 {
        self.number1 / self.number2
    }

    fn multiply(&self) -> f64 {
        self.number1 * self.number2
    }

    fn remainder(&self) -> f64 {
        self.number1 % self.number2
    }
}

fn main() {

    intro();

    let mut ans = String::from("n");

while ans == "n" || ans == "N" {

    println!("=>");
    let expression = build_calculator();
    let mut result = 0.0;
    
    match expression.sign {
        '*' => result = expression.multiply(),
        '/' => result = expression.divide(),
        '+' => result = expression.add(),
        '-' => result = expression.subtract(),
        '%' => result = expression.remainder(),
        _ => println!("not a valid expression"),
    }

    println!("\nResult = {}\n", result);

    println!("Quit? Y/N..");
    ans = get_input();
}
}

fn get_input() -> String{

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    input.trim().parse().expect("Failed")
}

fn convert_to_number(number: String) -> f64 {
    let number: f64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Not a valid number.. Set to 0!!"); 0.0},
    };
    number
}

fn build_calculator() -> Calculator{
    let input = get_input();
    let mut num1 = String::new();
    let mut num2 = String::new();

    let mut expression = Calculator {
        number1: 0.0,
        number2: 0.0,
        sign: ' ',
    };

    for element in input.chars(){

        match element{
            '*' => expression.sign = '*',
            '/' => expression.sign = '/',
            '%' => expression.sign = '%',
            '+' => expression.sign = '+',
            '-' => expression.sign = '-',
            _ => if expression.sign == ' '{
                num1.push_str(&String::from(element));
            }
            else {
                num2.push_str(&String::from(element));
            },
        }
    }

    expression.number1 = convert_to_number(num1);
    expression.number2 = convert_to_number(num2);
    expression
}

fn intro() {
    println!("\n\tThis is a simple calculator. You can ADD(+), SUBTRACT(-), DIVIDE(/),");
    println!("MULTIPLY(*), and find the REMAINDER(%).. Enter your expression as you would");
    println!("write it. For example, you want to add 10 plus 5. Enter 10 + 5..Ok\n");
}
