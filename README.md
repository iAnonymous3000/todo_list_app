# To-Do List Application in Rust

## Project Overview

This To-Do List Application is the capstone project for the Rust course by Zero Point Security. It demonstrates using Rust to create a command-line application with practical utility. This project encompasses essential features such as adding, listing, viewing, and deleting to-do items and showcases modular programming, error handling, and unit testing in Rust.

## Features

- **Add New To-Do Item:** Create a new item with a title and detailed description.
- **List To-Do Items:** Display all to-do items with their titles.
- **View To-Do Item:** Show detailed information about a specific to-do item.
- **Delete To-Do Item:** Remove a to-do item from the list.


## Getting Started

### Prerequisites

- Rust programming environment (see [Rust installation guide](https://www.rust-lang.org/tools/install))

### Installation

1. Clone the repository:
```
git clone https://github.com/iAnonymous3000/todo_list_app.git
```

2. Navigate to the project directory:
```
cd todo_list_app
```


### Running the Application

- Compile and run the application using Cargo:
```
cargo run
```


## Usage

This application is a command-line tool that allows users to manage their to-do list. Here's how you can use its various features:
### Adding a New To-Do Item
To add a new item to your list, use the following command:
add "Title of Item" "Detailed description of the item"
Replace `"Title of Item"` and `"Detailed description of the item"` with your desired title and description.
### Listing All To-Do Items
To view all the items on your list with their titles, use:
list
This will display a list of all to-do items along with an identifier or index for each item.
### Viewing a Specific To-Do Item
To view detailed information about a specific item, use:
view [item_id]
Replace `[item_id]` with the identifier or index of the item you wish to view.
### Deleting a To-Do Item
To remove an item from your list, use:
delete [item_id]
Replace `[item_id]` with the identifier or index of the item you want to delete
### Exiting the Application
To exit the application, use the `exit` command:
exit


## Running Tests

- To run the unit tests included in the application:
```
cargo test
```


## License

This project is licensed under the MIT License - see the `LICENSE` file for details.


## Acknowledgments
Zero Point Security for the Rust course
