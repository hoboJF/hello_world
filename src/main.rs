use std::io;
use rand::Rng;
use std::env;
use std::fs;

struct Rectangle{
    width: f64,
    height: f64
}

impl Rectangle{
    fn get_area(&self) -> f64{
        self.width * self.height
    }
    fn scale(& mut self, scale: f64){
        self.width *= scale;
        self.height *= scale;
    }
    fn new(width: f64, height: f64) -> Rectangle{
        Rectangle {
            width,
            height
        }
    }
}

fn main() {
    //dude i wrote a comment HOLY SHIT THIS IS CRAZY!!!
    /* this comment spans multiple lines, wicked
    why am i writing this */
    let x = 10;
    println!("x is {}", x);
    average();
    //code under this comment is for one of the linkedin challenges (the celsius to fahrenheit one)
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("the temperature in fahrenheit is {}", fahrenheit_temp);
    min_max_avg();
    let mut rectang = Rectangle::new(3.0, 5.0);
    rectang.scale(2.0);
    let temp = rectang.get_area();
    println!("the area of the rectangle is {}", temp);
}

fn average(){
    //challenge from the linkedin course
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let average = (a as f64 + b + c as f64)/3.0;
    println!("the average of a, b and c is {}", average);
}

fn celsius_to_fahrenheit(temperature : f64) -> f64{
    (1.8 * temperature) + 32 as f64
}

fn min_max_avg(){
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max:i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter(){
        if num > max {
            max = num;
        } else if num < min{
            min = num;
        }
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    println!("the max is {}", max);
    println!("the min is {}", min);
    println!("the mean is {}", mean);
}

fn random_number_guess(){
    let mut rng = rand::thread_rng();
    let guess_num = rng.gen_range(1..101);
    println!("guess the number thats between 1 and 100");
    loop{
        println!("guess: ");
        let mut buffer = String::new();
        //yes i know i didnt write any error checking, i fully trust my users
        io::stdin().read_line(&mut buffer);
        let guess: u32 =  buffer.trim().parse().unwrap();
        if guess > guess_num{
            println!("your guess was too high");
        }
        else if guess < guess_num{
            println!("your guess was too low");
        }
        else{
            println!("you got it, the number was {}", guess_num);
            break;
        }
    }
}

fn check_roster(){
    //i still trust my users
    let file_path = env::args().nth(1).unwrap();
    let input_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        if line == input_name {
            println!("the name {} was found! insane stuff!", input_name);
            return;
        }
    }
    println!("the name {} was not found! not so insane stuff!", input_name)
}