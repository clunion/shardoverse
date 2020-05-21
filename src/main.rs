/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     main.rs
## SYNOPSIS: main, start and entry point of the program
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
extern crate sdl2;

//--- MODULES: ---------------------------------------------------------------------------------------------------------------
use std::env;
use std::io;
use std::path::Path;
//use std::collections::HashSet;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::pixels;
//use sdl2::rect::Rect;
use sdl2::render::{Canvas};
use sdl2::surface::Surface;
use sdl2::video::{Window};

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
mod config;
mod assets;

use crate::config::*;
use crate::assets::cursors::*;

//--- CONSTANTS: -------------------------------------------------------------------------------------------------------------
const INITIAL_WINDOW_WIDTH:  u32 = 1024;
const INITIAL_WINDOW_HEIGHT: u32 =  768;


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
## FUNCTION:   pixel_fill
## TYPE:       simple local function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  canvas
## RETURNS:    Result
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Test for pixel-Drawing, fills the current Window with colored pixels
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn pixel_fill(canvas_p: &mut Canvas<Window>) -> bool
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
}


/*
## ---------------------------------------------------------------------------------------------------------------------------
*/
pub fn run(png: &Path) -> Result<(), String> {
    let mut lastx = 0;
    let mut lasty = 0;
    let mut tick = 0;

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
    let window = video_subsys.window("Shardoverse", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
                 .position_centered()
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
                Event::KeyDown {keycode: Some(keycode), ..} =>  {
                                                                if      keycode == Keycode::Escape { println!("Esc");  break 'main } 
                                                                else if keycode == Keycode::Q      { println!("Q");    break 'main }
                                                                else if keycode == Keycode::P      { println!("P");    pixel_fill(&mut canvas); }
                                                                }

                Event::MouseButtonDown {x, y, ..} =>            {
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

    } // end of: loop main

    Ok(())
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
    let result = run(Path::new("non_existant_file"));
    assert!(!result.is_ok());
  }
}


/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   main
## TYPE:       entry point, 
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  ?
## RETURNS:    ?
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## The one and only main: startup and entry point of this program
## here only the handling of commandline paramaters is done
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn main() -> Result<(), io::Error>
{
let args: Vec<String> = env::args().collect();
let mut i :i32 = 0;

if !args.is_empty()
    {
    for arg in &args
        {
        println!("Parameter[{}] {:?}",i, &arg);
        i+=1;
        }
    } 

match load_config()
    {
    Ok(_)  => {},
    Err(error) => { println!("Error loading config: {:?}", error); return Err(error); },
    }


match load_cursors()
    {
    Ok(_)  => {},
    Err(error) => { println!("Error loading cursors: {:?}", error); return Err(error); },
    }


match shardoverse_init()
    {
    Ok(_)  => {},
    Err(error) => { println!("Error initialising: {:?}", error); return Err(error); },
    }

    
match run(Path::new("assets/cursors/pointers_part_5/glove3.png"))
    {
    Ok(_)  => {},
    Err(error) => { println!("Error initialissing: {:?}", error); }, //return Err(error); },
    }


match save_config()
    {
    Ok(_)  => {},
    Err(error) => { println!("Error saving config: {:?}", error); return Err(error); },
    }

Ok(())
}
