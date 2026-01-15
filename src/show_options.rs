use std::io;

pub fn show_options()->String{
    println!("Welcome to CLI Based Todo Application.");
    println!("1. View Todos");
    println!("2. Add Todos");
    println!("3. Delete Todos");
    println!("Select the operation you want to perform (1-3) : ");
    let mut input_number=String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read lines.");
    return input_number;
}