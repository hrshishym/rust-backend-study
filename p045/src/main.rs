fn main() {
    // original
    println!("Hello, Rust!");
    print!("\n");
    println!("Hello, Cargo!");
    
    // after cargo clippy --fix --allow-dirty
    println!("Hello, Rust!");
    println!();
    println!("Hello, Cargo!");
    
    // origin2
    println!("Hello, Rust!");
       println!();
           println!("Hello, Cargo!");
           
    // after cargo fmt
    println!("Hello, Rust!");
    println!();
    println!("Hello, Cargo!");
}
