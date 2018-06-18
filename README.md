# Rustine

This is a quick and simple alarm clock, in Rust, that can be configured from a text file.

The aim of this project was to give Rust a try:
1. Use cargo for bin and lib
1. Use external crates (time, audio)
1. Have a better understanding of ownership
1. String manipulation
1. Read keyboard inputs
1. Read from file system
1. Use threads
1. Communicate between threads using channels
1. Documentation
1. Testing in file and in an external module

## Configuration

Create a text file <project_root>/rustine_config/config

The format is CSV-like.

For an alarm running from monday to friday at 6:30 am an audio file at 75% of max volume:
```csv
MTWTF__;06:30:00;/some/file/system/path/audio.ogg;75
# You may add other configurations, one per line
```
## Supported format

Audio files are read with `rodio` that support WAV, Vorbis, Flac.

## Running
Get `Cargo`.

* Use `cargo run` to start the application.
* Use `cargo test` to run the tests.
* Use `cargo doc --no-deps` to generate the docs.

Once running:
* `stop` to stop a running alarm
* `quit` to stop the whole application

## License

See LICENSE file

