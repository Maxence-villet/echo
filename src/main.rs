use core::time;
use std::io;
use std::thread;
fn main() {
    
    let cmd = String::from("echo ");

    let mut word_read = false;

    let mut word = String::new();
    match io::stdin().read_line(&mut word) {
        Ok(_n) => word_read = true,
        Err(error) => println!("error : {error}"),
    }

    if word.contains(&cmd)  {
        if &word[0..4] == &cmd[0..4] {
            if word_read == true  {
                word = word[5..].to_string();
                println!("{}", word);
            }
        } else {
            println!("command unknown!");
        }
    } else {
        println!("command unknown!");
    }

    let time_for_exit = time::Duration::from_secs(5);
    thread::sleep(time_for_exit);
}
