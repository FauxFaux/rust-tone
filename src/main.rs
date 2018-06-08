extern crate cast;
#[macro_use]
extern crate failure;
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use cast::f32;
use cast::u8;
use failure::Error;

fn main() -> Result<(), Error> {
    use psimple::Simple;
    use pulse::stream::Direction;

    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_S16NE,
        channels: 2,
        rate: 44100,
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

    let mut buf = [0u8; 44100 * 2];
    for i in 0..44100 {
        let val = u8((f32(i * 4000usize).sin() + 1.) * 127.)?;
        buf[i * 2] = val;
        buf[i * 2 + 1] = val;
    }
    s.write(&buf)
        .map_err(|e| format_err!("write failed: {:?}", e))?;

    Ok(())
}
