use std::path::Path;

use std::process::exit;
use std::{env, fs};

fn main() {
    // get home dir from $HOME env variable and construct config folder string
    let home_key = "HOME";
    let home_dir = env::var(home_key).expect("$HOME variable not set");
    let config_dir = format!("{}/.config/JetBrains", home_dir);

    // make sure the path exists
    if !Path::new(&config_dir).exists() {
        println!("JetBrains config folder doesn't exist");
        exit(1);
    }

    // get paths in config folder
    let paths = fs::read_dir(&config_dir).unwrap();

    let mut got_error = false;

    for path in paths {
        let p = path.unwrap();

        if p.file_type().unwrap().is_dir() {
            let lock_path = format!("{}/.lock", p.path().display());

            // try to delete the lock file, if an error occurs that's not the file not found error, print it out
            match fs::remove_file(&lock_path) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind().to_string() != "entity not found" {
                        println!("{}", e);
                        got_error = true;
                    }
                }
            };
        }
    }

    if got_error {
        exit(1)
    }
}
