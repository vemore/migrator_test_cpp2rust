

use std::fs::File;
use std::io::{BufReader, Read, stderr, Write};
use std::path::Path;
use std::error::Error;

mod error;

#[derive(Debug)]
struct WavHeader {
    riff: [u8; 4],
    wav_size: u32,
    wave: [u8; 4],
    fmt: [u8; 4],
    fmt_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    data: [u8; 4],
    data_size: u32,
}

fn parse_wav_header(file_path: &str) -> Result<WavHeader, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let mut reader = BufReader::new(file);
    let mut header = WavHeader {
        riff: [0; 4],
        wav_size: 0,
        wave: [0; 4],
        fmt: [0; 4],
        fmt_size: 0,
        audio_format: 0,
        num_channels: 0,
        sample_rate: 0,
        byte_rate: 0,
        block_align: 0,
        bits_per_sample: 0,
        data: [0; 4],
        data_size: 0,
    };

    reader.read_exact(&mut header.riff)?;

    if header.riff != [b'R', b'I', b'F', b'F'] {
        return Err(Box::new(error::InvalidWavHeaderError));
    }

    reader.read_exact(&mut header.wav_size.to_le_bytes())?;

    reader.read_exact(&mut header.wave)?;

    if header.wave != [b'W', b'A', b'V', b'E'] {
        return Err(Box::new(error::InvalidWavHeaderError));
    }

    reader.read_exact(&mut header.fmt)?;

    if header.fmt != [b'f', b'm', b't', b' '] {
        return Err(Box::new(error::InvalidWavHeaderError));
    }

    reader.read_exact(&mut header.fmt_size.to_le_bytes())?;

    reader.read_exact(&mut header.audio_format.to_le_bytes())?;

    reader.read_exact(&mut header.num_channels.to_le_bytes())?;

    reader.read_exact(&mut header.sample_rate.to_le_bytes())?;

    reader.read_exact(&mut header.byte_rate.to_le_bytes())?;

    reader.read_exact(&mut header.block_align.to_le_bytes())?;

    reader.read_exact(&mut header.bits_per_sample.to_le_bytes())?;

    reader.read_exact(&mut header.data)?;

    if header.data != [b'd', b'a', b't', b'a'] {
        return Err(Box::new(error::InvalidWavHeaderError));
    }

    reader.read_exact(&mut header.data_size.to_le_bytes())?;

    Ok(header)
}

mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct InvalidWavHeaderError;

    impl Error for InvalidWavHeaderError {}

    impl fmt::Display for InvalidWavHeaderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid WAV file header")
        }
    }
}