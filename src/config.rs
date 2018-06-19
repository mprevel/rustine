use chrono;
use chrono::DateTime;
use chrono::prelude::*;
use std::fs::OpenOptions;

/// Alarm time
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32
}

/// Alarm configuration
#[derive(Debug, Clone, PartialEq)]
pub struct AlarmConfig {
    pub days: Vec<bool>,
    pub time: Time,
    pub audio_file: String,
    pub volume: u32,
    pub last_run: DateTime<Local>
}

impl AlarmConfig {

    /// Checks if the alarm has been launched today or before the app started
    pub fn already_run_today(&self, now: &DateTime<Local>) -> bool {
        self.last_run.day() == now.day() && {
            hms_gte(
                (self.last_run.hour(), self.last_run.minute(), self.last_run.second()),
                (self.time.hours, self.time.minutes, self.time.seconds)
            )
        }
    }

    /// Checks if the current time is >= to the time set for the alarm to run
    pub fn is_expired(&self, now: &DateTime<Local>) -> bool {
        hms_gte(
            (now.hour(), now.minute(), now.second()),
            (self.time.hours, self.time.minutes, self.time.seconds)
        )
    }

    /// Checks if the current day is configured to run the alarm
    pub fn is_active_day(&self, now: &DateTime<Local>) -> bool {
        let day_index = (now.weekday().number_from_monday() - 1) as usize;
        *self.days.get(day_index).unwrap_or(&false)
    }

    /// Format the output string
    pub fn pretty_print(&self, now: &DateTime<Local>) {

        let days: Vec<char> = "MTWTFSS".chars().collect();
        let mut days_selection: Vec<char> = vec!['_','_','_','_','_','_','_'];
        
        for (index, day) in days.iter().enumerate() {
            if self.days[index]{
                days_selection[index] = *day
            }
        }

        println!("Days: {:?}, Time: {:#02}:{:#02}:{:#02}, File: {}, Volume: {}%, Last run: {}, Run today: {}",
                 days_selection,
                 self.time.hours, self.time.minutes, self.time.seconds,
                 self.audio_file,
                 self.volume,
                 self.last_run.to_rfc3339(),
                 !self.already_run_today(now)
        )
    }
}

/// Compares hours, minutes, seconnds for 2 times
fn hms_gte(hms1 : (u32, u32, u32), hms2: (u32, u32, u32)) -> bool {
    hms1.0 * 3600 + hms1.1 * 60 + hms1.2 >= hms2.0 * 3600 + hms2.1 * 60 + hms2.2
}

#[test]
fn test_hms_gte() {
    assert!(hms_gte((0,0,0), (0,0,0)));
    assert!(hms_gte((8,0,0), (7,59,59)));
    assert!(hms_gte((23,0,0), (0,59,59)));
    assert!(hms_gte((0,1,0), (0,0,59)));
}

/// Parses a string to an unsigned int given a min and a max
/// max is default value in case of parse failure
fn parse_int_with_min_max(i: &str, min: u32, max: u32) -> u32 {
    use std::cmp;
    cmp::min(max, cmp::max(min, i.parse().unwrap_or(max)))
}

#[test]
fn test_parse_int_with_min_max() {
    assert_eq!(parse_int_with_min_max(&"0".to_string(),0,0), 0);
    assert_eq!(parse_int_with_min_max(&"fail".to_string(),10,100), 100);
    assert_eq!(parse_int_with_min_max(&"110".to_string(),10,100), 100);
    assert_eq!(parse_int_with_min_max(&"-8".to_string(),10,100), 100);
    assert_eq!(parse_int_with_min_max(&"80".to_string(),10,100), 80)
}

/// Parses a string to a vec of 7 days, monday first
/// For each position true when day is active, false otherwise
fn parse_days(s: &str) -> Option<Vec<bool>> {
    match s.to_string().as_bytes() {
        [mon, tue, wed, fri, thu, sat, sun] => {
            Some(vec![
                *mon == ('M' as u8),
                *tue == ('T' as u8),
                *wed == ('W' as u8),
                *fri == ('T' as u8),
                *thu == ('F' as u8),
                *sat == ('S' as u8),
                *sun == ('S' as u8)
            ])
        },
        _ => None
    }
}

#[test]
fn test_parse_days() {
    assert_eq!(parse_days(&"MTWTFSS".to_string()), Some(vec![true,true,true,true,true,true,true]));
    assert_eq!(parse_days(&"M_W_F_S".to_string()), Some(vec![true,false,true,false,true,false,true]));
    assert_eq!(parse_days(&"_______".to_string()), Some(vec![false,false,false,false,false,false,false]));
    assert_eq!(parse_days(&"???????".to_string()), Some(vec![false,false,false,false,false,false,false]));
    assert_eq!(parse_days(&"BAD_LENGTH".to_string()), None);
    assert_eq!(parse_days(&"SMALL".to_string()), None);
}

/// Parses a time to hours, minutes, seconds
fn parse_time(s: &str) -> Option<Time> {
    let x = s.split(":");
    let split: Vec<&str> = x.collect();
    match split.as_slice() {
        [h, m, s] => {
            Some(Time {
                hours: parse_int_with_min_max(h, 0, 23),
                minutes: parse_int_with_min_max(m, 0, 59),
                seconds: parse_int_with_min_max(s, 0, 59)
            })
        },
        _ => None
    }
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time("00:00:00"), Some(Time { hours: 0, minutes: 0, seconds: 0 }));
    assert_eq!(parse_time("01:02:03"), Some(Time { hours: 1, minutes: 2, seconds: 3 }));
    assert_eq!(parse_time("23:59:59"), Some(Time { hours: 23, minutes: 59, seconds: 59 }));
    assert_eq!(parse_time("24:00:00"), Some(Time { hours: 23, minutes: 0, seconds: 0 })); // max is 23
    assert_eq!(parse_time("00:60:00"), Some(Time { hours: 0, minutes: 59, seconds: 0 })); // max is 59
    assert_eq!(parse_time("00:00:60"), Some(Time { hours: 0, minutes: 0, seconds: 59 })); // max is 59
    assert_eq!(parse_time("????????"), None);
}

/// Parses a string line to an alarm config
fn parse_configuration<'a>(s: &'a str) -> Option<AlarmConfig> {
    let x = s.split(";");
    let split: Vec<&str> = x.collect();
    let last_run = chrono::Local::now();

    let maybe_config: Option<AlarmConfig> = match split.as_slice() {
        [the_days, the_time, the_audio, the_volume] => {
            let time_opt = parse_time(the_time);

            let selected_days_opt: Option<Vec<bool>> = parse_days(the_days);
            let volume = parse_int_with_min_max(the_volume,0 ,100);
            match (selected_days_opt, time_opt) {
                (Some(selected_days), Some(time)) => {
                    let cfg = AlarmConfig {
                        days: selected_days,
                        time,
                        audio_file: the_audio.to_string(),
                        volume,
                        last_run
                    };
                    Some(cfg)
                },
                _ => {
                    println!("Days or time invalid");
                    None
                }
            }
        },
        _ => {
            println!("Invalid configuration: {}", s);
            None
        }
    };

    maybe_config
}

#[test]
fn test_parse_configuration() {
    let config_str = "MTWTF__;06:30:15;/home/myhome/audio.ogg;75";
    let config_opt = parse_configuration(config_str);
    assert!(config_opt.is_some());

    let config = config_opt.unwrap();
    assert_eq!(config.days, vec![true,true,true,true,true,false,false]);
    assert_eq!(config.time, Time { hours: 6, minutes: 30, seconds: 15});
    assert_eq!(config.audio_file, "/home/myhome/audio.ogg");
    assert_eq!(config.volume, 75);

    let fake_config_str = "MTWTF__;06:30:15;/home/myhome/audio.ogg;75;unknown;";
    let fake_config_opt = parse_configuration(fake_config_str);
    assert!(fake_config_opt.is_none());
}

/// Retrieve the configuration from a configuration file
pub fn retrieve_configuration(path: &str) -> Vec<AlarmConfig> {
    let mut buff: Vec<AlarmConfig> = Vec::new();

    match OpenOptions::new().truncate(false).read(true).open(path) {
        Ok(ref mut file) => {
            use std::io::Read;

            let mut buff_str = String::new();
            let read_result = file.read_to_string(&mut buff_str);

            match read_result {
                Ok(_) => {

                    let parsed = buff_str
                        .lines()
                        .flat_map(|line| parse_configuration(&line))
                        .collect::<Vec<_>>();

                    buff.extend(parsed.iter().cloned());
                },
                Err(err) => { panic!("Unable to read file: {}", err); }
            }
        },
        Err(err) => { panic!("Failed to open configuration file: {}", err); }
    }

    buff
}
