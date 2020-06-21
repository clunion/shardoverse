/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     main.rs
## SYNOPSIS: main, start and entry point of the program
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A Roguelike Peer-to-Peer Multi Player Dungeon Explorer and Fortres Builder (?) Game written in Rust
##----------------------------------------------------------------------------------------------------------------------------
## LICENSE:
## Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).
## MIT-License, see LICENSE.md file 
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:  DATE:       AUTHOR: CHANGES:
## 0.1       2020-04-04  CLu     creation
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##    - everything
## ---------------------------------------------------------------------------------------------------------------------------

*/

//--- CRATES EXTERNAL: -------------------------------------------------------------------------------------------------------
extern crate sdl2;

//--- MODULES EXTERNAL: ------------------------------------------------------------------------------------------------------
use std::env;
use std::io;
use std::path::Path;


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


/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   main
## TYPE:       program entry point
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  <none>
## RETURNS:    Result - state (OK)
##                    - Error
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## The one and only main: startup and entry point of this program
## here only the handling of commandline paramaters is done
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn main() -> Result<(), io::Error>
{
let args: Vec<String> = env::args().collect();
let mut i :i32 = 0;
let win_config: WindowConfig;

// check for command line arguments:
if !args.is_empty()
    {
    for arg in &args
        {
        println!("Command line argument(s)[{}] {:?}",i, &arg);
        i+=1;
        }
    } 


// load configuration, states and assets, initialise everything:
match config::init()
    {
    Ok(config)  => { win_config = config; },
    Err(error)  => { println!("Error initialising: {:?}", error); return Err(error); },
    }

println!("init returned title : {}", win_config.title ); 
println!("init returned pos_x : {}", win_config.pos_x );
println!("init returned pos_y : {}", win_config.pos_y );
println!("init returned width : {}", win_config.width );
println!("init returned height: {}", win_config.height);
println!("init returned active: {}", win_config.active);


// Hand over control to central core:
match run(Path::new("assets/cursors/pointers_part_5/glove3.png"))
    {
    Ok(_)      => {},
    Err(error) => { println!("Error initialising: {:?}", error); }, //return Err(error); },
    }


// save configuration and states, de-initialise everything:
match config::exit(win_config)
    {
    Ok(_)      => {},
    Err(error) => { println!("Error de-initialising: {:?}", error); return Err(error); },
    }

Ok(())
}
