use std::process::Command;
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

    let output = if cfg!(target_os = "windows") { //Checks if the host is Windows or Linux
        println!("Run this on an Arch derivative, otherwise this will not run!")

    } else {
        println!("Checked host OS: Linux")
    };

} 



