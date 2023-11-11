use core::time;
use std::io;
use std::thread;
fn main() {
    
    //echo command
    const CMD: &str = "echo ";

    // input
    let mut word = String::new();
    
    // check command
    match io::stdin().read_line(&mut word) {
        Ok(_n) => {
            if word.contains(CMD)  {
                if let Some(word) = word.trim().strip_prefix(CMD) {
                        println!("{}", word);
                    } else {
                        println!("command unknown!");
                    }
            } else {
                println!("command unknown!");
            }
        },
        Err(error) => println!("error : {error}"),
    }

    let time_for_exit = time::Duration::from_secs(5);
    thread::sleep(time_for_exit);
}
