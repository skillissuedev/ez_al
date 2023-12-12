use linear_model_allen::{AllenError, Context, Device, Orientation, Buffer, BufferData, Channels, Source};
use hound::WavReader;

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

static mut DEVICE: Option<Device> = None;
static mut CONTEXT: Option<Context> = None;

/// Call this function before creating any sound sources.
pub fn init() -> Result<(), SoundError> {
    unsafe {
        let device = Device::open(None);
        match device {
            None => {
                return Err(SoundError::CurrentDeviceGettingError);
            }
            Some(_) => (),
        }
        let device = device.unwrap();
        let context = device.create_context();
        match context {
            Err(err) => {
                return Err(SoundError::ContextCreationError(err));
            }
            Ok(_) => (),
        }
        DEVICE = Some(device);

        let context = context.unwrap();
        context.make_current();
        CONTEXT = Some(context);

        return Ok(());
    }
}

/// Sets position of listener.
pub fn set_listener_position(position: [f32; 3]) {
    let context = take_context();
    let _ = context.listener().set_position(position);
    return_context(context)
}

/// Sets orientation of listener.
pub fn set_listener_orientation(at: [f32; 3], up: [f32; 3]) {
    let context = take_context();
    let _ = context.listener().set_orientation(Orientation { at, up });
    return_context(context)
}

/// Sets position and orientation of listener.
pub fn set_listener_transform(position: [f32; 3], at: [f32; 3], up: [f32; 3]) {
    set_listener_position(position);
    set_listener_orientation(at, up);
}

pub(crate) fn take_context() -> Context {
    unsafe {
        return CONTEXT.take().unwrap();
    }
}

pub(crate) fn return_context(context: Context) {
    unsafe { CONTEXT = Some(context) }
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
    pub fn from_wav(path: &str) -> Result<Self, SoundError> {
        let context = take_context();

        let reader = WavReader::open(path);
        match reader {
            Ok(_) => (),
            Err(err) => {
                return_context(context);
                return Err(SoundError::SoundAssetLoadingError(err));
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

        return Ok(WavAsset { buffer });
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
    pub fn new(asset: &WavAsset, source_type: SoundSourceType) -> Result<SoundSource, SoundError> {
        let context = take_context();
        let source_result = context.new_source();
        let source: Source;
        match source_result {
            Ok(src) => source = src,
            Err(err) => {
                return_context(context);
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

        return_context(context);

        return Ok(SoundSource {
            source_type,
            source,
        });
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
                return Err(SoundError::WrongSoundSourceType);
            }
            SoundSourceType::Positional => {
                let _ = self.source.set_max_distance(distance);
                return Ok(());
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
                return Err(SoundError::WrongSoundSourceType);
            }
            SoundSourceType::Positional => return Ok(self.source.max_distance().unwrap()),
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

