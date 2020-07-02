//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** main.rs 🦀   
//! **`DESCRIPTION:`**   
//! The one and only start and entry point of the program.   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-04-04 | CLunion   | creation   
//! 1.1      | 2020-06-## | CLunion   | changed comment style to markdown for rustdoc   
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! shardoverse(.exe)
//!     Starts the program shardoverse. 
//!     Depending on the operating system the name may differ: on MS-Windows the programfile has the extension '.exe'.
//!
//! shardoverse(.exe) --help
//!     Writes a short help text to the console window, which shows all available commandline parameters and their meaning.
//!
//! shardoverse(.exe) --windowreset
//!     Resets the position and size of the programs window(s) to default values.
//!
//! shardoverse(.exe) --configfile=<anExistingConfigFile>
//!     Loads the configuration from the given configuration file and stores changes made during the run of the program in it.
//!     The config file stores for example the window size and position, these can be changed manually before the program starts.
//!     The format of the configuration file follows the INI-File format and is currently very strictly interpreted (comments will be lost)
//!```
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * everything (nearly)
//! ___________________________________________________________________________________________________________________________
//!    

//--- CRATES EXTERNAL: -------------------------------------------------------------------------------------------------------
extern crate sdl2;
extern crate clap;

//--- MODULES EXTERNAL: ------------------------------------------------------------------------------------------------------
use std::env;
use std::io;
use std::path::Path;
use clap::{Arg, App, SubCommand};

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
mod modules;                              // <dirname>
use crate::modules::*;                    // create::<dirname>::*

mod central_core;                         // <filename>
use crate::central_core::*;               // create::<filename>::*

use crate::modules::config::WindowConfig;
 
//--- CONSTANTS: -------------------------------------------------------------------------------------------------------------
//--- none ---

//--- TYPES: -----------------------------------------------------------------------------------------------------------------
//--- none ---

//--- ENUMS: -----------------------------------------------------------------------------------------------------------------
//--- none ---

//--- STRUCTS: ---------------------------------------------------------------------------------------------------------------
//--- none ---

//--- GLOBAL VARS: -----------------------------------------------------------------------------------------------------------
//--- none ---


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  main   
/// **`TYPE:       `**  program entry point   
/// ___________________________________________________________________________________________________________________________
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`<none>     `**    
/// **`RETURNS:    `** **`Result --> `** - OK(state)   
/// **`            `** **`       --> `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// The one and only main: startup and entry point of this program.   
/// Here the handling of commandline paramaters and calls to initialise und de-initialise are done.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-04-## | CLunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
///  * define commandline arguments for all configuration switches and variables    
///  * add recognition and handling of debug mode   
///  * add recognition and handling of testing mode   
/// ___________________________________________________________________________________________________________________________

fn main() -> Result<(), io::Error>
{
let args: Vec<String> = env::args().collect();
let mut i :i32 = 0;
let mut win_config: WindowConfig;

// check for command line arguments:
if !args.is_empty()
    {
    for arg in &args
        {
        println!("Command line argument(s)[{}] {:?}",i, &arg);
        i+=1;
        }
    } 

// use clap to parse the command line:
  let cmd_line = App::new("Shardoverse")
                        .version("0.1")
                        .author("Clunion <Christian.Lunau@gmx.de>")
                        .about("A Roguelike Peer-to-Peer Multi Player Dungeon Explorer.")
                        .arg(Arg::with_name("configfile")                   // <--------------------------------------------
                             .short("c")
                             .long("configfile")
                             .value_name("FILE")
                             .help("Sets a specific config file")
                             .takes_value(true))
                        .arg(Arg::with_name("windowreset")                  // <--------------------------------------------
                             .short("w")
                             .long("windowreset")
                             .help("resets the window size and position to default values")
                             .takes_value(false))
                        .arg(Arg::with_name("v")                            // <--------------------------------------------
                             .short("v")
                             .multiple(true)
                             .help("Sets the level of verbosity"))
                        .subcommand(SubCommand::with_name("test")           // <--------------------------------------------
                                    .about("controls testing features")
                                    .version("0.1")
                                    .author("Clunion <Christian.Lunau@gmx.de>")
                                    .arg(Arg::with_name("debug")            // <--------------------------------------------
                                        .short("d")
                                        .help("print debug information verbosely")))
                        .get_matches();

  // Gets a name for config-file if supplied by user, or defaults to config::NAME_OF_INI_FILE
  let config_filename = cmd_line.value_of("configfile").unwrap_or(config::NAME_OF_INI_FILE);
  println!("Value for config-file: {}", config_filename);

  // Vary the output based on how many times the user used the "verbose" flag
  // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
  match cmd_line.occurrences_of("v") 
      {
      0 => println!("No verbose info"),
      1 => println!("Some verbose info"),
      2 => println!("Tons of verbose info"),
      3 => println!("Don't be crazy"),
      _ => println!("Maximum verbosity"),
      }

  // You can handle information about subcommands by requesting their matches by name
  // (as below), requesting just the name used, or both at the same time
  if let Some(cmd_line) = cmd_line.subcommand_matches("test") 
      {
      if cmd_line.is_present("debug") { println!("Debug Mode enabled"); } 
      }


// load configuration, states and assets, initialise everything:
match config::init(config_filename)
    {
    Ok(config)  => { win_config = config; },
    Err(error)  => { println!("Error initialising: {:?}", error); return Err(error); },
    }

if  cmd_line.is_present("windowreset")
    {
    println!("windowreset detected"); 
    win_config = WindowConfig::default();
    };

// Hand over control to central core:
match run(&mut win_config,Path::new("assets/cursors/pointers_part_5/glove3.png"))
    {
    Ok(_win_config) => {},
    Err(error) => { println!("Error initialising: {:?}", error); }, //return Err(error); },
    }


// save configuration and states, de-initialise everything:
match config::exit(config_filename, win_config)
    {
    Ok(_)      => {},
    Err(error) => { println!("Error de-initialising: {:?}", error); return Err(error); },
    }

Ok(())
}
