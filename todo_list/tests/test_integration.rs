use todo_list::model::Task;

#[test]
fn test_add_task() {
    let task = Task::new(1, "Integration Test Task");
    assert_eq!(task.description, "Integration Test Task");
    assert!(!task.completed);
}
