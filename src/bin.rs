extern crate chrono;
extern crate notify;
extern crate rodio;
extern crate rustine;

use chrono::prelude::*;
use notify::{RecursiveMode, Watcher};
use rustine::config;
use rustine::config::AlarmConfig;
use rustine::input;
use rustine::input::Message;
use rustine::runner;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// This is the application entry point
/// Configuration file is located at ./rustine_config/config
fn main() {

    // timeout for channel message wait
    let channel_wait_timeout = Duration::from_secs(1);


    let path = String::from("./rustine_config");
    let path2 = path.clone();
    let config_file = path.clone() + "/config";

    // Create a channel to receive the events from the configuration update notifier.
    let (tx_config_update, rx_config_update) = mpsc::channel();
    // Create a channel to receive the events from the alarm manager.
    let (tx_alarm_manager, rx_alarm_manager) = mpsc::channel();
    // Create a channel to wait for user input.
    let (tx_keyboard_input, rx_keyboard_input) = mpsc::channel();
    // Create a channel to run alarm notification
    let (tx_alarm_runner, rx_alarm_runner) = mpsc::channel();


    // Configuration file update notification
    let debounce_timeout = Duration::from_secs(1);
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = notify::watcher(tx_config_update, debounce_timeout).unwrap();
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();


    // Thread that checks when an alarm should be launched
    // Launches/ stops the alarms
    let alarm_manager = thread::spawn(move || {

        let mut alarms: Vec<AlarmConfig> = vec![];

        let runner = runner::start(rx_alarm_runner);

        loop {

            match rx_alarm_manager.recv_timeout(channel_wait_timeout) {
                // most common received message
                Err(mpsc::RecvTimeoutError::Timeout) => {},
                // Update alarm configuration
                Ok(Message::Reconfigure(new_config)) => {
                    alarms = new_config;

                    println!("Configuration updated:");
                    for alarm in alarms.iter() {
                        alarm.pretty_print(&Local::now())
                    }
                },
                // Forward message to stop the running alarm
                Ok(Message::StopAlarm) => {
                    let _send_result = tx_alarm_runner.send(Message::StopAlarm);
                },
                // Stop the thread
                Ok(Message::Quit) => {
                    watcher.unwatch(path2).unwrap();
                    let _send_result = tx_alarm_runner.send(Message::Quit);
                    break;
                },
                Ok(_) => {},
                Err(e) => println!("Error in alarm manager channel: {:?}", e)
            };

            let current_time = chrono::Local::now();
            println!("{:#02}:{:#02}:{:#02}", current_time.hour(), current_time.minute(), current_time.second());

            let mut to_remove: Option<usize> = None;

            for (index, alarm_config) in alarms.iter().enumerate() {
                if alarm_config.is_active_day(&current_time) {
                    if alarm_config.is_expired(&current_time) {
                        if !alarm_config.already_run_today(&current_time) {
                            println!("starting alarm {}", current_time);
                            to_remove = Some(index);
                            let _send_result = tx_alarm_runner.send(Message::AudioAndVolume(alarm_config.audio_file.clone(), alarm_config.volume));
                        }
                    }
                }
            };

            if let Some(i) = to_remove {
                let cfg = alarms[i].clone();
                let updated_alarm = AlarmConfig {
                    last_run: current_time,
                    ..cfg
                };

                alarms[i] = updated_alarm;

            } else if current_time.minute() == 0 && current_time.second() == 0 {
                println!("[INFO]");
                for alarm in alarms.iter() {
                    alarm.pretty_print(&current_time)
                }
            }
        }

        runner.join().unwrap();
    });

    // read input
    let input_watcher = input::watch_input(tx_keyboard_input);

    let load_config = |config_file: &str|{
        let updated_config = config::retrieve_configuration(config_file);
        let _send_result = tx_alarm_manager.send(Message::Reconfigure(updated_config));
    };


    // loading the configuration without file system change is required at startup
    load_config(&config_file);

    loop {

        match rx_config_update.recv_timeout(channel_wait_timeout) {
            Ok(_event) => {
                println!("Reloading configuration");
                load_config(&config_file);
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {},
            Err(e) => println!("Error while watching configuration file: {:?}", e)
        };

        match rx_keyboard_input.recv_timeout(channel_wait_timeout) {
            Ok(Message::StopAlarm)=> {
                let _send_result = tx_alarm_manager.send(Message::StopAlarm);
            },
            Ok(Message::Quit) =>     {
                let _send_result = tx_alarm_manager.send(Message::Quit);
                break;
            },
            Err(mpsc::RecvTimeoutError::Timeout) | Ok(_) => {},
            Err(e) => println!("watch error: {:?}", e)
        };
    }

    input_watcher.join().unwrap();
    alarm_manager.join().unwrap();

    println!("Application stopped")
}
