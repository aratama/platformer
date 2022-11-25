use crate::sound::{play, Sound};

pub fn play_jump_se() {
    play(Sound {
        freq1: 490,
        freq2: 750,
        attack: 0,
        decay: 0,
        sustain: 0,
        release: 31,
        volume: 18,
        channel: 0,
        mode: 1,
    })
}

pub fn play_smash_se() {
    play(Sound {
        freq1: 140,
        freq2: 70,
        attack: 0,
        decay: 0,
        sustain: 0,
        release: 8,
        volume: 50,
        channel: 3,
        mode: 0,
    })
}
