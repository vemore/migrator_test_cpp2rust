

use std::fmt;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    subchunk_id: [u8; 4],
    subchunk_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl WavHeader {
    fn new(chunk_id: [u8; 4], chunk_size: u32, format: [u8; 4], subchunk_id: [u8; 4], subchunk_size: u32, audio_format: u16, num_channels: u16, sample_rate: u32, byte_rate: u32, block_align: u16, bits_per_sample: u16) -> Result<WavHeader, Box<dyn Error>> {
        if chunk_id != [b'R', b'I', b'F', b'F'] {
            return Err("Invalid chunk ID".into());
        }
        if format != [b'W', b'A', b'V', b'E'] {
            return Err("Invalid format".into());
        }
        if subchunk_id != [b'f', b'm', b't', b' '] {
            return Err("Invalid subchunk ID".into());
        }
        if audio_format != 1 {
            return Err("Invalid audio format".into());
        }
        if num_channels != 1 && num_channels != 2 {
            return Err("Invalid number of channels".into());
        }
        if sample_rate != 44100 {
            return Err("Invalid sample rate".into());
        }
        if byte_rate != sample_rate * num_channels * (bits_per_sample / 8) as u32 {
            return Err("Invalid byte rate".into());
        }
        if block_align != num_channels * (bits_per_sample / 8) as u16 {
            return Err("Invalid block align".into());
        }
        if bits_per_sample % 8 != 0 {
            return Err("Invalid bits per sample".into());
        }
        if subchunk_size < 16 {
            return Err("Invalid subchunk size".into());
        }
        if chunk_size < 36 {
            return Err("Invalid chunk size".into());
        }
        Ok(WavHeader {
            chunk_id,
            chunk_size,
            format,
            subchunk_id,
            subchunk_size,
            audio_format,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
        })
    }
}

impl Display for WavHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chunk ID: {}\n", String::from_utf8_lossy(&self.chunk_id))
            .and_then(|_| write!(f, "Chunk Size: {}\n", self.chunk_size))
            .and_then(|_| write!(f, "Format: {}\n", String::from_utf8_lossy(&self.format)))
            .and_then(|_| write!(f, "Subchunk ID: {}\n", String::from_utf8_lossy(&self.subchunk_id)))
            .and_then(|_| write!(f, "Subchunk Size: {}\n", self.subchunk_size))
            .and_then(|_| write!(f, "Audio Format: {}\n", self.audio_format))
            .and_then(|_| write!(f, "Number of Channels: {}\n", self.num_channels))
            .and_then(|_| write!(f, "Sample Rate: {}\n", self.sample_rate))
            .and_then(|_| write!(f, "Byte Rate: {}\n", self.byte_rate))
            .and_then(|_| write!(f, "Block Align: {}\n", self.block_align))
            .and_then(|_| write!(f, "Bits per Sample: {}", self.bits_per_sample))
    }
}