use std::process::Command;
use std::io::{Read, Write};

fn main() {
    println!("cargo:rerun-if-changed=./angular-ui/");

    if cfg!(debug_assertions) {
        return;
    }

    let build_command = "cd angular-ui && npm install && ng build --configuration production --aot --optimization --deploy-url /static/ --output-path ../artifacts/static";

    println!("Run build_command");
    let mut output = if cfg!(target_os = "windows") {
        println!("Windows");
        Command::new("cmd")
            .args(&["/C", build_command])
            .spawn()
            .expect("Failed to build angular-ui")
    } else {
        println!("Unix");
        Command::new("sh")
            .arg("-c")
            .arg(build_command)
            .spawn()
            .expect("Failed to build angular-ui")
    };

    println!("Started commands");

    output.wait().expect("Failed to build angular-ui");

    std::fs::create_dir("artifacts/config").unwrap();

    let mut default_config = String::new();
    std::fs::File::open("config/default.toml").unwrap().read_to_string(&mut default_config).unwrap();
    std::fs::File::create("artifacts/config/default.toml").unwrap().write_all(default_config.as_bytes()).unwrap();
}
