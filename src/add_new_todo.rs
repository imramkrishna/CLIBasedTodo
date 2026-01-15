use std::io;
use crate::write_to_file;
pub fn add_new_todo(){
    println!("Enter the Todo you want to add : ");
    let mut todo=String::new();
    io::stdin().read_line(&mut todo).expect("Failed to read the input.");
    write_to_file::write_to_file(todo);
}