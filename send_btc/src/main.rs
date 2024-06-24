use std::io;
use rand::Rng;

fn send_bitcoin(){
    println!("We are sending some bitcoin");

    let clients=vec!["homer","marge","bart","lisa"];

    println!("Who do you want to send Bitcoin to?]n");
    for client in &clients{
        println!("{}",client);
    }

    println("\n");

    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient);

    if clients.contains(recipient.trim()){
        println!("How many BTC do you want to send? : ");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

    }

}

fn recieve_bitcoin(){
println!("\nWe are going to recieve some bitcoin\n");

let amount = rand::thread_rng().gen_range(1,10);

println!("You just recieved {} BTC!\n",amount);
}

fn exit_console(){
    println!("Invalid option must be 'r' or 's'.");
}

fn console(){
    println!("Let's explore the world of solana\n");
    println!("Would you like to send (s) or recieve (r) BTC? : ");
    
    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("s"){
        send_bitcoin();
    }else if command.trim().eq("r"){
recieve_bitcoin();

    }
    else{
        exit_console();
    }
}

fn main() {
    println!("Hello, world!");
console();
}
