// Rust imports
use std::process::Command;
use std::string;
use std::io;

// Crates
use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;

pub fn basic_backup() {

    let scriptVerdict = false;         // Stays false until it reaches the end of the script successfully where it is true, otherwise will remain false.

    let _os = os_type::current_platform();
    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            testThing(); 
        } 
        os_type::OSType::Manjaro => {
            println!("Please note that using Manjaro may have slightly out of date packages compared to Arch and some things may not work.");
            testThing();
        }
        _ => {
            println!("This operating system is invalid, please use Arch or Manjaro!");
            std::process::exit(1);
        }
    }
    
    // Worry about exiting the program if something fails later, more shit to do.

    fn testThing() {

        let string1 = "yes".to_string();
        let other_string1 = String::from("no");

        println!("Beginning backup...");
        println!("Testing command capabilities...");

        let fun1 = run_fun!(rustc --version).unwrap(); // Sets output of command to variable.
        eprintln!("Your Rust version is {}", fun1);

        let username1 = run_fun!(whoami).unwrap();

        if run_cmd! {
            ls /home/${username1};                      // ${variable_here} is a way to use Rust defined variables in Shell.

        }.is_err() {
            println!("Something here doesn't seem right. ");
        }

        if run_cmd! {
            echo "Please enter your password to proceed.";

        }.is_err() {
                println!("Well, either you exited the root terminal or something else went wrong.")
        }

        

        println!("The current folders that will be backed up are:\n/home/jeames8kin/BasicBackupTest");
        println!("Are you sure you want to proceed?");
        let mut beginRestore = String::new();
        io::stdin() 
            .read_line(&mut beginRestore)
            .expect("That isn't a valid answer, answer yes/Yes/YES/y, or no/No/NO/n");

            match beginRestore.as_str() {

                "yes" => {
                    println!("string1");
                }

                "no" => {
                    println!("other_string1")
                }

                _ => {
                    println!("Not sure what you did there.");
                }

            }
            



    }

    

    fn backup() {

        let string = "abort".to_string();
        let other_string = String::from("abort");

        println!("Copying files into /tmp...");

        let username2 = run_fun!(whoami).unwrap();


        if run_cmd! {
            mkdir /tmp/LinuxBackup_Rust;
            cp -R /home/${username2}/rustBackupTest/ - | pv /tmp/LinuxBackup_Rust;

        }.is_err() {

            println!("Something went wrong. Retry, or abort? (r/a)");
            let mut copyFailed = String::from("");
            io::stdin()
                .read_line(&mut copyFailed)
                .expect("Take the L");

            match copyFailed {
                
                string => {
                    std::process::exit(1);
                }

                other_string => {

                    assert_eq!(string, other_string);
                    backup();
                }

                _ => {
                        println!("No idea what you wrote there, but that's fucken wrong");
                        println!("{}", copyFailed);
                        backup();
                        
                }
            }
        }
    }

} 



