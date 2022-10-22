use std::{io::{self, stdin, Write}, process::Command};

// Import some shell buitins
mod builtins;
mod details;

fn main() {
    loop{
        // Print initial tick

        let d: details::Details = details::get_details();

        print!("{}@{} [{}]>> ", d.user, d.hostname, d.cwd);
        io::stdout().flush().unwrap();

        // read in user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        // Parse the input
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;


        // Handle shell builtins
        match command{
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                builtins::cd::cd(new_dir);
            },
            "exit" => {
                let value = args.peekable().peek().map_or("0", |x| *x);

                let exit_code = value.parse::<i32>();
                // handle error
                match exit_code{
                    Ok(code) => { builtins::exit::exit(code); },
                    Err(_) => eprintln!("incorect argument supplied to exit")
                }

            }
            command => {
                // Run command
                let child = Command::new(command)
                .args(args)
                .spawn();

                // handle error
                match child{
                    Ok(mut child) => { child.wait().expect("Failed to run command"); },
                    Err(e) => eprintln!("{}", e)
                }
            }
        }


        
    } 
}
