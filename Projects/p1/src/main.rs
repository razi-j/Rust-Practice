use std::io; // acts like modules in python

fn main() {
    let  x = 4; // i32 is int32, mut declares a variable as mutable or can be changed. without mut, all variables are immutable
    println!("x = {}", x);

    { // this is counted as a different scope
        let x = x + 7; // x outside this scope can be called in the interior scope
        println!("x = {}", x)
    }

    let x: i32 = 5; // without mut, variables are redeclarable as they we essentially remake a variable
    println!("x = {}", x);

    // if we do not want to use make a mutable variable we can simply redeclare a variable with let

    const SECONDS_IN_MINS:i32 = 60;
    println!("Seconds in Minutes: {}", SECONDS_IN_MINS);

    // Rust has Primitive Data types and 2 subtypes Scalar and Compound
    // Primitive data types have basic or fundamental data types used to declare a variable
    // Scalar data types have finite possible set of values following some scale: can be compared as greater, equal or lesser: int, float, bools, chars
    // compounds include tuples and arrays
    // uint is unsigned int or cant be negative
    // tuple has a fixed length sequence of elements that is immutable (can be mutable with mut) 

    let tup = (1, 'a', true);
    println!("{}", tup.1); // the whole tuple is not printable... only elements of it can be printed: elements can be printed by referencing the index by using tup.index

    // arrays need same elements in them: they use []
    let arr = [1,2,3,4,5,6]; // declaring ints need a data type and number of data inside it: we cannot add but we can redeclare: usind mut with arrays changes the values in them
    println!("{}",arr[0]);

    let mut input = String::new(); // initializes a new string
    io::stdin().read_line(&mut input).expect("Failed to read"); // expect checks for an error
    let num_input: i64 = input.trim().parse().unwrap(); // trim removes the hidden character when submitting input; parse returns the result; unwrap is similar to expect but it takes valid data type results from input and unwarp it to the data type
    println!("num is {}\n num+2 is {}", num_input, (num_input as u8) +2);
    
    let y = 45;
    let z:u8 = 5;

    let ans = y - (z as i32); // type casting allows you to use variables and cast them as another data type
    println!("{}", ans);

}

