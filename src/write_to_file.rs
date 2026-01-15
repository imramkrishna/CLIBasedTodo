use std::io;
use std::fs::OpenOptions;
use std::io::Write;
pub fn write_to_file(todo:String){
    let mut file=OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt");
    match file{
        Ok(mut f)=>{
            writeln!(f, "{}",todo.trim()).unwrap();
        },
        Err(err)=>{
            eprintln!("Error opening file: {}", err);
        }
    }
}
