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
//! 0.1      | 2020-04-04 | Clunion   | creation   
//! 1.1      | 2020-06-## | Clunion   | changed comment style to markdown for rustdoc   
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! shardoverse(.exe)
//!     Starts the program shardoverse. 
//!     Depending on the operating system the name may differ: on MS-Windows the program file has the extension '.exe'.
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

//___ CRATES EXTERNAL: ________________________________________________________________________________________________________
extern crate clap;

//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
mod modules;                              // <dirname>
mod central_core;                         // <filename>

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
//use std::env;
use std::io;

use clap::Arg;

use log::{trace, debug, info, warn, error};
use flexi_logger::{Logger, Duplicate, Cleanup, Criterion, Naming};

use crate::modules::*;                    // crate::<dirname>::*
use crate::modules::config::*;            // crate::<dirname>::<filename>::*
use crate::central_core::*;               // crate::<filename>::*

 
//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  main   
/// **`TYPE:       `**  program entry point   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`<none>     `**    
/// **`RETURNS:    `** **`Result --> `** - OK(state)   
/// **`            `** **`       --> `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// The one and only main: startup and entry point of this program.   
/// Here the handling of commandline parameters and calls to initialize und de-initialize are done.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-04-## | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
///  * define command line arguments for all configuration switches and variables    
///  * add recognition and handling of debug mode (compile switch/definition?)   
///  * add recognition and handling of testing mode   
/// ___________________________________________________________________________________________________________________________

fn main() -> Result<(), io::Error>
{
let mut shard_config: ShardConfig = ShardConfig::default();

// Initialize flexi_logger, see documentation of Struct flexi_logger::LogSpecification:
match Logger::with_env_or_str("warn, shardoverse::central_core=debug, shardoverse::modules::pixel_draw=debug")
            .check_parser_error()
            .unwrap_or_else(|e| panic!("Logger initialization failed with {:?}", e))
            .log_to_file()
            .rotate(Criterion::Size(100_000), Naming::Timestamps, Cleanup::KeepLogAndZipFiles(4,10))
            .duplicate_to_stderr(Duplicate::Trace)
            .directory("log")
            .format_for_stderr(shard_log::console_line_format)
            .format_for_files( shard_log::file_line_format)     
            .start()
    {
    Ok(_reconf_handle) => {},
    Err(error)         => { println!("ERROR initializing flexi_logger: {:?}", error); }, // return Err(error); },
    }

if   cfg!(debug_assertions) {println!("compiled in DEBUG mode");   }
else                        {println!("compiled in RELEASE mode"); }

trace!("this is a  trace message");
debug!("this is a  debug {}", "message");
info!( "this is an info"); 
warn!( "this is a  warn message");
error!("this is an error");

// Parse the command line using clap:
let cmd_line = clap::App::new("Shardoverse")
                   .version("0.1")
                   .author("Clunion <Christian.Lunau@gmx.de>")
                   .about("A Roguelike Peer-to-Peer Multi Player Dungeon Explorer.")
                   .arg(Arg::with_name("configfile")                   // <--CONFIG-File-------------------------------
                       .short("c")
                       .long("configfile")
                       .value_name("FILE")
                       .help("Sets a specific config file.")
                       .takes_value(true))
                   .arg(Arg::with_name("windowreset")                 // <--WINDOW-RESET -----------------------------
                       .short("w")
                       .long("windowreset")
                       .help("Resets the window size and position to default values.")
                       .takes_value(false))
                   .arg(Arg::with_name("verbosity")                    // <--VERBOSITY --------------------------------
                       .short("v")
                       .multiple(true)
                       .help("Sets the level of verbosity, more vs, more chatter."))
                   .arg(Arg::with_name("test-mode")                    // <--TEST-MODE---------------------------------
                       .help("Lets the program run in testing mode.")
                       .short("t")
                       .long("test")
                       .takes_value(false))
                   .arg(Arg::with_name("debug-mode")                   // <--DEBUG-MODE--------------------------------
                       .short("d")
                       .long("debug")
                       .help("Lets the program run in debug mode.")
                       .takes_value(false))
                   .arg(Arg::with_name("training-mode")                // <--TRAINING-MODE-----------------------------
                       .short("r")
                       .long("train")
                       .help("Lets the program run in training mode.")
                       .takes_value(false))
                   .get_matches();

// Get the name of a config-file, if supplied on commandline, or defaults to config::INI_FILE_NAME
let config_filename = cmd_line.value_of("configfile").unwrap_or(config::INI_FILE_NAME);
shard_config.ini_file_name   = config_filename.to_string();
info!("config-file: {}", shard_config.ini_file_name);

// Load configuration, states and assets, initialize everything:
match config::init(config_filename)
    {
    Ok(config)  => { shard_config = config; },
    Err(error)  => { error!("Error initializing: {:?}", error); return Err(error); },
    }

// Increase the amount of output based on how many times the user used the "verbose" flag (i.e. 'myprog -v' or 'myprog -vvv' vs 'myprog -v -v -v':
match cmd_line.occurrences_of("verbosity") 
    {
    0 => {shard_config.verbosity = 0; info!("Verbosity={}, No verbose info"     ,shard_config.verbosity); }, 
    1 => {shard_config.verbosity = 1; info!("Verbosity={}, Some verbose info"   ,shard_config.verbosity); },
    2 => {shard_config.verbosity = 2; info!("Verbosity={}, Tons of verbose info",shard_config.verbosity); },
    3 => {shard_config.verbosity = 3; info!("Verbosity={}, Don't be crazy"      ,shard_config.verbosity); },
    _ => {shard_config.verbosity = 9; info!("Verbosity={}, Maximum verbosity"   ,shard_config.verbosity); },
    }

// Handle the existence of command line parameters by matching over name:
if  cmd_line.is_present("test-mode")     {info!("Test Mode enabled")    ; shard_config.test        = true; }
if  cmd_line.is_present("debug-mode")    {info!("Debug Mode enabled")   ; shard_config.debug       = true; } 
if  cmd_line.is_present("training-mode") {info!("Training Mode enabled"); shard_config.training    = true; }
if  cmd_line.is_present("windowreset")   
    {
    info!("Window reset detected"); 
    // Reset the windows coordinates and size:
    shard_config = shard_config.reset_all_windows(); 
    };

if  cmd_line.is_present("configrecreate")   
    {
    info!("Config recreate detected"); 
    // Recreates the configuration settings, including config sections, windows coordinates, sizes, titles, ...:
    shard_config = shard_config.recreate_all_configs(); 
    };


// Hand over control to central core:
match run_central_core(&mut shard_config)
    {
    Ok(_shard_config) => {},
    Err(error) => { error!("running central core: {:?}", error); }, // return Err(error); },
    }

debug!("back in main()"); 

// Save configuration and states, de-initialize everything:
match config::exit(config_filename, shard_config)
    {
    Ok(_)      => {},
    Err(error) => { error!("de-initializing: {:?}", error); return Err(error); },
    }

Ok(())
}
