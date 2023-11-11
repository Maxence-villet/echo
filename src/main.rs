use std::io;

fn main() {
    
    //echo command
    const CMD: &str = "echo ";

    const EXIT: &str = ":q";

    loop {

        // input
        let mut word = String::new();

        // check command
        match io::stdin().read_line(&mut word) {
            Ok(_n) => {
                // exit program
                if word.contains(EXIT) {
                    break;
                }

                //echo program
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

    }
}
