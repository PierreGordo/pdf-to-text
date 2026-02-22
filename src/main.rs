use std::env;

//This function gather the command line arguments and parses out the path of the .pdf file
fn gather_arguments() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();

    //first argument is the path to the executable
    //second argument should be -T for target
    //thirt argument will be the .pdf (either the whole path or just the filename in which case I can obtain the filepath by combining it with the path to the executable)
    match args.len() {
        3 => {
            //If there is a correct amount of arguments, check whether the -T argument is correct
            match args[1].as_str() {
                "-T" => {
                    //Return the possible path
                    //This here hopes that the .pdf will be in the same directory as the .pdf and will have to be fixed in the future
                    //Ok(args[0].clone())

                    let filename = args[2].clone();
                    let path = args[0].clone();

                    match path.rsplit_once("/") {
                        Some(split_path) => {
                            //Now that we have the path split, we just append the name provided to us which will create a potential path - if it is wrong, it is user error
                            let mut path_to_file = String::from(split_path.0);
                            path_to_file.push('/');
                            path_to_file.push_str(filename.as_str());
                            Ok(path_to_file)
                        }
                        None => Err("Failed to parse the filepath"),
                    }
                }
                _ => {
                    println!("An invalid argument {} was provided.", args[1]);
                    Err("Incorrect amounth of arguments")
                }
            }
        }
        _ => {
            println!(
                "You have inputted {} arguments, which is not the correct amount (3)",
                args.len()
            );
            Err("Incorrect amounth of arguments")
        }
    }
}

fn main() {
    match gather_arguments() {
        Ok(path) => println!("The filename is {path}"),
        Err(_) => return,
    }
}
