//Tic Tic Toe
//Get three in a row on a 3 X 3 board and win the game
//Date Created: 3/12/2022 Time: 9:59 AM

use std::io;

fn main() {
    let rows = [[0,1,2], [3,4,5], [6,7,8],
                [0,3,6], [1,4,7], [2,5,8],
                [2,4,6], [0,4,8]];
    loop {
    let mut board = ['1','2','3','4','5','6','7','8','9'];
    let mut kill = true;

    clearscreen::clear().expect("failed to clear screen");      //clears console
    println!("\nWelcome to Tic Tac Toe!!!");
    println!("Get 3 in a row and you Win.");
    println!("You will be X and I will be O.");
    println!("Good Luck!!\n");

    draw_board(board);

    while kill == true {

        let position = get_position(board);               //players turn               
        board[position as usize] = 'X';
        draw_board(board); 
        kill = check_board(board, rows);

        if kill == false {
            break;
        }

        let position = computers_turn(board, rows);         //computers turn
        board[position as usize] = 'O';
        clearscreen::clear().expect("failed to clear screen");
        draw_board(board);
        kill = check_board(board, rows);
    }

    println!("Would you like to play again?");
    let mut again =  String::new();

    io::stdin()
        .read_line(&mut again).expect("Failed to read line");

    let again = match again.trim() {
            "Y" => "Y",
            "y" => "y",
            _ => "n",
        };
    if again == "Y" || again == "y" {
        continue;
    }
    else {
        println!("Bye Bye!!!");
        break;
    }
   }
}

fn draw_board(board: [char;9],) {

    println!("\n   {} | {} | {}", &board[0], &board[1], &board[2]);
    println!("  ---------------");
    println!("   {} | {} | {}", &board[3], &board[4], &board[5]);
    println!(" ----------------");
    println!("   {} | {} | {}", &board[6], &board[7], &board[8]);

}

//Checks, gets, and places valid user pick
fn get_position(board: [char;9]) -> i32 {

    let position;
    loop {
        println!("Enter postion number");
        let mut user = String::new();

        io::stdin()
            .read_line(&mut user).expect("Failed to read line.");

        let user: i32 = match user.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let user = match user {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => 6,
            8 => 7,
            9 => 8,
            _ => continue,
        };
        if board[user as usize] == 'X' || board[user as usize] == 'O' {
            continue;
        }
        else{
            position = user;
            break;
        }
    }
    position
}

//Checks game for winner, loser and tie
fn check_board(board: [char;9], rows: [[usize;3];8]) -> bool {
    let mut cmp_count = 0;
    let mut player_count = 0;
    let mut kill = true;
    let mut empty = 0;

    for i in rows {
        for j in i{
            match board[j] {
                'X' => player_count += 1,
                'O' => cmp_count += 1,
                _ => empty += 1,
            }
        }
        if player_count == 3 {
            println!("You win!!!");
            kill = false;
            break;
        }
        else if cmp_count == 3 {
            println!("You Lose!!");
            kill = false;
            break;
        }
        else {
            cmp_count = 0;
            player_count =0;
        }
    }
    if empty == 0 {
        println!("Its a tie!!");
        kill = false;
    }
    kill
}

fn computers_turn(board: [char;9], rows: [[usize;3];8])  -> i32{
    let new_board = copy_board(board, rows);
    let mut index = 0;
    let mut ind_rank = (0,0);
    let mut arr_ranks = [0;9];
    let mut row = 0;

    //this double for loop places index and rank of
    //each empty space saving index and rank into a tuple
    //then adding index and rank to an arr
    for i in new_board {
        for j in i {
            if j != 'X' && j != 'O' {
                ind_rank.0 = rows[index][row];
               ind_rank.1 = rank_spot(i, row);

               arr_ranks[ind_rank.0] = ind_rank.1 + arr_ranks[ind_rank.0];
            }
            row += 1;
        }
        index += 1;
        row = 0;
    }

    println!("{:?}", arr_ranks);
    let mut tmp = arr_ranks[0];
    let mut highest = 0;
    let mut loc = 0;

//Finds highest rank out of all saved moves
    for i in arr_ranks {
        if tmp < i {
            tmp = i;
            highest = loc;
        }
        loc += 1;
    }
    highest
}

//Ranks empty places on board
//Computer is calculating every possible move 
fn rank_spot(mut row:[char;3], loc_of_change: usize)  -> i32{
    row[loc_of_change] = 'O';
    let mut cmp_count = 0;
    let mut player_count = 0;

    for i in row {
        match i {
            'X' => player_count += 1,
            'O' => cmp_count += 1,
            _ => cmp_count += 0,
        }
    }

    let ranked;

    if cmp_count == 3 {
        ranked = 50;
    }
    else if cmp_count == 2 && player_count == 0{
        ranked = 7;
    }
    else if cmp_count == 2 && player_count == 1 {
        ranked = -2;
    }
     else if cmp_count == 1 && player_count == 0 {
        ranked = 3;
    }
    else if cmp_count == 1 && player_count == 1 {
        ranked = -1;
    }
    else if cmp_count == 1 && player_count == 2 {
        ranked = 11;
    }
    else {
        ranked = 0;
    }

    ranked
}

//Turn arr into double arr and makes it easier to check rows
fn copy_board(board: [char;9], rows: [[usize; 3]; 8]) -> [[char; 3]; 8] {
    let mut new_board = [[' '; 3]; 8];
    let mut index = 0;
    let mut ind2 = 0;

    for i in rows {
        for j in i {
            new_board[index][ind2] = board[j];
            ind2 += 1;
        }
        index += 1;
        ind2 = 0;
    }
    new_board
}