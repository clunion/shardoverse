//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** config.rs 🦀   
//! **`DESCRIPTION:`** functions for loading, storing, reading and manipulating the configuration information   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-04-04 | Clunion   | creation   
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * everything (nearly)   
//! ___________________________________________________________________________________________________________________________
//!    


//___ MODULES EXTERNAL: _______________________________________________________________________________________________________
// Extern crate declarations only in main.rs (to be reevaluated later)

extern crate ini;
use ini::Ini;

use std::io;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

//___ MODULES LOCAL: __________________________________________________________________________________________________________
use crate::modules::assets::cursors;   // <dirname>::<filename>::<explicit mod name>


//___ CONSTANTS: ______________________________________________________________________________________________________________
pub const NAME_OF_INI_FILE:  &str = "shardoverse.ini";

#[allow(dead_code)]
const NAME_OF_INI_FILE4TEST: &str = "shardoverse_test.ini";

const DEFAULT_TITLE : &str = "Shardoverse (default)";
const DEFAULT_POS_X :  i32 = 100;
const DEFAULT_POS_Y :  i32 = 100;
const DEFAULT_WIDTH :  u32 = 800;
const DEFAULT_HEIGHT:  u32 = 600;
const DEFAULT_ACTIVE: bool = true;


//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCT DEFINITIONS: _____________________________________________________________________________________________________

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

#[derive(Debug)]
pub struct ShardConfig
{
    pub verbosity:   u8,
    pub debug:       bool,
    pub test:        bool,
    pub training:    bool,
    pub windowreset: bool,
    pub file:        String,
    pub window:      WindowConfig,
}


//___ METHODS: ________________________________________________________________________________________________________________
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

impl Default for ShardConfig 
{
    fn default() -> Self 
    {
        ShardConfig 
        {
        verbosity:   0,
        debug:       false,
        test:        false,
        training:    false,
        windowreset: false,
        file:        NAME_OF_INI_FILE.to_owned(),
        window:      WindowConfig::default(),
        }
    }
}

 
/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  load   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` ini_filename_p`** Name of the INI-file to load the config from   
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = succesfull, flase = failed)   
/// **`            `** **`     or -->    `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// loads the current configuration from the file with the name given in ini_filename_p.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-06-20 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub fn load(ini_filename_p: &str) -> Result<ShardConfig, io::Error>  
{
debug!("config::load() called");

// create config struct with default values: ---------
let mut shard_config = ShardConfig::default();

debug!("Values in local default struct:");
debug!("default title      {:?}", shard_config.window.title);
debug!("default win_pos_x  {:?}", shard_config.window.pos_x);
debug!("default win_pos_y  {:?}", shard_config.window.pos_y);
debug!("default win_width  {:?}", shard_config.window.width);
debug!("default win_height {:?}", shard_config.window.height);
debug!("default active     {:?}", shard_config.window.active);


// reading content of ini-file: ---------------------
let mut conf = Ini::load_from_file(ini_filename_p).unwrap();

// print all key-value data from the config ini-file:
debug!("All values in ini file:");
for (key, value) in &conf 
    {
    debug!("{:?}:{:?}", key, value);
    }

//  transfering ini-file content to local struct:
//let mut  config          = Window::default();
let     _section_global  = conf.delete(None::<String>).unwrap();
let mut  section_window  = conf.delete(Some("window")).unwrap();

if let Some(mut value)  = section_window.remove("title")  { shard_config.window.title  = value .drain(..).collect(); }
if let Some(pos_x )     = section_window.remove("pos_x")  { shard_config.window.pos_x  = pos_x .parse().unwrap() }
if let Some(pos_y )     = section_window.remove("pos_y")  { shard_config.window.pos_y  = pos_y .parse().unwrap() }
if let Some(width )     = section_window.remove("width")  { shard_config.window.width  = width .parse().unwrap() }
if let Some(height)     = section_window.remove("height") { shard_config.window.height = height.parse().unwrap() }
if let Some(active)     = section_window.remove("active") { shard_config.window.active = active.parse().unwrap() }

debug!("Values read into struct:");
debug!("win_pos_x  {:?}", shard_config.window.title);
debug!("win_pos_x  {:?}", shard_config.window.pos_x);
debug!("win_pos_y  {:?}", shard_config.window.pos_y);
debug!("win_width  {:?}", shard_config.window.width);
debug!("win_height {:?}", shard_config.window.height);
debug!("win_active {:?}", shard_config.window.active);

//  putting ini-files content into struct:

Ok(shard_config)
}


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  save   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` ini_filename_p `** Name of the INI-file to store the config in   
/// **`            `** **` shard_config_p `** struct holding the current config to be stored   
/// **`RETURNS:    `** **` Result -->     `** OK(status flag: true = succesfull, flase = failed)   
/// **`            `** **`     or -->     `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Saves the current configuration into the file with the name given in ini_filename_p.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-06-20 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub fn save(ini_filename_p: &str, shard_config_p: ShardConfig) -> Result<bool, io::Error> 
{
debug!("config::save() called");

debug!("config::save() got title : {}", shard_config_p.window.title ); 
debug!("config::save() got pos_x : {}", shard_config_p.window.pos_x );
debug!("config::save() got pos_y : {}", shard_config_p.window.pos_y );
debug!("config::save() got width : {}", shard_config_p.window.width );
debug!("config::save() got height: {}", shard_config_p.window.height);
debug!("config::save() got active: {}", shard_config_p.window.active);

let mut conf = Ini::new();

conf.with_section(None::<String>)
    .set("encoding", "utf-8");

conf.with_section(Some("window"))
    .set("title",  shard_config_p.window.title )
    .set("pos_x",  shard_config_p.window.pos_x .to_string() )
    .set("pos_y",  shard_config_p.window.pos_y .to_string() )
    .set("width",  shard_config_p.window.width .to_string() )
    .set("height", shard_config_p.window.height.to_string() )
    .set("active", shard_config_p.window.active.to_string() )
    ;

conf.write_to_file(ini_filename_p).unwrap();

Ok(true)
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  init   
/// **`TYPE:       `**  initializing function, meant to be called exactly once at startup of the program   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` ini_filename_p `** Name of the INI-file to store the config in   
/// **`RETURNS:    `** **` Result -->     `** OK(status flag: true = succesfull, flase = failed)   
/// **`            `** **`     or -->     `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// <textual description of the function, stating:   
///   1.: the purpose of this function (goal, WHAT shall be achieved with it)   
///   2.: the way this function works (HOW its works)>    
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-06-20 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub fn init(ini_filename_p: &str) -> Result<ShardConfig, io::Error> 
{
debug!("init() called");

let shard_config: ShardConfig;

match load(ini_filename_p)
    {
    Ok(config) => { shard_config = config; },
    Err(error) => { error!("Error loading config: {:?}", error); return Err(error); },
    }

debug!("load config found title : {}", shard_config.window.title ); 
debug!("load config found pos_x : {}", shard_config.window.pos_x );
debug!("load config found pos_y : {}", shard_config.window.pos_y );
debug!("load config found width : {}", shard_config.window.width );
debug!("load config found height: {}", shard_config.window.height);
debug!("load config found active: {}", shard_config.window.active);

match cursors::load()
    {
    Ok(_)      => {},
    Err(error) => { error!("Error loading cursors: {:?}", error); return Err(error); },
    }
    
Ok(shard_config)
}

/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `** exit   
/// **`TYPE:       `** de-initializing function, meant to be called exactly once at teardown of the program   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` ini_filename_p `** Name of the INI-file to store the config in   
/// **`            `** **` shard_config_p `** struct holding the current config to be stored   
/// **`RETURNS:    `** **` Result -->     `** OK(status flag: true = succesfull, false = failed)   
/// **`            `** **`     or -->     `** Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// de-initialises everything which needs to be freed, saves current state 
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 0.1     | 2020-06-20 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub fn exit(ini_filename_p: &str, shard_config: ShardConfig) -> Result<bool, io::Error> 
{
debug!("exit() called");

match save(ini_filename_p,shard_config)
    {
    Ok(_)      => {},
    Err(error) => { error!("Error saving config: {:?}", error); return Err(error); },
    }

Ok(true)
}

/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for config   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
  use super::*;            // importing names from outer (for mod tests) scope
  use crate::config::WindowConfig;
  use std::fs;
 
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_init()   
  /// **`TYPE:       `** unit test   
  /// ___________________________________________________________________________________________________________________________
  /// **`RETURNS:    `** **` <none>     `**   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_init() 
  {
    let result = init(NAME_OF_INI_FILE);
    assert!(result.is_ok());
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_load()   
  /// **`TYPE:       `** unit test   
  /// ___________________________________________________________________________________________________________________________
  /// **`RETURNS:    `** **` <none>     `**   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_load() 
  {
    let conf = load(NAME_OF_INI_FILE);  // load is non-destructive, thus the normal ini-file is used for load-testing.
    assert!(conf.is_ok());
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_default_values()   
  /// **`TYPE:       `** unit test   
  /// ___________________________________________________________________________________________________________________________
  /// **`RETURNS:    `** **` <none>     `**   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_default_values() {
      let defaults = WindowConfig::default();

      assert_eq!(defaults.title , DEFAULT_TITLE );
      assert_eq!(defaults.pos_x , DEFAULT_POS_X );
      assert_eq!(defaults.pos_y , DEFAULT_POS_Y );
      assert_eq!(defaults.width , DEFAULT_WIDTH );
      assert_eq!(defaults.height, DEFAULT_HEIGHT);
      assert_eq!(defaults.active, DEFAULT_ACTIVE);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** set_and_check_all_values()   
  /// **`TYPE:       `** unit test   
  /// ___________________________________________________________________________________________________________________________
  /// **`RETURNS:    `** **` <none>     `**   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn set_and_check_all_values() 
  {
  let test_win_conf2write: WindowConfig = WindowConfig 
    {
    title : "test window title".to_string(),
    pos_x : 1 ,
    pos_y : 2 ,
    width : 3 ,
    height: 4 ,
    active: false ,
    };

  let test_conf2write: ShardConfig = ShardConfig 
    {
    verbosity:   2,
    debug:       true,
    test:        true,
    training:    true,
    windowreset: true,
    file:        "test ini filename".to_string(),
    window:      test_win_conf2write ,
    };

    match save(NAME_OF_INI_FILE4TEST,test_conf2write)
        {
        Ok(_)  => {},
        Err(_) => { assert!(false) },
        }

  let mut test_conf2load: ShardConfig = ShardConfig::default();
  
  match load(NAME_OF_INI_FILE4TEST)
      {
      Ok(config) => { test_conf2load = config; },
      Err(_)     => { assert!(false) },
      }

  fs::remove_file(NAME_OF_INI_FILE4TEST).unwrap();

  assert_eq!(test_conf2load.verbosity    , 0);                             // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!
  assert_eq!(test_conf2load.debug        , false);                         // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!
  assert_eq!(test_conf2load.test         , false);                         // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!
  assert_eq!(test_conf2load.training     , false);                         // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!
  assert_eq!(test_conf2load.windowreset  , false);                         // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!
  assert_eq!(test_conf2load.file         , NAME_OF_INI_FILE.to_string());  // default werte, da diese werte derzeit nicht in die Ini-Datzei gespeichert werden!

  assert_eq!(test_conf2load.window.title , "test window title");
  assert_eq!(test_conf2load.window.pos_x , 1);
  assert_eq!(test_conf2load.window.pos_y , 2);
  assert_eq!(test_conf2load.window.width , 3);
  assert_eq!(test_conf2load.window.height, 4);
  assert_eq!(test_conf2load.window.active, false);

  }

} // End of: mod test

