mod add_new_todo;
mod read_file;
mod show_options;
mod write_to_file;
mod delete_todo;
fn main() {
    let input = show_options::show_options();
    let num: i32 = input.trim().parse().unwrap();
    if num == 1 {
        read_file::view_todos();
    } else if num == 2 {
        add_new_todo::add_new_todo();
    } else if num == 3 {
        read_file::view_todos();
        delete_todo::delete_todo();
    } else {
        println!("Invalid Input Provided. Use Number between 1-3. ")
    }
}
