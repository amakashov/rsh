use std::io::{stdin, stdout, Read, Write, Stdout};
use termion::{raw::IntoRawMode, input::TermRead, event::Key};
use termion;
use std::env;


pub(crate) fn read_string() -> String
{
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut result = String::new();
    let mut stdin = termion::async_stdin().keys();

    let path = env::current_dir().unwrap();        
    print!("{} > ", path.as_path().display());
    stdout.lock().flush().unwrap();

    
    loop 
    {
        let input = stdin.next();
        if let Some(Ok(key)) = input
        {
            match key
            {
                Key::Char('\n') => 
                {
                    stdout.write_fmt(core::format_args!("{}", "\r\n")).unwrap();
                    stdout.lock().flush().unwrap();
                    return result;
                },
                Key::Char('\t') =>
                {
                    write!(stdout, "\r\nAutocompletion not available yet\r\n");
                    write!(stdout, "{} > {} ", path.as_path().display(), result);
                    stdout.lock().flush().unwrap();

                },
                Key::Ctrl('c') =>
                {
                    result.clear();
                    write!(stdout, "\r\n{} > ", path.as_path().display());
                    stdout.lock().flush().unwrap();
                },
                Key::Char(c) 
                    if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace()=> 
                {
                    write!(stdout,"{}", c).unwrap();
                    stdout.lock().flush().unwrap();
                    result.push(c);
                },
                Key::Backspace =>
                {
                    if !result.is_empty()
                    {
                        result.pop();
                        write!(stdout,"{}\r{} > {}", termion::clear::CurrentLine, path.as_path().display(),result).unwrap();
                        stdout.lock().flush().unwrap();
                    }
                },
                Key::Up | Key::Down =>
                {
                    write!(stdout, "\r\nHistory not available yet\r\n");
                    write!(stdout, "{} > {} ", path.as_path().display(), result);
                    stdout.lock().flush().unwrap();
                }
                _ => ()
            }        
        }
    }
}