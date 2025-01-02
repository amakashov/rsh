use std::io::{stdin, stdout, Read, Write, Stdout};
use termion::{raw::IntoRawMode, input::TermRead};
use termion;


pub(crate) fn read_string() -> String
{
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut result = String::new();
    let mut stdin = termion::async_stdin().keys();
    // let mut buffer: [u8; 1] = [0; 1];
    
    loop 
    {
        // let mut handle = stdin().lock(); 
        // handle.read_exact(&mut buffer).unwrap();
        let input = stdin.next();
        if let Some(Ok(key)) = input
        {
            match key
            {
                termion::event::Key::Char('\n') => 
                {
                    stdout.write_fmt(core::format_args!("{}", "\r\n")).unwrap();
                    stdout.lock().flush().unwrap();
                    return result;
                }
                termion::event::Key::Char('\t') =>
                {
                    println!("Autocompletion not available yet");
                }
                termion::event::Key::Char(c) => 
                {
                    write!(stdout,"{}", c).unwrap();
                    stdout.lock().flush().unwrap();
                    result.push(c);
                }
                _ => ()
            }        
        }
    }
}