use std::error::Error;

use sola_raylib::prelude::*;

pub struct Textures {
    pub player: Texture2D,
    pub laser: Texture2D,
    pub meteor: Texture2D,
}

pub struct AudioAssets<'aud> {
    pub laser: Sound<'aud>,
    pub music: Music<'aud>,
}

pub struct Assets<'aud> {
    pub textures: Textures,
    pub audio: AudioAssets<'aud>,
}

impl Textures {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            player: rl.load_texture(thread, "../assets/images/spaceship.png")?,
            laser: rl.load_texture(thread, "../assets/images/laser.png")?,
            meteor: rl.load_texture(thread, "../assets/images/meteor.png")?,
        })
    }

    pub fn size_of(texture: &Texture2D) -> Vector2 {
        Vector2::new(texture.width() as f32, texture.height() as f32)
    }
}

impl<'aud> AudioAssets<'aud> {
    pub fn load(audio: &'aud RaylibAudio) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            laser: audio.new_sound("../assets/audio/laser.wav")?,
            music: audio.new_music("../assets/audio/music.wav")?,
        })
    }
}

impl<'aud> Assets<'aud> {
    pub fn load(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        audio: &'aud RaylibAudio,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            textures: Textures::load(rl, thread)?,
            audio: AudioAssets::load(audio)?,
        })
    }
}
