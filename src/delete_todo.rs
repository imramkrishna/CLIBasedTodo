use std::io;
use std::fs;

pub fn delete_todo(){
    println!("Enter the todo number you want to delete : ");
    let mut todoNumber=String::new();
    io::stdin().read_line(&mut todoNumber).expect("Failed to read the todo number.");
    let num:usize=todoNumber.trim().parse().unwrap();
    let contents=fs::read_to_string("todos.txt");
    match contents {
        Ok(content)=>{
            let lines: Vec<&str> = content.lines().collect();
            
            if num == 0 || num > lines.len() {
                println!("Invalid todo number.");
                return;
            }
            
            // Collect all lines except the one to delete
            let remaining_lines: Vec<&str> = lines
                .into_iter()
                .enumerate()
                .filter(|(index, _)| index + 1 != num)
                .map(|(_, line)| line)
                .collect();
            
            // Write remaining lines back to file
            let new_content = remaining_lines.join("\n");
            match fs::write("todos.txt", new_content) {
                Ok(_) => println!("Todo {} deleted successfully!", num),
                Err(err) => println!("Error writing to file: {}", err),
            }
        },
        Err(err)=>{
            println!("Error while opening file : {}",err)
        }
    }
}