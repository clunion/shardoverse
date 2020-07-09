//! ___________________________________________________________________________________________________________________________
//! **`PROJECT:    `** Shardoverse    
//! **`HOME:       `** [Shardoverse on GitHub](https://github.com/clunion/shardoverse)    
//! **`SYNOPSIS:   `** A Roguelike Peer-to-Peer Multi Player Dungeon Explorer Game written in Rust    
//! ___________________________________________________________________________________________________________________________
//! **`FILE:       `** central_core.rs 🦀   
//! **`DESCRIPTION:`** the core of shardoverse, where it all comes together, contains the main event loop, ...   
//! ___________________________________________________________________________________________________________________________
//! **`LICENSE:    `**   
//! Copyright 2020 by Christian Lunau (clunion), Julian Lunau (someone-out-there) and Jaron Lunau (endless-means).   
//! MIT-License, see LICENSE.md file    
//! ___________________________________________________________________________________________________________________________
//! VERSION: | DATE:      | AUTHOR:   | CHANGES:   
//! :---     | :---       | :---:     | :---   
//! 0.1      | 2020-04-04 | Clunion   | creation   
//! 0.2      | 2020-07-02 | Clunion   | changed comment style to markdown for rustdoc   
//! ___________________________________________________________________________________________________________________________
//!# Examples
//!```
//! Hmm, this is a logic module, have to think about what examples could be here...
//!     xxx
//!```
//! ___________________________________________________________________________________________________________________________
//! **`TODO:       `**   
//! * everything (nearly)
//! ___________________________________________________________________________________________________________________________
//!    

//___ MODULES EXTERNAL: _______________________________________________________________________________________________________
// Extern crate declarations only in main.rs (to be reevaluated later)

//use std::io;
use std::path::Path;
use std::time::Duration;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::pixels;
use sdl2::surface::Surface;


//___ MODULES LOCAL: _________________________________________________________________________________________________________
use crate::modules::pixel_draw;        // <dirname>::<filename>

use crate::modules::config::ShardConfig;

//___ CONSTANTS: _____________________________________________________________________________________________________________
//___ none ___

//___ TYPES: _________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: _______________________________________________________________________________________________________________
//___ none ___


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  run   
/// **`TYPE:       `**  common function, only called once from main.rs!   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` <changing> `**    
/// **`RETURNS:    `** **` Result --> `** - OK(WindowConfig)   
/// **`            `** **`     or --> `** - Error   
/// ___________________________________________________________________________________________________________________________
/// **`DESCRIPTION:`**   
/// Here it all comes together, this contains the central event loop, ties input to logic to outputup.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-04-## | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub fn run<'a>(shard_config_p: &'a mut ShardConfig, png: &Path) -> Result<&'a ShardConfig, String> {
let mut lastx = 0;
let mut lasty = 0;
let mut tick  = 0;

debug!("Values in shard_config_p:");
debug!("title      {:?}", shard_config_p.window.title);
debug!("win_pos_x  {:?}", shard_config_p.window.pos_x);
debug!("win_pos_y  {:?}", shard_config_p.window.pos_y);
debug!("win_width  {:?}", shard_config_p.window.width);
debug!("win_height {:?}", shard_config_p.window.height);
debug!("active     {:?}", shard_config_p.window.active);

let sdl_context = sdl2::init()?;
let video_subsys = sdl_context.video()?;

let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

let window = video_subsys.window("Shardoverse", shard_config_p.window.width, shard_config_p.window.height)
             .position(shard_config_p.window.pos_x, shard_config_p.window.pos_y)
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
                                                           if      keycode == Keycode::Escape { info!("Esc");  break 'main } 
                                                           else if keycode == Keycode::P      { info!("P");    pixel_draw::formula_fill(&mut canvas); }
                                                           }
                                                           
            Event::MouseButtonDown {x, y, ..} =>           {
                                                           let color = pixels::Color::RGB(x as u8, y as u8, 255);
                                                           let  _    = canvas.line(lastx, lasty, x as i16, y as i16, color);
                                                           lastx     = x as i16;
                                                           lasty     = y as i16;
                                                           info!("mouse btn down at ({},{})", x, y);
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

    shard_config_p.window.title  = win_title;           // todo: move outside of loop!
    shard_config_p.window.pos_x  = win_position.0;      // todo: move outside of loop!
    shard_config_p.window.pos_y  = win_position.1;      // todo: move outside of loop!
    shard_config_p.window.width  = win_size.0;          // todo: move outside of loop!
    shard_config_p.window.height = win_size.1;          // todo: move outside of loop!
    shard_config_p.window.active = true;                // todo: move outside of loop!

    } // end of: loop main


debug!("Values in shard_config_p before return:");
debug!("title      {:?}", shard_config_p.window.title);
debug!("win_pos_x  {:?}", shard_config_p.window.pos_x);
debug!("win_pos_y  {:?}", shard_config_p.window.pos_y);
debug!("win_width  {:?}", shard_config_p.window.width);
debug!("win_height {:?}", shard_config_p.window.height);
debug!("active     {:?}", shard_config_p.window.active);

Ok(shard_config_p)
}


/// ___________________________________________________________________________________________________________________________
/// **`TESTMODULE: `** for config   
/// **`TYPE:       `** unit tests   
/// ___________________________________________________________________________________________________________________________
#[cfg(test)]
mod tests 
{
  // importing names from outer (for mod tests) scope:
  use super::*;
  
  /// ___________________________________________________________________________________________________________________________
  /// **`FUNCTION:   `** test_run()   
  /// **`TYPE:       `** unit test   
  /// ___________________________________________________________________________________________________________________________
  /// **`RETURNS:    `** **` <none>     `**   
  /// ___________________________________________________________________________________________________________________________
  #[test]
  fn test_run() 
  {
    let mut shard_config = ShardConfig::default();
    let result = run(&mut shard_config, Path::new("non_existent_file"));
    assert!(!result.is_ok());
  }
}

