

use std::path::Display;

use walkdir::WalkDir;



fn main() {
    // println!("Hello, world!");

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
println!("Locations: {:?}", ds_path)


}
