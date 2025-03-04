use chrono::{DateTime, Utc};
use std::{fs, path::Path, process::Command};

fn main() {
    // println!("Hello, world!");
    let path = Path::new("D:\\Temporary");
    if !folder_exists(path) {
        fs::create_dir(path).unwrap();
    }
    let now: DateTime<Utc> = Utc::now();
    let formatted_path2 = format!("D:\\Temporary\\{}", now.format("%y%m"));
    let path2 = Path::new(&formatted_path2);
    // println!("{:?}", format!("D:\\Temporary\\{}", now.format("%y%m")));
    if !folder_exists(path2) {
        fs::create_dir(path2).unwrap();
    }
    let formatted_path3 = format!("{}\\{}", formatted_path2, now.format("%d"));
    let path3 = Path::new(&formatted_path3);
    // println!("{:?}", formatted_path3);
    if !folder_exists(path3) {
        fs::create_dir(path3).unwrap();
    }
    Command::new("explorer")
        .arg(formatted_path3) // <- Specify the directory you'd like to open.
        .spawn()
        .unwrap();
}

fn folder_exists(path: &Path) -> bool {
    return path.exists() && path.is_dir();
}
