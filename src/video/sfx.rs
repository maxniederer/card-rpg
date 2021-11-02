use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::{self};
use sdl2::{Sdl,AudioSubsystem};
use sdl2::audio::{AudioCVT,AudioSpecDesired,AudioSpecWAV,AudioCallback,AudioDevice,AudioStatus};
use crate::video::sdl_core::SDLCore;
use std::time::Duration;
pub struct SoundData<'a>{
    pub bytes: Vec<u8>,
    pub volume: f32,
    pub position: usize,
    pub stopped: &'a mut bool,
}

impl AudioCallback for SoundData<'_>{
    //TODO Add audio system via sdl_core
    type Channel = u8;

    fn callback(&mut self,data: &mut [u8]){
        for x in data.iter_mut() {
            *x = match self.bytes.get(self.position) {
                Some(v) => { self.position += 1; *v},
                None => { *self.stopped=true; 0 as u8 }
            }
        }
    }
}

pub struct audio_subsystem{
    audio_subsystem: AudioSubsystem,
    end_music_arc: Arc<AtomicBool>,
}

impl audio_subsystem{
    pub fn init(audio_subsystem:AudioSubsystem,end_music_arc:Arc<AtomicBool>)->audio_subsystem{
        audio_subsystem{audio_subsystem,end_music_arc}
    }

    pub fn playsong(&self,choice:&str){
        let audio_spec = AudioSpecDesired{freq: Some(44100), channels: Some(2), samples: None};
        let mut stop = false;

        while true{
            stop = false;
            print!("looped\n");
        let audio_device = self.audio_subsystem.open_playback(None, &audio_spec, |spec| {
        let audio_wav = AudioSpecWAV::load_wav(choice.to_string()).expect("Wav Load failed.");

        let cvt = AudioCVT::new(
            audio_wav.format,
            audio_wav.channels,
            audio_wav.freq,
            spec.format,
            spec.channels,
            spec.freq,
        ).expect("CVT failed.");

        let data = cvt.convert(audio_wav.buffer().to_vec());

        // initialize the audio callback
        SoundData {
            bytes: data,
            volume: 1.0,
            position: 0,
            stopped:&mut stop,
        }
    }).unwrap();



        audio_device.resume();

        while !stop{
            std::thread::sleep(Duration::from_millis(100));
        }

    }
    }
}
