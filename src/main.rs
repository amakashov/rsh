use std::{io::{stdin, stdout, Error, Write}, path::Path, process::{Command, Stdio, Child}};
use std::env;

pub mod forming;

fn main() ->std::io::Result<()>
{
    loop 
    {
        let path = env::current_dir()?;
        
        print!("{} > ", path.as_path().display());
        let _result = stdout().flush();
        let input =  forming::read_string();
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous: Option<Child> = None;

        while let Some(command) = commands.next()
        {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts;
            match command
            {
                "cd" =>
                {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let path = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&path)
                    {
                        eprintln!("{}", e);
                    }
                    previous = None;
                },
                "exit" => return Ok(()),
                command =>
                {
                    let stdin = previous.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some()
                    {
                        Stdio::piped()
                    }
                    else 
                    {
                        Stdio::inherit()
                    };

                    let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();

                    match output
                    {
                        Ok(child) => 
                        {
                            previous = Some(child);
                        },
                        Err(e) =>
                        {
                            eprintln!("{}", e);
                            previous = None;
                        }
                    }  
                }
            } 
            if let Some(ref mut final_comand) = previous
            {
                final_comand.wait().unwrap();
            }
        }
    }
}
