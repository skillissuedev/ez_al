# A simple audio library

This library makes it easy to play sounds.

    // Initializing ez_al
    ez_al::init().expect("Failed to init ez_al!");
    
    // Creating an asset
    let asset = WavAsset::from_wav("sound.wav")
      .expect("failed to load a wav file");
        
    // Creating sources
    let mut pos_source = SoundSource::new(&asset, SoundSourceType::Positional)
      .expect("Failed to create a positional sound source");
        
    let mut simple_source = SoundSource::new(&asset, SoundSourceType::Simple)
      .expect("Failed to create a simple sound source");

    // Setting listener position and orientation
    ez_al::set_listener_transform(cam_pos, cam_at, cam_up);

    // Playing sounds
    pos_source.play_sound();
    simple_source.play_sound();

## Important note

You should only use 16-bit mono .wav files.

## Prerequirements

Installed cmake and clang 

Installation example(Arch Linux): `sudo pacman -S clang cmake`
