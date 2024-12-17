use todo_list::model::Task;

fn main() {
    let task = Task::new(1, "Hello from main!");
    println!("{:?}", task);
}
