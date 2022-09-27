use std::env;
use std::path::Path;

/*
* Joins path with home
*/
pub fn get_home_path(path: &str) -> String {
    let home = env::var("HOME").unwrap();
    let home = Path::new(&home);

    home.join(Path::new(path)).to_str().unwrap().to_string()
}
