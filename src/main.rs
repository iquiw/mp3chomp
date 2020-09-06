use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use rmp3::{Decoder, Frame};

const MIN_SAMPLE: i16 = 1;

fn read_file<P>(path: P) -> Result<Vec<u8>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn process_file<P>(path: P, verbose: bool) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let data = read_file(&path)?;
    let mut decoder = Decoder::new(&data);
    let mut zero_begin = None;
    let mut length = 0.0;
    while let Some(Frame {
        sample_rate,
        sample_count,
        samples,
        ..
    }) = decoder.next_frame()
    {
        if sample_count != 0 {
            length += sample_count as f32 / sample_rate as f32;
        }
        if samples.iter().all(|s| *s <= MIN_SAMPLE && *s >= -MIN_SAMPLE) {
            if zero_begin.is_none() {
                zero_begin = Some(length);
            }
        } else {
            zero_begin = None;
        }
    }
    let possible_trailing = zero_begin.and_then(|t| {
        let t5 = t + 5.0;
        if t5 < length {
            Some(t5)
        } else {
            None
        }
    });
    if verbose {
        if let Some(trailing) = possible_trailing {
            println!(
                "{}: Total: {}, Trailing: {}",
                path.as_ref().display(),
                length,
                trailing
            );
        } else {
            println!(
                "{}: Total: {}, No Trailing",
                path.as_ref().display(),
                length
            );
        }
    } else if possible_trailing.is_some() {
        println!(
            "{}: {}",
            path.as_ref().display(),
            possible_trailing.unwrap()
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut verbose = false;
    for arg in env::args_os().skip(1) {
        if arg == "--verbose" {
            verbose = true;
            continue;
        }
        process_file(&arg, verbose)?;
    }
    Ok(())
}
