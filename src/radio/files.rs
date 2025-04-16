use std::fs;
pub fn get_files() -> Vec<String> {
    let paths = fs::read_dir("./music").unwrap();

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