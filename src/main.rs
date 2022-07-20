use std::{io::{self, BufRead}, error::Error, process::Command};
use tempfile::NamedTempFile;
use std::io::Write;

fn fill_file(file: &mut NamedTempFile, data: Vec<String>) -> Result<(), Box<dyn Error>> {
    for line in data {
        file.write_all(line.as_bytes())?;
        file.write(b"\n")?;
    }
    Ok(())
}

fn launch(interpreter: String, program: Vec<String>, data: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut data_file = NamedTempFile::new()?;
    fill_file(&mut data_file, data)?;
        let mut program_file = NamedTempFile::new()?;
    fill_file(&mut program_file, program)?;

    let proc = Command::new(interpreter)
        .stdin(data_file.reopen()?)
        .arg(program_file.path().as_os_str())
        .output()
        .expect("failed command");

    print!("{}", String::from_utf8(proc.stdout)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let interpreter = iterator.next();
    let mut program = Vec::new();
    let mut data = Vec::new();
    let mut ended = false;
    iterator.next();
    for line in iterator {
        let l = line.unwrap();
        if l == "--" {
            ended = true;
        } else {
            if ended {
                data.push(l);
            } else {
                program.push(l);
            }
        }
    }
    //println!("{:?}\n--\n{:?}\n--\n{:?}\n", interpreter, program, data); 
    if let Some(Ok(x)) = interpreter {
        launch(x, program, data)?;
    }
    Ok(())
}
