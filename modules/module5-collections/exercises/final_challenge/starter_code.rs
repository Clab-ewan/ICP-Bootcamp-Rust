// Starter code for the Rust Task Manager challenge
use std::io;
// Task status enum
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        print!("Task successfully added");
        self.tasks.last().unwrap()
    }
    
    // TODO: Implement methods for listing, completing, deleting tasks
    fn list_task(&mut self) {
        for task in self.tasks.iter() {
            let due_date_str = task.due_date.as_ref().map_or("no due date", |date| date);
            println!("Task: {} with id: {} is for {}", task.title, task.id, due_date_str);
        }
    }

    fn complete_task(&mut self, task_id: u32) {

    }


    // TODO: Implement methods for saving to and loading from a file
    // TODO: Implement methods for filtering tasks
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();
    
    // Main application loop
    loop {
        println!("Which command do you want to use ?");
        println!("Available commands:");
        println!("  add <title> <description> [due_date]");
        println!("  list");
        println!("  complete <id>");
        println!("  delete <id>");
        println!("  save <filename>");
        println!("  load <filename>");
        println!("  quit");
        // TODO: Get user command
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("there is a problem");
        // TODO: Process command
        let command = command.split_whitespace().collect::<Vec<&str>>();
        if  command[0] == "add" {
            if command.len() != 4 {
                println!("Error in the command line, refer to the doc");
            }
            else {
                task_manager.add_task(command[1].to_string(), command[2].to_string(), Some(command[3].to_string()));
            }
        } else if command[0] == "list" {
            task_manager.list_task();
        } else if command[0] == "quit" {
            break;
        } else {
            print!("Command not recognised\n");
        }
    }
}