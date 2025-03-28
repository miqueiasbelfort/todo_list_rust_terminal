use std::io::Write;
use colored::*;
use csv::Writer;
use std::error::Error;
use std::fs::File;

struct Task {
    description: String,
    done: bool,
    tags: Vec<String>,
}
impl Task {
    // Task constructor
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
            tags: Vec::new(),
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}
impl TodoApp {
    // TodoApp constructor
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
        println!("{}", "Task added".green());
    }

    fn mark_task_as_done(&mut self, index: usize) {

        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red());
        }

        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
            let line = format!("{} - Marked as done!", task.description);
            println!("{}", line.green());
        }
    }

    fn show_tasks(&self) {

        if self.tasks.is_empty() {
            println!("{}", "No tasks found".magenta());
        }

        for (_i, task) in self.tasks.iter().enumerate() {
            let line = self.format_task(task, _i);

            if task.done {
                println!("{}", line.blue());
            } else {
                println!("{}", line.cyan());
            }
        }
    }

    fn remove_task(&mut self, index: usize) {
        // Check if task is done
        match self.tasks.get(index) {
            Some(task) => {
                if task.done {
                    println!("{}", "You cannot remove a done task".red());
                } else {
                    self.tasks.remove(index);
                    println!("{}", "Task removed".green());
                }
            }
            None => println!("{}", "Invalid task".red()),
        }
    }

    fn add_tag_to_task(&mut self, index: usize, tag: &str) {
        match self.tasks.get_mut(index) {
            Some(task) => {
                if task.done {
                    println!("{}", "You cannot add a tag to a done task".red());
                } else {
                    task.tags.push(tag.to_string());
                    println!("{}", "Tag added".green());
                }
            },
            None => println!("{}", "Invalid task".red()),
        }
    }

    fn find_task_by_tag(&self, tags: &str) -> Vec<&Task> {
        let mut result  = Vec::new();

        for task in &self.tasks {
            if task.tags.contains(&tags.to_string()) {
                result.push(task);
            }
        }

        result
    }

    fn format_task(&self, task: &Task, index: usize) -> String {
        let status = if task.done { "[X]" } else { "[ ]" };
        format!("{}: {} {} - tags: ({})", index + 1, status, task.description, task.tags.join(", "))
    }

    fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let results: Vec<&Task> = self.tasks.iter().filter(|task| task.description.to_lowercase().contains(query.to_lowercase().as_str())).collect();

        if results.is_empty() {
            println!("{}", "No tasks found".magenta());
        }

        results
    }

    fn export_to_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Logic to export tasks to CSV
        let file = File::create(filename)?;
        let mut wtr = Writer::from_writer(file);

        // Escreve o cabeçalho do CSV (opcional)
        wtr.write_record(&["description", "done", "tags"])?;

        // Escreve os dados das tarefas no CSV
        for task in &self.tasks {
            // Transformando os tags para uma string
            let tags = task.tags.join(" - ");
            wtr.write_record(&[
                &task.description,
                &task.done.to_string(),
                &tags,
            ])?;
        }

        wtr.flush()?;
        println!("Tasks exported to /{}.csv", filename.green());
        Ok(())
    }
}

fn main() {

    let mut todo_list_app: TodoApp = TodoApp::new();

    loop {
        // Display menu options
        println!("{}", "\n1. Add a new task".yellow());
        println!("{}", "2. Mark a task as done".yellow());
        println!("{}", "3. Show tasks".yellow());
        println!("{}", "4. Remove a task".yellow());
        println!("{}", "5. Add tag to task".yellow());
        println!("{}", "6. Search tasks".yellow());
        println!("{}", "7. Search tasks by tag".yellow());
        println!("{}", "8. Export tasks to CSV".yellow());
        println!("{}", "9. Exit\n".yellow());

        // Get user input for mnu choice
        let choice = match get_numeric_input("Enter your choice: ") {
            Some(value) => value,
            None => {
                println!("{}", "Invalid input. enter valid number".red());
                continue;
            }
        };

        match choice {
            1 => {

                let description = get_string_input("Enter task description: ");
                todo_list_app.add_new_task(&description);

            }
            2 => {

                todo_list_app.show_tasks();

                let index = match get_numeric_input("Enter the task index to mark as done: ") {
                    Some(value) => value as usize,
                    None => {
                        println!("{}", "Invalid input, enter proper number".red());
                        continue;
                    }
                };
                todo_list_app.mark_task_as_done(index - 1);

            }
            3 => todo_list_app.show_tasks(),
            4 => {

                todo_list_app.show_tasks();

                let index = match get_numeric_input("Enter the task index to remove: ") {
                    Some(value) => value as usize,
                    None => {
                        println!("{}", "Invalid input, enter proper number".red());
                        continue;
                    }
                };
                todo_list_app.remove_task(index - 1);

            },
            5 => {

                todo_list_app.show_tasks();

                let index = match get_numeric_input("Enter the task index to add tag: ") {
                    Some(value) => value as usize,
                    None => {
                        println!("{}", "Invalid input, enter proper number".red());
                        continue;
                    }
                };

                let tag = get_string_input("Enter tag: ");
                todo_list_app.add_tag_to_task(index - 1, &tag);

            },
            6 => {

                let query = get_string_input("Enter search query: ");
                let tasks = todo_list_app.search_tasks(&query);

                for (_i, task) in tasks.iter().enumerate() {
                    let line = todo_list_app.format_task(task, _i);
                    if task.done {
                        println!("{}", line.blue());
                    } else {
                        println!("{}", line.cyan());
                    }
                }

            }
            7 => {
                let search_tag = get_string_input("Enter tag to search: ");
                let tasks = todo_list_app.find_task_by_tag(&search_tag);

                for (_i, task) in tasks.iter().enumerate() {
                    let line = todo_list_app.format_task(task, _i);
                    if task.done {
                        println!("{}", line.blue());
                    } else {
                        println!("{}", line.cyan());
                    }
                }
            },
            8 => {
                let filename = get_string_input("Enter filename: ");
                todo_list_app.export_to_csv(&filename).unwrap();
            }
            9 => break,
            _ => println!("{}", "Invalid option, enter number between 1-6".red()),
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    print!("{} ", prompt.yellow());

    std::io::stdout().flush().ok();

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{} ", prompt.yellow());

    std::io::stdout().flush().ok();

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(v) => Some(v),
        Err(_) => {
            println!("{}", "Invalid input. enter valid number".red());
            None
        }
    }
}
