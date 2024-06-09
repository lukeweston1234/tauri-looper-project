// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use audio::AudioClip;
use color_eyre::eyre::Result;
use std::sync::{Mutex, Arc, mpsc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use tauri::{AppHandle};

pub struct App {
    bpm: i32,
    is_metronome_on: Arc<AtomicBool>,
    audio_clips: Mutex<Vec<Arc<AudioClip>>>,
    metronome_clip: Option<Arc<AudioClip>>, // Add this line
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

    pub fn set_metronome_clip(&mut self, clip: Arc<AudioClip>) {
        self.metronome_clip = Some(clip);
    }

    pub fn stop_metronome(&self) {
        self.is_metronome_on.store(false, Relaxed);
    }

    pub fn start_metronome(&self) {
        self.is_metronome_on.store(true, Relaxed);
    }

    pub fn start_clock(&self) -> Result<()> {
        let bpm = self.bpm;
        let is_metronome_on = self.is_metronome_on.clone();
        let metronome_clip = self.metronome_clip.clone();
        let (sender, receiver) = mpsc::channel();

        // Timer thread
        thread::spawn(move || {
            let interval = Duration::from_secs_f32(60.0 / bpm as f32);
            println!("{:?}", interval);
            loop {
                if is_metronome_on.load(Relaxed) {
                    if sender.send(()).is_err() {
                        break; // Exit the loop if the receiver is dropped
                    }
                }
                thread::sleep(interval);
            }
        });

        let is_metronome_on_for_playback = self.is_metronome_on.clone();

        // Playback thread
        thread::spawn(move || {
            while let Ok(_) = receiver.recv() {
                if let Some(clip) = &metronome_clip {
                    if is_metronome_on_for_playback.load(Relaxed) {
                        let clip_clone = clip.clone();
                        thread::spawn(move || {
                            clip_clone.play().unwrap();
                        });
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stream_feedback(&self) -> Result<()> {
        std::thread::spawn(move || {
            AudioClip::stream_feedback();
        });
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

#[tauri::command(async)]
fn record_clip(state: tauri::State<'_, Arc<Mutex<App>>>) -> Result<Vec<f32>, String>{
    let clip = AudioClip::record().map_err(|err| err.to_string())?;
    let app = state.lock().map_err(|err| err.to_string())?;
    let downsampled = clip.downsample(192);
    app.add_clip(Arc::new(clip));
    Ok(downsampled)
}

#[tauri::command]
fn start_metronome(state: tauri::State<'_, Arc<Mutex<App>>>) -> Result<(), String> {
    let app = state.lock().map_err(|err| err.to_string())?;
    app.start_metronome();
    Ok(())
}

#[tauri::command]
fn stop_metronome(state: tauri::State<'_, Arc<Mutex<App>>>) -> Result<(), String> {
    let app = state.lock().map_err(|err| err.to_string())?;
    app.stop_metronome();
    Ok(())
}

fn setup_metronome(handle: &AppHandle, app_state: &Arc<Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
    // Resolve the path to the metronome sound
    let resource_dir = handle.path_resolver().resource_dir().expect("Failed to resolve resource dir");
    let metronome_path = resource_dir.join("assets/metronome.wav");
    let metronome_clip = AudioClip::load_wav(metronome_path.to_str().unwrap()).unwrap();
    app_state.lock().unwrap().set_metronome_clip(Arc::new(metronome_clip));

    // Start the clock
    let app_state_clone = Arc::clone(app_state);
    std::thread::spawn(move || {
        app_state_clone.lock().unwrap().start_clock().unwrap();
    });

    Ok(())
}

fn main() {
    // Initialize the Tauri application and manage the app state
    let app_state = Arc::new(Mutex::new(App {
        bpm: 120,
        audio_clips: Mutex::new(vec![]),
        metronome_clip: None,
        is_metronome_on: Arc::new(AtomicBool::new(false)),
    }));

    // Start the Tauri application
    tauri::Builder::default()
        .manage(app_state.clone())
        .setup(move |app| {
            let handle = app.handle();
            setup_metronome(&handle, &app_state)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, record_clip, play_clips, start_metronome, stop_metronome])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}