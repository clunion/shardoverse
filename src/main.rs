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

//___ CRATES EXTERNAL: ________________________________________________________________________________________________________
extern crate sdl2;
extern crate clap;

//___ MODULES EXTERNAL: _______________________________________________________________________________________________________
//use std::env;
use std::io;
use std::path::Path;

use clap::{Arg, App};

use log::{trace, debug, info, warn, error};
use flexi_logger::{Logger, Duplicate, Record, DeferredNow, Cleanup, Criterion, Naming};
use yansi::{Color, Style};

//use crate::DeferredNow;
//use self::DeferredNow;
//use log::Record;

//___ MODULES LOCAL: __________________________________________________________________________________________________________
mod modules;                              // <dirname>
use crate::modules::*;                    // create::<dirname>::*

mod central_core;                         // <filename>
use crate::central_core::*;               // create::<filename>::*

use crate::modules::config::ShardConfig;
use crate::modules::config::WindowConfig;
 
//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

pub fn basename(path: &str) -> &str 
{
let mut pieces = path.rsplitn(2, |c| c == '/' || c == '\\');
match pieces.next() 
    {
    Some(p) => p.into(),
    None => path.into(),
    }
}


/// A logline-formatter that produces log lines with timestamp and file location, like
/// <br>
/// ```[2016-01-13 15:25:01.640870 +01:00] INFO [src/foo/bar:26] Task successfully read from conf.json```
/// <br>
pub fn shard_log_console_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
{
let level = record.level();
let shard_style: Style;

let  error_style: Style = Style::new(Color::Red).bold().italic();  // todo: move to a one-time initializer or change into static
let   warn_style: Style = Style::new(Color::Yellow).bold()      ;  // todo: move to a one-time initializer or change into static
let   info_style: Style = Style::new(Color::Cyan)               ;  // todo: move to a one-time initializer or change into static
let  debug_style: Style = Style::new(Color::Default)            ;  // todo: move to a one-time initializer or change into static
let  trace_style: Style = Style::new(Color::Blue).dimmed()      ;  // todo: move to a one-time initializer or change into static

match level 
    {
    log::Level::Error => {shard_style = error_style;},
    log::Level::Warn  => {shard_style =  warn_style;},
    log::Level::Info  => {shard_style =  info_style;},
    log::Level::Debug => {shard_style = debug_style;},
    log::Level::Trace => {shard_style = trace_style;},
    };

write!( w, 
        "{} {:>5}{:>18}[{:4}] {}",
        now.now().format("%H:%M:%S"),
        shard_style.paint(record.level()),
//      record.file().unwrap_or("<unnamed>"),   // <-- shorten with basename here
        basename(record.file().unwrap_or("<unnamed>")),   // <-- shorten with basename here
        record.line().unwrap_or(0),
        &record.args()
      )
}

/// A logline-formatter that produces log lines like
/// <br>
/// ```[2016-01-13 15:25:01.640870 +01:00] INFO [foo::bar] src/foo/bar.rs:26: Task successfully read from conf.json```
/// <br>
/// i.e. with timestamp, module path and file location.
///
/// # Errors
///
/// See `std::write`
pub fn shard_log_file_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
{
write!( w,
        "{} {:5}:{:>32}[{:4}]: {}",
        now.now().format("%Y-%m-%d %H:%M:%S%.6f %:z"),
        record.level(),
 //     record.module_path().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
      )
}

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

//initialise flexi_logger:
Logger::with_env_or_str("myprog=trace, mylib=trace")
            .log_to_file()
            .rotate(Criterion::Size(100_000), Naming::Timestamps, Cleanup::KeepLogAndZipFiles(4,10))
            .duplicate_to_stderr(Duplicate::Trace)
            .directory("log")
            .format_for_stderr(shard_log_console_line_format)
            .format_for_files( shard_log_file_line_format)     
            .start()
            .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

trace!("this is a  trace message");
debug!("this is a  debug {}", "message");
info!( "this is an info"); 
warn!( "this is a  warn message");
error!("this is an error");

// parse the command line using clap:
let cmd_line = App::new("Shardoverse")
                   .version("0.1")
                   .author("Clunion <Christian.Lunau@gmx.de>")
                   .about("A Roguelike Peer-to-Peer Multi Player Dungeon Explorer.")
                   .arg(Arg::with_name("configfile")                   // <--CONFIG-File-------------------------------
                       .short("c")
                       .long("configfile")
                       .value_name("FILE")
                       .help("Sets a specific config file.")
                       .takes_value(true))
                   .arg(Arg::with_name("window-reset")                 // <--WINDOW-RESET -----------------------------
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

// Get the name of a config-file, if supplied by user, or defaults to config::NAME_OF_INI_FILE
let config_filename = cmd_line.value_of("configfile").unwrap_or(config::NAME_OF_INI_FILE);
shard_config.file = config_filename.to_string();
info!("config-file: {}", shard_config.file);

// Vary the output based on how many times the user used the "verbose" flag
// (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
match cmd_line.occurrences_of("verbosity") 
    {
    0 => {shard_config.verbosity = 0; info!("Verbosity={}, No verbose info"     ,shard_config.verbosity); }, 
    1 => {shard_config.verbosity = 1; info!("Verbosity={}, Some verbose info"   ,shard_config.verbosity); },
    2 => {shard_config.verbosity = 2; info!("Verbosity={}, Tons of verbose info",shard_config.verbosity); },
    3 => {shard_config.verbosity = 3; info!("Verbosity={}, Don't be crazy"      ,shard_config.verbosity); },
    _ => {shard_config.verbosity = 9; info!("Verbosity={}, Maximum verbosity"   ,shard_config.verbosity); },
    }

// You can handle information about subcommands by requesting their matches by name
// (as below), requesting just the name used, or both at the same time
if cmd_line.is_present("test-mode")     {shard_config.test        = true; info!("Test Mode enabled")    ; }
if cmd_line.is_present("debug-mode")    {shard_config.debug       = true; info!("Debug Mode enabled")   ; } 
if cmd_line.is_present("training-mode") {shard_config.training    = true; info!("Training Mode enabled"); }
if cmd_line.is_present("windowreset")   {shard_config.windowreset = true; info!("Window reset detected"); }



// load configuration, states and assets, initialise everything:
match config::init(config_filename)
    {
    Ok(config)  => { shard_config = config; },
    Err(error)  => { error!("Error initialising: {:?}", error); return Err(error); },
    }

if  shard_config.windowreset
    {
    info!("windowreset will be done"); 
    shard_config.window = WindowConfig::default();
    };


// Hand over control to central core:
match run(&mut shard_config,Path::new("assets/cursors/pointers_part_5/glove3.png"))
    {
    Ok(_shard_config) => {},
    Err(error) => { error!("Error initialising: {:?}", error); }, //return Err(error); },
    }


// save configuration and states, de-initialise everything:
match config::exit(config_filename, shard_config)
    {
    Ok(_)      => {},
    Err(error) => { error!("Error de-initialising: {:?}", error); return Err(error); },
    }

Ok(())
}
