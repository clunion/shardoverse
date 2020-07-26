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

//___ DECLARATIONS OF SUBMODULES: _____________________________________________________________________________________________
//___ none ___

//___ PATHS TO MODULES TO USE: ________________________________________________________________________________________________
//use std::io;
use std::path::Path;
use std::time::Duration;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{InitFlag, LoadTexture, LoadSurface};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels;
use sdl2::surface::Surface;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::rect::Rect;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::modules::pixel_draw;            // crate::<dirname>::<filename>
use crate::modules::*;                     // crate::<dirname>::<filename>
use crate::modules::config::ShardConfig;   // crate::<dirname>::<filename>::<modulename>

//___ CONSTANTS: _____________________________________________________________________________________________________________
//___ none ___
static SCREEN_WIDTH : u32 = 800;
static SCREEN_HEIGHT : u32 = 600;


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
/// Here it all comes together, this contains the central event loop, ties input to logic to output.   
/// ___________________________________________________________________________________________________________________________
/// VERSION:| DATE:      | AUTHOR:   | CHANGES:   
/// :---    | :---       | :---:     | :---   
/// 1.0     | 2020-04-## | Clunion   | initial version   
/// ___________________________________________________________________________________________________________________________
/// **`TODO:       `**   
/// * everything   
/// ___________________________________________________________________________________________________________________________

pub(crate) fn run<'a>(shard_config_p: &'a mut ShardConfig) -> Result<&'a ShardConfig, String> 
{
let mut lastx = 0;
let mut lasty = 0;
let mut tick  = 0;

let cursor_img: &Path   = Path::new("assets/cursors/pointers_part_5/glove3.png");
let image_path:  String =           "assets/graphics/2D/tiles/DungeonCrawlStoneSoupFull/dungeon/floor/sandstone_floor_0.png".to_string();
let font_path:   String =           "assets/fonts/NugieRomanticItalic-8P6D.ttf".to_string();

debug!("Values in shard_config_p:");
debug!("title      {:?}", shard_config_p.window.title);
debug!("win_pos_x  {:?}", shard_config_p.window.pos_x);
debug!("win_pos_y  {:?}", shard_config_p.window.pos_y);
debug!("win_width  {:?}", shard_config_p.window.width);
debug!("win_height {:?}", shard_config_p.window.height);
debug!("active     {:?}", shard_config_p.window.active);

let default_creature: creature::Creature = creature::Creature::default();
match creature::display_values(&default_creature)
    {
    Ok(_)       => {},
    Err(error)  => { println!("ERROR displaying default_creature: {:?}", error); }, // return Err(error); },
    }
;

let default_item: item::Item = item::Item::default();
match item::display_values(&default_item)
    {
    Ok(_)       => {},
    Err(error)  => { println!("ERROR displaying default_item: {:?}", error); }, // return Err(error); },
    }
;

let sdl_context = sdl2::init()?;
let video_subsys = sdl_context.video()?;
let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

let window = video_subsys
            .window("Shardoverse", shard_config_p.window.width, shard_config_p.window.height)
            .position(shard_config_p.window.pos_x, shard_config_p.window.pos_y)
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

let mut canvas = window.into_canvas()
                .present_vsync()
                .build()
                .map_err(|e| e.to_string())?;

let surface = Surface::from_file(cursor_img)
             .map_err(|err| format!("failed to load cursor image: {}", err))?;

let cursor = Cursor::from_surface(surface, 0, 0)
            .map_err(|err| format!("failed to load cursor: {}", err))?;

let texture_creator = canvas.texture_creator();
let mut texture_manager = TextureManager::new(&texture_creator);
let mut font_manager = FontManager::new(&font_context);
let font_details = FontDetails { path: font_path.clone(), size: 64, };

cursor.set();

canvas.set_draw_color(pixels::Color::RGBA( 50, 50, 50, 255));
canvas.clear();

    // Load a font
    let mut font = font_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
                 .render("Hello Rust!")
                 .blended(pixels::Color::RGBA(255, 0, 0, 255))
                 .map_err(|e| e.to_string())?;

    let texture = texture_creator
                 .create_texture_from_surface(&surface)
                 .map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGBA(195, 217, 255, 255));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    canvas.copy(&texture, None, Some(target))?;


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
            } // end of: main-loop
        } // end of events-poll

    // will load the image texture + font only once
    let texture = texture_manager.load(image_path.as_str())?;
    let font    = font_manager.load(&font_details)?;

    // not recommended to create a texture from the font each iteration
    // but it is the simplest thing to do for this example
    let surface = font
                 .render("Shardoverse")
                 .blended(pixels::Color::RGBA(200, 200, 200, 155))
                 .map_err(|e| e.to_string())?;

    let font_texture = texture_creator
                      .create_texture_from_surface(&surface)
                      .map_err(|e| e.to_string())?;

    //let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 16;
    let target = get_centered_rect(200, 80, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    //draw all
    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.copy(&font_texture, None, Some(target))?;
    canvas.present();

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
}// end of: run()


//---------------------------------------------------------------------------------------------
type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

//---------------------------------------------------------------------------------------------
// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

//---------------------------------------------------------------------------------------------
impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
        where L: ResourceLoader<'l, R, Args = D>,
              D: Eq + Hash + ?Sized,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                             let resource = Rc::new(self.loader.load(details)?);
                             self.cache.insert(details.into(), resource.clone());
                             Ok(resource)
                         },
                         Ok)
    }
}

//---------------------------------------------------------------------------------------------
// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    fn load(&'l self, path: &str) -> Result<Texture, String> {
        println!("LOADED A TEXTURE");
        self.load_texture(path)
    }
}

//---------------------------------------------------------------------------------------------
// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}


//---------------------------------------------------------------------------------------------
// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        println!("LOAD(ED) A FONT");
        self.load_font(&details.path, details.size)
    }
}

//---------------------------------------------------------------------------------------------
// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

//---------------------------------------------------------------------------------------------
impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}


//---------------------------------------------------------------------------------------------
// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);


//---------------------------------------------------------------------------------------------
// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH  as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}



// /// ___________________________________________________________________________________________________________________________
// /// **`TESTMODULE: `** for config   
// /// **`TYPE:       `** unit tests   
// /// ___________________________________________________________________________________________________________________________
// #[cfg(test)]
// mod tests 
// {
//   // importing names from outer (for mod tests) scope:
//   use super::*;
//   
//   /// ___________________________________________________________________________________________________________________________
//   /// **`FUNCTION:   `** test_run()   
//   /// **`TYPE:       `** unit test   
//   /// **`TESTS:      `** checks if run(config_ok, wrong_path) returns !ok()   
//   /// ___________________________________________________________________________________________________________________________
//   #[test]
//   fn test_run() 
//   {
//     let mut shard_config = ShardConfig::default();
//     let result = run(&mut shard_config);
//     assert!(!result.is_ok());
//   }
// }

