mod command_line;
use command_line::command_line_app;

use std::fs;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut counter: i32 = 0;
    let mut ds_path: Vec<String> = vec![];

    let commands = command_line_app().get_matches();

    let dir: &str = commands.value_of("dir").unwrap_or("./");
    let directory_to_scan = dir.to_string();

    let yes_to_all = commands.is_present("yesToAll");

    for files in WalkDir::new(&directory_to_scan) {
        match files {
            Ok(f) => process_file(f, &mut counter, &mut ds_path),
            Err(e) => println!("error reading file {}", e),
        }
    }

    println!("total DS_Store found: {}", counter);
    println!("Locations: {:?}", ds_path);

    if ds_path.is_empty() {
        println!("No .DS_Store files found. Yay!")
    } else {
        //we has .ds_store files

        if yes_to_all {
            delete_files(&ds_path)
        } else {
            println!("Do yo want to delete the files? Press 'y' to confirm");
            let mut user_wants_to_delete: String = String::from("");

            match std::io::stdin().read_line(&mut user_wants_to_delete) {
                Ok(_) => println!(""),
                Err(e) => println!("error reading user input {}", e),
            }

            if user_wants_to_delete.trim() == "y" {
                delete_files(&ds_path)
            } else {
                println!("user has stopped operation")
            }
        }
    }
}

fn process_file(file: DirEntry, counter: &mut i32, ds_path: &mut Vec<String>) {
    if file.file_name() == ".DS_Store" {
        *counter += 1; // dereferencing because counter is a pointer and we are increasing the value not the pointer itself
        ds_path.push(file.path().display().to_string())
    }
    println!("{:?} -> {}", file.file_name(), file.path().display());
}

fn delete_files(paths: &Vec<String>) {
    for file_path in paths {
        fs::remove_file(file_path).unwrap_or_else(|why| {
            println!("Error deleting the file because {:?}", why.kind());
        });
    }
}
