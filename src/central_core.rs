/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     central_core.rs
## SYNOPSIS: the core of shardoverse where it all comes together, contains the main event loop, ...
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
use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::pixels;
use sdl2::surface::Surface;


//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
use crate::modules::pixel_draw;        // <dirname>::<filename>

use crate::modules::config::WindowConfig;

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
## FUNCTION:   run
## TYPE:       common function, only called once from main.rs! 
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  ?
## RETURNS:    ?
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## the central event loop, tying input to logic to output
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn run<'a>(win_config_p: &'a mut WindowConfig, png: &Path) -> Result<&'a WindowConfig, String> {
let mut lastx = 0;
let mut lasty = 0;
let mut tick  = 0;
// let mut win_position : (i32,i32) = (199,188);
// let mut win_size     : (u32,u32) = (400,600);

println!("Values in win_config_p:");
println!("title      {:?}", win_config_p.title);
println!("win_pos_x  {:?}", win_config_p.pos_x);
println!("win_pos_y  {:?}", win_config_p.pos_y);
println!("win_width  {:?}", win_config_p.width);
println!("win_height {:?}", win_config_p.height);
println!("active     {:?}", win_config_p.active);

let sdl_context = sdl2::init()?;
let video_subsys = sdl_context.video()?;

let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

let window = video_subsys.window("Shardoverse", win_config_p.width, win_config_p.height)
             .position(win_config_p.pos_x, win_config_p.pos_y)
             .resizable()
             .opengl()
             .build()
             .map_err(|e| e.to_string())?;

let mut canvas = window.into_canvas()
                 .present_vsync()
                 .build()
                 .map_err(|e| e.to_string())?;

let surface = Surface::from_file(png)
             .map_err(|err| format!("failed to load cursor image: {}", err))?;

let cursor = Cursor::from_surface(surface, 0, 0)
            .map_err(|err| format!("failed to load cursor: {}", err))?;

cursor.set();

canvas.set_draw_color(Color::RGBA( 50, 50, 50, 255));
canvas.clear();
canvas.present();

let mut events = sdl_context.event_pump()
                .map_err(|e| e)?;

'main: loop 
    {
    for event in events.poll_iter() 
        {
        match event 
            {
            Event::Quit {..}                            => break 'main,
            Event::KeyDown {keycode: Some(keycode), ..} => {
                                                           if      keycode == Keycode::Escape { println!("Esc");  break 'main } 
                                                           else if keycode == Keycode::Q      { println!("Q");    break 'main }
                                                           else if keycode == Keycode::P      { println!("P");    pixel_draw::formula_fill(&mut canvas); }
                                                           }
                                                           
            Event::MouseButtonDown {x, y, ..} =>           {
                                                           let color = pixels::Color::RGB(x as u8, y as u8, 255);
                                                           let  _    = canvas.line(lastx, lasty, x as i16, y as i16, color);
                                                           lastx     = x as i16;
                                                           lasty     = y as i16;
                                                           println!("mouse btn down at ({},{})", x, y);
                                                           canvas.present();
                                                           }
            _ => {}
            }
        }


    // Update the window title:
    let window       = canvas.window_mut();
    let win_position = window.position();
    let win_size     = window.size();
    let win_title    = format!("Window - pos({}x{}), size({}x{}): {}", win_position.0, win_position.1, win_size.0, win_size.1, tick);
    window.set_title(&win_title).map_err(|e| e.to_string())?;

    tick += 1;
    std::thread::sleep(Duration::from_millis(100));

    win_config_p.title  = win_title;           // todo: move outside of loop!
    win_config_p.pos_x  = win_position.0;      // todo: move outside of loop!
    win_config_p.pos_y  = win_position.1;      // todo: move outside of loop!
    win_config_p.width  = win_size.0;          // todo: move outside of loop!
    win_config_p.height = win_size.1;          // todo: move outside of loop!
    win_config_p.active = true;                // todo: move outside of loop!

    } // end of: loop main


println!("Values in win_config_p before return:");
println!("title      {:?}", win_config_p.title);
println!("win_pos_x  {:?}", win_config_p.pos_x);
println!("win_pos_y  {:?}", win_config_p.pos_y);
println!("win_width  {:?}", win_config_p.width);
println!("win_height {:?}", win_config_p.height);
println!("active     {:?}", win_config_p.active);

Ok(win_config_p)
}


#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /*
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## FUNCTION:   test_run()
  ## TYPE:       unit test function
  ## ---------------------------------------------------------------------------------------------------------------------------
  ## PARAMETER:  -
  ## RETURNS:    -
  ## ---------------------------------------------------------------------------------------------------------------------------
  */
  #[test]
  fn test_run() 
  {
    let mut win_config = WindowConfig::default();
    let result = run(&mut win_config, Path::new("non_existant_file"));
    assert!(!result.is_ok());
  }
}

