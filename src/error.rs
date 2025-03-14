

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum WavHeaderError {
    InvalidHeader,
    Io(io::Error),
    FileOpenError(io::Error),
    FileReadError(io::Error),
    InvalidWavFile,
    InvalidChunkId,
    InvalidChunkSize,
    InvalidAudioFormat,
    InvalidNumChannels,
    InvalidSampleRate,
    InvalidByteRate,
    InvalidBlockAlign,
    InvalidBitsPerSample,
}

impl fmt::Display for WavHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WavHeaderError::InvalidHeader => write!(f, "Invalid WAV header"),
            WavHeaderError::Io(err) => write!(f, "I/O error: {}", err),
            WavHeaderError::FileOpenError(err) => write!(f, "Failed to open file: {}", err),
            WavHeaderError::FileReadError(err) => write!(f, "Failed to read file: {}", err),
            WavHeaderError::InvalidWavFile => write!(f, "Invalid WAV file"),
            WavHeaderError::InvalidChunkId => write!(f, "Invalid chunk ID"),
            WavHeaderError::InvalidChunkSize => write!(f, "Invalid chunk size"),
            WavHeaderError::InvalidAudioFormat => write!(f, "Invalid audio format"),
            WavHeaderError::InvalidNumChannels => write!(f, "Invalid number of channels"),
            WavHeaderError::InvalidSampleRate => write!(f, "Invalid sample rate"),
            WavHeaderError::InvalidByteRate => write!(f, "Invalid byte rate"),
            WavHeaderError::InvalidBlockAlign => write!(f, "Invalid block align"),
            WavHeaderError::InvalidBitsPerSample => write!(f, "Invalid bits per sample"),
        }
    }
}

impl Error for WavHeaderError {}

impl From<io::Error> for WavHeaderError {
    fn from(err: io::Error) -> WavHeaderError {
        WavHeaderError::Io(err)
    }
}

pub fn handle_error(err: WavHeaderError) -> ! {
    eprintln!("Error: {}", err);
    std::process::exit(1);
}

pub fn handle_file_open_error(err: io::Error) -> ! {
    let error = WavHeaderError::FileOpenError(err);
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_file_read_error(err: io::Error) -> ! {
    let error = WavHeaderError::FileReadError(err);
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_wav_file() -> ! {
    let error = WavHeaderError::InvalidWavFile;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_chunk_id() -> ! {
    let error = WavHeaderError::InvalidChunkId;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_chunk_size() -> ! {
    let error = WavHeaderError::InvalidChunkSize;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_audio_format() -> ! {
    let error = WavHeaderError::InvalidAudioFormat;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_num_channels() -> ! {
    let error = WavHeaderError::InvalidNumChannels;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_sample_rate() -> ! {
    let error = WavHeaderError::InvalidSampleRate;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_byte_rate() -> ! {
    let error = WavHeaderError::InvalidByteRate;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_block_align() -> ! {
    let error = WavHeaderError::InvalidBlockAlign;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

pub fn handle_invalid_bits_per_sample() -> ! {
    let error = WavHeaderError::InvalidBitsPerSample;
    eprintln!("Error: {}", error);
    std::process::exit(1);
}