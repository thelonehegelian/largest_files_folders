use std::env;
use std::fs;
use std::process::{Command, Stdio};

fn main() {
    // get environment variables
    let env_vars = env::vars();
    // for (key, value) in env_vars {
    //     println!("{}: {}", key, value);
    // }

    // read from Config file
    // let config_path = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());
    // let config: String = fs::read_to_string(config_path).unwrap();

    // println!("Config: {}", config);

    /***********************************
     * du -ah . | sort -hr | head -n 10
     ***********************************/

    // get current directory
    let current_dir = std::env::current_dir().unwrap();
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&current_dir)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    du_output_child.wait().unwrap();
}
