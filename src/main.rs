use std::io::{self, BufRead};
use tempfile::tempfile;
use std::io::Write;

fn launch(interpreter: String, program: Vec<String>, data: Vec<String>) {

}

fn main() {
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
    println!("{:?}\n--\n{:?}\n--\n{:?}\n", interpreter, program, data); 
    if let Some(Ok(x)) = interpreter {
        launch(x, program, data);
    }
}
