use std::fmt::Debug;
use allen::Source;
use crate::{sound_asset::SoundAsset, SoundError, take_context, return_context};

pub struct SoundEmitter {
    pub name: String,
    pub emitter_type: SoundEmitterType,
    source: Source,
}

#[derive(Debug)]
pub enum SoundEmitterType {
    Simple,
    Positional,
}

impl SoundEmitter {
    pub fn new(name: &str, asset: &SoundAsset, emitter_type: SoundEmitterType) -> Result<SoundEmitter, SoundError> {
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
        match emitter_type {
            SoundEmitterType::Simple => source.set_relative(true).unwrap(),
            SoundEmitterType::Positional => {
                let _ = source.set_reference_distance(0.0);
                let _ = source.set_rolloff_factor(1.0);
                let _ = source.set_min_gain(0.0);
            }
        }

        return_context(context);

        return Ok(SoundEmitter {
            name: name.to_string(),
            emitter_type,
            source,
        });
    }

    pub fn set_looping(&mut self, should_loop: bool) {
        let _ = self.source.set_looping(should_loop);
    }

    pub fn is_looping(&self) -> bool {
        self.source.is_looping().unwrap()
    }

    pub fn play_sound(&mut self) {
        let _ = self.source.play();
    }

    pub fn set_max_distance(&mut self, distance: f32) -> Result<(), SoundError> {
        match self.emitter_type {
            SoundEmitterType::Simple => {
                return Err(SoundError::WrongEmitterType);
            }
            SoundEmitterType::Positional => {
                let _ = self.source.set_max_distance(distance);
                return Ok(());
            }
        }
    }

    pub fn get_max_distance(&mut self) -> Result<f32, SoundError> {
        match self.emitter_type {
            SoundEmitterType::Simple => {
                return Err(SoundError::WrongEmitterType);
            }
            SoundEmitterType::Positional => return Ok(self.source.max_distance().unwrap()),
        }
    }

    pub fn update(&mut self, sound_position: [f32; 3]) -> Result<(), SoundError> {
        let position_result_result = self.source.set_position(sound_position.into());
        match position_result_result {
            Ok(()) => Ok(()),
            Err(error) => Err(SoundError::SettingPositionError(error)),
        }
    }
}

