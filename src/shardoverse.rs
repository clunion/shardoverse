/*
## ---------------------------------------------------------------------------------------------------------------------------
## PROJECT:             Shardoverse
## HOME:      https://github.com/clunion/shardoverse
## ---------------------------------------------------------------------------------------------------------------------------
## FILE:     shardoverse.rs
## SYNOPSIS: main, start and entry point of the program
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## A Roguelike Peer-to-Peer Multi Player Dungeon Explorer and Fortres Builder (?) Game written in Rust
##----------------------------------------------------------------------------------------------------------------------------
## LICENSE:
## Copyright 2020 by Christian Lunau (clunion), Julian Lunau and Jaron Lunau.
## MIT-License, see LICENSE.md file 
##
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

//--- MODULES LOCAL: ---------------------------------------------------------------------------------------------------------
//--- none ---

//--- MODULE USES: -----------------------------------------------------------------------------------------------------------
use std::collections::HashSet;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window};
use sdl2::render::{Canvas};
use sdl2::gfx::primitives::DrawRenderer;


// old C-Style section comments, to be replaced by Rust-equivalents:
//--- CONSTANTS: -------------------------------------------------------------------------------------------------------------
const INITIAL_WINDOW_WIDTH:  u32 = 1024;
const INITIAL_WINDOW_HEIGHT: u32 =  768;

//--- TYPE DEFINITIONS: ------------------------------------------------------------------------------------------------------
//--- none ---

//--- CLASS DECLARATIONS: ----------------------------------------------------------------------------------------------------
//--- none ---

//--- GLOBAL VARS: -----------------------------------------------------------------------------------------------------------
//--- none ---

//--- EXPORTS: ---------------------------------------------------------------------------------------------------------------
//--- none ---

//--- PROTOTYPES: ------------------------------------------------------------------------------------------------------------
//--- callbacks:




/*
## ---------------------------------------------------------------------------------------------------------------------------
## FUNCTION:   pixel_fill
## TYPE:       simple local function
## ---------------------------------------------------------------------------------------------------------------------------
## PARAMETER:  canvas
## RETURNS:    Result
## ---------------------------------------------------------------------------------------------------------------------------
## DESCRIPTION:
## Fills the current Window with colored pixels
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn pixel_fill(canvas_p: &mut Canvas<Window>) -> bool
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
        let red   = ( (x as f64 / stretch_factor_x )+(((fl_x/fl_y).cos() *  23.0)) % 256.0 ) as u8;
        let green = ( (y as f64 / stretch_factor_y )+(((fl_x/fl_y).cos() *  72.0)) % 256.0 ) as u8;
        let blue  = ( (y as f64 / stretch_factor_y )+(((fl_x/fl_y).cos() * 156.0)) % 256.0 ) as u8;

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

return retval;
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
## ---------------------------------------------------------------------------------------------------------------------------
## VERSION:    DATE:       AUTHOR: CHANGES:
## 1.0         2020        CLu     initial version
## ---------------------------------------------------------------------------------------------------------------------------
## TODO:     
## ---------------------------------------------------------------------------------------------------------------------------
*/
fn main() -> Result<(), String>
{
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

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

    canvas.set_draw_color(pixels::Color::RGB(60, 42, 89));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;
    let mut tick = 0;

    let mut events = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut prev_buttons = HashSet::new();

    'main: loop 
        {
        for event in events.poll_iter() 
            {

            match event 
                {
                Event::Quit {..}                            => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => 
                    {
                    if      keycode == Keycode::Escape { println!("Esc");  break 'main } 
                    else if keycode == Keycode::Q      { println!("Q");    break 'main }
                    else if keycode == Keycode::P      { println!("P");    pixel_fill(&mut canvas); }
                    }

                Event::MouseButtonDown {x, y, ..} => 
                    {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                    }

                _ => {}
                }
            }

        // get a mouse state
        let state = events.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!("X = {:?}, Y = {:?} : {:?} -> {:?}", state.x(), state.y(),  new_buttons, old_buttons);
        } // end of: loop main

        prev_buttons = buttons;

        std::thread::sleep(Duration::from_millis(100));

        // Update the window title.
        let window = canvas.window_mut();

        let position = window.position();
        let size = window.size();
        let title = format!("Window - pos({}x{}), size({}x{}): {}",
                            position.0,
                            position.1,
                            size.0,
                            size.1,
                            tick);
        window.set_title(&title).map_err(|e| e.to_string())?;

        tick += 1;

    }

    Ok(())
}
