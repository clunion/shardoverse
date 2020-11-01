//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** pixel_painter.rs 🦀   
//! **`DESCRIPTION:`** a simple test for drawing pixels into an SDL2-window   
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
//use std::io;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

// use sdl2::gfx::primitives::DrawRenderer;
// use sdl2::pixels::*;
// //use sdl2::pixels::Color;
// use sdl2::render::{Canvas};
// use sdl2::video::{Window};

//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___

//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
//___ none ___

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________

/// Representation of the application state. In this example, a box will bounce around the screen.
pub (crate) struct PixelWorld 
{
    box_pos_x:   i32,
    box_pos_y:   i32,
    box_width:   i32,
    box_height:  i32,
    box_speed_x: i32,
    box_speed_y: i32,
    win_width:   u32,
    win_height:  u32,
}


//___ METHODS: ________________________________________________________________________________________________________________
impl PixelWorld {

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  new   
/// **`TYPE:       `**  method of PixelWorld   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Creates a new empty PixelWorld struct   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-10 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________

    /// Create a new `PixelWorld` instance that can draw a moving box.
    pub (crate) fn new() -> Self 
    {
    Self {
         box_pos_x:    32,
         box_pos_y:    32,
         box_width:    32,
         box_height:   32,
         box_speed_x:   8,
         box_speed_y:   4,
         win_width:   320,
         win_height:  200,
         }
    }

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  update   
/// **`TYPE:       `**  method of PixelWorld   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Update the `PixelWorld` internal state; bounce the box around the screen.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-10 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
    pub (crate) fn update(&mut self) 
    {
        if self.box_pos_x <= 0 || self.box_pos_x + self.box_width  > self.win_width  as i32 
            {
            self.box_speed_x *= -1;
            }

        if self.box_pos_y <= 0 || self.box_pos_y + self.box_height > self.win_height as i32 
            {
            self.box_speed_y *= -1;
            }

        self.box_pos_x += self.box_speed_x;
        self.box_pos_y += self.box_speed_y;
    }

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  resize   
/// **`TYPE:       `**  method of PixelWorld   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **`               `** None   
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Resize the `PixelWorld`.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-10 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________
    pub (crate) fn resize(&mut self, win_width_p: u32, win_height_p: u32 ) 
    {
        self.win_width  = win_width_p;
        self.win_height = win_height_p;
    }

/// ___________________________________________________________________________________________________________________________
/// **`METHOD:     `**  draw   
/// **`TYPE:       `**  method of PixelWorld   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` frame         `**        
/// **`RETURNS:    `** **`               `** None   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Draw the PixelWorld's state to the frame buffer.   
/// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`].   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-10-10 | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// *   
/// ___________________________________________________________________________________________________________________________

    pub (crate) fn  draw(&self, frame: &mut [u8]) 
        {

        if self.win_width == 0 {return;}
        
        let  stretch_factor_x: f64 = self.win_width  as f64 / 256.0;
        let  stretch_factor_y: f64 = self.win_height as f64 / 256.0 ;
            
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() 
            {
            let x = (i % self.win_width  as usize) as i32;
            let y = (i / self.win_width  as usize) as i32;

            let inside_the_box  = x >= self.box_pos_x
                               && x <  self.box_pos_x + self.box_width
                               && y >= self.box_pos_y
                               && y <  self.box_pos_y + self.box_height;

            let fl_x  = x as f64;
            let fl_y  = y as f64;
    
            // linear functions:
            // let red   = (x as f64 /stretch_factor_x ) as u8;
            // let green = (y as f64 /stretch_factor_y ) as u8;
            // let blue  = (y as f64 /stretch_factor_y ) as u8;
    
            // trigonometric functions:
            // let red   = (((fl_x.sin()+10.0).abs() *  9.0)       % 255.0) as u8;
            // let green = (((fl_x.sin()+10.0).abs() * 10.0)       % 255.0) as u8;
            // let blue  = (((fl_x.sin()+10.0).abs() * 11.0)       % 255.0) as u8;
    
            // combination of linear and trigonometric functions:
            let red   = ( (x as f64 / stretch_factor_x )+((fl_x/fl_y).cos() *  23.0) % 256.0 ) as u8;
            let green = ( (y as f64 / stretch_factor_y )+((fl_x/fl_y).cos() *  72.0) % 256.0 ) as u8;
            let blue  = ( (y as f64 / stretch_factor_y )+((fl_x/fl_y).cos() * 156.0) % 256.0 ) as u8;
            let alpha = 0xff;
    
            // debug!("x={}, y={} --> red={}, green={}, blue={}", x, y, red, green, blue);

            let  rgba = if inside_the_box { [green , blue    , red    , 0x04 ] } 
            else                          { [red   , green   , blue   , alpha] };

            pixel.copy_from_slice(&rgba);
            }
        }
}


/*
/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for pixel_painter   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________

#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `**  test_formula_fill()   
  /// **`TYPE:       `**  unit test   
  /// **`TESTS:      `**  checks if formula_fill() returns ok()   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_formula_fill() 
  {
    let result = pixel_painter(Path::new("non_existent_file"));
    assert!(!result.is_ok());
  }
}
*/
