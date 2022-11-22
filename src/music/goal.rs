use crate::sound::{Music, Track};

pub static GOAL_BGM_SCORE: &Music = &Music {
    unit: 10,
    tracks: [
        Track {
            volume: 60,
            notes: &[],
        },
        Track {
            volume: 20,
            notes: &[
                (00, 2, 0),
                // 1
                (60, 1, 0),
                (62, 1, 0),
                (64, 1, 0),
                (65, 1, 0),
                // 2
                (67, 2, 1),
                (67, 1, 0),
                (67, 1, 1),
                // 3
                (69, 2, 1),
                (69, 1, 0),
                (69, 1, 1),
                // 4
                (71, 10, 0),
            ],
        },
        Track {
            volume: 30,
            notes: &[
                (00, 2, 0),
                // 1
                (00, 4, 0),
                // 2
                (60, 2, 1),
                (60, 1, 0),
                (60, 1, 1),
                // 3
                (62, 2, 1),
                (62, 1, 0),
                (62, 1, 1),
                // 4
                (64, 10, 0),
            ],
        },
        Track {
            volume: 100,
            notes: &[],
        },
    ],
};
