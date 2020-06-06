/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     pixel_draw.rs
## SYNOPSIS: a simple test for drawing pixels into an SDL2-window
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
// Extern crate declarations only in main.rs (to be reevaluated later)

//use std::io;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::*;
//use sdl2::pixels::Color;
use sdl2::render::{Canvas};
use sdl2::video::{Window};

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
//pub mod pixel_draw;

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
## FUNCTION:   formula_fill
## TYPE:       simple local function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  canvas
## RETURNS:    Result
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Test for pixel-drawing, fills the current SDL2-window with colored pixels based on mathematical formulae
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn formula_fill(canvas_p: &mut Canvas<Window>) -> bool
{
let retval: bool = true;

let     width:  i16;
let     height: i16;

match canvas_p.output_size()
    {
    Ok((w,h))  => {width=w as i16; height=h as i16; },
    Err(error) => { println!("Error getting canvas size: {}", error); return false},
    }

println!("canvas size: width={}, height={}",width,height);

let     stretch_factor_x: f64 = width  as f64 / 256.0;
let     stretch_factor_y: f64 = height as f64 / 256.0 ;
    
for y in 1..height 
    {
    for x in 1..width 
        {
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

        // println!("x={}, y={} --> red={}, green={}, blue={}", x, y, red, green, blue);

        let color = Color::RGB(red, green, blue);

        match canvas_p.pixel(x, y, color)
            {
            Ok(_)      => { },
            Err(error) => { println!("Error, draw pixel x='{}' y={}: {}", x, y, error); return false},
            };
        }
    }

canvas_p.present();

retval
} // end of: pixel_draw()


/*
#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /*
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## FUNCTION:   test_formula_fill()
  ## TYPE:       unit test function
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## PARAMETER:  -
  ## RETURNS:    -
  ## ---------------------------------------------------------------------------------------------------------------------------
  */
  #[test]
  fn test_pixel_draw() 
  {
    let result = pixel_draw(Path::new("non_existant_file"));
    assert!(!result.is_ok());
  }
}
*/