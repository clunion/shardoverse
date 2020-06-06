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

//--- MODULES EXTERNAL: ------------------------------------------------------------------------------------------------------
extern crate sdl2;

//--- MODULES: ---------------------------------------------------------------------------------------------------------------
use std::env;
use std::io;
use std::path::Path;

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
mod modules;                              // <dirname>

use crate::modules::assets::cursors::*;   // <dirname>::<filename>::<explicit mod name>::*
use crate::modules::config::*;            // <dirname>::<filename>::*

mod central_core;                         // <filename>
use crate::central_core::*;               // <filename>::*


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
## TYPE:       entry point, 
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  <none>
## RETURNS:    Result - state OK
##                    - io-Error
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

if !args.is_empty()
    {
    for arg in &args
        {
        println!("Parameter[{}] {:?}",i, &arg);
        i+=1;
        }
    } 

match load_config()
    {
    Ok(_)      => {},
    Err(error) => { println!("Error loading config: {:?}", error); return Err(error); },
    }


match load_cursors()
    {
    Ok(_)      => {},
    Err(error) => { println!("Error loading cursors: {:?}", error); return Err(error); },
    }


match shardoverse_init()
    {
    Ok(_)      => {},
    Err(error) => { println!("Error initialising: {:?}", error); return Err(error); },
    }


// Hand over control to central core:
match run(Path::new("assets/cursors/pointers_part_5/glove3.png"))
    {
    Ok(_)      => {},
    Err(error) => { println!("Error initialissing: {:?}", error); }, //return Err(error); },
    }


match save_config()
    {
    Ok(_)      => {},
    Err(error) => { println!("Error saving config: {:?}", error); return Err(error); },
    }

Ok(())
}
