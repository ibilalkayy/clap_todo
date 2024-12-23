use std::fs::File;
use clap::Parser;
use std::io::Write;
use crate::commands::commands::{Todo, ManageTasks};
use std::fs;
use serde_json::Value;

pub fn todo_tasks() {
    let args = Todo::parse();
    match args.manage_tasks {
        ManageTasks::Add(task) => {
            let mut file = File::create(task.filename.clone()).expect("failed to create");

            let json_data = serde_json::json!({
                "title": task.title,
                "description": task.description
            });
        
            // Write JSON data to the file
            file.write_all(json_data.to_string().as_bytes())
                .expect("Failed to write to file");
        
            println!("Task saved in {}", task.filename);

            // Serialize the task to JSON
            serde_json::to_string(&task).unwrap();
        }

        ManageTasks::Update(task) => {
            let mut file = File::create(task.filename.clone()).expect("failed to create");

            let json_data = serde_json::json!({
                "title": task.title,
                "description": task.description
            });
        
            // Write JSON data to the file
            file.write_all(json_data.to_string().as_bytes())
                .expect("Failed to write to file");
        
            println!("Task updated in {}", task.filename);
        }

        ManageTasks::View(task) => {
            let file = fs::read_to_string(task.filename.clone()).expect("failed to read the file");

            let json_data: Value = serde_json::from_str(&file).expect("failed to parse JSON");

            if let Some(title) = json_data.get("title") {
                if *title == *task.title {
                    println!("Found title: {}", title);
                } else {
                    println!("File not found");
                }
            } else {
                println!("No title found in the JSON file");
            }
        }
    }
}