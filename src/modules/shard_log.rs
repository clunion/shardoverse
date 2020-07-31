//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** shard_log.rs 🦀   
//! **`DESCRIPTION:`** configuration and additional functions for logging. Requiers the crates log and flexi_logger   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-07-10 | Clunion   | creation   
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * add more tests
//! ___________________________________________________________________________________________________________________________
//!    

//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use flexi_logger::{Record, DeferredNow};
use yansi::{Color, Style};

use crate::modules::*;      // crate::<dirname>::*

//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  console_line_format   
/// **`TYPE:       `**  public, callback for felxi_logger   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` w             `** (output stream?)   
/// **`            `** **` now           `** the timestamp of the moment the log macro is called   
/// **`            `** **` record        `** a struct with the log metadata and the message   
/// **`RETURNS:    `** **` Result -->    `** OK()   
/// **`            `** **`     or -->    `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// logline-formatter that produces log lines with timestamp and file location, like
/// <br>
/// ```[15:25:01  INFO       file.rs[ 26] This is an info message```
/// <br>
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-07-08 | Clunion   | initial version, based on flexi_logger::colored_opt_format   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * move the definition of the styles to a one-time initialiser or change into static or a macro...   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn console_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
{
let level = record.level();
let shard_style: Style;

let  error_style: Style = Style::new(Color::Red).bold().italic();  // todo: move to a one-time initializer or change into static
let   warn_style: Style = Style::new(Color::Yellow).italic()    ;  // todo: move to a one-time initializer or change into static
let   info_style: Style = Style::new(Color::Cyan)               ;  // todo: move to a one-time initializer or change into static
let  debug_style: Style = Style::new(Color::Default)            ;  // todo: move to a one-time initializer or change into static
let  trace_style: Style = Style::new(Color::Blue)               ;  // todo: move to a one-time initializer or change into static

match level 
    {
    log::Level::Error => {shard_style = error_style;},
    log::Level::Warn  => {shard_style =  warn_style;},
    log::Level::Info  => {shard_style =  info_style;},
    log::Level::Debug => {shard_style = debug_style;},
    log::Level::Trace => {shard_style = trace_style;},
    };

write!( w, 
        "{} {:5}{:>18}[{:4}] {}",
        now.now().format("%H:%M:%S"),
        shard_style.paint(record.level()),
        shard_tool::basename(record.file().unwrap_or("<unnamed>")),
        record.line().unwrap_or(0),
        &record.args()
      )
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  file_line_format   
/// **`TYPE:       `**  public, callback for felxi_logger   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` w             `** (output stream?)   
/// **`            `** **` now           `** the timestamp of the moment the log macro is called   
/// **`            `** **` record        `** a struct with the log metadata and the message   
/// **`RETURNS:    `** **` Result -->    `** OK()   
/// **`            `** **`     or -->    `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// A logline-formatter that produces log lines like
/// <br>
/// ```[2016-01-13 15:25:01.640870 +01:00]  INFO [ src/foo/bar.rs[26] This is an info message```
/// <br>
/// i.e. with timestamp and file location.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-07-08 | Clunion   | initial version, based on flexi_logger::detailed_format   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * nothing   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn file_line_format( w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record, ) -> Result<(), std::io::Error> 
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



// /// ___________________________________________________________________________________________________________________________
// /// **`TESTMODULE: `** for shard_log   
// /// **`TYPE:       `** unit test functions   
// /// ___________________________________________________________________________________________________________________________
// #[cfg(test)]
// mod tests 
// {
//   // importing names from outer (for mod tests) scope:
//   use super::cursors::*;
//   
//   /// ___________________________________________________________________________________________________________________________
//   /// **`FUNCTION:   `**  test_load()   
//   /// **`TYPE:       `**  unit test function   
//   /// ___________________________________________________________________________________________________________________________
//   /// **`PARAMETER:  `** **`<none>        `**    
//   /// **`RETURNS:    `** **`<none>        `**    
//   /// ___________________________________________________________________________________________________________________________
// 
//   #[test]
//   fn test_load() 
//   {
//     let result = load();
//     assert!(result.is_ok());
//   }
//   
// } // End of: mod test



