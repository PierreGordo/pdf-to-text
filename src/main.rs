use std::env;
use std::fs;
use std::path::PathBuf;

//This function gather the command line arguments and parses out the path of the .pdf file
fn gather_arguments() -> Result<PathBuf, &'static str> {
    let args: Vec<String> = env::args().collect();

    //first argument is the path to the executable
    //second argument should be -T for target
    //thirt argument will be the .pdf - just the filename, but the path is also fine, since I added
    match args.len() {
        3 => {
            //If there is a correct amount of arguments, check whether the -T argument is correct
            match args[1].as_str() {
                "-T" => {
                    //Return the possible path
                    //This here hopes that the .pdf will be in the same directory as the .pdf and will have to be fixed in the future
                    //Ok(args[0].clone())

                    let path = PathBuf::from(&args[2]);
                    //let filename = args[2].clone();

                    //path.push(&args[2]);
                    Ok(path)
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
    //Extracting the path from the function - gather_arguments()
    let path = gather_arguments().unwrap();

    println!("The filename is {:?}", path);

    //Opens the file, since I can't do fs::read_to_string
    let f = fs::read(path).unwrap();

    //Outputs the raw bytes, will have to think on this a while, how to approach this issue
    println!("{:?}", f);
}
