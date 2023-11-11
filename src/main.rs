use std::io;
use colored::Colorize;

fn main() {
    

    const CMD: [&str; 3] = ["echo ", ":q", "?"];


    loop {

        // input
        let mut word = String::new();

        // check command
        match io::stdin().read_line(&mut word) {
            Ok(_n) => {
                // exit program
                if word.contains(CMD[1]) {
                    break;
                }

                if word.contains(CMD[2])  {
                    if let Some(_word) = word.trim().strip_prefix(CMD[2]) {

                        println!("{}           exit program\n{}[text]  repeat text\n{}            help command", CMD[1].blue(), CMD[0].blue(), CMD[2].blue());
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
