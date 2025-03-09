use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, os::windows::process::CommandExt, path::Path, process::Command};

#[derive(Serialize, Deserialize)]
struct Setting {
    base_path: String,
    dayout: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setting_text = match fs::read_to_string("setting.json") {
        Ok(setting_text) => {
            println!("setting.json found");
            setting_text
        }
        Err(_) => {
            println!("setting.json not found");
            let setting = Setting {
                base_path: "D:\\Temporary".to_string(),
                dayout: -1,
            };
            let path = Path::new("setting.json");
            let tmp = serde_json::to_string(&setting).unwrap();
            fs::write(path, &tmp).unwrap();
            tmp
        }
    };
    let setting: Setting = serde_json::from_str(&setting_text).unwrap();
    // 判断是否需要定期清除文件（根据上次更改时间）
    if setting.dayout > 0 {
        let path_list = visit_dir(Path::new(setting.base_path.as_str()))?;
        for path in path_list {
            let metadata = fs::metadata(&path)?;
            let last_modified_day = metadata.modified()?.elapsed()?.as_secs() / 3600 / 24;
            if last_modified_day > setting.dayout.try_into().unwrap() {
                println!("{:?}超时删除", path);
                fs::remove_file(&path)?;
            }
        }
    }
    let path = Path::new(setting.base_path.as_str());
    if !folder_exists(path) {
        println!("{:?} not found", path);
        fs::create_dir(path).unwrap();
    }
    let now: DateTime<Utc> = Utc::now();
    let formatted_path2 = format!("{}\\{}", setting.base_path.as_str(), now.format("%y%m"));
    let path2 = Path::new(&formatted_path2);
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
        .creation_flags(0x08000000) // <-隐藏窗口
        .arg(formatted_path3) // <- 打开目录
        .spawn()
        .unwrap();
    Ok(())
}
// 文件夹是否存在
fn folder_exists(path: &Path) -> bool {
    return path.exists() && path.is_dir();
}
// 递归所有文件夹目录
fn visit_dir(dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut dirs = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 递归访问子目录
                let sub_dirs = visit_dir(&path)?;
                dirs.extend(sub_dirs);
                // 将当前目录路径加入列表
                dirs.push(path.to_string_lossy().into_owned());
            }
        }
    }
    Ok(dirs)
}
