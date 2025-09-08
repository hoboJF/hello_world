fn main() {
    //dude i wrote a comment HOLY SHIT THIS IS CRAZY!!!
    /* this comment spans multiple lines, wicked
    why am i writing this */
    let x = 10;
    println!("x is {}", x);
    average();
    //code under this comment is for one of the linkedin challenges
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("the temperature in fahrenheit is {}", fahrenheit_temp)
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
