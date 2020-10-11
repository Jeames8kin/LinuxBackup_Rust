use std::io;
//use rand::Rng;

mod basic_restore;
mod basic_backup;


fn main() {

    let title = "Backup and restore thing";    
    let version = "Current version: v1.0";
    let disclaimer = "Note: This program is subject to change at anytime.";

    println!("{}\n", title);
    println!("{}", version);
    println!("{}\n", disclaimer);
    
    menu1();

    }

    fn menu1() {

    loop {

        let mut option1 = String::new();

        println!("Select your option you want to run:");
        println!("1. Backup the current user (basic, no other input really required)");
        println!("2. Backup the current user (advanced, more options)");
        println!("3. Restore from the most recent backup in the repository/branch (will check against current packages before beginning)");

        io::stdin() 
            .read_line(&mut option1)
            .expect("Error 1: Invalid character, please choose a number.");

        let option1: u32 = match option1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option1 {                                         //This will throw an error if the line on line 48 does not exist.
            1 => {
                    println!("Backup of current user will begin shortly...");
                    basic_backup::basic_backup();
                    break;
                 }

            2 => {
                    println!("Starting the backup configurator...");
                    break;
                 }

            3 => {
                    println!("Restore will begin shortly...");
                    basic_restore::basic_restore();
                    break;
                 }

            _ => {                                                                                                                                  
                    println!("Invalid option, try again.\n");   //match will NOT work without a line like this, it's supposed to be if the variable is equal to nothing.
                          
                 }
        }
    }
}