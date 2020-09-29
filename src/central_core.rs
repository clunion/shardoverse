#![deny(clippy::all)]
#![forbid(unsafe_code)]

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

// use std::path::Path;
// use std::borrow::Borrow;
// use std::collections::HashMap;
// use std::hash::Hash;
// use std::rc::Rc;

// use std::io::{self, BufRead};
// use std::sync::mpsc::{self, TryRecvError, Receiver};
// use std::thread;

extern crate image;

use std::thread;
use std::time;
use std::time::{Duration, Instant};
use std::path::Path;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use winit::
{
    window::{WindowBuilder, Fullscreen, Icon},
//  dpi::{LogicalSize, PhysicalPosition},
    dpi::{LogicalPosition, PhysicalSize},
    event::{ElementState, Event, StartCause, KeyboardInput, VirtualKeyCode, ModifiersState, WindowEvent},
//  event::{DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
};

//use pixels::{Error, Pixels, SurfaceTexture};
use pixels::{Pixels, SurfaceTexture};

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World 
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


//use crate::modules::*;                     // crate::<dirname>::<filename>
//use crate::modules::pixel_draw;            // crate::<dirname>::<filename>
use crate::modules::config::ShardConfig;   // crate::<dirname>::<filename>::<modulename>


//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___


//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

//___ ENUMS: __________________________________________________________________________________________________________________
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Wait,
    WaitUntil,
    Poll,
}


//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___


//___ MACROS: _________________________________________________________________________________________________________________

// build a Rect with i32s:
// macro_rules! rect( ( $x:expr, $y:expr, $w:expr, $h:expr) => ( Rect::new($x as i32, $y as i32, $w as u32, $h as u32) ) );

// fn thread_fun(rx_p: Receiver<()> ) -> ()
// {
// loop {
//         println!("Working...");
//         thread::sleep(Duration::from_millis(500));
//         match rx_p.try_recv() {
//             Ok(_) | Err(TryRecvError::Disconnected) => {
//                 println!("Terminating.");
//                 break;
//             }
//             Err(TryRecvError::Empty) => {}
//         }
//     }    
// }


fn load_icon(path: &Path) -> Icon 
{
let image = image::open(path)
           .expect("Failed to open icon path")
           .into_rgba();
let (icon_width, icon_height) = image.dimensions();
let icon_rgba = image.into_raw();
    
Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}



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

pub(crate) fn run_central_core(shard_config_p: &mut ShardConfig) -> Result<&ShardConfig, String> 
{

debug!("Values in shardoverse config:");
for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("conf_name: ...{:?}", win_conf.conf_name);
    debug!("conf_num: ....{}"  , win_conf.conf_num );
    debug!("title: .......{}"  , win_conf.title    );
    debug!("pos_x .=......{}"  , win_conf.pos_x    );
    debug!("pos_y .=......{}"  , win_conf.pos_y    );
    debug!("width .=......{}"  , win_conf.width    );
    debug!("height.=......{}"  , win_conf.height   );
    debug!("active.=......{}"  , win_conf.active   );
    } // end of: for


// let cursor_path: &Path  = Path::new("assets/cursors/pointers_part_5/glove3.png");
let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/gui/icons/shardoverse_icon_01.png");

let  game_icon = load_icon(Path::new(path));
let  maps_icon = load_icon(Path::new(path));
let  invt_icon = load_icon(Path::new(path));
let  jrnl_icon = load_icon(Path::new(path));
let  help_icon = load_icon(Path::new(path));
let  pixl_icon = load_icon(Path::new(path));


// -- Initialisation of winit: ----------------------------------------
let mut event_loop = EventLoop::new();


let     game_window = create_window(shard_config_p.window_configs[0].title.as_str(), game_icon, &event_loop, shard_config_p.window_configs[0].width , shard_config_p.window_configs[0].height, shard_config_p.window_configs[0].pos_x, shard_config_p.window_configs[0].pos_y);
let     maps_window = create_window(shard_config_p.window_configs[1].title.as_str(), maps_icon, &event_loop, shard_config_p.window_configs[1].width , shard_config_p.window_configs[1].height, shard_config_p.window_configs[1].pos_x, shard_config_p.window_configs[1].pos_y);
let     invt_window = create_window(shard_config_p.window_configs[2].title.as_str(), invt_icon, &event_loop, shard_config_p.window_configs[2].width , shard_config_p.window_configs[2].height, shard_config_p.window_configs[2].pos_x, shard_config_p.window_configs[2].pos_y);
let     jrnl_window = create_window(shard_config_p.window_configs[3].title.as_str(), jrnl_icon, &event_loop, shard_config_p.window_configs[3].width , shard_config_p.window_configs[3].height, shard_config_p.window_configs[3].pos_x, shard_config_p.window_configs[3].pos_y);
let     help_window = create_window(shard_config_p.window_configs[4].title.as_str(), help_icon, &event_loop, shard_config_p.window_configs[4].width , shard_config_p.window_configs[4].height, shard_config_p.window_configs[4].pos_x, shard_config_p.window_configs[4].pos_y);
let     pixl_window = create_window(shard_config_p.window_configs[5].title.as_str(), pixl_icon, &event_loop, shard_config_p.window_configs[5].width , shard_config_p.window_configs[5].height, shard_config_p.window_configs[5].pos_x, shard_config_p.window_configs[5].pos_y);
                                                                                     
let     pixl_window_size     = pixl_window.inner_size();
let     pixl_surface_texture = SurfaceTexture::new(pixl_window_size.width, pixl_window_size.height, &pixl_window);
let mut pixels               = Pixels::new(        pixl_window_size.width, pixl_window_size.height, pixl_surface_texture).unwrap();
let mut pixl_world           = World::new();


info!("Usage:");
info!("  Esc - Quit");
info!("  1   - switch to Wait mode.");
info!("  2   - switch to WaitUntil mode.");
info!("  3   - switch to Poll mode.");
info!("  R   - toggle request_redraw() calls.");
info!("  F   - Toggle borderless fullscreen (on current desktop screen)");
info!("  M   - Toggle minimized");
info!("  X   - Toggle maximized");
info!("  V   - Toggle visibility");

// event-loop:
let mut delay_in_milis:  u64 = 100;
let mut wait_time:       time::Duration = time::Duration::from_millis(delay_in_milis);
let mut poll_sleep_time: time::Duration = time::Duration::from_millis(delay_in_milis);

// window:
let mut minimized = false;
let mut maximized = false;
let mut visible   = true;

// control-flow:
let mut mode                   = Mode::WaitUntil;
let mut request_redraw:  bool  = true;
let mut wait_cancelled:  bool  = false;
let mut close_requested: bool  = false;
let mut delay_in_loop:   bool  = true;


// initialize values for FPS-counting:
let mut start     = Instant::now();
let mut delta     = Duration::new(0,0);  // just to avoid error[E0381]: borrow of possibly-uninitialized variable: `delta`
let mut tick: u32 = 0;
let mut fps:  f64 = 0.0;

let mut modifers_state: ModifiersState = ModifiersState::default();

event_loop.run_return(|event, _, control_flow| 
    {
    trace!("{:?}", event);
        
    *control_flow = ControlFlow::WaitUntil(time::Instant::now() + wait_time);

    // actual FPS calculation inside loop:
    delta = start.elapsed();
    if  delta > Duration::new(1,0)  // if delta is > 1 second
        {
        fps = (tick as f64/delta.as_millis() as f64) * 1_000.0 as f64;
        debug!("delay_in_milis={}, start={:?}, delta={:?}, tick={}, fps={}", delay_in_milis, start, delta.as_secs(), tick, fps);
        tick=0;                  // re-init for counting the loops in the next second
        start = Instant::now();  // re-init for counting the loops in the next second
        // or better (c++ code, averaging some time samples):
        // static const int NUM_FPS_SAMPLES = 64;float fpsSamples[NUM_FPS_SAMPLES]int currentSample = 0;float CalcFPS(int dt){    fpsSamples[currentSample % NUM_FPS_SAMPLES] = 1.0f / dt;    float fps = 0;    for (int i = 0; i < NUM_FPS_SAMPLES; i++)        fps += fpsSamples;    fps /= NUM_FPS_SAMPLES;    return fps;}
        }
    trace!("delay_in_milis={}, start={:?}, delta={:?}, tick={}, fps={}", delay_in_milis, start, delta.as_secs(), tick, fps);
    tick += 1;

    // Update the window title:
    pixl_window.set_title(&format!("Shardoverse - scale: {:>3}, tick: {:>5}, delay: {:>3}, FPS: {:>-8.2}", 0, tick, delay_in_milis, fps));

    match event 
        {
        Event::NewEvents(start_cause)      => {
                                              wait_cancelled = match start_cause {
                                                                                 StartCause::WaitCancelled { .. } => mode == Mode::WaitUntil,
                                                                                                                _ => false,
                                                                                 }
                                              }
        Event::WindowEvent { event, window_id }   => match event {
                                                        WindowEvent::ModifiersChanged(m) => { // Key-Modifier states: (ctrl, alt, shift, logo)
                                                                                            modifers_state = m;
                                                                                            debug!("ModifiersChanged to state: {:?}",modifers_state);
                                                                                            }
                                                        WindowEvent::CloseRequested => {
                                                                                       println!("Window {:?} has received the signal to close", window_id);
                                                                                       }
                                                                                       
                                                        WindowEvent::KeyboardInput  {input: KeyboardInput {virtual_keycode: Some(virtual_code), state: ElementState::Pressed, .. }, .. } 
                                                                                    => match virtual_code {
                                                                                                         VirtualKeyCode::Escape   => {
                                                                                                                                     debug!("close_requested");
                                                                                                                                     close_requested = true;
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Multiply => {
                                                                                                                                     if  modifers_state.ctrl() 
                                                                                                                                         {
                                                                                                                                         debug!("toggle delay in loop to: {}",!delay_in_loop);
                                                                                                                                         delay_in_loop = !delay_in_loop; 
                                                                                                                                         }
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Subtract => { // decrease speed
                                                                                                                                     if  modifers_state.ctrl() 
                                                                                                                                         {
                                                                                                                                         if delay_in_milis <= 990 {delay_in_milis += 10}; 
                                                                                                                                         debug!("raise delay in loop to: {}",delay_in_milis);
                                                                                                                                         wait_time       = time::Duration::from_millis(delay_in_milis);
                                                                                                                                         poll_sleep_time = time::Duration::from_millis(delay_in_milis);
                                                                                                                                         }
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Add      => { // increase speed 
                                                                                                                                     if  modifers_state.ctrl() 
                                                                                                                                         {
                                                                                                                                         if delay_in_milis >= 10 {delay_in_milis -= 10}; 
                                                                                                                                         debug!("lower delay in loop to: {}",delay_in_milis);
                                                                                                                                         wait_time       = time::Duration::from_millis(delay_in_milis);
                                                                                                                                         poll_sleep_time = time::Duration::from_millis(delay_in_milis);
                                                                                                                                         }
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Key1     => {
                                                                                                                                     debug!("event_loop-mode: {:?}", mode);
                                                                                                                                     mode = Mode::Wait;
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Key2     => {
                                                                                                                                     debug!("event_loop-mode: {:?}", mode);
                                                                                                                                     mode = Mode::WaitUntil;
                                                                                                                                     }
                                                                                                         VirtualKeyCode::Key3     => {
                                                                                                                                     debug!("event_loop-mode: {:?}", mode);
                                                                                                                                     mode = Mode::Poll;
                                                                                                                                     }
                                                                                                         VirtualKeyCode::R        => {
                                                                                                                                     debug!("request_redraw: {}", request_redraw);
                                                                                                                                     request_redraw = !request_redraw;
                                                                                                                                     }
                                                                                                         VirtualKeyCode::F        => {
                                                                                                                                     debug!("Toggle borderless fullscreen");
                                                                                                                                     if  pixl_window.fullscreen().is_some() 
                                                                                                                                         {
                                                                                                                                         pixl_window.set_fullscreen(None);
                                                                                                                                         } 
                                                                                                                                     else{
                                                                                                                                         let monitor = pixl_window.current_monitor();
                                                                                                                                         pixl_window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
                                                                                                                                         }
                                                                                                                                     }
                                                                                                         VirtualKeyCode::M        => {
                                                                                                                                     debug!("toggle minimized to: {}", !minimized);
                                                                                                                                     minimized = !minimized;
                                                                                                                                     pixl_window.set_minimized(minimized);
                                                                                                                                     }
                                                                                                         VirtualKeyCode::V        => {
                                                                                                                                     debug!("toggle {:?} visible to: {}", window_id, !visible);
                                                                                                                                     visible = !visible;
                                                                                                                                     pixl_window.set_visible(visible);
                                                                                                                                     }
                                                                                                         VirtualKeyCode::X        => {
                                                                                                                                     debug!("toggle maximized to: {}", !maximized);
                                                                                                                                     maximized = !maximized;
                                                                                                                                     pixl_window.set_maximized(maximized);
                                                                                                                                     }
                                                                                                         _                        => {
                                                                                                                                     info!("unused virtual_code = '{:?}'",virtual_code);
                                                                                                                                     }
                                                                                                         },  // end of match virtual_code
                                                        WindowEvent::Resized(new_size) => {
                                                                                          debug!("Window resized to: {:?}",new_size);
                                                                                          pixl_world.resize( new_size.width, new_size.height);
                                                                                          pixels.resize(new_size.width, new_size.height);
                                                                                          //pixl_window.request_redraw();  // ???
                                                                                          }
                                                                                    _  => {trace!("Window-event={:?}",event);},
                                                        }, // end of: match WindowEvent(event)
        Event::MainEventsCleared           => {
                                              if  request_redraw && !wait_cancelled && !close_requested 
                                                  {
                                                  pixl_window.request_redraw();
                                                  }
                                              if  close_requested 
                                                  {
                                                  *control_flow = ControlFlow::Exit;   // Urgs, when using the winit::run() function, this does an immediate program exit...
                                                  }
                                              }
        Event::RedrawRequested(window_id)  => {
                                              info!("Redraw requested for window_id {:?}",window_id);

                                              // Update internal state and request a redraw:
                                              pixl_world.update();
                                              let frame = pixels.get_frame();

                                              pixl_world.draw( frame );   // draws content into a frame-buffer

                                              if  pixels
                                                  .render()
                                                  .map_err(|e| error!("pixels.render() failed: {}", e))
                                                  .is_err()
                                                {
                                                *control_flow = ControlFlow::Exit;
                                                return;
                                                }

                                              }
        Event::RedrawEventsCleared         => {
                                              *control_flow = match mode {
                                                                         Mode::Wait      => ControlFlow::Wait,
                                                                         Mode::WaitUntil => {
                                                                                            if  wait_cancelled 
                                                                                                {
                                                                                                *control_flow
                                                                                                } 
                                                                                            else{
                                                                                                ControlFlow::WaitUntil(time::Instant::now() + wait_time)
                                                                                                }
                                                                                            }
                                                                         Mode::Poll      => {
                                                                                            thread::sleep(poll_sleep_time);
                                                                                            ControlFlow::Poll
                                                                                            }
                                                                         };
                                              }
        _                                  => {
//                                              info!("outer event={:?}",event);
                                              },
        };  // end of: match event
    }  // end of: event loop
    ); // end of: event clojure loop run[_return]()


let game_window_pos  = game_window.outer_position().unwrap();  debug!("game_window_inner_pos.x: {}, y: {}"    , game_window_pos.x     , game_window_pos.y);
let game_window_size = game_window.inner_size();               debug!("game_window_inner_size: {}, height: {}", game_window_size.width, game_window_size.height);

let maps_window_pos  = maps_window.outer_position().unwrap();  debug!("maps_window_inner_pos.x: {}, y: {}"    , maps_window_pos.x     , maps_window_pos.y);
let maps_window_size = maps_window.inner_size();               debug!("maps_window_inner_size: {}, height: {}", maps_window_size.width, maps_window_size.height);

let invt_window_pos  = invt_window.outer_position().unwrap();  debug!("invt_window_inner_pos.x: {}, y: {}"    , invt_window_pos.x     , invt_window_pos.y);
let invt_window_size = invt_window.inner_size();               debug!("invt_window_inner_size: {}, height: {}", invt_window_size.width, invt_window_size.height);

let jrnl_window_pos  = jrnl_window.outer_position().unwrap();  debug!("jrnl_window_inner_pos.x: {}, y: {}"    , jrnl_window_pos.x     , jrnl_window_pos.y);
let jrnl_window_size = jrnl_window.inner_size();               debug!("jrnl_window_inner_size: {}, height: {}", jrnl_window_size.width, jrnl_window_size.height);

let help_window_pos  = help_window.outer_position().unwrap();  debug!("help_window_inner_pos.x: {}, y: {}"    , help_window_pos.x     , help_window_pos.y);
let help_window_size = help_window.inner_size();               debug!("help_window_inner_size: {}, height: {}", help_window_size.width, help_window_size.height);

let pixl_window_pos  = pixl_window.outer_position().unwrap();  debug!("window_inner_pos.x: {}, y: {}",          pixl_window_pos.x     , pixl_window_pos.y);
let pixl_window_size = pixl_window.inner_size();               debug!("window_inner_size: {}, height: {}",      pixl_window_size.width, pixl_window_size.height);

shard_config_p.window_configs[0].pos_x  = game_window_pos.x;      
shard_config_p.window_configs[0].pos_y  = game_window_pos.y;      
shard_config_p.window_configs[0].width  = game_window_size.width;          
shard_config_p.window_configs[0].height = game_window_size.height;          
shard_config_p.window_configs[0].active = true;                

shard_config_p.window_configs[1].pos_x  = maps_window_pos.x;      
shard_config_p.window_configs[1].pos_y  = maps_window_pos.y;      
shard_config_p.window_configs[1].width  = maps_window_size.width;          
shard_config_p.window_configs[1].height = maps_window_size.height;          
shard_config_p.window_configs[1].active = true;                

shard_config_p.window_configs[2].pos_x  = invt_window_pos.x;      
shard_config_p.window_configs[2].pos_y  = invt_window_pos.y;      
shard_config_p.window_configs[2].width  = invt_window_size.width;          
shard_config_p.window_configs[2].height = invt_window_size.height;          
shard_config_p.window_configs[2].active = true;                

shard_config_p.window_configs[3].pos_x  = jrnl_window_pos.x;      
shard_config_p.window_configs[3].pos_y  = jrnl_window_pos.y;      
shard_config_p.window_configs[3].width  = jrnl_window_size.width;          
shard_config_p.window_configs[3].height = jrnl_window_size.height;          
shard_config_p.window_configs[3].active = true;                

shard_config_p.window_configs[4].pos_x  = help_window_pos.x;      
shard_config_p.window_configs[4].pos_y  = help_window_pos.y;      
shard_config_p.window_configs[4].width  = help_window_size.width;          
shard_config_p.window_configs[4].height = help_window_size.height;          
shard_config_p.window_configs[4].active = true;                

shard_config_p.window_configs[5].pos_x  = pixl_window_pos.x;      
shard_config_p.window_configs[5].pos_y  = pixl_window_pos.y;      
shard_config_p.window_configs[5].width  = pixl_window_size.width;          
shard_config_p.window_configs[5].height = pixl_window_size.height;          
shard_config_p.window_configs[5].active = true;                

debug!("Values in shard_config_p before return:");
for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("conf_name: .....{:?}", win_conf.conf_name  );
    debug!("Conf-Num: ....{}"  , win_conf.conf_num );
    debug!("Window-Title: {}"  , win_conf.title    );
    debug!("pos_x .=......{}"  , win_conf.pos_x    );
    debug!("pos_y .=......{}"  , win_conf.pos_y    );
    debug!("width .=......{}"  , win_conf.width    );
    debug!("height.=......{}"  , win_conf.height   );
    debug!("active.=......{}"  , win_conf.active   );
    } // end of: for

Ok(shard_config_p)
} // end of: run()


/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, pos_x, pos_y, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window( title_p: &str, icon_p: Icon, event_loop_p: &EventLoop<()>, try_width_p: u32, try_height_p: u32, try_pos_x: i32, try_pos_y: i32) -> winit::window::Window
{

    // Create a hidden window so we can derive a scaling-factor:
    let window = WindowBuilder::new()
                .with_title(title_p)
                .with_window_icon(Some(icon_p))
                .with_visible(false)
                .build(&event_loop_p)
                .unwrap();

    window.set_outer_position(LogicalPosition::new(try_pos_x, try_pos_y));  // to get the correct current_monitor()

    let hidpi_factor   = window.scale_factor();

    // calculate dimensions which are fitting on the monitor:
    //let monitor_size   = window.current_monitor().size();
    //let monitor_width  = monitor_size.width  as f64 / hidpi_factor; 
    //let monitor_height = monitor_size.height as f64 / hidpi_factor;
    
    //let hori_scale: f64 = (monitor_width  / try_width_p  as f64 * 2.0 / 3.0).round();
    //let vert_scale: f64 = (monitor_height / try_height_p as f64 * 2.0 / 3.0).round();

//  info!("monitor_size={:?}",monitor_size);
//  info!("hidpi_factor={:?}",hidpi_factor);
//  info!("hori_scale={:?}", hori_scale);
//  info!("vert_scale={:?}", vert_scale);

    // calculate sizes and position of the new window:
//  let try_cur_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new(  try_width_p as f64 * hori_scale                ,  try_height_p as f64 * vert_scale                ).to_logical(hidpi_factor);
//  let try_min_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new(((try_width_p as f64 * hori_scale) / 4.0) as f64, ((try_height_p as f64 * vert_scale) / 4.0) as f64 ).to_logical(hidpi_factor);
    let try_cur_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new( try_width_p as f64       , try_height_p as f64       ).to_logical(hidpi_factor);
    let try_min_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new( try_width_p as f64 / 4.0 , try_height_p as f64 / 4.0 ).to_logical(hidpi_factor);

 // let center = LogicalPosition::new( (monitor_width  - try_width_p  as f64 * scale) / 2.0,    (monitor_height - try_height_p as f64 * scale) / 2.0 );

    // apply the calculated values and display the window:
    window.set_inner_size(         try_cur_size);
    window.set_min_inner_size(Some(try_min_size));
    window.set_outer_position(LogicalPosition::new(try_pos_x, try_pos_y));
    window.set_visible(true);

    // return:
    window
}


impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self 
    {
    Self {
         box_pos_x:    32,
         box_pos_y:    32,
         box_width:    32,
         box_height:   32,
         box_speed_x:   4,
         box_speed_y:   4,
         win_width:   320,
         win_height:  200,
         }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) 
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

    /// Resize the `World`.
    fn resize(&mut self, win_width_p: u32, win_height_p: u32 ) 
    {
        self.win_width  = win_width_p;
        self.win_height = win_height_p;
    }

    /// Draw the World's state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn  draw(&self, frame: &mut [u8]) 
        {
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
            let alpha = 0xff as u8;
    
            // debug!("x={}, y={} --> red={}, green={}, blue={}", x, y, red, green, blue);

            let  rgba = if inside_the_box { [green , blue    , red    , 0x04 ] } 
            else                          { [red   , green   , blue   , alpha] };

            pixel.copy_from_slice(&rgba);
            }
        }
}




//  pub(crate) fn render(rx_p: Receiver<()>, window_p: Window) -> Result<(), String> 
//  {
//  let cursor_path: &Path  = Path::new("assets/cursors/pointers_part_5/glove3.png");
//  let image_path:  String =           "assets/graphics/2D/tiles/DungeonCrawlStoneSoupFull/dungeon/floor/sandstone_floor_0.png".to_string();
//  let font_path:   String =           "assets/fonts/NugieRomanticItalic-8P6D.ttf".to_string();
//  
//  let mut tick  = 0;
//  
//  // initialize values for FPS-counting:
//  #[allow(unused_assignments)]
//  let mut start_ticks:             u32 = 0; 
//  #[allow(unused_assignments)]
//  let mut start_perfcounter:       u64 = 0; 
//  #[allow(unused_assignments)]     
//  let mut delta_ticks:             u32 = 0;
//  #[allow(unused_assignments)]     
//  let mut delta_perfcounter:       u64 = 0;
//  let mut current_fps_ticks:       u32 = 0;
//  let mut current_fps_perfcounter: f64 = 0.0;
//  
//  let delay_in_loop: bool = true;
//  let rect_size: i32   = 32;          // 32 is the tile-size, this also used as scaling-factor
//  
//  //let mut win              = canvas.window_mut();
//  ////#[allow(unused_assignments)]
//  ////let mut win_position     = win.position();
//  //#[allow(unused_assignments)]
//  //let mut win_size         = win.size();
//  //let mut win_title        = format!("Shardoverse - scale: {:>3}, FPS: {:>4}, tick: {:>5}", rect_size, current_fps_ticks, tick);
//  //win.set_title(&win_title).map_err(|e| e.to_string())?;
//  
//  
//  // FPS-Counting at beginning of loop:
//  //#[allow(unused_assignments)]
//  //start_ticks       = timer.ticks(); 
//  //start_perfcounter = timer.performance_counter(); 
//  
//  let mut cnt = 0;
//  'render_loop: loop 
//      {
//      match rx_p.try_recv() {
//          Ok(_) | Err(TryRecvError::Disconnected) => {
//              println!("Terminating.");
//              break 'render_loop;
//          }
//          Err(TryRecvError::Empty) => {}
//      }
//      
//      // actual FPS calculation inside loop:
//  //    let current_ticks = timer.ticks();
//  //    delta_ticks = current_ticks - start_ticks;
//  //    start_ticks = current_ticks;
//  //    if delta_ticks != 0 {current_fps_ticks = 1000 / delta_ticks;}  
//  //
//  //    let current_perfcounter = timer.performance_counter();
//  //    delta_perfcounter = current_perfcounter - start_perfcounter;
//  //    start_perfcounter = current_perfcounter;
//  //    if delta_perfcounter != 0 {current_fps_perfcounter = 10_000_000.0 / delta_perfcounter as f64;}  
//  //
//  //    tick += 1;
//  
//      // Update the window title:
//    //  win_title    = format!("Shardoverse - scale: {:>3}, Ticks-FPS: {:>-5}, Perf-FPS: {:>-8.2}, tick: {:>5}", rect_size, current_fps_ticks, current_fps_perfcounter, tick);
//    //  win          = canvas.window_mut();
//    //  win.set_title(&win_title).map_err(|e| e.to_string())?;
//    //  win_size     = win.size(); 
//  
//      //draw everything:
//  //    for col in (0 ..= win_size.1).step_by(rect_size as usize)
//  //        {
//  //        for row in (0 ..= win_size.0).step_by(rect_size as usize)
//  //            {// todo: replace scale-on-copy with pre-scale and blit
//  //            canvas.copy(&ground_texture, rect!(   0,   0, rect_size, rect_size), rect!(row, col, rect_size, rect_size))?;
//  //            }
//  //        }
//  //
//  //    canvas.copy(&font_texture, None , rect!( 20,  20, 464,  64))?;
//  //    canvas.present();  // display new content of the window
//  //    
//  //    if delay_in_loop {std::thread::sleep(Duration::from_millis(100));}
//  //
//  //    cnt += 1;
//  //    if cnt >= 100 {break 'render_loop; }  // TEST exit
//  //    } // end of render_loop
//  
//  Ok(())
//  } // end of: render()





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

