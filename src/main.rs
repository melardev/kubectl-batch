use std::env;
use std::process::{Command, Stdio};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize variables to store kubectl arguments and paths
    let mut kubectl_args: Vec<String> = vec![];
    let mut paths: Vec<String> = vec![];

    // Iterate through the command-line arguments
    let mut is_path = false;
    for arg in args.iter().skip(1) {
        if is_path {
            // This argument is a path
            paths.push(arg.clone());
            is_path = false;
        } else if arg.starts_with('/') {
            // This argument starts with '/', so it's treated as a path
            paths.push(arg.clone());
        } else {
            // This argument is not a path, so it's added to kubectl_args
            kubectl_args.push(arg.clone());
        }
    }

    // Execute kubectl for each path
    for path in paths {
        let mut kubectl_command = Command::new("kubectl");
        kubectl_command.args(&kubectl_args);
        kubectl_command.arg(&path);

        // println!("Command: {:?}", kubectl_command);
        // println!("kubectl {} {}", kubectl_args.join(" "), path);

        // Redirect stdout and stderr of kubectl to the parent's stdout
        kubectl_command.stdout(Stdio::inherit());
        kubectl_command.stderr(Stdio::inherit());

        match kubectl_command.spawn() {
            Ok(mut child) => {
                if let Err(err) = child.wait() {
                    eprintln!("Error executing kubectl: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Error spawning kubectl: {}", err);
            }
        }
    }
}
