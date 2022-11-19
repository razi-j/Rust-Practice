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
}
