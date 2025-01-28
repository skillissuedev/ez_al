cd ez_al_example/
cargo b --release
xcopy src\sound.wav target\release
xcopy src\sound_stereo_32bit.wav target\release
cargo r --release

