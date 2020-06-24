/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     config.rs
## SYNOPSIS: functions for loading, storing, reding and manipulating the configuration information
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A Roguelike Peer-to-Peer Multi Player Dungeon Explorer and Fortres Builder (?) Game written in Rust
##----------------------------------------------------------------------------------------------------------------------------
## LICENSE:
## Copyright 2020 by Christian Lunau (clunion), Julian Lunau and Jaron Lunau.
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
// Extern crate declarations only in main.rs (to be reevaluated later)

extern crate ini;
use ini::Ini;

use std::io;

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
use crate::modules::assets::cursors;   // <dirname>::<filename>::<explicit mod name>


//--- CONSTANTS: -------------------------------------------------------------------------------------------------------------
const NAME_OF_INI_FILE:      &str = "shardoverse.ini";

#[allow(dead_code)]
const NAME_OF_INI_FILE4TEST: &str = "shardoverse_test.ini";

const DEFAULT_TITLE : &str = "Shardoverse (default)";
const DEFAULT_POS_X :  i32 = 100;
const DEFAULT_POS_Y :  i32 = 100;
const DEFAULT_WIDTH :  u32 = 800;
const DEFAULT_HEIGHT:  u32 = 600;
const DEFAULT_ACTIVE: bool = true;


//--- TYPES: -----------------------------------------------------------------------------------------------------------------
//--- none ---

//--- ENUMS: -----------------------------------------------------------------------------------------------------------------
//--- none ---

//--- STRUCT DEFINITIONS: ----------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct WindowConfig
{
    pub title:  String,
    pub pos_x:  i32,
    pub pos_y:  i32,
    pub height: u32,
    pub width:  u32,
    pub active: bool,
}

//--- GLOBAL VARS: -----------------------------------------------------------------------------------------------------------
//--- none ---


//--- METHODS: ---------------------------------------------------------------------------------------------------------------
impl Default for WindowConfig 
{
    fn default() -> Self 
    {
        WindowConfig 
        {
        title:  DEFAULT_TITLE.to_owned(),
        pos_x:  DEFAULT_POS_X,
        pos_y:  DEFAULT_POS_Y,
        height: DEFAULT_HEIGHT,
        width:  DEFAULT_WIDTH,
        active: DEFAULT_ACTIVE,
        }
    }
}

 
/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   load
## TYPE:       local
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  -- ini_filename_p  - Name of the INI-file to load the config from
## RETURNS:    -- status flag: true = succesfull, flase = failed
##             or if failed: error info 
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## loads the current configuration from the pre defined file NAME_OF_INI_FILE
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 0.1         2020-06-20  CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##  everything
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn load(ini_filename_p: &str) -> Result<WindowConfig, io::Error>  
{
println!("config::load() called");

// create config struct with default values: ---------
let mut win_config = WindowConfig::default();

println!("Values in local default struct:");
println!("default title      {:?}", win_config.title);
println!("default win_pos_x  {:?}", win_config.pos_x);
println!("default win_pos_y  {:?}", win_config.pos_y);
println!("default win_width  {:?}", win_config.width);
println!("default win_height {:?}", win_config.height);
println!("default active     {:?}", win_config.active);


// reading content of ini-file: ---------------------
let mut conf = Ini::load_from_file(ini_filename_p).unwrap();

// print all key-value data from the config ini-file:
println!("\nAll values in ini file:");
for (key, value) in &conf 
    {
    println!("{:#?}:{:?}\n", key, value);
    }

//  transfering ini-file content to local struct:
//let mut  config          = Window::default();
let     _section_global  = conf.delete(None::<String>).unwrap();
let mut  section_window  = conf.delete(Some("window")).unwrap();

if let Some(mut value)  = section_window.remove("title")  { win_config.title  = value .drain(..).collect(); }
if let Some(pos_x )     = section_window.remove("pos_x")  { win_config.pos_x  = pos_x .parse().unwrap() }
if let Some(pos_y )     = section_window.remove("pos_y")  { win_config.pos_y  = pos_y .parse().unwrap() }
if let Some(width )     = section_window.remove("width")  { win_config.width  = width .parse().unwrap() }
if let Some(height)     = section_window.remove("height") { win_config.height = height.parse().unwrap() }
if let Some(active)     = section_window.remove("active") { win_config.active = active.parse().unwrap() }

println!("Values read into struct:");
println!("win_pos_x  {:?}", win_config.title);
println!("win_pos_x  {:?}", win_config.pos_x);
println!("win_pos_y  {:?}", win_config.pos_y);
println!("win_width  {:?}", win_config.width);
println!("win_height {:?}", win_config.height);
println!("win_active {:?}", win_config.active);

//  putting ini-files content into struct:

Ok(win_config)
}


/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   save
## TYPE:       common function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  -- ini_filename_p  - Name of the INI-file to store the config in
##                win_config_p    - structz holding the current config to be stored
## RETURNS:    -- status flag: true = succesfull, flase = failed
##             or if failed: error info 
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Saves the current configuration into the pre defined INI-file NAME_OF_INI_FILE
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 0.1         2020-06-20  CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##  everything
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn save(ini_filename_p: &str, win_config_p: WindowConfig) -> Result<bool, io::Error> 
{
println!("config::save() called");

println!("config::save() got title : {}", win_config_p.title ); 
println!("config::save() got pos_x : {}", win_config_p.pos_x );
println!("config::save() got pos_y : {}", win_config_p.pos_y );
println!("config::save() got width : {}", win_config_p.width );
println!("config::save() got height: {}", win_config_p.height);
println!("config::save() got active: {}", win_config_p.active);

let mut conf = Ini::new();

conf.with_section(None::<String>)
    .set("encoding", "utf-8");

conf.with_section(Some("window"))
    .set("title",  win_config_p.title )
    .set("pos_x",  win_config_p.pos_x .to_string() )
    .set("pos_y",  win_config_p.pos_y .to_string() )
    .set("width",  win_config_p.width .to_string() )
    .set("height", win_config_p.height.to_string() )
    .set("active", win_config_p.active.to_string() )
    ;

conf.write_to_file(ini_filename_p).unwrap();

Ok(true)
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   init
## TYPE:       initializing function, meant to be called exactly once at startup of the program
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  -- <none>
## RETURNS:    -- status flag: true = succesfull, flase = failed
##             or if failed: error info 
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## <textual description of the function, stating:
##  1.: the purpose of this function (goal, WHAT shall be achieved with it)
##  2.: the way this function works (HOW its works)> 
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 0.1         2020-MM-DD  CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##  everything
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn init() -> Result<WindowConfig, io::Error> 
{
println!("init() called");

let win_config: WindowConfig;

match load(NAME_OF_INI_FILE)
    {
    Ok(config) => { win_config = config; },
    Err(error) => { println!("Error loading config: {:?}", error); return Err(error); },
    }

println!("load config found title : {}", win_config.title ); 
println!("load config found pos_x : {}", win_config.pos_x );
println!("load config found pos_y : {}", win_config.pos_y );
println!("load config found width : {}", win_config.width );
println!("load config found height: {}", win_config.height);
println!("load config found active: {}", win_config.active);

match cursors::load()
    {
    Ok(_)      => {},
    Err(error) => { println!("Error loading cursors: {:?}", error); return Err(error); },
    }
    
Ok(win_config)
}

/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   exit
## TYPE:       de-initializing function, meant to be called exactly once at teardown of the program
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  -- <none>
## RETURNS:    -- status flag: true = succesfull, flase = failed
##             or if failed: error info 
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## <textual description of the function, stating:
##  1.: the purpose of this function (goal, WHAT shall be achieved with it)
##  2.: the way this function works (HOW its works)> 
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 0.1         2020-MM-DD  CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
##  everything
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn exit(win_config: WindowConfig) -> Result<bool, io::Error> 
{
println!("exit() called");

match save(NAME_OF_INI_FILE,win_config)
    {
    Ok(_)      => {},
    Err(error) => { println!("Error saving config: {:?}", error); return Err(error); },
    }

Ok(true)
}

  /*
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## TESTMODULE: for config  
  ## TYPE:       unit test functions
  ## ---------------------------------------------------------------------------------------------------------------------------
  */
#[cfg(test)]
mod tests 
{
  use super::*;            // importing names from outer (for mod tests) scope:
  use crate::config::WindowConfig;
  use std::fs;
 
//----------------------------------------------------------------
  #[test]
  fn test_init() 
  {
    let result = init();
    assert!(result.is_ok());
  }

//----------------------------------------------------------------
  #[test]
  fn test_load() 
  {
    let conf = load(NAME_OF_INI_FILE);  // load is non-destructive, thus the normal ini-file is used for load-testing.
    assert!(conf.is_ok());
  }

//----------------------------------------------------------------
  #[test]
  fn default_values() {
      let defaults = WindowConfig::default();

      assert_eq!(defaults.title , DEFAULT_TITLE );
      assert_eq!(defaults.pos_x , DEFAULT_POS_X );
      assert_eq!(defaults.pos_y , DEFAULT_POS_Y );
      assert_eq!(defaults.width , DEFAULT_WIDTH );
      assert_eq!(defaults.height, DEFAULT_HEIGHT);
      assert_eq!(defaults.active, DEFAULT_ACTIVE);
  }

//----------------------------------------------------------------
  #[test]
  fn set_and_check_all_values() 
  {
  let test_conf2write: WindowConfig = WindowConfig 
    {
    title : "test window title".to_string(),
    pos_x : 1 ,
    pos_y : 2 ,
    width : 3 ,
    height: 4 ,
    active: false ,
    };

    match save(NAME_OF_INI_FILE4TEST,test_conf2write)
        {
        Ok(_)  => {},
        Err(_) => { assert!(false) },
        }

  let mut test_conf2load: WindowConfig = WindowConfig::default();
  
  match load(NAME_OF_INI_FILE4TEST)
      {
      Ok(config) => { test_conf2load = config; },
      Err(_)     => { assert!(false) },
      }

  fs::remove_file(NAME_OF_INI_FILE4TEST).unwrap();

  assert_eq!(test_conf2load.title , "test window title");
  assert_eq!(test_conf2load.pos_x , 1);
  assert_eq!(test_conf2load.pos_y , 2);
  assert_eq!(test_conf2load.width , 3);
  assert_eq!(test_conf2load.height, 4);
  assert_eq!(test_conf2load.active, false);

  }

} // End of: mod test

