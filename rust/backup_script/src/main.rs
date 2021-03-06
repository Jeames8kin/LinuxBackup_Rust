mod util;// In this circumstance, mod looks for a file called mod inside the ui folder.
mod ui;

mod basic_restore;  // While with the ones below, it just looks for the file name.
mod basic_backup;

use std::io;
use std::{error::Error};
use termion::event::Key;
//use termion::input::TermRead;
use termion::screen::AlternateScreen;
use crate::util::event::{Event, Events};


use termion::raw::IntoRawMode;


fn main() {

    // Use this line below to jump to a test point for neatness (doesn't work anyway?).
    

    let title = "Backup and restore thing";    
    let version = "Current build: 8";
    let disclaimer = "Note: This program is subject to change at anytime.";

    println!("{}\n", title);
    println!("{}", version);
    println!("{}\n", disclaimer);
    
    menu1();

}

#[allow(unused_must_use)]


fn menu1() {

    loop {

        let mut option1 = String::new();

        println!("Select your option you want to run:");
        println!("1. Backup the current user (basic, no other input really required)");
        println!("2. Backup the current user (advanced, more options)");
        println!("3. Restore from backup");
        println!("4. Run the tui test");

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
                    basic_backup::bb_check_os();
                    break;                                      //Breaks out of the loop.
                 }

            2 => {
                    println!("Starting the backup configurator...");
                    break;
                 }

            3 => {
                    println!("Restore will begin shortly...");
                    basic_restore::br_check_os();
                    break;
                 }

            4 => {
                    println!("Running tui test...");
                    tui_main();
                    break;

                 }

            _ => {                                                                                                                                  
                    println!("Invalid option, try again.\n");   //match will NOT work without a line like this, it's supposed to be if the variable is equal to nothing or simply not a valid option.
                          
                 }

        }

    }

}

// The tui shit.

#[allow(unused_imports)]

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge},
    Terminal,
};

struct App {
    progress_bar1: u16,
}

impl App {
    fn new() -> App {
        App{
            progress_bar1: 0
        }   
    }

    fn update(&mut self) {
        self.progress_bar1 += 1;
        if self.progress_bar1 > 100 {
            self.progress_bar1 = 0;
        }
    }
}
    

pub fn tui_main() -> Result<(), Box<dyn Error>> {

    let events = Events::new();

    let mut _app = App::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();


        loop {    

            terminal.draw(|f| {

                let size = f.size();

                let _chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(25)
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Main block")
                    .border_type(BorderType::Rounded);
                f.render_widget(block, size);

        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('c') {
                    break;
                }
            }
            Event::Tick => {
                app.update();
            }
        }

        }
        Ok(())
    } 