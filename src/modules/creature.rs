//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** creature.rs 🦀   
//! **`DESCRIPTION:`** handling creatures of the world of Shardoverse, including characters, animals, enemies, monsters,     
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
pub(crate) const DEFAULT_NAME_OF_CREATURE:  &str  = "nameless creature";

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________

#[derive(Debug)]
pub(crate) struct Creature
{
    pub(crate) name:                   String,
    pub(crate) species:                u32,
    pub(crate) level:                  u16,
    pub(crate) experience:             u16,
                                       
    pub(crate) livepoints_max:         u32,
    pub(crate) livepoints_cur:         u32,
    pub(crate) livepoints_regen_base:  i32,
    pub(crate) livepoints_regen_cur:   i32,

    pub(crate) magicpoints_max:        u32,
    pub(crate) magicpoints_cur:        u32,
    pub(crate) magicpoints_regen_base: u32,
    pub(crate) magicpoints_regen_cur:  u32,
                                       
    pub(crate) strength:               u16,
    pub(crate) endurance:              u16,
    pub(crate) dexterity:              u16,
                                       
    pub(crate) willpower:              u16, 
    pub(crate) wisdom:                 u16, 
    pub(crate) intelligence:           u16,
                                       
    pub(crate) carry_capacity_base:    u16,
    pub(crate) carry_capacity_cur:     u16,
    pub(crate) carry_capacity_used:    u16,
                                       
    pub(crate) movement_range_base:    u16,
    pub(crate) movement_range_cur:     u16,
    pub(crate) movement_range_used:    u16,
                                       
    pub(crate) armor_base:             u16,
    pub(crate) armor_cur:              u16,

    pub(crate) attack_hit_chance_base: u16,  // to investigate: rules for critical hit chance?
    pub(crate) attack_hit_chance_cur:  u16,

    pub(crate) parry_base:             u16,
    pub(crate) parry_cur:              u16,

    pub(crate) evasion_base:           u16,
    pub(crate) evasion_cur:            u16,

    // List (Vector?) of the current modifier states: 
    // paralyzed
    // poisoned
    // burning
    // confused
    // silenced
    // sleeping
    // frozen
    // sick
    // corrupted
    // possessed 

//    pub(crate) inventory:      SpeciesInventory,
//    pub(crate) equipment:      SpeciesEquipment,
}


//___ METHODS: ________________________________________________________________________________________________________________
impl Default for Creature 
{
    fn default() -> Self 
    {
        Creature 
        {
        name:                               DEFAULT_NAME_OF_CREATURE.to_string(), // Str
        species:                shard_tool::DEFAULT_VALUE_U32,                    // u32
        level:                  shard_tool::DEFAULT_VALUE_U16,                    // u16
        experience:             shard_tool::DEFAULT_VALUE_U16,                    // u16
        livepoints_max:         shard_tool::DEFAULT_VALUE_U32,                    // u32
        livepoints_cur:         shard_tool::DEFAULT_VALUE_U32,                    // u32
        livepoints_regen_base:  shard_tool::DEFAULT_VALUE_I32,                    // i32
        livepoints_regen_cur:   shard_tool::DEFAULT_VALUE_I32,                    // i32
        magicpoints_max:        shard_tool::DEFAULT_VALUE_U32,                    // u32
        magicpoints_cur:        shard_tool::DEFAULT_VALUE_U32,                    // u32
        magicpoints_regen_base: shard_tool::DEFAULT_VALUE_U32,                    // u32
        magicpoints_regen_cur:  shard_tool::DEFAULT_VALUE_U32,                    // u32
        strength:               shard_tool::DEFAULT_VALUE_U16,                    // u16
        endurance:              shard_tool::DEFAULT_VALUE_U16,                    // u16
        dexterity:              shard_tool::DEFAULT_VALUE_U16,                    // u16
        willpower:              shard_tool::DEFAULT_VALUE_U16,                    // u16
        wisdom:                 shard_tool::DEFAULT_VALUE_U16,                    // u16
        intelligence:           shard_tool::DEFAULT_VALUE_U16,                    // u16
        carry_capacity_base:    shard_tool::DEFAULT_VALUE_U16,                    // u16
        carry_capacity_cur:     shard_tool::DEFAULT_VALUE_U16,                    // u16
        carry_capacity_used:    shard_tool::DEFAULT_VALUE_U16,                    // u16
        movement_range_base:    shard_tool::DEFAULT_VALUE_U16,                    // u16
        movement_range_cur:     shard_tool::DEFAULT_VALUE_U16,                    // u16
        movement_range_used:    shard_tool::DEFAULT_VALUE_U16,                    // u16
        armor_base:             shard_tool::DEFAULT_VALUE_U16,                    // u16
        armor_cur:              shard_tool::DEFAULT_VALUE_U16,                    // u16
        attack_hit_chance_base: shard_tool::DEFAULT_VALUE_U16,                    // u16
        attack_hit_chance_cur:  shard_tool::DEFAULT_VALUE_U16,                    // u16
        parry_base:             shard_tool::DEFAULT_VALUE_U16,                    // u16
        parry_cur:              shard_tool::DEFAULT_VALUE_U16,                    // u16
        evasion_base:           shard_tool::DEFAULT_VALUE_U16,                    // u16
        evasion_cur:            shard_tool::DEFAULT_VALUE_U16,                    // u16
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
/// **`RETURNS:    `** **` Result -->    `** - OK(status flag: true = successful, false = failed)   
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

#[allow(dead_code)]
pub(crate) fn display_values(creature_p: &Creature) -> Result<(), io::Error>  
{
debug!("creature::display_values() called");

    debug!("name:                   '{}'", creature_p.name                     );  // String,
    debug!("species:                {}",   creature_p.species                  );  // u32,
    debug!("level:                  {}",   creature_p.level                    );  // u16,
    debug!("experience:             {}",   creature_p.experience               );  // u16,
    debug!("livepoints_max:         {}",   creature_p.livepoints_max           );  // u32,
    debug!("livepoints_cur:         {}",   creature_p.livepoints_cur           );  // u32,
    debug!("livepoints_regen_base:  {}",   creature_p.livepoints_regen_base    );  // i32,
    debug!("livepoints_regen_cur:   {}",   creature_p.livepoints_regen_cur     );  // i32,
    debug!("magicpoints_max:        {}",   creature_p.magicpoints_max          );  // u32,
    debug!("magicpoints_cur:        {}",   creature_p.magicpoints_cur          );  // u32,
    debug!("magicpoints_regen_base: {}",   creature_p.magicpoints_regen_base   );  // u32,
    debug!("magicpoints_regen_cur:  {}",   creature_p.magicpoints_regen_cur    );  // u32,
    debug!("strength:               {}",   creature_p.strength                 );  // u16,
    debug!("endurance:              {}",   creature_p.endurance                );  // u16,
    debug!("dexterity:              {}",   creature_p.dexterity                );  // u16,
    debug!("willpower:              {}",   creature_p.willpower                );  // u16, 
    debug!("wisdom:                 {}",   creature_p.wisdom                   );  // u16, 
    debug!("intelligence:           {}",   creature_p.intelligence             );  // u16,
    debug!("carry_capacity_base:    {}",   creature_p.carry_capacity_base      );  // u16,
    debug!("carry_capacity_cur:     {}",   creature_p.carry_capacity_cur       );  // u16,
    debug!("carry_capacity_used:    {}",   creature_p.carry_capacity_used      );  // u16,
    debug!("movement_range_base:    {}",   creature_p.movement_range_base      );  // u16,
    debug!("movement_range_cur:     {}",   creature_p.movement_range_cur       );  // u16,
    debug!("movement_range_used:    {}",   creature_p.movement_range_used      );  // u16,
    debug!("armor_base:             {}",   creature_p.armor_base               );  // u16,
    debug!("armor_cur:              {}",   creature_p.armor_cur                );  // u16,
    debug!("attack_hit_chance_base: {}",   creature_p.attack_hit_chance_base   );  // u16,
    debug!("attack_hit_chance_cur:  {}",   creature_p.attack_hit_chance_cur    );  // u16,
    debug!("parry_base:             {}",   creature_p.parry_base               );  // u16,
    debug!("parry_cur:              {}",   creature_p.parry_cur                );  // u16,
    debug!("evasion_base:           {}",   creature_p.evasion_base             );  // u16,
    debug!("evasion_cur:            {}",   creature_p.evasion_cur              );  // u16,

Ok(())
}

// let default_creature: creature::Creature = creature::Creature::default();
// match creature::display_values(&default_creature)
//     {
//     Ok(_)       => {},
//     Err(error)  => { println!("ERROR displaying default_creature: {:?}", error); return Err(error.to_string()); },
//     }
// ;


/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for config   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
    use super::*;            // importing names from outer (for mod tests) scope
  
    /// ___________________________________________________________________________________________________________________________
    /// **`FUNCTION:   `** test_default_values()   
    /// **`TYPE:       `** unit test   
    /// **`TESTS:      `** checks if method Creature::default() indeed sets the DEFAULT-VALUES   
    /// ___________________________________________________________________________________________________________________________
    #[test]
    fn test_default_values() 
    {
    let defaults = Creature::default();
    assert_eq!(defaults.name                    ,             DEFAULT_NAME_OF_CREATURE );
    assert_eq!(defaults.name                    ,             DEFAULT_NAME_OF_CREATURE.to_string(), );  // String,
    assert_eq!(defaults.species                 , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.level                   , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.experience              , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.livepoints_max          , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.livepoints_cur          , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.livepoints_regen_base   , shard_tool::DEFAULT_VALUE_I32,                    );  // i32,
    assert_eq!(defaults.livepoints_regen_cur    , shard_tool::DEFAULT_VALUE_I32,                    );  // i32,
    assert_eq!(defaults.magicpoints_max         , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.magicpoints_cur         , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.magicpoints_regen_base  , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.magicpoints_regen_cur   , shard_tool::DEFAULT_VALUE_U32,                    );  // u32,
    assert_eq!(defaults.strength                , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.endurance               , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.dexterity               , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.willpower               , shard_tool::DEFAULT_VALUE_U16,                    );  // u16, 
    assert_eq!(defaults.wisdom                  , shard_tool::DEFAULT_VALUE_U16,                    );  // u16, 
    assert_eq!(defaults.intelligence            , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.carry_capacity_base     , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.carry_capacity_cur      , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.carry_capacity_used     , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.movement_range_base     , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.movement_range_cur      , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.movement_range_used     , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.armor_base              , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.armor_cur               , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.attack_hit_chance_base  , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.attack_hit_chance_cur   , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.parry_base              , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.parry_cur               , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.evasion_base            , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    assert_eq!(defaults.evasion_cur             , shard_tool::DEFAULT_VALUE_U16,                    );  // u16,
    }

} // End of: mod test

