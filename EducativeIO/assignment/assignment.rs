fn main() {
    let a = 2;
    let b = a;
    println!("b = a");
    println!("Value of a:{}", a);
    println!("Value of b:{}", b);

    // Compound Assignment Operator
    //define a mutable variable
    let mut a = 2;
    println!("a:{}", a);
    a += 1;
    println!("a+=1:{}", a);
    println!("a:{}", a);
    a -= 1;
    println!("a-=1:{}", a);
    println!("a:{}", a);
    a /= 1;
    println!("a/=1:{}", a);
    println!("a:{}", a);
    a *= 3;
    println!("a*=3:{}", a);
}