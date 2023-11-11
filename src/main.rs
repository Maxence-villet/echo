use std::io;
use colour::*;
fn main() {
    

    const CMD: [&str; 3] = ["echo ", ":q", "?"];


    loop {

        // input
        let mut word = String::new();

        // check command
        match io::stdin().read_line(&mut word) {
            Ok(_n) => {

                if word.len() < 3 {
                    continue;
                }
                // exit program
                if word.contains(CMD[1]) {
                    break;
                }

                if word.contains(CMD[2]) && word.len() == 3 {
                    if let Some(_word) = word.trim().strip_prefix(CMD[2]) {
                        blue!("{}", CMD[0]);
                        white!("[text]  repeat text \n");

                        blue!("{}", CMD[1]);
                        white!("           exit program \n");

                        blue!("{}", CMD[2]);
                        white!("            help command \n");

                        continue;
                    }
                }


                //echo program
                if word.contains(CMD[0])  {
                    if let Some(word) = word.trim().strip_prefix(CMD[0]) {
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

    }
}
