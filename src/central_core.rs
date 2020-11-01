#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(clippy::suspicious_else_formatting)]
#![allow(clippy::collapsible_if)]

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


//use std::thread;
//use std::time;
//use std::time::{Duration, Instant};
use std::path::Path;

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use bevy::{
    prelude::*,
//    render::{
//        camera::{ActiveCameras, Camera},
//        pass::*,
//        render_graph::{
//            base::MainPass, CameraNode, PassNode, RenderGraph, WindowSwapChainNode,
//            WindowTextureNode,
//        },
//        texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
//    },
    window::{CreateWindow, WindowDescriptor, WindowId},
};

//use crate::modules::*;                  // crate::<dirname>::<filename>
use crate::modules::config::*;            // crate::<dirname>::<filename>::*
use crate::modules::config::ShardConfig;  // crate::<dirname>::<filename>::<modulename>
use crate::modules::asset::*;             // crate::<dirname>::<filename>::<modulename>
//use crate::modules::pixel_painter::*;     // crate::<dirname>::<filename>

// use winit::
// {
//     window::{WindowBuilder, Fullscreen, Icon},
// //  dpi::{LogicalSize, PhysicalPosition},
//     dpi::{LogicalPosition, PhysicalSize},
//     event::{ElementState, Event, StartCause, KeyboardInput, VirtualKeyCode, ModifiersState, WindowEvent},
// //  event::{DeviceEvent},
//     event_loop::{ControlFlow, EventLoop},
//     platform::desktop::EventLoopExtDesktop,
// };
//extern crate image;

// use pixels::{Pixels, SurfaceTexture};


//___ CONSTANTS: ______________________________________________________________________________________________________________
//___ none ___


//___ TYPES: __________________________________________________________________________________________________________________
//___ none ___

struct GreetTimer(Timer);

//___ ENUMS: __________________________________________________________________________________________________________________
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Mode {
//     Wait,
//     WaitUntil,
//     Poll,
// }

//___ MACROS: _________________________________________________________________________________________________________________
//___ none ___

//___ STRUCTS: ________________________________________________________________________________________________________________
//___ none ___

// Components;
    struct Person;
    struct Name(String);
pub struct HelloPlugin;


//___ METHODS: ________________________________________________________________________________________________________________
//___ none ___
// a test plugin for Bevy:
// impl Plugin for HelloPlugin 
// {
// fn build(&self, app: &mut AppBuilder) 
//     {
//     // the reason we call from_seconds with the true flag is to make the timer repeat itself
//     app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//         .add_startup_system(add_people.system())
//         .add_system(greet_people.system());
//     }
// }


// Systems:
fn add_people(mut commands: Commands) 
{
commands
    .spawn((Person, Name("Elaina Proctor".to_string())))  // Entities?
    .spawn((Person, Name("Renzo Hume".to_string())))      // Entities?
    .spawn((Person, Name("Zayna Nieves".to_string())));   // Entities?
}


fn greet_people( time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) 
{
timer.0.tick(time.delta_seconds);
if timer.0.finished 
    {
    for (_person, name) in &mut query.iter() 
        {
        println!("hello {}!", name.0);
        }
    }
}


struct GlobalShardConfig(ShardConfig);

fn dump_config( shard_config_p: Res<ShardConfig> ) 
{
debug!("Values in shardoverse config:");
debug!("verbosity: {}", shard_config_p.verbosity );
debug!("debug    : {}", shard_config_p.debug     );
debug!("test     : {}", shard_config_p.test      );
debug!("training : {}", shard_config_p.training  );
debug!("language : {}", shard_config_p.language  );
debug!("delay    : {}", shard_config_p.delay     );
debug!("scale    : {}", shard_config_p.scale     );
debug!("encoding : {}", shard_config_p.encoding  );

for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("conf_name = {:?}", win_conf.conf_name);
    debug!("{:9} = {}"  ,CONF_WIN_TITLE  , win_conf.title    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_X  , win_conf.pos_x    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_Y  , win_conf.pos_y    );
    debug!("{:9} = {}"  ,CONF_WIN_WIDTH  , win_conf.width    );
    debug!("{:9} = {}"  ,CONF_WIN_HEIGHT , win_conf.height   );
    debug!("{:9} = {}\n",CONF_WIN_VISIBLE, win_conf.visible  );
    } // end of: for
}



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



/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows_p: ResMut<Windows>) 
{
let window = windows_p.get_primary_mut().unwrap();
window.set_title(format!( "Seconds since startup: {}",  time.seconds_since_startup.round() ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows_p: ResMut<Windows>) 
{
let window = windows_p.get_primary_mut().unwrap();
if input.just_pressed(KeyCode::Space) 
    {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
    }
}


/// ___________________________________________________________________________________________________________________________
/// **`FUNCTION:   `**  run_central_core   
/// **`TYPE:       `**  common function, only called once from main.rs!   
/// ___________________________________________________________________________________________________________________________
/// **`PARAMETER:  `** **` shard_config_p `** - struct containing the whole configuration, as loaded from config ini-file   
/// **`RETURNS:    `** **`     Result --> `** - OK(WindowConfig)   
/// **`            `** **`         or --> `** - Error(Error-Message)   
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
info!("Usage:");
info!("     'Esc' - Quit");
info!("      '1'  - switch to Wait mode.");
info!("      '2'  - switch to WaitUntil mode.");
info!("      '3'  - switch to Poll mode.");
info!("      'R'  - toggle request_redraw() calls.");
info!("      'F'  - Toggle borderless fullscreen (on current desktop screen)");
info!("      'H'  - Toggle Help-window");
info!("      'J'  - Toggle Journal-window");
info!("      'M'  - Toggle Maps-window");
info!(" Ctrl-'M'  - Toggle minimized the active window");
info!(" Ctrl-'X'  - Toggle maximized");
info!("      'V'  - Toggle visibility");
info!(" ctrl-'+'  - decrease speed");
info!(" ctrl-'-'  - increase speed");
info!("shift-'+'  - decrease scale");
info!("shift-'-'  - increase scale");

debug!("Values in shardoverse config:");
debug!("verbosity: {}", shard_config_p.verbosity );
debug!("debug    : {}", shard_config_p.debug     );
debug!("test     : {}", shard_config_p.test      );
debug!("training : {}", shard_config_p.training  );
debug!("language : {}", shard_config_p.language  );
debug!("delay    : {}", shard_config_p.delay     );
debug!("scale    : {}", shard_config_p.scale     );
debug!("encoding : {}", shard_config_p.encoding  );

for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("conf_name = {:?}", win_conf.conf_name);
    debug!("{:9} = {}"  ,CONF_WIN_TITLE  , win_conf.title    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_X  , win_conf.pos_x    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_Y  , win_conf.pos_y    );
    debug!("{:9} = {}"  ,CONF_WIN_WIDTH  , win_conf.width    );
    debug!("{:9} = {}"  ,CONF_WIN_HEIGHT , win_conf.height   );
    debug!("{:9} = {}\n",CONF_WIN_VISIBLE, win_conf.visible  );
    } // end of: for

// let cursor_path: &Path  = Path::new("assets/cursors/pointers_part_5/glove3.png");
let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/gui/icons/shardoverse_icon_01.png");

let  game_icon = load_icon(Path::new(path));
let  maps_icon = load_icon(Path::new(path));
let  invt_icon = load_icon(Path::new(path));
let  jrnl_icon = load_icon(Path::new(path));
let  help_icon = load_icon(Path::new(path));
let  pixl_icon = load_icon(Path::new(path));

let mut scale_in_pixels = shard_config_p.scale;

let  game_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_GAME.to_string()).unwrap(); // this better shall panic if not successful!
let  maps_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_MAPS.to_string()).unwrap(); // this better shall panic if not successful!
let  invt_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_INVT.to_string()).unwrap(); // this better shall panic if not successful!
let  jrnl_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_JRNL.to_string()).unwrap(); // this better shall panic if not successful!
let  help_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_HELP.to_string()).unwrap(); // this better shall panic if not successful!
let  pixl_index: usize = shard_config_p.find_window_conf_index_by_conf_name(CONF_WINDOW_PIXL.to_string()).unwrap(); // this better shall panic if not successful!

// -- Initialization of winit: ----------------------------------------
// let mut event_loop = EventLoop::new();   // needed to create the windows

// let     game_window = create_window(shard_config_p.window_configs[game_index].title.as_str(), game_icon, &event_loop, shard_config_p.window_configs[game_index].width, shard_config_p.window_configs[game_index].height, shard_config_p.window_configs[game_index].pos_x, shard_config_p.window_configs[game_index].pos_y, shard_config_p.window_configs[game_index].visible );
// let     maps_window = create_window(shard_config_p.window_configs[maps_index].title.as_str(), maps_icon, &event_loop, shard_config_p.window_configs[maps_index].width, shard_config_p.window_configs[maps_index].height, shard_config_p.window_configs[maps_index].pos_x, shard_config_p.window_configs[maps_index].pos_y, shard_config_p.window_configs[maps_index].visible );
// let     invt_window = create_window(shard_config_p.window_configs[invt_index].title.as_str(), invt_icon, &event_loop, shard_config_p.window_configs[invt_index].width, shard_config_p.window_configs[invt_index].height, shard_config_p.window_configs[invt_index].pos_x, shard_config_p.window_configs[invt_index].pos_y, shard_config_p.window_configs[invt_index].visible );
// let     jrnl_window = create_window(shard_config_p.window_configs[jrnl_index].title.as_str(), jrnl_icon, &event_loop, shard_config_p.window_configs[jrnl_index].width, shard_config_p.window_configs[jrnl_index].height, shard_config_p.window_configs[jrnl_index].pos_x, shard_config_p.window_configs[jrnl_index].pos_y, shard_config_p.window_configs[jrnl_index].visible );
// let     help_window = create_window(shard_config_p.window_configs[help_index].title.as_str(), help_icon, &event_loop, shard_config_p.window_configs[help_index].width, shard_config_p.window_configs[help_index].height, shard_config_p.window_configs[help_index].pos_x, shard_config_p.window_configs[help_index].pos_y, shard_config_p.window_configs[help_index].visible );
// let     pixl_window = create_window(shard_config_p.window_configs[pixl_index].title.as_str(), pixl_icon, &event_loop, shard_config_p.window_configs[pixl_index].width, shard_config_p.window_configs[pixl_index].height, shard_config_p.window_configs[pixl_index].pos_x, shard_config_p.window_configs[pixl_index].pos_y, shard_config_p.window_configs[pixl_index].visible );
// 
// let     game_window_id = game_window.id();  // these ids are used in the event-loop
// let     maps_window_id = maps_window.id();  // these ids are used in the event-loop
// let     invt_window_id = invt_window.id();  // these ids are used in the event-loop
// let     jrnl_window_id = jrnl_window.id();  // these ids are used in the event-loop
// let     help_window_id = help_window.id();  // these ids are used in the event-loop
// let     pixl_window_id = pixl_window.id();  // these ids are used in the event-loop


// prepare the Pixel-Painter window:
// let     pixl_window_size     = pixl_window.inner_size();
// let     pixl_surface_texture = SurfaceTexture::new(pixl_window_size.width, pixl_window_size.height, &pixl_window);
// let mut pixels               = Pixels::new(        pixl_window_size.width, pixl_window_size.height, pixl_surface_texture).unwrap();
// let mut pixl_world           = PixelWorld::new();


// event-loop:
// let mut delay_in_milis:  u64 = shard_config_p.delay.into();
// let mut wait_time:       time::Duration = time::Duration::from_millis(delay_in_milis);
// let mut poll_sleep_time: time::Duration = time::Duration::from_millis(delay_in_milis);

// window:
let mut game_window_visible   = shard_config_p.window_configs[game_index].visible;
let mut maps_window_visible   = shard_config_p.window_configs[maps_index].visible;
let mut invt_window_visible   = shard_config_p.window_configs[invt_index].visible;
let mut jrnl_window_visible   = shard_config_p.window_configs[jrnl_index].visible;
let mut help_window_visible   = shard_config_p.window_configs[help_index].visible;
let mut pixl_window_visible   = shard_config_p.window_configs[pixl_index].visible;

let mut game_window_minimized = false;
let mut maps_window_minimized = false;
let mut invt_window_minimized = false;
let mut jrnl_window_minimized = false;
let mut help_window_minimized = false;
let mut pixl_window_minimized = false;

let mut game_window_maximized = false;
let mut maps_window_maximized = false;
let mut invt_window_maximized = false;
let mut jrnl_window_maximized = false;
let mut help_window_maximized = false;
let mut pixl_window_maximized = false;

// control-flow:
//let mut mode                   = Mode::WaitUntil;
//let mut request_redraw:  bool  = true;
//let mut wait_cancelled:  bool  = false;
//let mut close_requested: bool  = false;
//let mut delay_in_loop:   bool  = true;


// initialize values for FPS-counting:
// let mut start     = Instant::now();
// let mut delta     = Duration::new(0,0);  // just to avoid error[E0381]: borrow of possibly-uninitialized variable: `delta`
// let mut tick: u32 = 0;
// let mut fps:  f64 = 0.0;
// 
// let mut modifiers_state: ModifiersState = ModifiersState::default();


//-----------------------------------------------------
// Bevy:
App::build()
    .add_default_plugins()
    .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
    .add_resource(GlobalShardConfig(shard_config_p.clone()))
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[game_index].title.clone(), width: shard_config_p.window_configs[game_index].width, height: shard_config_p.window_configs[game_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[maps_index].title.clone(), width: shard_config_p.window_configs[maps_index].width, height: shard_config_p.window_configs[maps_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[invt_index].title.clone(), width: shard_config_p.window_configs[invt_index].width, height: shard_config_p.window_configs[invt_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[jrnl_index].title.clone(), width: shard_config_p.window_configs[jrnl_index].width, height: shard_config_p.window_configs[jrnl_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[help_index].title.clone(), width: shard_config_p.window_configs[help_index].width, height: shard_config_p.window_configs[help_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(WindowDescriptor { title: shard_config_p.window_configs[pixl_index].title.clone(), width: shard_config_p.window_configs[pixl_index].width, height: shard_config_p.window_configs[pixl_index].height, vsync: false, resizable: true,  ..Default::default() })
    .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
    .add_startup_system(setup.system())
    .add_startup_system(add_people.system())
    .add_system(change_title.system())
    .add_system(toggle_cursor.system())
    .add_system(greet_people.system())
//  .add_plugin(HelloPlugin)
    .run();

debug!("After App::build()");   // will never be reached...
//-----------------------------------------------------

// event_loop.run_return(|event, _, control_flow| 
//     {
//     trace!("{:?}", event);
//         
//     *control_flow = ControlFlow::WaitUntil(time::Instant::now() + wait_time);
// 
//     // actual FPS calculation inside loop:
//     delta = start.elapsed();
//     if  delta > Duration::new(1,0)  // if delta is > 1 second
//         {
//         fps = (tick as f64/delta.as_millis() as f64) * 1_000.0 as f64;
//         debug!("delay_in_milis={}, start={:?}, delta={:?}, tick={}, fps={}", delay_in_milis, start, delta.as_secs(), tick, fps);
//         tick=0;                  // re-init for counting the loops in the next second
//         start = Instant::now();  // re-init for counting the loops in the next second
//         // or better (c++ code, averaging some time samples):
//         // static const int NUM_FPS_SAMPLES = 64;float fpsSamples[NUM_FPS_SAMPLES]int currentSample = 0;float CalcFPS(int dt){    fpsSamples[currentSample % NUM_FPS_SAMPLES] = 1.0f / dt;    float fps = 0;    for (int i = 0; i < NUM_FPS_SAMPLES; i++)        fps += fpsSamples;    fps /= NUM_FPS_SAMPLES;    return fps;}
//         }
// 
//     trace!("delay_in_milis={}, start={:?}, delta={:?}, tick={}, fps={}", delay_in_milis, start, delta.as_secs(), tick, fps);
//     tick += 1;
// 
//     // Update the window title:
//     game_window.set_title(&format!("Shardoverse - scale: {:>3}, tick: {:>5}, delay: {:>3}, FPS: {:>-8.2}", scale_in_pixels, tick, delay_in_milis, fps));
// 
//     match event 
//         {
//         Event::NewEvents(start_cause)      => {
//                                               wait_cancelled = match start_cause {
//                                                                                  StartCause::WaitCancelled { .. } => mode == Mode::WaitUntil,
//                                                                                                                 _ => false,
//                                                                                  }
//                                               }
//         Event::WindowEvent { event, window_id } => match event {
//                                                          WindowEvent::ModifiersChanged(m) => { // Key-Modifier states: (ctrl, alt, shift, logo)
//                                                                                              modifiers_state = m;
//                                                                                              debug!("ModifiersChanged to state: {:?}",modifiers_state);
//                                                                                              }
//                                                          WindowEvent::CloseRequested => {
//                                                                                         println!("Window {:?} has received the signal to close", window_id);
//                                                                                         // This drops the window, causing it to close.
//                                                                                         }
//                                                                                         
//                                                          WindowEvent::KeyboardInput  {input: KeyboardInput {virtual_keycode: Some(virtual_code), state: ElementState::Pressed, .. }, .. } 
//                                                                                      => match virtual_code {
//                                                                                               VirtualKeyCode::Escape   => {
//                                                                                                                           debug!("close_requested");
//                                                                                                                           close_requested = true;
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Multiply => {
//                                                                                                                           if  modifiers_state.ctrl() 
//                                                                                                                               {
//                                                                                                                               debug!("toggle delay in loop to: {}",!delay_in_loop);
//                                                                                                                               delay_in_loop = !delay_in_loop; 
//                                                                                                                               }
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Subtract => { 
//                                                                                                                           if  modifiers_state.ctrl() // decrease speed
//                                                                                                                               {
//                                                                                                                               if delay_in_milis <= 990 {delay_in_milis += 10}; 
//                                                                                                                               debug!("raise delay in loop to: {}",delay_in_milis);
//                                                                                                                               wait_time       = time::Duration::from_millis(delay_in_milis);
//                                                                                                                               poll_sleep_time = time::Duration::from_millis(delay_in_milis);
//                                                                                                                               }
//                                                                                                                           else    
//                                                                                                                           if  modifiers_state.shift() // increase scale
//                                                                                                                               {
//                                                                                                                               if scale_in_pixels > 10 {scale_in_pixels -= 1};
//                                                                                                                               }
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Add      => {
//                                                                                                                           if  modifiers_state.ctrl() // increase speed 
//                                                                                                                               {
//                                                                                                                               if delay_in_milis >= 10 {delay_in_milis -= 10}; 
//                                                                                                                               debug!("lower delay in loop to: {}",delay_in_milis);
//                                                                                                                               wait_time       = time::Duration::from_millis(delay_in_milis);
//                                                                                                                               poll_sleep_time = time::Duration::from_millis(delay_in_milis);
//                                                                                                                               }
//                                                                                                                           else
//                                                                                                                           if  modifiers_state.shift() // increase scale
//                                                                                                                               {
//                                                                                                                               if scale_in_pixels < 250 {scale_in_pixels += 1};
//                                                                                                                               }
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Key1     => {
//                                                                                                                           debug!("event_loop-mode: {:?}", mode);
//                                                                                                                           mode = Mode::Wait;
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Key2     => {
//                                                                                                                           debug!("event_loop-mode: {:?}", mode);
//                                                                                                                           mode = Mode::WaitUntil;
//                                                                                                                           }
//                                                                                               VirtualKeyCode::Key3     => {
//                                                                                                                           debug!("event_loop-mode: {:?}", mode);
//                                                                                                                           mode = Mode::Poll;
//                                                                                                                           }
//                                                                                               VirtualKeyCode::F        => {
//                                                                                                                           debug!("Toggle borderless fullscreen");
//                                                                                                                           if  game_window.fullscreen().is_some() 
//                                                                                                                               {
//                                                                                                                               game_window.set_fullscreen(None);
//                                                                                                                               } 
//                                                                                                                           else{
//                                                                                                                               let monitor = game_window.current_monitor();
//                                                                                                                               game_window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
//                                                                                                                               }
//                                                                                                                           }
// 
//                                                                                               VirtualKeyCode::H        => {
//                                                                                                                               debug!("toggle Help-Window visibility to: {}", !help_window_visible);
//                                                                                                                               help_window_visible   = !help_window_visible;
//                                                                                                                               help_window.set_visible( help_window_visible);
//                                                                                                                           }
// 
//                                                                                               VirtualKeyCode::G        => {
//                                                                                                                               debug!("toggle Game-Window visibility to: {}", !game_window_visible);
//                                                                                                                               game_window_visible   = !game_window_visible;
//                                                                                                                               game_window.set_visible( game_window_visible);
//                                                                                                                           }
//                                                                                                                           
//                                                                                               VirtualKeyCode::I        => {
//                                                                                                                               debug!("toggle Inventory-Window visibility to: {}", !invt_window_visible);
//                                                                                                                               invt_window_visible   = !invt_window_visible;
//                                                                                                                               invt_window.set_visible( invt_window_visible);
//                                                                                                                           }
//                                                                                                                           
//                                                                                               VirtualKeyCode::J        => {
//                                                                                                                               debug!("toggle Help-Window visibility to: {}", !jrnl_window_visible);
//                                                                                                                               jrnl_window_visible   = !jrnl_window_visible;
//                                                                                                                               jrnl_window.set_visible( jrnl_window_visible);
//                                                                                                                           }
//                                                                                                                           
//                                                                                               VirtualKeyCode::M        => {
//                                                                                                                           if  modifiers_state.ctrl() 
//                                                                                                                               {
//                                                                                                                               debug!("toggle minimized state of window-id: {:?}", window_id );
//                                                                                                                                    if window_id == game_window_id {game_window_minimized = !game_window_minimized; game_window.set_minimized(game_window_minimized);}
//                                                                                                                               else if window_id == maps_window_id {maps_window_minimized = !maps_window_minimized; maps_window.set_minimized(maps_window_minimized);}
//                                                                                                                               else if window_id == invt_window_id {invt_window_minimized = !invt_window_minimized; invt_window.set_minimized(invt_window_minimized);}
//                                                                                                                               else if window_id == jrnl_window_id {jrnl_window_minimized = !jrnl_window_minimized; jrnl_window.set_minimized(jrnl_window_minimized);}
//                                                                                                                               else if window_id == help_window_id {help_window_minimized = !help_window_minimized; help_window.set_minimized(help_window_minimized);}
//                                                                                                                               else if window_id == pixl_window_id {pixl_window_minimized = !pixl_window_minimized; pixl_window.set_minimized(pixl_window_minimized);}
//                                                                                                                               }
//                                                                                                                           else{    
//                                                                                                                               debug!("toggle Maps-Window visibility to: {}", !maps_window_visible);
//                                                                                                                               maps_window_visible = !maps_window_visible;
//                                                                                                                               maps_window.set_visible( maps_window_visible);
//                                                                                                                               }
//                                                                                                                           }
//                                                                                                                           
//                                                                                               VirtualKeyCode::P        => {
//                                                                                                                               debug!("toggle PixelPainter-Window visibility to: {}", !pixl_window_visible);
//                                                                                                                               pixl_window_visible   = !pixl_window_visible;
//                                                                                                                               pixl_window.set_visible( pixl_window_visible);
//                                                                                                                           }
//                                                                                               VirtualKeyCode::R        => {
//                                                                                                                           debug!("toggle request_redraw to {}", !request_redraw);
//                                                                                                                           request_redraw = !request_redraw;
//                                                                                                                           }
//                                                                                               VirtualKeyCode::X        => {
//                                                                                                                           if  modifiers_state.ctrl() 
//                                                                                                                               {
//                                                                                                                               debug!("toggle maximized state of window-id: {:?}", window_id );
//                                                                                                                                    if window_id == game_window_id {game_window_maximized = !game_window_maximized; game_window.set_maximized(game_window_maximized);}
//                                                                                                                               else if window_id == maps_window_id {maps_window_maximized = !maps_window_maximized; maps_window.set_maximized(maps_window_maximized);}
//                                                                                                                               else if window_id == invt_window_id {invt_window_maximized = !invt_window_maximized; invt_window.set_maximized(invt_window_maximized);}
//                                                                                                                               else if window_id == jrnl_window_id {jrnl_window_maximized = !jrnl_window_maximized; jrnl_window.set_maximized(jrnl_window_maximized);}
//                                                                                                                               else if window_id == help_window_id {help_window_maximized = !help_window_maximized; help_window.set_maximized(help_window_maximized);}
//                                                                                                                               else if window_id == pixl_window_id {pixl_window_maximized = !pixl_window_maximized; pixl_window.set_maximized(pixl_window_maximized);}
//                                                                                                                               }
//                                                                                                                           }
//                                                                                               _                        => {
//                                                                                                                           info!("unused virtual_code = '{:?}'",virtual_code);
//                                                                                                                           }
//                                                                                               },  // end of match virtual_code
//                                                          WindowEvent::Resized(new_size) => {
//                                                                                            debug!("Window resized to: {:?}",new_size);
//                                                                                            pixl_world.resize( new_size.width, new_size.height);
//                                                                                            pixels.resize(new_size.width, new_size.height);
//                                                                                            //pixl_window.request_redraw();  // ???
//                                                                                            }
//                                                                                      _  => {trace!("Window-event={:?}",event);},
//                                                          }, // end of: match WindowEvent(event)
//         Event::MainEventsCleared           => {
//                                               if  request_redraw && !wait_cancelled && !close_requested 
//                                                   {
//                                                   game_window.request_redraw();
//                                                   }
//                                               if  close_requested 
//                                                   {
//                                                   *control_flow = ControlFlow::Exit;   // Urgs, when using the winit::run() function, this does an immediate program exit...
//                                                   }
//                                               }
//         Event::RedrawRequested(window_id)  => {
//                                               info!("Redraw requested for window_id {:?}",window_id);
//                                               // drawing with Pixels will fail if the window is minimized.
//                                               //if  pixl_window.visible() // Grumbl, no such function available in winit?
//                                               //  {
//                                                   // Update internal state and request a redraw:
//                                                   pixl_world.update();
//                                                   let frame = pixels.get_frame();
//                                                       
//                                                   pixl_world.draw( frame );   // draws content into a frame-buffer
//                                                       
//                                                   if  pixels
//                                                       .render()
//                                                       .map_err(|e| error!("pixels.render() failed: {}", e))
//                                                       .is_err()
//                                                       {
//                                                       *control_flow = ControlFlow::Exit;
//                                                       return;
//                                                       }
//                                               //  }
// 
//                                               }
//         Event::RedrawEventsCleared         => {
//                                               *control_flow = match mode {
//                                                                          Mode::Wait      => ControlFlow::Wait,
//                                                                          Mode::WaitUntil => {
//                                                                                             if  wait_cancelled 
//                                                                                                 {
//                                                                                                 *control_flow
//                                                                                                 } 
//                                                                                             else{
//                                                                                                 ControlFlow::WaitUntil(time::Instant::now() + wait_time)
//                                                                                                 }
//                                                                                             }
//                                                                          Mode::Poll      => {
//                                                                                             thread::sleep(poll_sleep_time);
//                                                                                             ControlFlow::Poll
//                                                                                             }
//                                                                          };
//                                               }
//         _                                  => {
// //                                              info!("outer event={:?}",event);
//                                               },
//         };  // end of: match event
//     }  // end of: event loop
//     ); // end of: event clojure loop run[_return]()
// 
// 
// let game_window_pos  = game_window.outer_position().unwrap();
// let game_window_size = game_window.inner_size();             
// 
// let maps_window_pos  = maps_window.outer_position().unwrap();
// let maps_window_size = maps_window.inner_size();             
// 
// let invt_window_pos  = invt_window.outer_position().unwrap();
// let invt_window_size = invt_window.inner_size();             
// 
// let jrnl_window_pos  = jrnl_window.outer_position().unwrap();
// let jrnl_window_size = jrnl_window.inner_size();             
// 
// let help_window_pos  = help_window.outer_position().unwrap();
// let help_window_size = help_window.inner_size();             
// 
// let pixl_window_pos  = pixl_window.outer_position().unwrap();
// let pixl_window_size = pixl_window.inner_size();
// 
// // shard_config_p.verbosity = 2;                         // currently unchanged
// // shard_config_p.debug     = false;                     // currently unchanged
// // shard_config_p.test      = false;                     // currently unchanged
// // shard_config_p.training  = false;                     // currently unchanged
// // shard_config_p.language  = "Gremlinese".to_string();  // currently unchanged
// shard_config_p.delay     = delay_in_milis as u16;
// shard_config_p.scale     = scale_in_pixels;
// 
// shard_config_p.window_configs[game_index].pos_x   = game_window_pos.x;
// shard_config_p.window_configs[game_index].pos_y   = game_window_pos.y;
// shard_config_p.window_configs[game_index].width   = game_window_size.width; 
// shard_config_p.window_configs[game_index].height  = game_window_size.height;
// shard_config_p.window_configs[game_index].visible = game_window_visible;
// 
// shard_config_p.window_configs[maps_index].pos_x   = maps_window_pos.x;
// shard_config_p.window_configs[maps_index].pos_y   = maps_window_pos.y;
// shard_config_p.window_configs[maps_index].width   = maps_window_size.width; 
// shard_config_p.window_configs[maps_index].height  = maps_window_size.height;
// shard_config_p.window_configs[maps_index].visible = maps_window_visible;
//                                                   
// shard_config_p.window_configs[invt_index].pos_x   = invt_window_pos.x;
// shard_config_p.window_configs[invt_index].pos_y   = invt_window_pos.y;
// shard_config_p.window_configs[invt_index].width   = invt_window_size.width; 
// shard_config_p.window_configs[invt_index].height  = invt_window_size.height;
// shard_config_p.window_configs[invt_index].visible = invt_window_visible;
//                                                   
// shard_config_p.window_configs[jrnl_index].pos_x   = jrnl_window_pos.x;
// shard_config_p.window_configs[jrnl_index].pos_y   = jrnl_window_pos.y;
// shard_config_p.window_configs[jrnl_index].width   = jrnl_window_size.width; 
// shard_config_p.window_configs[jrnl_index].height  = jrnl_window_size.height;
// shard_config_p.window_configs[jrnl_index].visible = jrnl_window_visible;
//                                                   
// shard_config_p.window_configs[help_index].pos_x   = help_window_pos.x;
// shard_config_p.window_configs[help_index].pos_y   = help_window_pos.y;
// shard_config_p.window_configs[help_index].width   = help_window_size.width; 
// shard_config_p.window_configs[help_index].height  = help_window_size.height;
// shard_config_p.window_configs[help_index].visible = help_window_visible;
//                                                   
// shard_config_p.window_configs[pixl_index].pos_x   = pixl_window_pos.x;
// shard_config_p.window_configs[pixl_index].pos_y   = pixl_window_pos.y;
// shard_config_p.window_configs[pixl_index].width   = pixl_window_size.width; 
// shard_config_p.window_configs[pixl_index].height  = pixl_window_size.height;
// shard_config_p.window_configs[pixl_index].visible = pixl_window_visible;

for win_conf in shard_config_p.window_configs.iter() 
    {
    debug!("conf_name = {:?}", win_conf.conf_name);
    debug!("{:9} = {}"  ,CONF_WIN_TITLE  , win_conf.title    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_X  , win_conf.pos_x    );
    debug!("{:9} = {}"  ,CONF_WIN_POS_Y  , win_conf.pos_y    );
    debug!("{:9} = {}"  ,CONF_WIN_WIDTH  , win_conf.width    );
    debug!("{:9} = {}"  ,CONF_WIN_HEIGHT , win_conf.height   );
    debug!("{:9} = {}\n",CONF_WIN_VISIBLE, win_conf.visible  );
    } // end of: for

Ok(shard_config_p)
} // end of: run()



fn setup(
    mut commands: Commands,
    mut create_window_events: ResMut<Events<CreateWindow>>,
) {
    let mut window_id = WindowId::new();

    // sends out a "CreateWindow" event, which will be received by the windowing backend
    create_window_events.send(CreateWindow {
        id: window_id,
        descriptor: WindowDescriptor {
            width: 800,
            height: 600,
            vsync: false,
            title: "second window".to_string(),
            ..Default::default()
        },
    });

    window_id = WindowId::new();

    create_window_events.send(CreateWindow {
        id: window_id,
        descriptor: WindowDescriptor {
            width: 820,
            height: 620,
            vsync: false,
            title: "third window".to_string(),
            ..Default::default()
        },
    });

    window_id = WindowId::new();

    create_window_events.send(CreateWindow {
        id: window_id,
        descriptor: WindowDescriptor {
            width: 840,
            height: 640,
            vsync: false,
            title: "fourth window".to_string(),
            ..Default::default()
        },
    });

}


// Create a window for the game.
//
// Automatically scales the window to cover about 2/3 of the monitor height.
//
// # Returns
//
// Tuple of `(window, surface, width, height, pos_x, pos_y, hidpi_factor)`
// `width` and `height` are in `PhysicalSize` units.
//#[allow(clippy::too_many_arguments)]
//fn create_window( title_p: &str, icon_p: Icon, event_loop_p: &EventLoop<()>, try_width_p: u32, try_height_p: u32, try_pos_x_p: i32, try_pos_y_p: i32, visible_p: bool) -> winit::window::Window
//{
//    // Create a hidden window so we can derive a scaling-factor:
//    let window = WindowBuilder::new()
//                .with_title(title_p)
//                .with_window_icon(Some(icon_p))
//                .with_visible(false)
//                .build(&event_loop_p)
//                .unwrap();
//
//    window.set_outer_position(LogicalPosition::new(try_pos_x_p, try_pos_y_p));  // to get the correct current_monitor()
//
//    let hidpi_factor   = window.scale_factor();
//
//    // calculate dimensions which are fitting on the monitor:
//    //let monitor_size   = window.current_monitor().size();
//    //let monitor_width  = monitor_size.width  as f64 / hidpi_factor; 
//    //let monitor_height = monitor_size.height as f64 / hidpi_factor;
//    
//    //let hori_scale: f64 = (monitor_width  / try_width_p  as f64 * 2.0 / 3.0).round();
//    //let vert_scale: f64 = (monitor_height / try_height_p as f64 * 2.0 / 3.0).round();
//
////  info!("monitor_size={:?}",monitor_size);
////  info!("hidpi_factor={:?}",hidpi_factor);
////  info!("hori_scale={:?}", hori_scale);
////  info!("vert_scale={:?}", vert_scale);
//
//    // calculate sizes and position of the new window:
////  let try_cur_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new(  try_width_p as f64 * hori_scale                ,  try_height_p as f64 * vert_scale                ).to_logical(hidpi_factor);
////  let try_min_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new(((try_width_p as f64 * hori_scale) / 4.0) as f64, ((try_height_p as f64 * vert_scale) / 4.0) as f64 ).to_logical(hidpi_factor);
//    let try_cur_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new( try_width_p as f64       , try_height_p as f64       ).to_logical(hidpi_factor);
//    let try_min_size: winit::dpi::LogicalSize<f64> = PhysicalSize::new( try_width_p as f64 / 4.0 , try_height_p as f64 / 4.0 ).to_logical(hidpi_factor);
//
// // let center = LogicalPosition::new( (monitor_width  - try_width_p  as f64 * scale) / 2.0,    (monitor_height - try_height_p as f64 * scale) / 2.0 );
//
//    // apply the calculated values and display the window:
//    window.set_inner_size(         try_cur_size);
//    window.set_min_inner_size(Some(try_min_size));
//    window.set_outer_position(LogicalPosition::new(try_pos_x_p, try_pos_y_p));
//    window.set_visible(visible_p);
//
//    // return:
//    window
//}



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
// /// **`TESTMODULE: `** for central_core   
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

