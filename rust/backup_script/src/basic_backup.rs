use std::process::Command;

extern crate os_type;

pub fn basic_backup() {
    println!("basic backup test");

/*    let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start")
        .ok();
        
        let pacman_command = "pacman -Qqe | tr '\n' ' '";

        let pacman_output = Command::new(pacman_command)
        
        .spawn()
        .expect("pacman failed to reply.");
*/

/*    let output = if cfg!(target_os = "windows") { //Checks if the host is Windows or Linux
        println!("Run this on an Arch derivative, otherwise this will not run!");
        std::process::exit(1);
    } else {
        println!("Checked host OS: Linux")
    }; */

    let _os = os_type::current_platform();
    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            backup_start();
        } 
        os_type::OSType::Manjaro => {
            println!("Please note that using Manjaro may have slightly out of date packages compared to Arch, so some things may not work.");
            backup_start();
        }
        _ => {
            println!("This operating system is invalid, please use Arch or Manjaro!");
            std::process::exit(1);
        }
    }
    

    fn backup_start() {
        println!("Beginning backup...")

        

    
    }

} 



