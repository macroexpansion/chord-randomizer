use rand::{seq::SliceRandom, thread_rng};
use std::{thread, time};

fn main() {
    loop {
        let mut chords = [
            "C", "Cm", "C#/Db", "C#m/Dbm", "D", "Dm", "D#/Eb", "D#m/Ebm", "E", "Em", "F", "Fm",
            "F#/Gb", "F#m/Gbm", "G", "Gm", "G#/Ab", "G#m/Abm", "A", "Am", "A#/Bb", "A#m/Bbm", "B",
            "Bm",
        ];
        chords.shuffle(&mut thread_rng());
        for chord in chords {
            println!("{}", chord);
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
