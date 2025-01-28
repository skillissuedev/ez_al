# A simple audio library

This library makes it easy to play .wav sounds.

    // Initializing ez_al
    let al = EzAl::new().expect("Failed to open current device or create OpenAL context!");
    
    // Creating an asset
    let asset = WavAsset::from_wav(&al, "sound.wav")
      .expect("failed to load a wav file");
        
    // Creating sources
    let mut pos_source = SoundSource::new(&al, &asset, SoundSourceType::Positional)
      .expect("Failed to create a positional sound source");
        
    let mut simple_source = SoundSource::new(&al, &asset, SoundSourceType::Simple)
      .expect("Failed to create a simple sound source");

    // Setting listener position and orientation
    ez_al::set_listener_transform(&al, cam_pos, cam_at, cam_up);

    // Playing sounds
    pos_source.play_sound();
    simple_source.play_sound();

## Prerequirements

### Linux

Installed openal

Installation example (Arch Linux): `sudo pacman -S openal`

### Windows

Make sure you've installed OpenAL 1.1 Core SDK in the default directory (C:\Program Files (x86)\OpenAL 1.1 SDK).
