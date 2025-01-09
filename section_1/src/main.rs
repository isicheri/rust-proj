use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guessing Game");
    let random_num = rand::thread_rng().gen_range(1..=50);
    loop {
        println!("\nplease input a number....");
        print!("the random_num: {}",random_num);
    
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("\nyou guessed: {}", guess);

        match guess.cmp(&random_num) {
            Ordering::Less => print!("it's too small"),
            Ordering::Equal => {
                print!("you win");
                break;
            },
            Ordering::Greater =>  print!("it's to big")
        }
    } 
}


// enum Shapes<T> {
// Circle(T),
// Rectangle(T,T)
// }


// fn calculate_area(shape:Shapes<f64>) -> f64{

// match shape  {
//     Shapes::Circle(radius) => std::f64::consts::PI * radius * radius,
//     Shapes::Rectangle(length,height) => length * height
// }

// }

// struct User {
//     username: String,
//     age: u64
// }

// impl User {
//     fn print_user(&self) -> &Self {
//         return &self;
//     }
// }