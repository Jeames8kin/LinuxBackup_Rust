# LinuxBackup_Rust

This is just for me to learn Rust and overall get better with programming while experimenting.
With that in mind, I have a bit of a plan for this project and where I want it to go.

  0. Learn to use the basics of Rust. (Working on it)
  1. Get Bash commands executed in Rust and have the output of them stored in variables (25%)
  - I have gotten the very basics of Rust, but it has been causing a bit of trouble, in the way of running `pacman -Qqe | tr '\n' ' '`.

  2. Make a copy of some files, such as Downloads, Documents, config files, etc and put them all into a compressed file (either .gz or .tar) (0%)
  3. Make a versioning/dating scheme so that I can backup and restore without it being outdated or it not existing. (0%)
  4. Have it upload to GitHub (for file server, see Extra #2 below) 
  <br>

  Extra #1: Have a service running to get package list and files of other PC running rather than backing everything up and going the long way. 
  <br>
  Extra #2: Have it upload to my file server for easier management and faster restoration processes.
  <br>
  Extra #3: Have it remove outdated versions or create a manager in Rust (or even GTK3/Qt5) to manage them.
  <br>
  
  This will be interesting to make, so time for me to commit to this.
