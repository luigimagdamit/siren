use std::fs;
use std::collections::HashMap;

pub fn get_files(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut res = Vec::new();
    for path in paths {
        if let Ok(p) = path {
            if let Some(pt) = p.path().to_str() {
                res.push(pt.to_string());
            }
        }
    }
    res
}

pub fn  create_table(path: &str) -> HashMap<String, Vec<String>> {
    let mut table = HashMap::new();
    
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if let Ok(p) = path {
            if let Some(pt) = p.path().to_str() {
                let path =  p.path().display().to_string();
                let name = get_name(&path);
                
                if !table.contains_key(name) {
                    table.insert(name.to_string(), Vec::new());
                }
                if let Some(path_list) = table.get_mut(name) {
                    path_list.push(path);
                }
            }
        }
    }
    for (name, paths) in &table {
        println!("=={name}==");
        for path in paths {
            println!("{path}");
        }
    }
    table
}

pub fn get_name(name: &str) -> &str {
    let mut start = 0;
    for (i, c) in name.chars().enumerate() {
        match c {
            '-' => {
                return &name[start..i]
            },
            '/' => { start = i + 1; println!("{}", start); }
            _ => ()
        }
    }
    &name[start..]
} 