use chrono::{DateTime, Local};
use config::{AlarmConfig, Time};
use chrono::prelude::*;
use tests::oldtime::Duration;

#[test]
fn already_run_today_check_true() {

    let now: DateTime<Local> = Local::now();
    let before: DateTime<Local> = now - Duration::seconds(1);

    let time_for_alarm = Time {
        hours: before.hour(),
        minutes: before.minute(),
        seconds: before.second()
    };

    let alarm_config = AlarmConfig {
        days: vec![],
        time: time_for_alarm,
        audio_file: "/fake/path".to_string(),
        volume: 100,
        last_run: before
    };

    assert_eq!(alarm_config.already_run_today(&now), true);
}

#[test]
fn already_run_today_check_false() {

    let now: DateTime<Local> = Local::now();
    let after: DateTime<Local> = now + Duration::seconds(1);

    let time_for_alarm = Time {
        hours: after.hour(),
        minutes: after.minute(),
        seconds: after.second()
    };

    let alarm_config = AlarmConfig {
        days: vec![],
        time: time_for_alarm,
        audio_file: "/fake/path".to_string(),
        volume: 100,
        last_run: now
    };

    assert_eq!(alarm_config.already_run_today(&now), false);
}

#[test]
fn is_expired_check_true() {

// should the alarm be launched
    let now: DateTime<Local> = Local::now();
    let before: DateTime<Local> = now - Duration::seconds(1);

    // time_for_alarm <= now
    let time_for_alarm = Time {
        hours: now.hour(),
        minutes: now.minute(),
        seconds: now.second()
    };

    let alarm_config = AlarmConfig {
        days: vec![],
        time: time_for_alarm,
        audio_file: "/fake/path".to_string(),
        volume: 100,
        last_run: before
    };

    assert_eq!(alarm_config.is_expired(&now), true);
}

#[test]
fn is_expired_check_false() {

// should the alarm be launched
    let now: DateTime<Local> = Local::now();
    let after: DateTime<Local> = now + Duration::seconds(1);

    // time_for_alarm >= now
    let time_for_alarm = Time {
        hours: after.hour(),
        minutes: after.minute(),
        seconds: after.second()
    };

    let alarm_config = AlarmConfig {
        days: vec![],
        time: time_for_alarm,
        audio_file: "/fake/path".to_string(),
        volume: 100,
        last_run: after
    };

    assert_eq!(alarm_config.is_expired(&now), false);
}

