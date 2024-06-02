use chrono::prelude::*;
use std::sync::mpsc::{Sender, channel, Receiver};
use color_eyre::eyre::{eyre, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::{interpolate::linear::Linear, signal, Signal};
use std::sync::Arc;
use std::sync::Mutex;

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

    pub fn record_with_preview() -> Result<AudioClip> {
        // TODO: in the future, we could configure input devices
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

        let clip = AudioClip {
            id: None,
            date: Utc::now(),
            samples: Vec::new(),
            sample_rate: input_config.sample_rate().0,
        };

        let clip = Arc::new(Mutex::new(Some(clip)));
        let clip_two = clip.clone();

        println!("Starting the recording with preview...");

        let (sender, receiver) = channel::<Vec<f32>>();
        let err_fn = move |err: cpal::StreamError| {
            eprintln!("An error occurred on stream: {}", err);
        };

        let input_channels = input_config.channels();

        fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle, sender: &Sender<Vec<f32>>)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some(clip) = guard.as_mut() {
                    let mut samples = Vec::with_capacity(input.len() / channels as usize);
                    for frame in input.chunks(channels.into()) {
                        let sample = frame[0].to_f32();
                        clip.samples.push(sample);
                        samples.push(sample);
                    }
                    sender.send(samples).unwrap();
                }
            }
        }

        let input_stream = match input_config.sample_format() {
            cpal::SampleFormat::F32 => input_device.build_input_stream(
                &input_config.into(),
                move |data, _: &_| write_input_data::<f32>(data, input_channels, &clip_two, &sender),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => input_device.build_input_stream(
                &input_config.into(),
                move |data, _: &_| write_input_data::<i16>(data, input_channels, &clip_two, &sender),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => input_device.build_input_stream(
                &input_config.into(),
                move |data, _: &_| write_input_data::<u16>(data, input_channels, &clip_two, &sender),
                err_fn,
            )?,
        };

        let output_channels = output_config.channels();
        let err_fn = move |err: cpal::StreamError| {
            eprintln!("An error occurred on stream: {}", err);
        };

        fn write_output_data<T>(output: &mut [T], channels: u16, receiver: &Receiver<Vec<f32>>)
        where
            T: cpal::Sample,
        {
            if let Ok(samples) = receiver.try_recv() {
                for (i, frame) in output.chunks_mut(channels.into()).enumerate() {
                    for sample in frame.iter_mut() {
                        *sample = cpal::Sample::from::<f32>(&samples[i % samples.len()]);
                    }
                }
            }
        }

        let output_stream = match output_config.sample_format() {
            cpal::SampleFormat::F32 => output_device.build_output_stream(
                &output_config.into(),
                move |data, _: &_| write_output_data::<f32>(data, output_channels, &receiver),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => output_device.build_output_stream(
                &output_config.into(),
                move |data, _: &_| write_output_data::<i16>(data, output_channels, &receiver),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => output_device.build_output_stream(
                &output_config.into(),
                move |data, _: &_| write_output_data::<u16>(data, output_channels, &receiver),
                err_fn,
            )?,
        };

        input_stream.play()?;
        output_stream.play()?;

        std::thread::sleep(std::time::Duration::from_secs(10));
        
        drop(input_stream);
        drop(output_stream);

        let clip = clip.lock().unwrap().take().unwrap();

        eprintln!("Recorded {} samples", clip.samples.len());

        Ok(clip)
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
