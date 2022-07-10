use rand::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game ");
    let secret_number = rand::thread_rng().gen_range(1..101); // 1 to 101
    //println!("Secret Number is : {} ", secret_number);
    loop{
        println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess) 
        .expect("Failed to readline");
    println!("Your guessed : {}", guess);
    let guess:u8 = guess.trim().parse().expect("Type an integer within range 1 to 101"); // converting to integer data 
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too Small"),
        Ordering::Greater =>println!("Too Big"),
        Ordering::Equal =>{
            println!("You Win");
            break;

        },
    }

    }
    
}
