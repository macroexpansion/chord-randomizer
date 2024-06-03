use rand::{seq::SliceRandom, thread_rng};
use std::{thread, time};

fn main() {
    loop {
        let chords = [
            "C", "Cm", "C#/Db", "C#m/Dbm", "D", "Dm", "D#/Eb", "D#m/Ebm", "E", "Em", "F", "Fm",
            "F#/Gb", "F#m/Gbm", "G", "Gm", "G#/Ab", "G#m/Abm", "A", "Am", "A#/Bb", "A#m/Bbm", "B",
            "Bm",
        ];
        let mut choices: Vec<_> = (0..chords.len()).collect();
        choices.shuffle(&mut thread_rng());
        for choice in choices {
            println!("{}", chords[choice]);
            thread::sleep(time::Duration::from_secs(8));
        }
    }
}
