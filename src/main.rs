use std::fs;

use walkdir::WalkDir;



fn main() {

    let current_dir: String = String::from("./");

    let mut counter: i32 = 0;
    let mut ds_path: Vec<String> = vec![];

    for entry in WalkDir::new(&current_dir) {
    let entry = entry.unwrap();

    if entry.file_name() == ".DS_Store" {
        counter+=1;
        ds_path.push(entry.path().display().to_string())
    }

    println!("{}", entry.path().display());
}

    println!("total DS_Store found: {}", counter);
    println!("Locations: {:?}", ds_path);

    println!("Removing found .DS_Store files");

    for file_path in ds_path {

        fs::remove_file(file_path).unwrap_or_else(|why| {
            println!("Error deleting the file because {:?}", why.kind());
        });
    }




}
