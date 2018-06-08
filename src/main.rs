extern crate cast;
#[macro_use]
extern crate failure;
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::fs;
use std::io::Write;

use cast::f32;
use cast::u8;
use cast::usize;
use failure::Error;

fn main() -> Result<(), Error> {
    use psimple::Simple;
    use pulse::stream::Direction;

    let rate = 44100;

    let spec = pulse::sample::Spec {
        format: pulse::sample::Format::U8,
        channels: 1,
        rate,
    };
    ensure!(spec.is_valid(), "spec invalid!");

    let s = Simple::new(
        None,                // Use the default server
        "TunnelApp",         // Our application's name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    ).map_err(|e| format_err!("simple failed: {:?}", e))?;

    let freq = 4000f32;

    let buf = (0..rate)
        .map(|i| u8(((f32(i) * freq / f32(rate)).cos() + 1.) * 127.))
        .collect::<Result<Vec<u8>, cast::Error>>()?;

    s.write(&buf)
        .map_err(|e| format_err!("write failed: {:?}", e))?;

    fs::File::create("a.wav")?.write_all(&buf)?;

    Ok(())
}
