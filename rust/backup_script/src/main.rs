use std::io;

fn main() {

let title = "Backup and restore thing";    
let version = "Current version: v1.0";
let disclaimer = "Note: This program is subject to change at anytime.";
let selection1 = "Select your option you want to run:";

println!("{}\n", title);
println!("{}", version);
println!("{}\n", disclaimer);

println!("{}", selection1);
println!("1. Backup the current user (basic, no other input really required)");
println!("2. Backup the current user (advanced, more options)");
println!("3. Restore from the most recent backup in the cloud (will check against current packages before beginning)");

}