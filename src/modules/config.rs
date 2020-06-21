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
const NAME_OF_INI_FILE:  &str = "shardoverse.ini";

const DEFAULT_TITLE:  &str = "Shardoverse (default)";
const DEFAULT_POS_X:   u16 = 100;
const DEFAULT_POS_Y:   u16 = 100;
const DEFAULT_HEIGHT:  u16 = 600;
const DEFAULT_WIDTH:   u16 = 800;
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
    pub pos_x:  u16,
    pub pos_y:  u16,
    pub height: u16,
    pub width:  u16,
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
## PARAMETER:  -- <none>
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
pub fn load() -> Result<WindowConfig, io::Error>  
{
println!("config::load() called");

// create config struct with default values: ---------
let mut win_config = WindowConfig::default();

println!("Values in local default struct:");
println!("default title      {:?}", win_config.title);
println!("default win_pos_x  {:?}", win_config.pos_x);
println!("default win_pos_y  {:?}", win_config.pos_y);
println!("default win_width  {:?}", win_config.width);
println!("default win_pos_x  {:?}", win_config.pos_x);
println!("default active     {:?}", win_config.active);


// reading content of ini-file: ---------------------
let mut conf = Ini::load_from_file(NAME_OF_INI_FILE).unwrap();

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
## PARAMETER:  -- <none>
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
pub fn save(win_config: WindowConfig) -> Result<bool, io::Error> 
{
println!("config::save() called");

println!("config::save() got title : {}", win_config.title ); 
println!("config::save() got pos_x : {}", win_config.pos_x );
println!("config::save() got pos_y : {}", win_config.pos_y );
println!("config::save() got width : {}", win_config.width );
println!("config::save() got height: {}", win_config.height);
println!("config::save() got active: {}", win_config.active);

let mut conf = Ini::new();

conf.with_section(None::<String>)
    .set("encoding", "utf-8");

conf.with_section(Some("window"))
    .set("title",  win_config.title )
    .set("pos_x",  win_config.pos_x .to_string() )
    .set("pos_y",  win_config.pos_y .to_string() )
    .set("width",  win_config.width .to_string() )
    .set("height", win_config.height.to_string() )
    .set("active", win_config.active.to_string() )
    ;

conf.write_to_file(NAME_OF_INI_FILE).unwrap();

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

match load()
    {
    Ok(config) => { win_config = config; },
    Err(error) => { println!("Error loading config: {:?}", error); return Err(error); },
    }

println!("load config gave title : {}", win_config.title ); 
println!("load config gave pos_x : {}", win_config.pos_x );
println!("load config gave pos_y : {}", win_config.pos_y );
println!("load config gave width : {}", win_config.width );
println!("load config gave height: {}", win_config.height);
println!("load config gave active: {}", win_config.active);

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

match save(win_config)
    {
    Ok(_)      => {},
    Err(error) => { println!("Error saving config: {:?}", error); return Err(error); },
    }

Ok(true)
}


#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /*
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## FUNCTION:   test_init()
  ## TYPE:       unit test function
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## PARAMETER:  -
  ## RETURNS:    -
  ## ---------------------------------------------------------------------------------------------------------------------------
  */
  #[test]
  fn test_init() 
  {
    let result = init();
    assert!(result.is_ok());
  }

    use std::fs::File;
    use std::io::Write;
    use crate::config::Config;
    use super::*;
    use std::{env, fs};

    #[test]
    fn new_value() {
        let default = Config::default();
        assert_ne!("http://127.0.0.1", default.url);

        let simple = r#"
        url=http://127.0.0.1
        "#;

        let config = Config::from(simple.as_bytes()).unwrap();

        assert_eq!("http://127.0.0.1", config.url);
    }

    #[test]
    fn all_values() {
        let default = Config::default();
        assert_ne!("http://127.0.0.1", default.url);

        let simple = r#"
        url=http://127.0.0.1
        limit=5
        command=echo
        keys=abcdefghij
        "#;

        let config = Config::from(simple.as_bytes()).unwrap();

        assert_eq!("http://127.0.0.1", config.url);
        assert_eq!(5, config.limit);
        assert_eq!("echo", config.command);
        assert_eq!(Alphabet::from("abcdefghij"), config.keys);
    }

    #[test]
    fn default_values() {
        let default = Config::default();

        let simple = r#"
        url=http://127.0.0.1
        "#;

        let config = Config::from(simple.as_bytes()).unwrap();

        assert_eq!(default.keys, config.keys);
        assert_eq!(default.command, config.command);
        assert_eq!(default.limit, config.limit);
    }

    #[test]
    fn from_file() {
        let path = env::temp_dir().join("config");
        let mut file = File::create(path.clone()).unwrap();
        file.write_all(b"url=http://127.0.0.1").unwrap();

        let config = Config::from(File::open(path.clone()).unwrap()).unwrap();

        assert_eq!("http://127.0.0.1", config.url);

        fs::remove_file(path).unwrap();
    }

} // End of: mod test

