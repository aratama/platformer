use crate::wasm4;

pub struct Sound {
    pub freq1: u32,
    pub freq2: u32,
    pub attack: u32,
    pub decay: u32,
    pub sustain: u32,
    pub release: u32,
    pub volume: u32,
    pub channel: u32,
    pub mode: u32,
}

pub fn play(sound: Sound) {
    let freq = sound.freq1 | sound.freq2 << 16;
    let duration = sound.attack << 24 | sound.decay << 16 | sound.sustain | sound.release << 8;
    let flags = sound.channel | sound.mode << 2;
    wasm4::tone(freq, duration, sound.volume, flags);
}
