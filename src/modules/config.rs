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


//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use std::io;
use ini::Ini;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use crate::modules::asset::cursors;   // <dirname>::<filename>::<explicit mod name>


//___ CONSTANTS: ______________________________________________________________________________________________________________
pub(crate) const NAME_OF_INI_FILE:  &str = "shardoverse.ini";

#[allow(dead_code)]
const NAME_OF_INI_FILE4TEST: &str = "shardoverse_test.ini";

const DEFAULT_POS_X :  i32 = 100;
const DEFAULT_POS_Y :  i32 = 100;
const DEFAULT_WIDTH :  u32 = 800;
const DEFAULT_HEIGHT:  u32 = 600;
const DEFAULT_ACTIVE: bool = true;

const INITIAL_NUMBER_OF_WINDOW_CONFIGS: usize = 10;

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCT DEFINITIONS: _____________________________________________________________________________________________________
#[derive(Debug)]
pub(crate) struct WindowConfig
{
    pub(crate) conf_name: String,
    pub(crate) conf_num:  usize,    // id/number/index of the window in the conf-vector, and this also is the index in the Array!!
    pub(crate) title:     String,   // this means: Window-Title, later to be changed to localisation-string-ID
    pub(crate) pos_x:     i32,
    pub(crate) pos_y:     i32,
    pub(crate) height:    u32,
    pub(crate) width:     u32,
    pub(crate) active:    bool,
}

#[derive(Debug)]
pub(crate) struct ShardConfig 
{
    pub(crate)     verbosity:       u8,
    pub(crate)     debug:           bool,
    pub(crate)     test:            bool,
    pub(crate)     training:        bool,
    pub(crate)     file:            String,
    pub(crate)     window_configs:  Vec<WindowConfig>,
}


//___ METHODS: ________________________________________________________________________________________________________________
impl <'lt_winconf> Default for WindowConfig
{
    fn default() -> Self 
    {
        WindowConfig 
        {
        conf_name: "unnamed".to_string(),
        conf_num:  0,
        title:     "untitled".to_string(),
        pos_x:     DEFAULT_POS_X,
        pos_y:     DEFAULT_POS_Y,
        height:    DEFAULT_HEIGHT,
        width:     DEFAULT_WIDTH,
        active:    DEFAULT_ACTIVE,
        }
    }

}

impl <'lt_winconf> WindowConfig 
{
    #[allow(clippy::too_many_arguments)]
    fn new_complete(conf_name_p: String, conf_num_p: usize, title_p: String, pos_x_p: i32, pos_y_p: i32, height_p: u32, width_p: u32, active_p: bool) -> WindowConfig
    {
        WindowConfig 
        {
        conf_name:   conf_name_p,
        conf_num:    conf_num_p,
        title:       title_p,
        pos_x:       pos_x_p,
        pos_y:       pos_y_p,
        height:      height_p,
        width:       width_p,
        active:      active_p,
        }
    }

    fn new(conf_name_p: String) -> WindowConfig
    {
        WindowConfig 
        {
        conf_name:   conf_name_p,
        conf_num:    0,
        title:       "untitled".to_string(),
        pos_x:       DEFAULT_POS_X,
        pos_y:       DEFAULT_POS_Y,
        height:      DEFAULT_HEIGHT,
        width:       DEFAULT_WIDTH,
        active:      DEFAULT_ACTIVE,
        }            
    }
}

impl <'lt_shardconf> Default for ShardConfig
{
    fn default() -> Self 
    {
        ShardConfig 
        {
        verbosity:           0,
        debug:               false,
        test:                false,
        training:            false,
        file:                NAME_OF_INI_FILE.to_owned(),
        window_configs:      Vec::with_capacity(INITIAL_NUMBER_OF_WINDOW_CONFIGS), 
        }
    }
}

impl <'lt_shardconf> ShardConfig
{
pub(crate) fn reset_all_windows(mut self) -> Self
    {
    self.verbosity       = 0;
    self.debug           = false;
    self.test            = false;
    self.training        = false;
    self.file            = NAME_OF_INI_FILE.to_owned();

    let mut pos_offset: i32 = 0;

    for mut conf in self.window_configs.iter_mut() 
        {
//      conf_num:     conf_num_p,
//      title:        title_p,
        conf.pos_x   = DEFAULT_POS_X + pos_offset;
        conf.pos_y   = DEFAULT_POS_Y + pos_offset;
        conf.width   = DEFAULT_HEIGHT;
        conf.height  = DEFAULT_WIDTH ;
        conf.active  = DEFAULT_ACTIVE;
        pos_offset  += 10;        // todo: change to a dynamic value depending on HiDPI setting
        } // end of: 'confloop
    self
    }

pub(crate) fn recreate_all_configs(mut self) -> Self
    {
    self.verbosity       = 0;
    self.debug           = false;
    self.test            = false;
    self.training        = false;
    self.file            = NAME_OF_INI_FILE.to_owned();

    let mut conf_idx: usize = 0;
    let mut pos_offset: i32 = 0;

    self.window_configs.push( WindowConfig::new_complete("game-window"         .to_string(), conf_idx, "Shardoverse: Game"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); conf_idx += 1; pos_offset  += 10;
    self.window_configs.push( WindowConfig::new_complete("maps-window"         .to_string(), conf_idx, "Shardoverse: Maps"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); conf_idx += 1; pos_offset  += 10;
    self.window_configs.push( WindowConfig::new_complete("inventory-window"    .to_string(), conf_idx, "Shardoverse: Inventory"    .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); conf_idx += 1; pos_offset  += 10;
    self.window_configs.push( WindowConfig::new_complete("journal-window"      .to_string(), conf_idx, "Shardoverse: Journal"      .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); conf_idx += 1; pos_offset  += 10;
    self.window_configs.push( WindowConfig::new_complete("help-window"         .to_string(), conf_idx, "Shardoverse: Help"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); conf_idx += 1; pos_offset  += 10;
    self.window_configs.push( WindowConfig::new_complete("pixel-painter-window".to_string(), conf_idx, "Shardoverse: Pixel-Painter".to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_ACTIVE)); 
    self
    }

pub(crate) fn find_window_conf_index_by_conf_name(&self, conf_name_p: String) ->  Result<usize, ()>
    {
     let index = self.window_configs.iter().position(|elem| elem.conf_name == conf_name_p);
     match index
        {
        Some(index)  => Ok(index),
        None         => Err(()),
        }        
    }

}

//___ MACROS: ________________________________________________________________________________________________________________
//___ none ___

 
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

pub(crate) fn load(ini_filename_p: &str) -> Result<ShardConfig, io::Error>
{
let mut ini_conf:          Ini            = Ini::default();

let mut window_conf:       WindowConfig;
let mut window_conf_found: bool;
let mut window_conf_index: usize          = 0; 

let mut shard_config:      ShardConfig;
let     shard_conf_loaded: bool;

debug!("config::load() called");

// create and pre-fill ALL needed config settings with defaults: -----------
shard_config = ShardConfig::default(); 
shard_config = shard_config.recreate_all_configs();  // todo: combine these function into one

// read content of ini-file: -----------------------------------------------
// match Ini::load_from_file(ini_filename_p)
//     {
//     Ok(config) => { shard_conf_loaded = true;  ini_conf = config; },
//     Err(error) => { shard_conf_loaded = false; warn!("could not read config file {}: {:?}. DEFAULT VALUES will be used!",ini_filename_p, error);},
//     }


let config = Ini::load_from_file(ini_filename_p);
match config
    {
    Ok(config) => { shard_conf_loaded = true;  ini_conf = config; },
    Err(error) => { shard_conf_loaded = false; warn!("could not read config file {}: {:?}. DEFAULT VALUES will be used!",ini_filename_p, error);},
    }

if !shard_conf_loaded          // if nothing could be loaded...
    {
    return Ok(shard_config)    // ...then return config initialised wit default data!
    } 


// if debug-mode: print all key-value data from the config ini-file:
if cfg!(debug_assertions) 
    {
    println!("compiled in DEBUG mode");

    debug!("created default window-configs:");
    for win_conf in shard_config.window_configs.iter() 
        {
        debug!("conf-name: {:?}", win_conf.conf_name);
        }

    debug!("All values read from ini file '{}':",ini_filename_p);
    for (key, value) in ini_conf.iter() 
        {
        debug!("{:?}:{:?}", key, value);
        }
    } 

'conf_name_loop: for (sec, prop) in ini_conf.iter() 
    {
    debug!("conf_name_: {:?}", sec);
    
    match sec
        {
        None                         => {debug!("found global Section"           ); continue 'conf_name_loop;},
        Some("game-window"         ) => {debug!("found game-window"              ); },
        Some("maps-window"         ) => {debug!("found maps-window"              ); },
        Some("inventory-window"    ) => {debug!("found inventory-window"         ); },
        Some("journal-window"      ) => {debug!("found journal-window"           ); },
        Some("help-window"         ) => {debug!("found help-window"              ); },
        Some("pixel-painter-window") => {debug!("found pixel-painter-window"     ); },
        _                            => {debug!("found unknown section: {:?}",sec); continue 'conf_name_loop;},
        }


    if sec == Some("game-window")
        {
        debug!("Bingo!");
        }


    match shard_config.find_window_conf_index_by_conf_name(sec.unwrap().to_string())
        {
        Ok(x)   =>  {window_conf_found = true; window_conf_index = x;},
        Err(()) =>  {window_conf_found = false                },
        }
    debug!("found index for section {:?}, it's: {}",sec, window_conf_index);
    
    
//-------------------

    if window_conf_found
        {
        'keyloop: for (key, value) in prop.iter() 
            {
            debug!("{}:{}", key, value);
            match key
                {
                "conf_num"  => { shard_config.window_configs[window_conf_index].conf_num  = value.parse().unwrap(); },
                "title"     => { shard_config.window_configs[window_conf_index].title     = value.parse().unwrap(); },
                "pos_x"     => { shard_config.window_configs[window_conf_index].pos_x     = value.parse().unwrap(); },
                "pos_y"     => { shard_config.window_configs[window_conf_index].pos_y     = value.parse().unwrap(); },
                "width"     => { shard_config.window_configs[window_conf_index].width     = value.parse().unwrap(); },
                "height"    => { shard_config.window_configs[window_conf_index].height    = value.parse().unwrap(); },
                "active"    => { shard_config.window_configs[window_conf_index].active    = value.parse().unwrap(); },
                _           => {debug!("found unknown key: {:?}",key); continue 'keyloop;},
                }
            } // end of: 'keyloop
        } 
    else{  // conf_name not found, create a new window-config for it:
        
        window_conf = WindowConfig::new(sec.unwrap().to_string());
        shard_config.window_configs.push(window_conf);
        }
    } // end of: 'conf_name_loop

debug!("Number of config-conf_names found in ini-file: {}", shard_config.window_configs.len() );

// check for missing window-configs:
//'confloop: for win_conf in shard_config.window_configs.iter() 
//    {
//    debug!("Window-Title: {}", win_conf.title);
//    match win_conf.conf_name_.as_str()
//        {
//        "game-window"          => {debug!("loaded game-window"                  ); },
//        "maps-window"          => {debug!("loaded maps-window"                  ); },
//        "inventory-window"     => {debug!("loaded inventory-window"             ); },
//        "journal-window"       => {debug!("loaded journal-window"               ); },
//        "help-window"          => {debug!("loaded help-window"                  ); },
//        "pixel-painter-window" => {debug!("loaded pixel-painter-window"         ); },
//         _                     => {debug!("found  unknown window_name: {:?}",win_conf.conf_name); continue 'confloop; },
//        }
//    }

// debug!("\nValues read into config struct:");
// debug!("window_configs[0].conf_name {:?}"  , shard_config.window_configs[0].conf_name);
// debug!("window_configs[0].conf_num  {:?}"  , shard_config.window_configs[0].conf_num );
// debug!("window_configs[0].title     {:?}"  , shard_config.window_configs[0].title );
// debug!("window_configs[0].pos_x     {:?}"  , shard_config.window_configs[0].pos_x );
// debug!("window_configs[0].pos_y     {:?}"  , shard_config.window_configs[0].pos_y );
// debug!("window_configs[0].width     {:?}"  , shard_config.window_configs[0].width );
// debug!("window_configs[0].height    {:?}"  , shard_config.window_configs[0].height);
// debug!("window_configs[0].active    {:?}\n", shard_config.window_configs[0].active);

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

pub(crate) fn save(ini_filename_p: &str, shard_config_p: ShardConfig) -> Result<bool, io::Error> 
{
debug!("config::save() called");

let mut ini_conf = Ini::new();
let mut sec: Option<String>;

ini_conf.with_section(None::<String>)
        .set("encoding", "utf-8");

'confloop: for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("Window-Title: {}", win_conf.conf_name);

    match win_conf.conf_name.as_str() 
        {
        "game-window"          => {debug!("found game-window"                  ); sec = Some("game-window"         .to_string() ); },
        "maps-window"          => {debug!("found maps-window"                  ); sec = Some("maps-window"         .to_string() ); },
        "inventory-window"     => {debug!("found inventory-window"             ); sec = Some("inventory-window"    .to_string() ); },
        "journal-window"       => {debug!("found journal-window"               ); sec = Some("journal-window"      .to_string() ); },
        "help-window"          => {debug!("found help-window"                  ); sec = Some("help-window"         .to_string() ); },
        "pixel-painter-window" => {debug!("found pixel-painter-window"         ); sec = Some("pixel-painter-window".to_string() ); },
         _                     => {debug!("found unknown window_name: {:?}", win_conf.conf_name); continue 'confloop; },
        }

    debug!("win_conf.conf_num: {}", win_conf.conf_num);

    ini_conf.with_section(sec)
            .set("conf_num",  shard_config_p.window_configs[win_conf.conf_num].conf_num .to_string() ) 
            .set("title",     shard_config_p.window_configs[win_conf.conf_num].title    .to_string() ) 
            .set("pos_x",     shard_config_p.window_configs[win_conf.conf_num].pos_x    .to_string() ) 
            .set("pos_y",     shard_config_p.window_configs[win_conf.conf_num].pos_y    .to_string() ) 
            .set("width",     shard_config_p.window_configs[win_conf.conf_num].width    .to_string() ) 
            .set("height",    shard_config_p.window_configs[win_conf.conf_num].height   .to_string() ) 
            .set("active",    shard_config_p.window_configs[win_conf.conf_num].active   .to_string() ) ;
    } // end of: 'confloop

ini_conf.write_to_file(ini_filename_p).unwrap();

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

pub(crate) fn init(ini_filename_p: &str) -> Result<ShardConfig, io::Error> 
{
debug!("init() called");

let shard_config: ShardConfig;

match load(ini_filename_p)
  {
  Ok(config) => { shard_config = config; },
  Err(error) => { error!("Error loading config: {:?}", error); return Err(error); },
  }

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

pub(crate) fn exit(ini_filename_p: &str, shard_config: ShardConfig) -> Result<bool, io::Error> 
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
  /// **`TESTS:      `** checks if the initialisation with the default configuration INI-File results in ok()   
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
  /// **`TESTS:      `** checks if the loading of the default configuration INI-File results in ok()   
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
  /// **`TESTS:      `** checks if method WindowConfig::default() indeed sets the DEFAULT-VALUES   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_default_values() 
  {
  let defaults = WindowConfig::default();
  
  assert_eq!(defaults.pos_x , DEFAULT_POS_X );
  assert_eq!(defaults.pos_y , DEFAULT_POS_Y );
  assert_eq!(defaults.width , DEFAULT_WIDTH );
  assert_eq!(defaults.height, DEFAULT_HEIGHT);
  assert_eq!(defaults.active, DEFAULT_ACTIVE);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** set_and_check_all_values()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if complete cycle (of storing configuration to file and reloading) returns all values intact   
  /// ___________________________________________________________________________________________________________________________
    #[test]
    fn set_and_check_all_values() 
    {

    let mut test_conf2write: ShardConfig = ShardConfig 
        {
        verbosity:           2,
        debug:               true,
        test:                true,
        training:            true,
        file:                "test ini filename".to_string(),
        window_configs:      Vec::with_capacity(2), 
        };
    

    let test_win_game_conf2write: WindowConfig = WindowConfig 
        {
        conf_name: "test_game_window".to_string(),
        conf_num:  0,    
        title:     "Game-Window".to_string(),
        pos_x :    11 ,
        pos_y :    12 ,
        width :    13 ,
        height:    14 ,
        active:    false ,
        };
    
    let test_win_maps_conf2write: WindowConfig = WindowConfig 
        {
        conf_name: "test_maps_window".to_string(),
        conf_num:  1,    
        title:     "Maps-Window".to_string(),
        pos_x :    21 ,
        pos_y :    22 ,
        width :    23 ,
        height:    24 ,
        active:    true ,
        };

    test_conf2write.window_configs[0] = test_win_game_conf2write;
    test_conf2write.window_configs[1] = test_win_maps_conf2write;
    
    match save(NAME_OF_INI_FILE4TEST,test_conf2write)
        {
        Ok(_)  => {},
        Err(errmsg)     => { panic!(errmsg) },
        }


    let test_conf2load: ShardConfig;  

    match load(NAME_OF_INI_FILE4TEST)
        {
        Ok(config)  => { test_conf2load = config; },
        Err(errmsg) => { panic!(errmsg) },
        }

    fs::remove_file(NAME_OF_INI_FILE4TEST).unwrap();
    
    assert_eq!(test_conf2load.verbosity    , 0);                             // default values, since currently these variables are not saved to the config-ini-file!
    assert_eq!(test_conf2load.debug        , false);                         // default values, since currently these variables are not saved to the config-ini-file!
    assert_eq!(test_conf2load.test         , false);                         // default values, since currently these variables are not saved to the config-ini-file!
    assert_eq!(test_conf2load.training     , false);                         // default values, since currently these variables are not saved to the config-ini-file!
    assert_eq!(test_conf2load.file         , NAME_OF_INI_FILE.to_string());  // default values, since currently these variables are not saved to the config-ini-file!
    
    assert_eq!(test_conf2load.window_configs[0].conf_name, "test_game_window".to_string());
    assert_eq!(test_conf2load.window_configs[0].conf_num ,  0);
    assert_eq!(test_conf2load.window_configs[0].title    , "Game-Window");
    assert_eq!(test_conf2load.window_configs[0].pos_x    , 11);
    assert_eq!(test_conf2load.window_configs[0].pos_y    , 12);
    assert_eq!(test_conf2load.window_configs[0].width    , 13);
    assert_eq!(test_conf2load.window_configs[0].height   , 14);
    assert_eq!(test_conf2load.window_configs[0].active   , false);
    
    assert_eq!(test_conf2load.window_configs[1].conf_name, "test_maps_window".to_string());
    assert_eq!(test_conf2load.window_configs[1].conf_num ,  1);
    assert_eq!(test_conf2load.window_configs[1].title    , "Maps-Window");
    assert_eq!(test_conf2load.window_configs[1].pos_x    , 21);
    assert_eq!(test_conf2load.window_configs[1].pos_y    , 22);
    assert_eq!(test_conf2load.window_configs[1].width    , 23);
    assert_eq!(test_conf2load.window_configs[1].height   , 24);
    assert_eq!(test_conf2load.window_configs[1].active   , true );
    }

} // End of: mod test

