// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod audio;

use audio::AudioClip;
use color_eyre::eyre::Result;
use std::sync::{Mutex, Arc};

pub struct App {
    bpm: i32,
    audio_clips: Mutex<Vec<Arc<AudioClip>>>
}

impl App {
    pub fn play_clips(&self) -> Result<()> {
        let audio_clips = self.audio_clips.lock().unwrap();
        let mut handles = vec![];
        for clip in audio_clips.iter() {
            let clip = Arc::clone(clip); // Create a clone of the Arc pointer
            let handle = std::thread::spawn(move || {
                clip.play().unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    pub fn add_clip(&self, clip: Arc<AudioClip>){
        let mut audio_clips = self.audio_clips.lock().unwrap();
        audio_clips.push(clip);
    }

    pub fn remove_clip(&self, index: usize) -> Option<Arc<AudioClip>> {
        let mut audio_clips = self.audio_clips.lock().unwrap();
        if index < audio_clips.len() {
            Some(audio_clips.remove(index))
        } else {
            None
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn play_clips(state: tauri::State<'_, Arc<Mutex<App>>>) -> Result<(), String>{
    let app = state.lock().map_err(|err| err.to_string())?;
    app.play_clips().map_err(|err| err.to_string())
}

#[tauri::command]
fn record_clip(state: tauri::State<'_, Arc<Mutex<App>>>) -> Result<(), String>{
    let clip = AudioClip::record().map_err(|err| err.to_string())?;
    let app = state.lock().map_err(|err| err.to_string())?;
    app.add_clip(Arc::new(clip));
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(App {
            bpm: 120,
            audio_clips: Mutex::new(vec![]),
        })))
        .invoke_handler(tauri::generate_handler![greet, play_clips, record_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
