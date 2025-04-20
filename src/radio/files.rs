use std::fs;
use std::collections::HashMap;
use crate::radio::radio::Song;
pub fn  create_table(path: &str) -> HashMap<String, Vec<String>> {
    let mut table = HashMap::new();
    
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if let Ok(p) = path {

            let path =  p
                .path()
                .display()
                .to_string();
            let name = get_name(&path);
                
            let path_list = table.entry(name.to_string()).or_insert(Vec::new());
            path_list.push(path);

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