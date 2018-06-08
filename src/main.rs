extern crate cast;
// libncursesw5-dev
extern crate cursive;
#[macro_use]
extern crate failure;
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::rc::Rc;

use cast::f32;
use cast::u8;
use cursive::traits::Boxable;
use cursive::traits::Identifiable;
use cursive::views::Dialog;
use cursive::views::EditView;
use failure::Error;
use failure::ResultExt;
use psimple::Simple;

const RATE: u32 = 44100;

fn main() -> Result<(), Error> {
    use psimple::Simple;
    use pulse::stream::Direction;

    let spec = pulse::sample::Spec {
        format: pulse::sample::Format::U8,
        channels: 1,
        rate: RATE,
    };
    ensure!(spec.is_valid(), "spec invalid!");

    let simple = Rc::new(
        Simple::new(
            None,                // Use the default server
            "TunnelApp",         // Our application's name
            Direction::Playback, // We want a playback stream
            None,                // Use the default device
            "Music",             // Description of our stream
            &spec,               // Our sample format
            None,                // Use default channel map
            None,                // Use default buffering attributes
        ).map_err(pulse)
            .with_context(|_| format_err!("connecting to server"))?,
    );

    let mut siv = cursive::Cursive::new();

    let ew = simple.clone();

    siv.add_layer(
        Dialog::new()
            .title("Frequency (Hz)")
            // Padding is (left, right, top, bottom)
            .padding((1, 1, 1, 0))
            .content(
                EditView::new()
                    .on_submit(move |s, val| gui_tone(&ew, s, val))
                    .with_id("freq")
                    .fixed_width(8),
            )
            .button("Play",  move|s| {
                // BORROW CHECKER: can't inline this local
                let val = s
                    .call_on_id("freq", |edit: &mut EditView| edit.get_content())
                    .expect("ui element must exist");
                gui_tone(&simple, s, &val);
            }),
    );
    siv.run();

    Ok(())
}

fn gui_tone(s: &Simple, siv: &mut cursive::Cursive, val: &str) {
    match val.parse() {
        Ok(freq) => if let Err(e) = tone(s, freq) {
            siv.add_layer(Dialog::info(format!("playback error: {:?}", e)));
        },
        Err(e) => siv.add_layer(Dialog::info(format!("parse error: {}", e))),
    }
}

fn tone(s: &Simple, freq: f32) -> Result<(), Error> {
    let buf = (0..RATE)
        .map(|i| u8(((f32(i) * freq / f32(RATE)).cos() + 1.) * 127.))
        .collect::<Result<Vec<u8>, cast::Error>>()?;

    s.write(&buf)
        .map_err(pulse)
        .with_context(|_| format_err!("writing samples"))?;

    Ok(())
}

fn pulse(err: pulse::error::PAErr) -> Error {
    format_err!("{}", err)
}
