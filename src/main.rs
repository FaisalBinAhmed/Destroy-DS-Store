use std::fs;

use walkdir::{WalkDir, DirEntry};



fn main() {

    let current_dir: String = String::from("./");
    let mut counter: i32 = 0;
    let mut ds_path: Vec<String> = vec![];

    for files in WalkDir::new(&current_dir) {
        match files {
            Ok(f) => process_file(f, &mut counter, &mut ds_path),
            Err(e) => println!("error reading file {}", e)
        }
    }

    println!("total DS_Store found: {}", counter);
    println!("Locations: {:?}", ds_path);

    // println!("Removing found .DS_Store files");

    if ds_path.is_empty(){

        println!("No .DS_Store files found. Yay!")

    }else{

        println!("Do yo want to delete the files? Press 'y' to confirm");
        let mut delete_files: String = String::from("");
        
        match std::io::stdin().read_line(&mut delete_files) {
            Ok(_) => println!(""),
            Err(e) => println!("error reading user input {}", e)
        } 

        if delete_files.trim() == "y" {

            for file_path in ds_path {
        
                fs::remove_file(file_path).unwrap_or_else(|why| {
                    println!("Error deleting the file because {:?}", why.kind());
                });
            }
            println!("Finished deleting {} files", counter)
        }else{

            println!("user has stopped operation")
        }

    }

}

fn process_file(file: DirEntry, counter: &mut i32, ds_path: &mut Vec<String>){
    if file.file_name() == ".DS_Store" {
        *counter+=1; // dereferencing because counter is a pointer and we are increasing the value not the pointer itself
        ds_path.push(file.path().display().to_string())
    }
    println!("{:?} -> {}", file.file_name(), file.path().display());
}