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
//! * add tests
//! ___________________________________________________________________________________________________________________________
//!    

//___ MODULES EXTERNAL: _______________________________________________________________________________________________________
// Extern crate declarations only in main.rs (to be reevaluated later)

use std::path::MAIN_SEPARATOR;

//___ none ___

//___ MODULES LOCAL: __________________________________________________________________________________________________________
//___ none ___

//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  basename   
/// **`TYPE:       `**  public, common helper function   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` path          `** a str, which is expected to contain a path with a file name   
/// **`RETURNS:    `** **` Result -->    `** a str containing only the file name   
/// **`            `** **`     or -->    `** gives back the provided str   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// small helper function, extracting the basename of a given &str. For occasions where the path can not be provided in a 
/// a variable of type Path.
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-07-08 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * nothing    
/// ___________________________________________________________________________________________________________________________
pub(crate) fn basename(path: &str) -> &str 
{
let mut pieces = path.rsplitn(2, MAIN_SEPARATOR);
match pieces.next() 
    {
    Some(p) => p,
    None    => path,
    }
}


/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for shard_tools   
/// **`TYPE:       `** unit test functions   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_basename_simple()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  abc/filename.ext -> filename.ext   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_basename_simple() 
  { 
    let mut pathconstr: String = "".to_string();
    let     pathpart1:  &str   = "abc";
    let     filename:   &str   = "filename.ext";

    pathconstr.push_str(pathpart1);
    pathconstr.push(MAIN_SEPARATOR);
    pathconstr.push_str(filename);
    let testpath: &str = pathconstr.as_str();

    let result = basename(testpath);
    assert_eq!(result,filename);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_basename_multiple_separator()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  qwert/abc/filename.ext -> filename.ext   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_basename_multiple_separator() 
  {
    let mut pathconstr: String = "".to_string();
    let     pathpart1:  &str   = "qwert";
    let     pathpart2:  &str   = "abc";
    let     filename:   &str   = "filename.ext";

    pathconstr.push_str(pathpart1);
    pathconstr.push(MAIN_SEPARATOR);
    pathconstr.push_str(pathpart2);
    pathconstr.push(MAIN_SEPARATOR);
    pathconstr.push_str(filename);
    let testpath: &str = pathconstr.as_str();

    let result = basename(testpath);
    assert_eq!(result,filename);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_basename_leading_separator()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  /filename.ext -> filename.ext   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_basename_leading_separator() 
  {
    let mut pathconstr: String = "".to_string();
    let     filename:   &str   = "filename.ext";

    pathconstr.push(MAIN_SEPARATOR);
    pathconstr.push_str(filename);
    let testpath: &str = pathconstr.as_str();

    let result = basename(testpath);
    assert_eq!(result,filename);
  }

  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_basename_no_separator()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  filename.ext -> filename.ext   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_basename_no_separator() 
  {
    let mut pathconstr: String = "".to_string();
    let     filename:   &str   = "filename.ext";

    pathconstr.push_str(filename);
    let testpath: &str = pathconstr.as_str();

    let result = basename(testpath);
    assert_eq!(result,filename);
  }
  
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_basename_trailing_separator()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  "filename.ext/" -> ""   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_basename_trailing_separator() 
  {
    let mut pathconstr: String = "".to_string();
    let     filename:   &str   = "filename.ext";

    pathconstr.push_str(filename);
    pathconstr.push(MAIN_SEPARATOR);
    let testpath: &str = pathconstr.as_str();

    let result = basename(testpath);
    assert_eq!(result,"");
  }
  
} // End of: mod test



