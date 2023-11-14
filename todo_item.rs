pub struct ToDoItem {
    pub title: String,
    pub body: String,
}

impl ToDoItem {
    pub fn new(title: String, body: String) -> ToDoItem {
        ToDoItem { title, body }
    }
}
