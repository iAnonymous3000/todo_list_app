mod todo_item;
mod todo_manager;

use todo_manager::ToDoManager;
use std::io::{self, Write};

fn main() {
    let mut todo_manager = ToDoManager::new();

    loop {
        println!("To-Do List Application");
        println!("1. Add a new to-do item");
        println!("2. List all to-do items");
        println!("3. View a to-do item");
        println!("4. Delete a to-do item");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => todo_manager.add_item(),
            "2" => todo_manager.list_items(),
            "3" => todo_manager.view_item(),
            "4" => todo_manager.delete_item(),
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
