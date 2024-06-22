use std::io;

fn console(){
    println!("Let's explore the world of solana\n");
    println!("Would you like to send (s) or recieve (r) BTC? : ");
    
    let mut command = String::new();
    io::stdin().read_line(&mut command);

}

fn main() {
    println!("Hello, world!");

}
