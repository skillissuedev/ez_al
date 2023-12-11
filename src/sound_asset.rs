use allen::{Buffer, BufferData, Channels};
use hound::WavReader;

use crate::{SoundError, take_context, return_context};

pub struct WavAsset {
    samples: Vec<i16>,
    pub buffer: Buffer,
}

impl WavAsset {
    pub fn from_wav(path: &str) -> Result<Self, SoundError> {
        let context = take_context();

        let reader = WavReader::open(path);
        match reader {
            Ok(_) => (),
            Err(err) => {
                return_context(context);
                return Err(SoundError::SoundAssetLoadingError);
            }
        }
        let mut reader = reader.unwrap();

        if reader.spec().channels > 1 {
            return_context(context);
            return Err(SoundError::NotMonoWavFileError);
        }

        if reader.spec().bits_per_sample != 16 {
            return_context(context);
            return Err(SoundError::Not16BitWavFileError);
        }

        let samples = reader
            .samples::<i16>()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>();

        let buffer = context.new_buffer();
        match buffer {
            Ok(_) => (),
            Err(err) => {
                return_context(context);
                return Err(SoundError::BufferCreationFailedError(err));
            }
        }
        let buffer = buffer.unwrap();

        match buffer.data(
            BufferData::I16(&samples),
            Channels::Mono,
            reader.spec().sample_rate as i32,
        ) {
            Ok(_) => (),
            Err(err) => {
                return_context(context);
                return Err(SoundError::BufferCreationFailedError(err));
            }
        };

        return_context(context);

        return Ok(WavAsset { samples, buffer });
    }
}

