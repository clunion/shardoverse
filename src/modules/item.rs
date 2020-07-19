//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** item.rs 🦀   
//! **`DESCRIPTION:`** handling items of the world of Shardoverse, including tools, valuables, armors, weapons,    
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-07-17 | Clunion   | creation   
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * everything (nearly)   
//! ___________________________________________________________________________________________________________________________
//!    


//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________

use std::io;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use crate::modules::*;      // crate::<dirname>::*


//___ CONSTANTS: ______________________________________________________________________________________________________________
pub(crate) const DEFAULT_NAME_OF_ITEM:  &str  = "nameless item";


//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCT DEFINITIONS: _____________________________________________________________________________________________________

#[derive(Debug)]
pub(crate) struct Item
{
    pub(crate) name:                   String,
    pub(crate) kind:                   u32,   // type: tool, weapon, armor, projectile, consumable, valuable, ...
    pub(crate) value:                  u32,
    pub(crate) quality:                u8,    // 
    pub(crate) weight:                 i32,
    pub(crate) weight_mod:             i32,
    pub(crate) livepoints_max_mod:     i32,   // modification to base attributes
    pub(crate) livepoints_regen_mod:   i32,   // modification to base attributes
    pub(crate) magicpoints_max_mod:    i32,   // modification to base attributes
    pub(crate) magicpoints_regen_mod:  i32,   // modification to base attributes
    pub(crate) strength_mod:           i32,   // modification to base attributes
    pub(crate) endurance_mod:          i32,   // modification to base attributes
    pub(crate) dexterity_mod:          i32,   // modification to base attributes
    pub(crate) willpower_mod:          i32,   // modification to base attributes
    pub(crate) wisdom_mod:             i32,   // modification to base attributes
    pub(crate) intelligence_mod:       i32,   // modification to base attributes
    pub(crate) armor_mod:              i32,

}


//___ METHODS: ________________________________________________________________________________________________________________
impl Default for Item 
{
    fn default() -> Self 
    {
        Item 
        {
        name:                                DEFAULT_NAME_OF_ITEM.to_string(), // String,
        kind:                    shard_tool::DEFAULT_VALUE_U32,                // u32,   
        value:                   shard_tool::DEFAULT_VALUE_U32,                // u32,   
        quality:                 shard_tool::DEFAULT_VALUE_U8,                 // u8,    
        weight:                  shard_tool::DEFAULT_VALUE_I32,                // i32,   
        weight_mod:              shard_tool::DEFAULT_VALUE_I32,                // i32,   
        livepoints_max_mod:      shard_tool::DEFAULT_VALUE_I32,                // i32,   
        livepoints_regen_mod:    shard_tool::DEFAULT_VALUE_I32,                // i32,   
        magicpoints_max_mod:     shard_tool::DEFAULT_VALUE_I32,                // i32,   
        magicpoints_regen_mod:   shard_tool::DEFAULT_VALUE_I32,                // i32,   
        strength_mod:            shard_tool::DEFAULT_VALUE_I32,                // i32,   
        endurance_mod:           shard_tool::DEFAULT_VALUE_I32,                // i32,   
        dexterity_mod:           shard_tool::DEFAULT_VALUE_I32,                // i32,   
        willpower_mod:           shard_tool::DEFAULT_VALUE_I32,                // i32,   
        wisdom_mod:              shard_tool::DEFAULT_VALUE_I32,                // i32,   
        intelligence_mod:        shard_tool::DEFAULT_VALUE_I32,                // i32,   
        armor_mod:               shard_tool::DEFAULT_VALUE_I32,                // i32,   
        }
    }
}

 
/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  display_values   
/// **`TYPE:       `**  local, common function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` item_p        `** Reference to an item struct to display   
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

pub(crate) fn display_values(item_p: &Item) -> Result<(), io::Error>  
{
debug!("item::display_values() called");

    println!("name:                   '{}'", item_p.name                    );  // String
    println!("kind:                   {}",   item_p.kind                    );  // u32
    println!("value:                  {}",   item_p.value                   );  // u32
    println!("quality:                {}",   item_p.quality                 );  // u8
    println!("weight:                 {}",   item_p.weight                  );  // i32
    println!("weight_mod:             {}",   item_p.weight_mod              );  // i32
    println!("livepoints_max_mod:     {}",   item_p.livepoints_max_mod      );  // i32
    println!("livepoints_regen_mod:   {}",   item_p.livepoints_regen_mod    );  // i32
    println!("magicpoints_max_mod:    {}",   item_p.magicpoints_max_mod     );  // i32
    println!("magicpoints_regen_mod:  {}",   item_p.magicpoints_regen_mod   );  // i32
    println!("strength_mod:           {}",   item_p.strength_mod            );  // i32
    println!("endurance_mod:          {}",   item_p.endurance_mod           );  // i32
    println!("dexterity_mod:          {}",   item_p.dexterity_mod           );  // i32
    println!("willpower_mod:          {}",   item_p.willpower_mod           );  // i32
    println!("wisdom_mod:             {}",   item_p.wisdom_mod              );  // i32
    println!("intelligence_mod:       {}",   item_p.intelligence_mod        );  // i32 
    println!("armor_mod:              {}",   item_p.armor_mod               );  // i32 

Ok(())
}



/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for item   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
  use super::*;            // importing names from outer (for mod tests) scope

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_default_values()   
  /// **`TYPE:       `** unit test   
  /// **`TESTS:      `** checks if method Item::default() indeed sets the DEFAULT-VALUES   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_default_values() 
  {
  let defaults = Item::default();

  assert_eq!(defaults.name                  , DEFAULT_NAME_OF_ITEM );
  assert_eq!(defaults.kind                  , shard_tool::DEFAULT_VALUE_U32 );
  assert_eq!(defaults.value                 , shard_tool::DEFAULT_VALUE_U32 );
  assert_eq!(defaults.quality               , shard_tool::DEFAULT_VALUE_U8  );
  assert_eq!(defaults.weight                , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.weight_mod            , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.livepoints_max_mod    , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.livepoints_regen_mod  , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.magicpoints_max_mod   , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.magicpoints_regen_mod , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.strength_mod          , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.endurance_mod         , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.dexterity_mod         , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.willpower_mod         , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.wisdom_mod            , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.intelligence_mod      , shard_tool::DEFAULT_VALUE_I32 );
  assert_eq!(defaults.armor_mod             , shard_tool::DEFAULT_VALUE_I32 );
  }

} // End of: mod test

