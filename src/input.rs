use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use config::AlarmConfig;

/// Messages that are sent in the channels
pub enum Message {
    /// Audio file path and output volume in percent [0, 100]
    AudioAndVolume(String, u32),
    /// New alarm configuration list
    Reconfigure(Vec<AlarmConfig>),
    /// Notify to stop the running alarm
    StopAlarm,
    /// Stop the application
    Quit
}

/// # Examples
/// ```
/// use rustine::input::Message;
/// assert_eq!(Message::StopAlarm.as_str(), "stop");
/// ```
impl Message {
    /// Stringify some messages to get the user input expectation
    pub fn as_str(&self) -> &str {
        match self {
            &Message::StopAlarm => "stop",
            &Message::Quit => "quit",
            _ => "other_message"
        }
    }
}

/// Thread to watch user keyboard inputs
pub fn watch_input(tx_keyboard_input: Sender<Message>) -> JoinHandle<()> {
    let join_handle = thread::spawn(move || {
        loop {

            use std::io::{stdin};
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("Did not enter a correct string");

            let forward = buffer.to_lowercase().trim().to_string();

            if forward != "" {
                if forward == Message::Quit.as_str() {
                    let _send_result = tx_keyboard_input.send(Message::Quit);
                    break;
                } else if forward == Message::StopAlarm.as_str() {
                    let _send_result = tx_keyboard_input.send(Message::StopAlarm);
                }
            }
        }
    });

    join_handle
}
