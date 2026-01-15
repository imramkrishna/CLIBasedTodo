use std::fs;

pub fn view_todos(){
    let contents=fs::read_to_string("todos.txt");
    match contents {
        Ok(content)=>{
            println!("File Content : {}",content);
            for (index,line) in content.lines().enumerate(){
                println!("{}. {}",index+1,line)
            }
        },
        Err(err)=>{
            println!("Error while opening file : {}",err)
        }
    }
}