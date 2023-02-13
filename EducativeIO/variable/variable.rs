const ID_1: i32 = 4; // define a global constant variable

fn main() {
    // let language = "Rust"; // define a variable
    // println!("Language: {}", language); // print the variable

    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable

    let (course,category) =("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value

    let outer_variable = 112;
    { // start of code block
        let inner_variable = 213;
        println!("block variable inner: {}", inner_variable);
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("inner variable: {}", outer_variable);

    //Shadowing
    let outer_variable = 112;
    { // start of code block
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117;
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("outer variable: {}", outer_variable);

    const ID_2: u32 = 3; // define a local constant variable
    println!("ID:{}", ID_1); // print the global constant variable
    println!("ID:{}", ID_2); // print the local constant variable
    
}