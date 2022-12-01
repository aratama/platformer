use crate::wasm4::*;

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
    tone(freq, duration, sound.volume, flags);
}

static mut BGM_MUSIC_COUNT: u32 = 0;

static mut CURRENT_BGM: Option<&'static Music> = Option::None;

static mut MASTER_VOLUE: u32 = 100;
// static mut MASTER_VOLUE: u32 = 100;

pub fn set_bgm(bgm: Option<&'static Music>) {
    unsafe {
        let reset = match (CURRENT_BGM, bgm) {
            (Some(r), Some(s)) => {
                let a: *const Music = r;
                let b: *const Music = s;
                a != b
            }
            _ => true,
        };

        if reset {
            BGM_MUSIC_COUNT = 0;
        }
        CURRENT_BGM = bgm;
    }
}

pub fn update_bgm() {
    unsafe {
        if let Option::Some(m) = CURRENT_BGM {
            music(m, &mut BGM_MUSIC_COUNT, 0, true);
        };
    }
}

pub fn music(music: &Music, music_count: &mut u32, pitch_offset: i32, loop_music: bool) {
    let current_position = if loop_music {
        *music_count % music_length(&music)
    } else {
        *music_count
    };

    for (channel, track) in music.tracks.iter().enumerate() {
        let mut position = 0;
        for note in (*track).notes {
            let (note_number, release, wait) = *note;
            if position == current_position && note_number != 0 {
                let freq = note_to_frequency((note_number as i32 + pitch_offset) as u32);
                play(Sound {
                    freq1: freq,
                    freq2: freq,
                    attack: 0,
                    decay: 0,
                    sustain: music.unit * release / 2,
                    release: music.unit * release / 2 - 1, // -1でずらさないと直後の音と被ってノイズが生じる
                    volume: (*track).volume * unsafe { MASTER_VOLUE } / 100,
                    channel: channel as u32,
                    mode: 0,
                })
            }
            position += music.unit * (release + wait);
        }
    }

    *music_count = *music_count + 1;
}

// (Pitch, Release)
pub type Note = (u32, u32, u32);

pub struct Track {
    pub notes: &'static [Note],
    pub volume: u32,
}

pub struct Music {
    pub unit: u32,
    pub tracks: [Track; 4],
}

fn music_length(music: &Music) -> u32 {
    let mut len: u32 = 0;
    for track in music.tracks.iter() {
        let mut position = 0;
        for note in (*track).notes {
            let (_, release, wait) = *note;
            position += music.unit * (release + wait);
        }
        len = u32::max(len, position);
    }
    len
}

// https://en.wikipedia.org/wiki/MIDI_tuning_standard
fn note_to_frequency(d: u32) -> u32 {
    (2.0f32.powf((d as f32 - 69.0) / 12.0) * 440.0) as u32
}
