// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Module declarations
mod game_state;
mod commands;
mod factories;

use tauri::Manager;
use std::{sync::{mpsc, Mutex}, thread::{self}, time::{Duration, Instant}};

use game_state::{CalcutingThreadState, GameState, Point, ThreadCommands, ThreadState};

const ITERATION_MINIMUM_DURATION: Duration = Duration::new(0, 5 * (10u32).pow(4));

fn main() {
    let game_state = Mutex::new(GameState::default());
    let calculating_thread_state = Mutex::new(CalcutingThreadState::<(), ThreadCommands> {
        thread_state: Option::None
    });

    tauri::Builder::default().setup(|app| {
        app.manage(game_state);
        app.manage(calculating_thread_state);

        let handle = app.handle();
        app.listen_global("notify", move |event: tauri::Event| {
            let state = handle.state::<Mutex<GameState>>();
            let mut state = state.lock().unwrap();

            match event.payload() {
                Some(payload) => {
                    let point: Point = serde_json::from_str(&payload).unwrap();
                    state.add_or_remove_point(point);
                }
                None => println!("Fauly event")
            }
        });

        let handle = app.handle();
        app.listen_global("run", move |_event: tauri::Event| {

            let calculating_thread_state = handle.state::<Mutex<CalcutingThreadState<(), ThreadCommands>>>();
            let mut calculating_thread_state = calculating_thread_state.lock().unwrap();
            let (tx, rx) = mpsc::channel::<ThreadCommands>();
            
            let inner_handle = handle.app_handle();
            let thread_handler = thread::spawn(move || {
                loop {

                    match rx.try_recv() {
                        Ok(value) => {
                            match value {
                                ThreadCommands::Kill => {return}
                                _ => {} // Left here for future commands perhaps
                                }
                            }
                        Err(_) => {}
                    }

                    let calculating_thead_handle = inner_handle.app_handle();
                    //let (tx, rx) = mpsc::channel();

                    let start_time: Instant = Instant::now();
                    // thread::spawn(move || {
                    //     let state = calculating_thead_handle.state::<Mutex<GameState>>();
                    //     let mut state = state.lock().unwrap();
                    //     let changed_points = state.apply_rules();
                    //     tx.send(changed_points).unwrap(); // Currently panics if fails. Should be handled later.
                    // });

                    let state = calculating_thead_handle.state::<Mutex<GameState>>();
                    let mut state = state.lock().unwrap();
                    let changed_points = state.apply_rules();
                    //tx.send(changed_points).unwrap(); // Currently panics if fails. Should be handled later.


                    //let changed_points = rx.recv().unwrap(); // Blocking on purpose
                    let end_time = Instant::now();
                    
                    if ITERATION_MINIMUM_DURATION > end_time - start_time {
                        thread::sleep(ITERATION_MINIMUM_DURATION - (end_time - start_time));    
                    }
                    

                    inner_handle.emit_all("update", changed_points).unwrap(); // Currently panics if failes.
                }
            });

            calculating_thread_state.thread_state = Some(ThreadState {
                thread_handler,
                tx
            });
        });

        let handle = app.handle();
        app.listen_global("stop", move |_event: tauri::Event| {
            let thread_state = handle.state::<Mutex<CalcutingThreadState<(), ThreadCommands>>>();
            let mut thread_state = thread_state.lock().unwrap();

            match &thread_state.thread_state {
                Option::None => {}
                Option::Some(state) => {
                    state.tx.send(ThreadCommands::Kill).unwrap(); // currently panics if failes
                }
            }

            thread_state.thread_state = Option::None;
        });
        Ok(())  
    })
    .invoke_handler(tauri::generate_handler![commands::step, commands::clear, commands::select])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
