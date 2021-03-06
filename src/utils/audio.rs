use sdl2;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub struct Audio {
    device: AudioDevice<SquareWave>
}

impl Audio{
    /// Audio driver using sdl2
    /// Mostly copied from this document:
    /// https://docs.rs/sdl2/0.12.1/sdl2/audio/index.html
    pub fn new(sdl2_context: &sdl2::Sdl) -> Self {
        let audio_subsystem = sdl2_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.05
            }
        }).unwrap();

        Audio { device }
    }

    pub fn start_audio(&mut self) {
        self.device.resume();
    }

    pub fn stop_audio(&mut self) {
        self.device.pause();
    }
}


struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0..= 0.5 =>self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

