use std::io;

fn main() {

    println!("Advinharei seu nome");
    println!("Digite seu nome");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    println!("Seu nome é {}" , input);
     

}
