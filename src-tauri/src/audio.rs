use chrono::prelude::*;
use std::sync::mpsc::{Sender, channel, Receiver};
use color_eyre::eyre::{eyre, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::{interpolate::linear::Linear, signal, Signal};
use std::sync::Arc;
use std::sync::Mutex;
use ringbuf::HeapRb;
use hound;

#[derive(Clone)]
pub struct AudioClip {
    pub id: Option<usize>,
    pub date: DateTime<Utc>,
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

type ClipHandle = Arc<Mutex<Option<AudioClip>>>;


impl AudioClip {
    pub fn resample(&self, sample_rate: u32) -> AudioClip {
        if self.sample_rate == sample_rate {
            return self.clone();
        }

        let mut signal = signal::from_iter(self.samples.iter().copied());
        let a = signal.next();
        let b = signal.next();

        let linear = Linear::new(a, b);

        AudioClip {
            id: self.id,
            date: self.date,
            samples: signal
                .from_hz_to_hz(linear, self.sample_rate as f64, sample_rate as f64)
                .take(self.samples.len() * (sample_rate as usize) / (self.sample_rate as usize))
                .collect(),
            sample_rate,
        }
    }

    pub fn downsample(&self, target_len: usize) -> Vec<f32> {
        let sample_count = self.samples.len();
        if sample_count <= target_len {
            return self.samples.clone();
        }

        let chunk_size = sample_count / target_len;
        let mut downsampled = Vec::with_capacity(target_len);

        for chunk in self.samples.chunks(chunk_size){
            let avg = chunk.iter().copied().sum::<f32>() / chunk.len() as f32;
            downsampled.push(avg);
        }

        downsampled
    }

    pub fn load_wav(file_path: &str) -> Result<AudioClip> {
        let reader = hound::WavReader::open(file_path)?;
        let spec = reader.spec();
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .filter_map(Result::ok)
            .map(|s| s as f32 / i16::MAX as f32)
            .collect();

        Ok(AudioClip {
            id: None,
            date: chrono::Utc::now(),
            samples,
            sample_rate: spec.sample_rate,
        })
    }

    pub fn stream_feedback() -> Result<()> {
        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .ok_or_else(|| eyre!("No default input device."))?;
        let output_device = host
            .default_output_device()
            .ok_or_else(|| eyre!("No default output device."))?;
        println!("Input device: {}", input_device.name()?);
        println!("Output device: {}", output_device.name()?);

        let input_config = input_device.default_input_config()?;
        let output_config = output_device.default_output_config()?;

        let latency = 30.0;

        // Create a delay in case the input and output devices aren't synced.
        let latency_frames = (latency / 1_000.0) * input_config.sample_rate().0 as f32;
        let latency_samples = latency_frames as usize * input_config.channels() as usize;

        let ring = HeapRb::<f32>::new(latency_samples * 2);

        let (mut producer, mut consumer) = ring.split();

        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
        };
    
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };
    
        let input_stream = input_device.build_input_stream(&input_config.into(), input_data_fn, err_fn)?;
        let output_stream = output_device.build_output_stream(&output_config.into(), output_data_fn, err_fn)?;

        println!("Successfully built streams.");
    
        // Play the streams.
        println!(
            "Starting the input and output streams with `{}` milliseconds of latency.",
            latency
        );
        
        input_stream.play()?;
        output_stream.play()?;
    
        // Run for 3 seconds before closing.
        println!("Playing for 3 seconds... ");

        loop {}
    }

        
    
    pub fn record() -> Result<AudioClip> {
        // TODO: in the future, we could configure input devices
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| eyre!("No default input device."))?;
        println!("Input device: {}", device.name()?);
        let config = device.default_input_config()?;

        let clip = AudioClip {
            id: None,
            date: Utc::now(),
            samples: Vec::new(),
            sample_rate: config.sample_rate().0,
        };

        let clip = Arc::new(Mutex::new(Some(clip)));
        let clip_two = clip.clone();

        println!("Starting the recording...");

        let err_fn = move |err: cpal::StreamError| {
            eprintln!("An error occured on stream: {}", err);
        };

        let channels = config.channels();

        fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some(clip) = guard.as_mut() {
                    for frame in input.chunks(channels.into()) {
                        clip.samples.push(frame[0].to_f32());
                    }
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32>(data, channels, &clip_two),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16>(data, channels, &clip_two),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<u16>(data, channels, &clip_two),
                err_fn,
            )?,
        };

        stream.play()?;

        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(stream);
        let clip = clip.lock().unwrap().take().unwrap();

        eprintln!("Recorded {} samples", clip.samples.len());

        Ok(clip)
    }
    
    pub fn play(&self) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| eyre!("No output device"))?;
        println!("Output device: {}", device.name()?);
        let config = device.default_output_config()?;

        println!("Begin playback...");

        type StateHandle = Arc<Mutex<Option<(usize, Vec<f32>, Sender<()>)>>>;
        let sample_rate = config.sample_rate().0;
        let (done_tx, done_rx) = channel::<()>();
        let state = (0, self.resample(sample_rate).samples, done_tx);
        let state = Arc::new(Mutex::new(Some(state)));
        let channels = config.channels();

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        fn write_output_data<T>(output: &mut [T], channels: u16, writer: &StateHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some((i, clip_samples, done)) = guard.as_mut() {
                    for frame in output.chunks_mut(channels.into()) {
                        for sample in frame.iter_mut() {
                            *sample = cpal::Sample::from(clip_samples.get(*i).unwrap_or(&0f32));
                        }
                        *i += 1;
                    }
                    if *i >= clip_samples.len() {
                        if let Err(_) = done.send(()) {
                            // Playback has already stopped. We'll be dead soon.
                        }
                    }
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<f32>(data, channels, &state),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<i16>(data, channels, &state),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<u16>(data, channels, &state),
                err_fn,
            )?,
        };

        stream.play()?;

        done_rx.recv()?;

        Ok(())
    }
}
