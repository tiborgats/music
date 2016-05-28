
use portaudio as pa;
// use std::thread;
// use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use sound::*;

const FRAMES_PER_BUFFER: u32 = 4 * 256;    // optimal = FRAMES_PER_BUFFER_UNSPECIFIED

lazy_static! {
    static ref PORTAUDIO: pa::PortAudio = {
        let pa = pa::PortAudio::new().unwrap();
        println!("PortAudio is initialized.");
        pa
    };
}

/// this is a wrapper around the sound output backend
#[allow(dead_code)]
pub struct SoundInterface<'a, T: 'static> {
    sample_rate: u32,
    channel_count: u16,
    stream: pa::Stream<'a, pa::NonBlocking, pa::stream::Output<SampleOutput>>,
    // synthesizer: Arc<Mutex<Wave>>,
    sender: Option<Sender<T>>, // receiver: Option<Receiver<T>,
}

impl<'a, T> SoundInterface<'a, T> {
    /// Creates a new backend for sound playback.
    /// At the moment all channels output the same sound.
    pub fn new(sample_rate: u32,
               channel_count: u16,
               mut generator: Box<SoundGenerator<T>>)
               -> SoundResult<SoundInterface<'a, T>> {
        println!("PortAudio version : {}", pa::version());
        println!("PortAudio version text : {:?}", pa::version_text());
        // println!("host count: {}", PORTAUDIO.host_api_count()?);
        println!("host count: {}", try!(PORTAUDIO.host_api_count()));
        let mut settings = try!(PORTAUDIO.default_output_stream_settings(channel_count as i32,
                                                                         sample_rate as f64,
                                                                         FRAMES_PER_BUFFER));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        let mut generator_buffer: Vec<SampleCalc> = vec![0.0; BUFFER_SIZE];

        let (sender, receiver) = ::std::sync::mpsc::channel();
        // This routine will be called by the PortAudio engine when audio is needed. It may
        // called at interrupt level on some machines so don't do anything that could mess
        // up the system like dynamic resource allocation or IO.
        let callback_fn = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            if let Ok(command) = receiver.try_recv() {
                generator.process_command(command);
            }
            generator.get_samples(frames, &mut generator_buffer);
            let mut idx = 0;
            for item in generator_buffer.iter().take(frames) {
                for _ in 0..(channel_count as usize) {
                    buffer[idx] = *item as SampleOutput;
                    idx += 1;
                }
            }
            //            for output_frame in buffer.chunks_mut(channel_count) {
            //                for channel_sample in output_frame {
            //                    *channel_sample = synthesizer.sample_next();
            // sender.send(time_start).ok();
            pa::Continue
        };

        // Open a non-blocking stream.
        //        let mut stream = PORTAUDIO.open_non_blocking_stream(settings, callback_fn)?;
        let stream = try!(PORTAUDIO.open_non_blocking_stream(settings, callback_fn));
        println!("Stream is created.");

        Ok(SoundInterface {
            sample_rate: sample_rate,
            channel_count: channel_count,
            stream: stream,
            // synthesizer: Arc::new(Mutex::new(Wave::default())),
            sender: Some(sender),
        })
    }

    pub fn start(&mut self) -> SoundResult<()> {
        try!(self.stream.start());
        println!("Successfully started the stream.");
        Ok(())
    }

    pub fn send_command(&mut self, command: T) {
        if let Some(ref sender) = self.sender {
            sender.send(command).ok();
        }
    }
}

impl<'a, T> Drop for SoundInterface<'a, T> {
    fn drop(&mut self) {
        use std::error::Error;
        if self.stream.is_active() == Ok(true) {
            if let Err(err) = self.stream.stop() {
                println!("PortAudio.stream.stop: {}", err.description());
            }
        }
        if let Err(err) = self.stream.close() {
            println!("PortAudio.stream.close: {}", err.description());
        }
    }
}
