fn main() {
    mutable_variable();
    shadowing_variables();
    creating_tuples(1, 2, 3);
    ownership()
}

fn mutable_variable(){
    // without the mut keyword the compiler will throw an error.
    let mut x = 5;
    println!("x is: {x}");
    x = 6;
    println!("x is: {x}");
}

fn shadowing_variables(){
    let shadow = 5;
    let shadow = shadow + 1;
    {
        let shadow = shadow * 2; 
        println!("The value of shadow in the inner scope: {shadow}"); // 12 
    }
    println!("The value of shadow in the inner scope: {shadow}"); // 6
}

fn creating_tuples(a: i8, b: i8, c: i8){
    let tup = (a, b, c);
    let (x, y, z) = tup;

    println!("tup: ({a}, {b}, {c})");
    println!("spread tup: {x}, {y}, {z}");
}

fn ownership(){
    let s = String::from("Hi");
    // passing a reference keeps ownership with s and does not pass it to takes_ownership function
    // if it was passed as a variable we would not be able to use it again in this scope as it
    // would have been dropped in the takes_ownership function 
    takes_ownership(&s);
    let x = 7;
    makes_copy(x);
    println!("{}", s) // this would not work if the ownership was given to takes_ownership 

}

fn takes_ownership(string: &String) {
    println!("{}", string)
}
fn makes_copy(integer: i32){
    println!("{}", integer)
}
