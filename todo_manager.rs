use crate::todo_item::ToDoItem;
use std::io::{self, Write};

pub struct ToDoManager {
    items: Vec<ToDoItem>,
}

impl ToDoManager {
    pub fn new() -> ToDoManager {
        ToDoManager { items: Vec::new() }
    }

    pub fn add_item(&mut self) {
        let title = self.prompt_input("Enter title: ");
        let body = self.prompt_input("Enter body: ");

        let item = ToDoItem::new(title, body);
        self.items.push(item);
        println!("Item added successfully.");
    }

    pub fn list_items(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{}: {}", index + 1, item.title);
        }
    }

    pub fn view_item(&self) {
        let index = self.prompt_input("Enter the item number to view: ");
        let index: usize = index.trim().parse().unwrap_or(0);

        if let Some(item) = self.items.get(index - 1) {
            println!("Title: {}", item.title);
            println!("Body: {}", item.body);
        } else {
            println!("Item not found.");
        }
    }

    pub fn delete_item(&mut self) {
        let index = self.prompt_input("Enter the item number to delete: ");
        let index: usize = index.trim().parse().unwrap_or(0);

        if index > 0 && index <= self.items.len() {
            self.items.remove(index - 1);
            println!("Item deleted successfully.");
        } else {
            println!("Item not found.");
        }
    }

    fn prompt_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
