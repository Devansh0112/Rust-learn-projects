use std::io;

fn _tuples() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five = x.0;
    let six = x.1;
    let one = x.2;

    println!("five: {five}, six: {six}, one: {one}");
}

fn _arrays() {
    let a = [1,2,3,4,5];

    println!("Please enter a index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");

    let element = a[index];

    println!("The value at index {index} is : {element}")
}

fn _another_function(x: i32) {
    println!("Another function value : {x}");
}

fn _five() -> i32 {
    5
}


fn main() {
    // println!("main function");
    // another_function(3);

    // let y = {
    //     let x = 3;
    //     x+1
    // };

    // println!("The value of y is : {y}");

    // let z = five();
    // println!("Value of z : {z}");

    let number = 3;

    loop {
        if number < 5 {
            println!("condition true");
        } else {
            break;
        }
    }
}