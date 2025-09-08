fn main() {
    //dude i wrote a comment HOLY SHIT THIS IS CRAZY!!!
    /* this comment spans multiple lines, wicked
    why am i writing this */
    let x = 10;
    println!("x is {}", x);
    average();
}

fn average(){
    //challenge from the linkedin course
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let average = (a as f64 + b + c as f64)/3.0;
    println!("the average of a, b and c is {}", average);
}
