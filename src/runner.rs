use input::Message;
use rodio;
use rodio::Sink;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;


/// Runs a thread that manages the audio file
pub fn start(rx_alarm_runner: mpsc::Receiver<Message>) -> JoinHandle<()> {
    let join_handle = thread::spawn(move || {

        let device = rodio::default_output_device().unwrap();
        let mut sink = Sink::new(&device);

        loop {
            match rx_alarm_runner.recv() {
                Ok(Message::AudioAndVolume(a_file_path, a_volume)) => {
                    let file = File::open(a_file_path).unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    sink = Sink::new(&device);
                    sink.append(source);
                    sink.set_volume(to_volume(a_volume));
                    sink.play(); // should already be ok
                },
                Ok(Message::StopAlarm) => {
                    sink.stop()
                },
                _ => {
                    sink.stop();
                    break;
                }
            };
        }
    });

    join_handle
}

/// Converts volume from an int [0,100] value to a float [0.0,1.0] representation
/// # Examples
/// ```
/// use rustine::runner::to_volume;
/// assert_eq!(to_volume(0), 0f32);
/// assert_eq!(to_volume(100), 1f32);
/// assert_eq!(to_volume(50), 0.5f32);
/// ```
pub fn to_volume(v: u32) -> f32 {
    let volume = cmp::min(cmp::max(0, v), 100) as f32;
    volume / 100f32
}
