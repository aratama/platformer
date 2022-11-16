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

static mut bgm_music_count: u32 = 0;
static mut current_bgm: Option<&'static Music> = Option::None;

pub fn set_bgm(bgm: Option<&'static Music>) {
    unsafe {
        bgm_music_count = 0;
        current_bgm = bgm;
    }
}

pub fn update_bgm() {
    unsafe {
        if let Option::Some(m) = current_bgm {
            music(m, &mut bgm_music_count, 0, true);
        };
    }
}

pub fn music(music: &Music, music_count: &mut u32, pitch_offset: i32, loop_music: bool) {
    let current_position = if loop_music {
        *music_count % music_length(&music)
    } else {
        *music_count
    };

    for (channel, notes) in music.iter().enumerate() {
        let mut position = 0;
        for note in *notes {
            let [note_number, release] = *note;
            if position == current_position && note_number != 0 {
                let freq = note_to_frequency((note_number as i32 + pitch_offset) as u32);
                play(Sound {
                    freq1: freq,
                    freq2: freq,
                    attack: 0,
                    decay: 0,
                    sustain: release / 2,
                    release: release / 2,
                    volume: 50,
                    channel: channel as u32,
                    mode: 0,
                })
            }
            position += release;
        }
    }

    *music_count = *music_count + 1;
}

// [Pitch, Release]
type Note = [u32; 2];

type Track = &'static [Note];

type Music = [Track; 4];

fn music_length(music: &Music) -> u32 {
    let mut len: u32 = 0;
    for notes in music.iter() {
        let mut position = 0;
        for note in *notes {
            let [_, release] = *note;
            position += release;
        }
        len = u32::max(len, position);
    }
    len
}

pub static TITLE_BGM_SCORE: &Music = &[
    &[
        [00, 60],
        [60, 10],
        [62, 10],
        [64, 10],
        [65, 10],
        [67, 20],
        [00, 10],
        [67, 10],
        [67, 10],
        [00, 10],
        [69, 20],
        [00, 10],
        [69, 10],
        [69, 10],
        [00, 10],
        [71, 100],
    ],
    &[],
    &[
        [00, 100],
        [60, 20],
        [00, 10],
        [60, 10],
        [60, 10],
        [00, 10],
        [62, 20],
        [00, 10],
        [62, 10],
        [62, 10],
        [00, 10],
        [64, 100],
    ],
    &[],
];

// https://en.wikipedia.org/wiki/MIDI_tuning_standard
fn note_to_frequency(d: u32) -> u32 {
    (2.0f32.powf((d as f32 - 69.0) / 12.0) * 440.0) as u32
}
