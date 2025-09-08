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