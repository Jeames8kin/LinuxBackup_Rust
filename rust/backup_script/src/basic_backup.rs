use std::process::Command;
pub fn basic_backup() {
    println!("basic backup test");

    Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
        
        

}



