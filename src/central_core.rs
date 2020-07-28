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

use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor};
//use sdl2::mouse::{Cursor, MouseButton, MouseState };
//use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
//use sdl2::pixels::Color;
use sdl2::image::{InitFlag, LoadTexture, LoadSurface};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::event::WindowEvent;

use crate::modules::pixel_draw;            // crate::<dirname>::<filename>
use crate::modules::*;                     // crate::<dirname>::<filename>
use crate::modules::config::ShardConfig;   // crate::<dirname>::<filename>::<modulename>

//___ CONSTANTS: _____________________________________________________________________________________________________________
//___ none ___

//___ TYPES: _________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: _______________________________________________________________________________________________________________
//___ none ___


//---------------------------------------------------------------------------------------------
// handle the annoying Rect i32
macro_rules! rect(
       ( $x:expr, $y:expr, $w:expr, $h:expr) 
    => ( Rect::new($x as i32, $y as i32, $w as u32, $h as u32) )
);



/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  run   
/// **`TYPE:       `**  common function, only called once from main.rs!   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` <changing> `**    
/// **`RETURNS:    `** **` Result --> `** - OK(WindowConfig)   
/// **`            `** **`     or --> `** - Error(Error-Message)   
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

pub(crate) fn run(shard_config_p: &mut ShardConfig) -> Result<&ShardConfig, String> 
{
let mut tick  = 0;

let cursor_path: &Path   = Path::new("assets/cursors/pointers_part_5/glove3.png");
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
    Err(error)  => { println!("ERROR displaying default_creature: {:?}", error); return Err(error.to_string()); },
    }
;

let default_item: item::Item = item::Item::default();
match item::display_values(&default_item)
    {
    Ok(_)       => {},
    Err(error)  => { println!("ERROR displaying default_item: {:?}", error); return Err(error.to_string()); },
    }
;

// -- Initialisation of SDL2: ----------------------------------------
let sdl_context    = sdl2::init()?;
let video_subsys   = sdl_context.video()?;
let mut timer      = sdl_context.timer()?;
let font_context   = sdl2::ttf::init().map_err(|e| e.to_string())?;
let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

let     window     = video_subsys
                    .window("Shardoverse", shard_config_p.window.width, shard_config_p.window.height)
                    .position(shard_config_p.window.pos_x, shard_config_p.window.pos_y)
                    .resizable()
                    .opengl()
                    .build()
                    .map_err(|e| e.to_string())?;

let mut canvas: Canvas<Window> = window
                                .into_canvas()
                                .present_vsync()
                                .build()
                                .map_err(|err| err.to_string())?;

let cursor_surface = Surface::from_file(cursor_path)
                    .map_err(|err| format!("failed to load cursor image: {}", err))?;

/*                    
pub fn blit_scaled<R1, R2>(
    &self,
    src_rect: R1,
    dst: &mut SurfaceRef,
    dst_rect: R2
) -> Result<Option<Rect>, String> 
*/

let cursor         = Cursor::from_surface(cursor_surface, 0, 0)
                    .map_err(|err| format!("failed to load cursor: {}", err))?;

cursor.set();

// this struct manages textures. For lifetime reasons, the canvas cannot directly create textures, you have to create a `TextureCreator` instead.
let texture_creator: TextureCreator<_> = canvas.texture_creator();

let mut texture_manager = TextureManager::new(&texture_creator);

// load the  font only once:
let mut font_manager   = FontManager::new(&font_context);
//let     font_details   = FontDetails { path: font_path.clone(), size: 64, };
let     font_details   = FontDetails { path: font_path, size: 64, };
let     font           = font_manager.load(&font_details)?;
let     font_surface   = font
                        .render("Shardoverse")
                        .blended(pixels::Color::RGBA(200, 200, 200, 155))
                        .map_err(|e| e.to_string())?;
                       
let     font_texture   = texture_creator
                        .create_texture_from_surface(&font_surface)
                        .map_err(|e| e.to_string())?;


// load the image texture only once:
let     ground_texture = texture_manager.load(image_path.as_str())?;

let mut events         = sdl_context.event_pump()
                        .map_err(|e| e)?;

// initialize values for FPS-counting:
#[allow(unused_assignments)]
let mut startclock:  u32 = 0; 
#[allow(unused_assignments)]
let mut deltaclock:  u32 = 0;
let mut current_fps: u32 = 0;

let mut rect_size        = 32;          // 32 is the tile-size, this also used as scaling-factor
let mut win              = canvas.window_mut();
#[allow(unused_assignments)]
let mut win_position     = win.position();
let mut win_size         = win.size();
let mut win_title        = format!("Shardoverse - scale: {:>3}, FPS: {:>4}, tick: {:>5}", rect_size, current_fps, tick);
win.set_title(&win_title).map_err(|e| e.to_string())?;

let mut row_tiles        = win_size.0 / rect_size;
let mut col_tiles        = win_size.1 / rect_size;


// sdl2::TimerSubsystem::ticks (SDL_GetTicks) - 32-bit millisecond precision, returns time since the SDL initialization, may not be enough
// or 
// sdl2::TimerSubsystem::performance_counter (SDL_GetPerformanceCounter) - 64-bit precision, returns the current value of the high resolution counter since the SDL initialization, 

// at beginning of loop
startclock = timer.ticks(); 

'main: loop 
    {
    canvas.set_draw_color(pixels::Color::RGBA( 50, 50, 50, 255));
    canvas.clear();

    for event in events.poll_iter() 
        {
        match event 
            {
            Event::Window {win_event: WindowEvent::Resized(w, h),..} => 
                                                           {
                                                           row_tiles = w as u32 / rect_size;
                                                           col_tiles = h as u32 / rect_size;
                                                           debug!("window resized to w: {} h: {}",w,h);
                                                           }

            Event::Window {win_event: WindowEvent::SizeChanged(w, h),..} => 
                                                           {
                                                           row_tiles = w as u32 / rect_size; col_tiles = h as u32 / rect_size;
                                                           debug!("window SizeChanged to w: {} h: {}",w,h);
                                                           }

            Event::KeyDown {keycode: Some(keycode), ..} => {
                                                           if      keycode == Keycode::Escape   { info!("Esc");     break 'main } 
                                                           else if keycode == Keycode::P        { info!("P");       pixel_draw::formula_fill(&mut canvas); }
                                                           else if keycode == Keycode::KpPlus   { info!("KpPlus");  if rect_size < 255  {rect_size +=  1;} }
                                                           else if keycode == Keycode::KpMinus  { info!("KpMinus"); if rect_size >   8  {rect_size -=  1;} }
                                                           else if keycode == Keycode::KpDivide { info!("KpDivide");                     rect_size  = 32;}
                                                           win       = canvas.window_mut();    // todo: exec only on scale-change
                                                           win_size  = win.size();             // todo: exec only on scale-change
                                                           row_tiles = win_size.0 / rect_size; // todo: exec only on scale-change
                                                           col_tiles = win_size.1 / rect_size; // todo: exec only on scale-change   
                                                           }
                                                           
            Event::MouseButtonDown {x, y, ..} =>           {
                                                           debug!("mouse button down at (x:{},y:{})", x, y);
                                                           }
            Event::MouseButtonUp {x, y, ..} =>             {
                                                           debug!("mouse button up   at (x:{},y:{})", x, y);
                                                           }
            Event::MouseWheel {timestamp, window_id, which, x, y, ..} => 
                                                           {
                                                           debug!("mouse wheel: timestamp {}, window_id {}, which {}, x {}, y {}, direction (?)",timestamp, window_id, which, x, y);
                                                           }

            Event::Quit {..}                            => break 'main,

                                                      _ => {}
            } // end of: match event
        } // end of events-poll

    // actual fps calculation inside loop
    let current_ticks = timer.ticks();
    deltaclock = current_ticks - startclock;
    startclock = current_ticks;
    if deltaclock != 0 {current_fps = 1000 / deltaclock;}  

    tick += 1;

    // Update the window title:
    win          = canvas.window_mut();
    win_title    = format!("Shardoverse - scale:{:>3}, FPS:{:>4}, tick:{:>5}", rect_size, current_fps, tick);
    win.set_title(&win_title).map_err(|e| e.to_string())?;

    //draw everything
    for col in 0 ..= col_tiles
        {
        for row in 0 ..= row_tiles 
            {
            trace!("row = {}, col = {}",row, col);
            canvas.copy(&ground_texture, rect!(   0,   0, rect_size, rect_size), rect!(  row*rect_size, col*rect_size,  rect_size,  rect_size))?;
            }
        }

    canvas.copy(&font_texture, None                    , rect!( 20,  20, 464,  64))?;
    canvas.present();  // display new content of the window

    std::thread::sleep(Duration::from_millis(100));
    } // end of: loop main

win          = canvas.window_mut();
win_position = win.position();
win_size     = win.size();
win_title    = format!("Shardoverse - scale: {:>3}, FPS: {:>4}, tick: {:>5}", rect_size, current_fps, tick);
win.set_title(&win_title).map_err(|e| e.to_string())?;

shard_config_p.window.title  = win_title;           
shard_config_p.window.pos_x  = win_position.0;      
shard_config_p.window.pos_y  = win_position.1;      
shard_config_p.window.width  = win_size.0;          
shard_config_p.window.height = win_size.1;          
shard_config_p.window.active = true;                


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
    pub fn new(loader_p: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader_p,
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
        println!("LOADED A FONT");
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

