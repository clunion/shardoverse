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
//! * replace unwraps() with more resilient error handling   
//! * check: replace strings with &strs?   
//! ___________________________________________________________________________________________________________________________
//!    


//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
use std::io;
use std::str::FromStr;

use ini::Ini;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use crate::modules::asset::cursors;   // <dirname>::<filename>::<explicit mod name>

//___ CONSTANTS: ______________________________________________________________________________________________________________
pub (crate) const INI_FILE_NAME:      &str = "shardoverse.ini";
#[allow(dead_code)]
            const INI_FILE_NAME4TEST: &str = "shardoverse_test.ini";

// Config/Section-Names in INI-File:
pub (crate) const CONF_WINDOW_GAME: &str = "game-window"         ;
pub (crate) const CONF_WINDOW_MAPS: &str = "maps-window"         ;
pub (crate) const CONF_WINDOW_INVT: &str = "inventory-window"    ;
pub (crate) const CONF_WINDOW_JRNL: &str = "journal-window"      ;
pub (crate) const CONF_WINDOW_HELP: &str = "help-window"         ;
pub (crate) const CONF_WINDOW_PIXL: &str = "pixel-painter-window";

// Config/Property-Names (within the  global section) in INI-File:
pub (crate) const CONF_VERBOSITY  : &str = "verbosity"         ;
pub (crate) const CONF_DEBUG      : &str = "debug"             ;
pub (crate) const CONF_TEST       : &str = "test"              ;
pub (crate) const CONF_TRAINING   : &str = "training"          ;
pub (crate) const CONF_LANGUAGE   : &str = "language"          ;
pub (crate) const CONF_DELAY      : &str = "delay"             ;
pub (crate) const CONF_SCALE      : &str = "scale"             ;
pub (crate) const CONF_ENCODING   : &str = "encoding"          ;  
                       
// Config/Property-Names (within the window-confs/sections) in INI-File:
pub (crate) const CONF_WIN_TITLE  : &str = "title"  ;
pub (crate) const CONF_WIN_POS_X  : &str = "pos_x"  ;
pub (crate) const CONF_WIN_POS_Y  : &str = "pos_y"  ;
pub (crate) const CONF_WIN_WIDTH  : &str = "width"  ;
pub (crate) const CONF_WIN_HEIGHT : &str = "height" ;
pub (crate) const CONF_WIN_VISIBLE: &str = "visible";


// Shard-Config-Defaults:
const DEFAULT_VERBOSITY:    u8 = 0;
const DEFAULT_DEBUG:      bool = false;
const DEFAULT_TEST:       bool = false;
const DEFAULT_TRAINING:   bool = false;
const DEFAULT_LANGUAGE:   &str = "English";
const DEFAULT_DELAY:       u16 = 40;
const DEFAULT_SCALE:       u16 = 32;
const DEFAULT_ENCODING:   &str = "utf-8";

// Window-Config-Defaults:
const DEFAULT_POS_X :      i32 = 100;
const DEFAULT_POS_Y :      i32 = 100;
const DEFAULT_WIDTH :      u32 = 800;
const DEFAULT_HEIGHT:      u32 = 600;
const DEFAULT_VISIBLE:    bool = true;

const DEFAULT_POS_OFFSET:  i32 =  16;
const INITIAL_NUMBER_OF_WINDOW_CONFIGS: usize = 10;

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
#[derive(Debug)]
pub(crate) struct WindowConfig
{
    pub(crate) conf_name: String,
    pub(crate) title:     String,   // this means: Window-Title, later to be changed to localisation-string-ID
    pub(crate) pos_x:     i32,
    pub(crate) pos_y:     i32,
    pub(crate) height:    u32,
    pub(crate) width:     u32,
    pub(crate) visible:   bool,
}

#[derive(Debug)]
pub(crate) struct ShardConfig 
{
    pub(crate) ini_file_name:  String,
    pub(crate) verbosity:      u8,
    pub(crate) debug:          bool,
    pub(crate) test:           bool,
    pub(crate) training:       bool,
    pub(crate) language:       String,
    pub(crate) delay:          u16,
    pub(crate) scale:          u16,
    pub(crate) encoding:       String,
    pub(crate) window_configs: Vec<WindowConfig>,
}


//___ METHODS: ________________________________________________________________________________________________________________

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  default   
/// **`TYPE:       `**  method of WindowConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Initialises a WindowConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________

impl <'lt_winconf> Default for WindowConfig
{
    fn default() -> Self 
    {
        WindowConfig 
        {
        conf_name: "unnamed".to_string(),
        title:     "untitled".to_string(),
        pos_x:     DEFAULT_POS_X,
        pos_y:     DEFAULT_POS_Y,
        height:    DEFAULT_HEIGHT,
        width:     DEFAULT_WIDTH,
        visible:   DEFAULT_VISIBLE,
        }
    }

}

//-- Window-Configs ------------------------------------------
impl <'lt_winconf> WindowConfig 
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  new_complete   
/// **`TYPE:       `**  method of WindowConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` conf_name_p   `** Name of the window-config, also this is a section name in the ini-file   
/// **`            `** **` title_p       `** Titel of the window, displayd in the GUI (Title bar). Language dependant.   
/// **`            `** **` pos_x_p       `** Initial horizontal position of the left edge of the window   
/// **`            `** **` pos_y_p       `** Initial vertical   position of the top  edge of the window   
/// **`            `** **` height_p      `** Initial vertical   height in pixels   
/// **`            `** **` width_p       `** Initial horizontal width  in pixels   
/// **`            `** **` visible_p      `** Flag, if the window is visible   
/// **`RETURNS:    `** **` WindowConfig  `** a newly created struct   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Initialises a WindowConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________

    #[allow(clippy::too_many_arguments)]
    fn new_complete(conf_name_p: String, title_p: String, pos_x_p: i32, pos_y_p: i32, height_p: u32, width_p: u32, visible_p: bool) -> WindowConfig
    {
        WindowConfig 
        {
        conf_name:   conf_name_p,
        title:       title_p,
        pos_x:       pos_x_p,
        pos_y:       pos_y_p,
        height:      height_p,
        width:       width_p,
        visible:     visible_p,
        }
    }

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  new   
/// **`TYPE:       `**  method of WindowConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` conf_name_p   `** Name of the window-config, also this is a section name in the ini-file   
/// **`RETURNS:    `** **` WindowConfig  `** a newly created struct   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Create a WindowConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
    fn new(conf_name_p: String) -> WindowConfig
    {
        WindowConfig 
        {
        conf_name:   conf_name_p,
        title:       "untitled".to_string(),
        pos_x:       DEFAULT_POS_X,
        pos_y:       DEFAULT_POS_Y,
        height:      DEFAULT_HEIGHT,
        width:       DEFAULT_WIDTH,
        visible:     DEFAULT_VISIBLE,
        }
    }
} // End of struct: WindowConfig

//-- SHARD-Config -------------------------------------------

impl <'lt_shardconf> Default for ShardConfig
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  default   
/// **`TYPE:       `**  method of ShardConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Initialises a ShardConfig struct with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
    fn default() -> Self 
    {
        ShardConfig 
        {
        ini_file_name:   INI_FILE_NAME.to_owned(),
        verbosity:       DEFAULT_VERBOSITY,
        debug:           DEFAULT_DEBUG,
        test:            DEFAULT_TEST,
        training:        DEFAULT_TRAINING,
        language:        DEFAULT_LANGUAGE.to_owned(),
        delay:           DEFAULT_DELAY,
        scale:           DEFAULT_SCALE,
        encoding:        DEFAULT_ENCODING.to_owned(),
        window_configs:  Vec::with_capacity(INITIAL_NUMBER_OF_WINDOW_CONFIGS), 
        }
    }
}

impl <'lt_shardconf> ShardConfig
{
/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  reset_all_windows   
/// **`TYPE:       `**  method of ShardConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Resets all window within the given ShardConfig struct to default positions and -sizes   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn reset_all_windows(mut self) -> Self
    {
    let mut pos_offset: i32 = 0;

    for mut conf in self.window_configs.iter_mut() 
        {
        conf.pos_x   = DEFAULT_POS_X + pos_offset;
        conf.pos_y   = DEFAULT_POS_Y + pos_offset;
        conf.width   = DEFAULT_WIDTH ;
        conf.height  = DEFAULT_HEIGHT;
        conf.visible = DEFAULT_VISIBLE;
        pos_offset  += DEFAULT_POS_OFFSET;              // todo: change to a dynamic value depending on HiDPI setting
        } // end of: 'confloop
    self
    }

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  recreate_all_configs   
/// **`TYPE:       `**  method of ShardConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// (Re)Create a ShardConfig struct completely with default values   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn recreate_all_configs(mut self) -> Self
    {
    self.verbosity       = DEFAULT_VERBOSITY;
    self.debug           = DEFAULT_DEBUG;
    self.test            = DEFAULT_TEST;
    self.training        = DEFAULT_TRAINING;
    self.ini_file_name   = INI_FILE_NAME.to_owned();

    let mut pos_offset: i32 = 0;

    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_GAME.to_string(), "Shardoverse: Game"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); pos_offset += DEFAULT_POS_OFFSET;
    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_MAPS.to_string(), "Shardoverse: Maps"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); pos_offset += DEFAULT_POS_OFFSET;
    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_INVT.to_string(), "Shardoverse: Inventory"    .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); pos_offset += DEFAULT_POS_OFFSET;
    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_JRNL.to_string(), "Shardoverse: Journal"      .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); pos_offset += DEFAULT_POS_OFFSET;
    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_HELP.to_string(), "Shardoverse: Help"         .to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); pos_offset += DEFAULT_POS_OFFSET;
    self.window_configs.push( WindowConfig::new_complete(CONF_WINDOW_PIXL.to_string(), "Shardoverse: Pixel-Painter".to_string(), DEFAULT_POS_X + pos_offset, DEFAULT_POS_Y + pos_offset, DEFAULT_HEIGHT, DEFAULT_WIDTH, DEFAULT_VISIBLE)); 
    self
    }


/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  find_window_conf_index_by_conf_name   
/// **`TYPE:       `**  method of ShardConfig   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` conf_name_p   `** Configuration name to search   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Find the index of the given config-name in the list of window-configs of the current ShardConfig   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-07 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
pub(crate) fn find_window_conf_index_by_conf_name(&self, conf_name_p: String) ->  Result<usize, ()>
   {
   let   index = self.window_configs.iter().position(|elem| elem.conf_name == conf_name_p);
   match index
       {
       Some(index)  => Ok(index),
       None         => Err(()),
       }
   }

} // End of struct: ShardConfig

 
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
let config = Ini::load_from_file(ini_filename_p);
match config
    {
    Ok(config) => { shard_conf_loaded = true;  ini_conf = config; },
    Err(error) => { shard_conf_loaded = false; warn!("could not read config file {}: {:?}. DEFAULT VALUES will be used!",ini_filename_p, error);},
    }

if !shard_conf_loaded          // if nothing could be loaded...
    {
    return Ok(shard_config)    // ...then return config initialised with the default data!
    } 

// if debug-mode: print all key-value data from the config ini-file:
if cfg!(debug_assertions) 
    {
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
    debug!(".");
    debug!("conf-name: {:?}", sec);

    if sec == None
        {
        debug!("Detected: we are in the Global Section!");

        shard_config.verbosity  = u8::from_str_radix( ini_conf.general_section().get(CONF_VERBOSITY).unwrap(), 10).unwrap();
        shard_config.debug      = FromStr::from_str(  ini_conf.general_section().get(CONF_DEBUG    ).unwrap())    .unwrap();
        shard_config.test       = FromStr::from_str(  ini_conf.general_section().get(CONF_TEST     ).unwrap())    .unwrap();
        shard_config.training   = FromStr::from_str(  ini_conf.general_section().get(CONF_TRAINING ).unwrap())    .unwrap();
        shard_config.language   =                     ini_conf.general_section().get(CONF_LANGUAGE ).unwrap().to_string();
        shard_config.delay      = u16::from_str_radix(ini_conf.general_section().get(CONF_DELAY    ).unwrap(), 10).unwrap();
        shard_config.scale      = u16::from_str_radix(ini_conf.general_section().get(CONF_SCALE    ).unwrap(), 10).unwrap();
        shard_config.encoding   =                     ini_conf.general_section().get(CONF_ENCODING ).unwrap().to_string();

        debug!("{:9} = {:8} -> {}",CONF_VERBOSITY,    ini_conf.general_section().get(CONF_VERBOSITY).unwrap(), shard_config.verbosity);
        debug!("{:9} = {:8} -> {}",CONF_DEBUG    ,    ini_conf.general_section().get(CONF_DEBUG    ).unwrap(), shard_config.debug    );
        debug!("{:9} = {:8} -> {}",CONF_TEST     ,    ini_conf.general_section().get(CONF_TEST     ).unwrap(), shard_config.test     );
        debug!("{:9} = {:8} -> {}",CONF_TRAINING ,    ini_conf.general_section().get(CONF_TRAINING ).unwrap(), shard_config.training );
        debug!("{:9} = {:8} -> {}",CONF_LANGUAGE ,    ini_conf.general_section().get(CONF_LANGUAGE ).unwrap(), shard_config.language );
        debug!("{:9} = {:8} -> {}",CONF_DELAY    ,    ini_conf.general_section().get(CONF_DELAY    ).unwrap(), shard_config.delay    );
        debug!("{:9} = {:8} -> {}",CONF_SCALE    ,    ini_conf.general_section().get(CONF_SCALE    ).unwrap(), shard_config.scale    );
        debug!("{:9} = {:8} -> {}",CONF_ENCODING ,    ini_conf.general_section().get(CONF_ENCODING ).unwrap(), shard_config.encoding );
        
        continue 'conf_name_loop;
        }

    // check if a section for the just loaded window-config already exists (it may have been created from the default values):
    match shard_config.find_window_conf_index_by_conf_name(sec.unwrap().to_string())
        {
        Ok(x)   =>  {window_conf_found = true ; window_conf_index = x; debug!("    found index for section {:?}, it's: {}",sec, window_conf_index);},
        Err(()) =>  {window_conf_found = false;                        debug!("not found index for section {:?}"          ,sec);},
        }

    if  ! window_conf_found   
        {  // conf_name not found, create a new window-config for it:
        debug!("Create new Window-Conf for {:?}",sec);
        window_conf = WindowConfig::new(sec.unwrap().to_string());
        shard_config.window_configs.push(window_conf);
        window_conf_index = shard_config.window_configs.len() -1;  // better way?
        }

    'keyloop: for (key, value) in prop.iter() 
        {
        debug!("{}:{}", key, value);
        match key   // replace the default-values with the loaded ones:
            {
            CONF_WIN_TITLE   => { shard_config.window_configs[window_conf_index].title     = value.parse().unwrap(); },
            CONF_WIN_POS_X   => { shard_config.window_configs[window_conf_index].pos_x     = value.parse().unwrap(); },
            CONF_WIN_POS_Y   => { shard_config.window_configs[window_conf_index].pos_y     = value.parse().unwrap(); },
            CONF_WIN_WIDTH   => { shard_config.window_configs[window_conf_index].width     = value.parse().unwrap(); },
            CONF_WIN_HEIGHT  => { shard_config.window_configs[window_conf_index].height    = value.parse().unwrap(); },
            CONF_WIN_VISIBLE => { shard_config.window_configs[window_conf_index].visible   = value.parse().unwrap(); },
            _                => {debug!("found unknown key: {:?}",key); continue 'keyloop;},
            }
        } // end of: 'keyloop
    } // end of: 'conf_name_loop

debug!("Number of config-conf_names found in ini-file: {}", shard_config.window_configs.len() );

Ok(shard_config)
} // End of: load()


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
let mut sec:               Option<String>;
let mut window_conf_index: usize = 0; 
let mut window_conf_found: bool;

ini_conf.with_section(None::<String>)
        .set(CONF_VERBOSITY, shard_config_p.verbosity.to_string())
        .set(CONF_DEBUG    , shard_config_p.debug    .to_string())
        .set(CONF_TEST     , shard_config_p.test     .to_string())
        .set(CONF_TRAINING , shard_config_p.training .to_string())
        .set(CONF_LANGUAGE , shard_config_p.language .to_string())
        .set(CONF_DELAY    , shard_config_p.delay    .to_string())
        .set(CONF_SCALE    , shard_config_p.scale    .to_string())
        .set(CONF_ENCODING , shard_config_p.encoding .to_owned())
        ;

for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("Window-Title: {}", win_conf.conf_name);

    sec = Some(win_conf.conf_name.clone());

    match shard_config_p.find_window_conf_index_by_conf_name(win_conf.conf_name.to_string())
        {
        Ok(x)   =>  {window_conf_found = true ; window_conf_index = x; debug!("    found index for section {:?}, it's: {}",sec, window_conf_index);},
        Err(()) =>  {window_conf_found = false;                        debug!("not found index for section {:?}"          ,sec);},
        }

    if window_conf_found
        {
        ini_conf.with_section(sec)
                .set(CONF_WIN_TITLE  , shard_config_p.window_configs[window_conf_index].title  .to_string() ) 
                .set(CONF_WIN_POS_X  , shard_config_p.window_configs[window_conf_index].pos_x  .to_string() ) 
                .set(CONF_WIN_POS_Y  , shard_config_p.window_configs[window_conf_index].pos_y  .to_string() ) 
                .set(CONF_WIN_WIDTH  , shard_config_p.window_configs[window_conf_index].width  .to_string() ) 
                .set(CONF_WIN_HEIGHT , shard_config_p.window_configs[window_conf_index].height .to_string() ) 
                .set(CONF_WIN_VISIBLE, shard_config_p.window_configs[window_conf_index].visible.to_string() ) ;
        }
// else ...

        
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
  use std::fs;

  use crate::config::WindowConfig;
  #[allow(unused_imports)]
  use log::{trace, debug, info, warn, error};
  use flexi_logger::{Logger, Duplicate, Cleanup, Criterion, Naming};
  use crate::shard_log; 


  
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_init()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if the initialisation with the default configuration INI-File results in ok()   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_init() 
  {
  let result = init(INI_FILE_NAME);
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
  let conf = load(INI_FILE_NAME);  // load is non-destructive, thus the normal ini-file is used for load-testing.
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
  
  assert_eq!(defaults.pos_x  , DEFAULT_POS_X  );
  assert_eq!(defaults.pos_y  , DEFAULT_POS_Y  );
  assert_eq!(defaults.width  , DEFAULT_WIDTH  );
  assert_eq!(defaults.height , DEFAULT_HEIGHT );
  assert_eq!(defaults.visible, DEFAULT_VISIBLE);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** set_and_check_all_values()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if complete cycle (of storing configuration to file and reloading) returns all values intact   
  /// ___________________________________________________________________________________________________________________________
    #[test]
    fn set_and_check_all_values() 
    {
    let mut window_conf_index: usize = 0; 
    let mut window_conf_found: bool;

    // Initialise flexi_logger, see documentation of Struct flexi_logger::LogSpecification:
    match Logger::with_env_or_str("warn, shardoverse::central_core=debug, shardoverse::modules::config=debug")
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
        Err(error)         => { println!("ERROR initialising flexi_logger: {:?}", error); }, // return Err(error); },
        }


    let mut test_conf2write: ShardConfig = ShardConfig 
        {
        ini_file_name:   INI_FILE_NAME4TEST.to_string(),
        verbosity:       DEFAULT_VERBOSITY,
        debug:           DEFAULT_DEBUG    ,
        test:            DEFAULT_TEST     ,
        training:        DEFAULT_TRAINING ,
        language:        DEFAULT_LANGUAGE.to_owned(),
        delay:           DEFAULT_DELAY,
        scale:           DEFAULT_SCALE,
        encoding:        DEFAULT_ENCODING.to_owned(),
        window_configs:  Vec::with_capacity(2), 
        };
    

    let test_win_game_conf2write: WindowConfig = WindowConfig 
        {
        conf_name: "test_game_window".to_string(),
        title:     "Game-Window".to_string(),
        pos_x :    11 ,
        pos_y :    12 ,
        width :    13 ,
        height:    14 ,
        visible:   false ,
        };
    
    let test_win_maps_conf2write: WindowConfig = WindowConfig 
        {
        conf_name: "test_maps_window".to_string(),
        title:     "Maps-Window".to_string(),
        pos_x :    21 ,
        pos_y :    22 ,
        width :    23 ,
        height:    24 ,
        visible:   true ,
        };

    test_conf2write.window_configs.push(test_win_game_conf2write);
    test_conf2write.window_configs.push(test_win_maps_conf2write);
    
    match save(INI_FILE_NAME4TEST,test_conf2write)
        {
        Ok(_)  => {},
        Err(errmsg)     => { panic!(errmsg) },
        }

    let test_conf2load: ShardConfig;  

    match load(INI_FILE_NAME4TEST)
        {
        Ok(config)  => { test_conf2load = config; },
        Err(errmsg) => { panic!(errmsg) },
        }

    fs::remove_file(INI_FILE_NAME4TEST).unwrap();

//  assert_eq!(test_conf2load.ini_file_name , INI_FILE_NAME4TEST.to_string());
    assert_eq!(test_conf2load.verbosity     , DEFAULT_VERBOSITY);
    assert_eq!(test_conf2load.debug         , DEFAULT_DEBUG    );
    assert_eq!(test_conf2load.test          , DEFAULT_TEST     );
    assert_eq!(test_conf2load.training      , DEFAULT_TRAINING );
    assert_eq!(test_conf2load.language      , DEFAULT_LANGUAGE.to_string()); 
    assert_eq!(test_conf2load.delay         , DEFAULT_DELAY);
    assert_eq!(test_conf2load.scale         , DEFAULT_SCALE);
    assert_eq!(test_conf2load.encoding      , DEFAULT_ENCODING.to_string());
    
    match test_conf2load.find_window_conf_index_by_conf_name("test_game_window".to_string())
        {
        Ok(x)   =>  {window_conf_found = true; window_conf_index = x;},
        Err(()) =>  {window_conf_found = false                       },
        }

    if  window_conf_found
        {
        assert_eq!(test_conf2load.window_configs[window_conf_index].conf_name, "test_game_window".to_string());
        assert_eq!(test_conf2load.window_configs[window_conf_index].title    , "Game-Window");
        assert_eq!(test_conf2load.window_configs[window_conf_index].pos_x    , 11);
        assert_eq!(test_conf2load.window_configs[window_conf_index].pos_y    , 12);
        assert_eq!(test_conf2load.window_configs[window_conf_index].width    , 13);
        assert_eq!(test_conf2load.window_configs[window_conf_index].height   , 14);
        assert_eq!(test_conf2load.window_configs[window_conf_index].visible  , false);
        }
    
    match test_conf2load.find_window_conf_index_by_conf_name("test_maps_window".to_string())
        {
        Ok(x)   =>  {window_conf_found = true; window_conf_index = x;},
        Err(()) =>  {window_conf_found = false                       },
        }

    if  window_conf_found
        {
        assert_eq!(test_conf2load.window_configs[window_conf_index].conf_name, "test_maps_window".to_string());
        assert_eq!(test_conf2load.window_configs[window_conf_index].title    , "Maps-Window");
        assert_eq!(test_conf2load.window_configs[window_conf_index].pos_x    , 21);
        assert_eq!(test_conf2load.window_configs[window_conf_index].pos_y    , 22);
        assert_eq!(test_conf2load.window_configs[window_conf_index].width    , 23);
        assert_eq!(test_conf2load.window_configs[window_conf_index].height   , 24);
        assert_eq!(test_conf2load.window_configs[window_conf_index].visible  , true );
        }
    }

} // End of: mod test





