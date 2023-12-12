use allen::{AllenError, Context, Device, Orientation};

pub mod sound_asset;
pub mod sound_source;

#[derive(Debug)]
pub enum SoundError {
    CurrentDeviceGettingError,
    ContextCreationError(AllenError),
    Not16BitWavFileError,
    NotMonoWavFileError,
    SoundAssetLoadingError,
    BufferCreationFailedError(AllenError),
    SourceCreationFailedError(AllenError),
    WrongEmitterType,
    SettingPositionError(AllenError)
}

static mut DEVICE: Option<Device> = None;
pub static mut CONTEXT: Option<Context> = None;

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

pub fn set_listener_position(position: [f32; 3]) {
    let context = take_context();
    let _ = context.listener().set_position(position);
    return_context(context)
}

pub fn set_listener_orientation(at: [f32; 3], up: [f32; 3]) {
    let context = take_context();
    let _ = context.listener().set_orientation(Orientation { at, up });
    return_context(context)
}

pub fn set_listener_transform(position: [f32; 3], at: [f32; 3], up: [f32; 3]) {
    set_listener_position(position);
    set_listener_orientation(at, up);
}


pub fn take_context() -> Context {
    unsafe {
        return CONTEXT.take().unwrap();
    }
}

pub fn return_context(context: Context) {
    unsafe { CONTEXT = Some(context) }
}
