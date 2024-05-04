use linear_model_allen::{AllenError, Context, Device, Orientation, Buffer, BufferData, Channels, Source};
use hound::WavReader;

pub struct EzAl {
    _device: Device,
    context: Context
}
impl EzAl {
    pub fn new() -> Result<Self, SoundError> {
        let device = match Device::open(None) {
            None => {
                return Err(SoundError::CurrentDeviceGettingError);
            }
            Some(device) => device,
        };

        let context = match device.create_context() {
            Err(err) => {
                return Err(SoundError::ContextCreationError(err));
            }
            Ok(context) => {
                context.make_current();
                context
            },
        };
        //DEVICE = Some(device);

        //CONTEXT = Some(context);

        Ok(EzAl {
            _device: device,
            context,
        })
    }
}

/// All errors that may occur when loading/playing sounds
#[derive(Debug)]
pub enum SoundError {
    /// Failed to get current audio device
    CurrentDeviceGettingError,
    /// Failed to create OpenAL context
    ContextCreationError(AllenError),
    /// All Wav files should contain 16-bit mono audio
    Not16BitWavFileError,
    /// All Wav files should contain 16-bit mono audio
    NotMonoWavFileError,
    /// Failed to load Wav file(probably failed to access file)
    SoundAssetLoadingError(hound::Error),
    /// Failed to create OpenAL buffer
    BufferCreationFailedError(AllenError),
    /// Failed to create an audio source
    SourceCreationFailedError(AllenError),
    /// Returned when trying to access functions, that are not available for this sound source type
    /// 
    /// Example: trying to access position when source's type is Simple 
    WrongSoundSourceType,
    /// Failed to set source position 
    SettingPositionError(AllenError)
}

/// Sets position of listener.
pub fn set_listener_position(al: &EzAl, position: [f32; 3]) {
    let context = &al.context;
    let _ = context.listener().set_position(position);
}

/// Sets orientation of listener.
pub fn set_listener_orientation(al: &EzAl, at: [f32; 3], up: [f32; 3]) {
    let context = &al.context;
    let _ = context.listener().set_orientation(Orientation { at, up });
}

/// Sets position and orientation of listener.
pub fn set_listener_transform(al: &EzAl, position: [f32; 3], at: [f32; 3], up: [f32; 3]) {
    set_listener_position(al, position);
    set_listener_orientation(al, at, up);
}



/// Wav file sound asset.
///
/// # Important note:
///
/// All .wav files should conatin only mono 16 bit sounds.
pub struct WavAsset {
    pub(crate) buffer: Buffer,
}

impl WavAsset {
    /// Loads Wav file and creates an asset.
    pub fn from_wav(al: &EzAl, path: &str) -> Result<Self, SoundError> {
        let context = &al.context;

        let reader = WavReader::open(path);
        match reader {
            Ok(mut reader) => {
                if reader.spec().channels > 1 {
                    return Err(SoundError::NotMonoWavFileError);
                }

                if reader.spec().bits_per_sample != 16 {
                    return Err(SoundError::Not16BitWavFileError);
                }

                let samples = reader
                    .samples::<i16>()
                    .map(|s| s.unwrap())
                    .collect::<Vec<_>>();

                let buffer = context.new_buffer();
                match buffer {
                    Ok(buffer) => {
                        if let Err(err) = buffer.data(
                            BufferData::I16(&samples),
                            Channels::Mono,
                            reader.spec().sample_rate as i32,
                        ) {
                            return Err(SoundError::BufferCreationFailedError(err));
                        };

                        Ok(WavAsset { buffer })
                    },
                    Err(err) => {
                        Err(SoundError::BufferCreationFailedError(err))
                    }
                }
            },
            Err(err) => {
                Err(SoundError::SoundAssetLoadingError(err))
            }
        }
    }
}





/// Sound source
/// 
/// You can play audio using this struct
pub struct SoundSource {
    pub source_type: SoundSourceType,
    source: Source,
}

/// Type of SoundSource
/// 
/// Source can be positional(sound is changing when source/listener position/orientation changed)
/// or simple(doesn't have position, can be used, for example, to play music)
#[derive(Debug, Clone, Copy)]
pub enum SoundSourceType {
    /// When SoundSourceType is Simple, you won't be able to set source position.
    /// Listener position and orientation won't affect sound
    Simple, 
    /// When SoundSourceType is Positional, position of source could be set.
    /// Position and orientation of listener would affect source sound.
    Positional,
}

impl SoundSource {
    /// This funcion creates new SoundSource
    pub fn new(al: &EzAl, asset: &WavAsset, source_type: SoundSourceType) -> Result<SoundSource, SoundError> {
        let context = &al.context;
        let source_result = context.new_source();
        let source: Source;
        match source_result {
            Ok(src) => source = src,
            Err(err) => {
                return Err(SoundError::SourceCreationFailedError(err));
            }
        }

        let _ = source.set_buffer(Some(&asset.buffer));
        match source_type {
            SoundSourceType::Simple => source.set_relative(true).unwrap(),
            SoundSourceType::Positional => {
                let _ = source.set_reference_distance(0.0);
                let _ = source.set_rolloff_factor(1.0);
                let _ = source.set_min_gain(0.0);
            }
        }

        Ok(SoundSource {
            source_type,
            source,
        })
    }

    /// Sets looping value of source
    pub fn set_looping(&mut self, should_loop: bool) {
        let _ = self.source.set_looping(should_loop);
    }

    /// Sets looping value of source
    pub fn is_looping(&self) -> bool {
        self.source.is_looping().unwrap()
    }

    /// Makes source play it's sound
    pub fn play_sound(&mut self) {
        let _ = self.source.play();
    }

    /// Sets max distance from listener to source.
    /// 
    /// If distance is more than max, user won't hear sound of source.
    /// 
    /// Type of source should be positional to use this function.
    pub fn set_max_distance(&mut self, distance: f32) -> Result<(), SoundError> {
        match self.source_type {
            SoundSourceType::Simple => {
                Err(SoundError::WrongSoundSourceType)
            }
            SoundSourceType::Positional => {
                let _ = self.source.set_max_distance(distance);
                Ok(())
            }
        }
    }

    /// Gets max distance from listener to source.
    /// 
    /// If distance is more than max, user won't hear sound of source.
    /// 
    /// Type of source should be positional to use this function.
    pub fn get_max_distance(&mut self) -> Result<f32, SoundError> {
        match self.source_type {
            SoundSourceType::Simple => {
                Err(SoundError::WrongSoundSourceType)
            }
            SoundSourceType::Positional => Ok(self.source.max_distance().unwrap()),
        }
    }

    /// Sets position of the source.
    /// 
    /// Type of source should be positional to use this function.
    pub fn update(&mut self, sound_position: [f32; 3]) -> Result<(), SoundError> {
        let position_result_result = self.source.set_position(sound_position.into());
        match position_result_result {
            Ok(()) => Ok(()),
            Err(error) => Err(SoundError::SettingPositionError(error)),
        }
    }
}
