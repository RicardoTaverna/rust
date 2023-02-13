fn main() {
    //explicitly define an integer
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;
    //print the values
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    //implicitly define an integer
    let a = 21; 
    let b = 1;
    let c = 54;
    let d = 343434;
    //print the variable
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    //explicitly define a float type
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
    //implicitly define a float type
    let pi = 3.14;
    let e = 2.17828;
    println!("pi: {}", pi);
    println!("e: {}", e);

    //explicitly define a bool
    let is_bool:bool = true;
    println!("explicitly_defined: {}", is_bool);
    // assign a boolean value
    let a = true;
    let b = false;
    println!("a: {}", a);
    println!("b: {}", b);

    // explicitly define 
    let char_1:char = 'e';
    println!("character1: {}", char_1); 
    // implicitly define
    let char_2 = 'a';
    let char_3 = 'b';
    println!("character2: {}", char_2);
    println!("character3: {}", char_3);

    // explicitly define 
    let str_1:&str = "Rust Programming";
    println!("String 1: {}", str_1); 
    // implicitly define
    let str_2 = "Rust Programming";
    println!("String 2: {}", str_2);

    

}