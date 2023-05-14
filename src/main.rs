use std::{
    iter::{Cycle, Enumerate},
    slice::{Iter, Windows},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
    usize,
};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static VALID_COUNT: AtomicUsize = AtomicUsize::new(0);

static MELODIC_CONSONANTS: [i32; 8] = [0, 3, 4, 5, 7, 8, 9, 12];

// static VALID_INTERVALS: [i32; 6] = [1, 2, 5, 7, 10, 11];

// static LOOP_LENGTH: usize = 8;
// static WINDOWS: [usize; 3] = [2, 4, 6];

// GUT!
static VALID_INTERVALS: [i32; 6] = [1, 2, 5, 7, 10, 11];
// static LOOP_LENGTH: usize = 7;
// static WINDOWS: [usize; 2] = [4, 6];
// static WINDOWS2: [(usize, i32); 4] =[(7, -1), (6, -1), (4, -1), (3, -1)];
static WINDOWS2: [(usize, i32); 0] =[];

static LOOP_LENGTH: usize = 9;
// static WINDOWS2: [(usize, i32); 3] =[(2, -1), (4, -1), (6, -1)];
//  static WINDOWS2: [(usize, i32); 1] =[(2, -1)];


fn main() {
    println!("Generating sequences");

    let root = vec![0];
    let start = Instant::now();

    fn calculate_next(tones: &[i32], iteration: usize) {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        // let durchlauf =CALL_COUNT.load(Ordering::SeqCst);
        // if durchlauf % 1000 == 0 {
        //     println!("Durchlauf: {}, Iteration: {}", durchlauf, iteration);
        // }

        if iteration == LOOP_LENGTH {
            if check_all_rules(tones) {
                dump("☑", tones);
                VALID_COUNT.fetch_add(1, Ordering::SeqCst);
            } else {
                // dump("☒", tones);
            }

            return;
        }

        for interval in all_directions(&VALID_INTERVALS) {
            // for interval in -11..11 {
            let previous_tone = tones[iteration - 1];
            let next_note: i32 = previous_tone + interval;

            // neue note in einen klon der tonreihe einfügen.
            let mut klon = tones.to_vec();
            klon.push(next_note);

            // prüfen, ob bis hier hin gültig
            // zum beispiel keine Töne doppelt.
            if check_part(&klon) {
                calculate_next(&klon, iteration + 1)
            }
        }
    }

    calculate_next(&root[..], 1);

    let duration = start.elapsed();

    println!(
        "{} valid sequences found in {} generated sequences ({:?} time)",
        VALID_COUNT.load(Ordering::SeqCst),
        CALL_COUNT.load(Ordering::SeqCst),
        duration
    )
}

fn all_directions(intervalls: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    result.extend_from_slice(intervalls);

    let inverse = intervalls.into_iter().map(|i| -i).collect::<Vec<i32>>();
    result.extend_from_slice(&inverse);

    result
}

fn check_part(seq: &[i32]) -> bool {
    all_intervalls_are_valid(seq)
}

fn all_intervalls_are_valid(seq: &[i32]) -> bool {
    for i in 0..(seq.len() - 1) {
        let first = seq[i];
        let second = seq[i + 1];
        if !is_valid_intervall(first, second) {
            return false;
        }
    }

    true
}

fn check_all_rules(seq: &[i32]) -> bool {
    check_ambitus(seq)
        // && some_windows_are_loops(seq, &WINDOWS)
        && overlapping_windows_are_loops(seq, &WINDOWS2)
        && counterpoint_1(seq)
        && counterpoint_2(seq)
}

fn check_ambitus(seq: &[i32]) -> bool {
    for i in seq {
        // c1 - g2: 0..19
        if *i < 0 || *i > 19 {
            return false;
        }
    }

    true
}

// Intervall mit Richtung (Vorzeichen)
fn interval(first: i32, second: i32) -> i32 {
    // first - second
    second -first
}

fn is_valid_intervall(first: i32, second: i32) -> bool {
    let interval = interval(first, second).abs();
    VALID_INTERVALS.contains(&interval)
}

const tonhöhen: [&str; 12] = [
    "C", "D♭", "D", "E♭", "E", "F", "G♭", "G", "A♭", "A", "B", "H",
];

fn print_note(note: i32) -> &'static str {
    let normalized = note.rem_euclid(12);
    let index = normalized as usize;
    tonhöhen[index]
}

fn dump(checkmark: &str, tones: &[i32]) {
    let output = tones
        .into_iter()
        // .take(iteration)
        .map(|tone| print_note(*tone))
        .collect::<Vec<&str>>()
        .join(" ");

    println!(
        "{} {}: {} ({:?})",
        checkmark,
        CALL_COUNT.load(Ordering::SeqCst),
        output,
        tones.into_iter().collect::<Vec<&i32>>()
    )
}

fn x_is_loop(seq: &[i32]) -> bool {
    // jeweils zwei benachbarte Töne müssen den Regeln für
    // gültige Intervalle entsprechen. Eine Schleife existiert
    // genau dann, wenn auch der letzte und erste Ton diesen
    // Regeln entspricht.

    let mut l = vec![0; seq.len()];
    l.clone_from_slice(seq);
    l.push(seq[0]);

    for i in 0..(l.len() - 1) {
        if !is_valid_intervall(l[i], l[i + 1]) {
            return false;
        }
    }

    true
}

type WindowEnumeration = (usize, dyn Iterator<Item = i32>);

/// Prüft, ob alle Ausschnitte der Tonfolge Schleifen sind.
///
/// tones: tonfolge, die geprüft wird.
/// min: Länge des kleinsten Ausschnitts aus der Tonfolge
/// max: Länge des längsten Ausschnitts aus der Tonfolge
fn all_windows_are_loops(tones: &[i32], min: usize, max: usize) -> bool {
    let mut ring = vec![];
    // max window muss kleiner gleich tones.len() sein.
    // falls window größer ist, dann ist das equivalent zu max % tones.len()
    ring.extend_from_slice(tones);
    ring.extend_from_slice(tones);

    let result = (min..=max)
        .map(|size| ring.windows(size).enumerate())
        .map(|mut e| e.all(|i| x_is_loop(i.1)))
        .all(|result| result);

    // if !result {
    //     println!("all_windows_are_loops failed: tones {:?}, min {}, max {}", &tones, min, max);
    //     // dump("->", tones)
    // }

    result
}

struct SubLoop<'a> {
    ring: &'a [i32],
    ringer: Cycle<Iter<'a, i32>>,
    size: usize,
    windows: Enumerate<Windows<'a, i32>>,
}

impl<'a> SubLoop<'a> {
    fn new(ring: &'a [i32], size: usize) -> SubLoop<'a> {
        let ringer = ring.into_iter().cycle();
        let windows: std::iter::Enumerate<std::slice::Windows<i32>> =
            ring.windows(size).enumerate();
        SubLoop {
            ring,
            ringer,
            size,
            windows: windows,
        }
    }
}

fn some_windows_are_loops(seq: &[i32], windows: &[usize]) -> bool {
    let mut ring = vec![];
    // max window muss kleiner gleich tones.len() sein.
    // falls window größer ist, dann ist das equivalent zu max % tones.len()
    ring.extend_from_slice(seq);
    ring.extend_from_slice(seq);

    let result = windows
        .into_iter()
        .map(|size| ring.windows(*size))
        .map(|mut w| w.all(x_is_loop))
        .all(|result| result);

    // if !result {
    //     println!("all_windows_are_loops failed: tones {:?}, min {}, max {}", &tones, min, max);
    //     // dump("->", tones)
    // }

    result
}

/// Problem: 
/// * Teile der Tonfolge sollen alle Regeln erfüllen.
/// * Die Teiltonfolgen folgen auf anderen Tonfolgen.
///   * Der Übergang muss ebenfalls alle Regeln erfüllen.
/// 
/// Beispiel: Tonfolge C, G, F, H, C
/// Fenster: (3, -1)
/// => Untertonfolgen: C, G F; G F H; F, H, C; H, C, C
/// => Untertonfolgen geprüft mit Vorgänger:
/// 
/// (C) C, G, F
/// (C) G, F, H
/// (G) F, H, C
/// (F) H, C, C
/// (H) C, C, G
/// 
/// Fenster: (3, 1)
/// (G) C, G, F
/// (F) G, F, H
/// (H) F, H, C
/// (C) H, C, C
/// (C) C, C, G
///
/// usw.
fn overlapping_windows_are_loops(seq: &[i32], windows: &[(usize, i32)]) -> bool {

    let result = windows.iter()
        .map(| (size, back)| {
            (0..seq.len()).map(move |position| {
                let mut local_loop = vec![];
                let sequenzlänge = seq.len() as i32;
                let position_raw = position as i32 + *back;
                let previous_tone_position = (position as i32 + *back).rem_euclid(sequenzlänge);
                let previous_tone = seq[previous_tone_position as usize];
                local_loop.push(previous_tone);

                let cycle1 = seq.iter().cycle().skip(position).take(*size);
                local_loop.extend(cycle1);

                let cycle2 = seq.iter().cycle().skip(position).take(*size);
                local_loop.extend(cycle2);

                let next_tone_position = ((position + seq.len()) as i32).rem_euclid(seq.len() as i32);
                let next_tone = seq[next_tone_position as usize];

                let mut intervalls = local_loop.windows(2)
                    .map(|x| interval(x[0], x[1]).abs());

                let itcol = intervalls.collect::<Vec<i32>>();

                let result = itcol.iter().all(|i| VALID_INTERVALS.contains(&i));
                // assert!(result);
                result
            }).fold(true, |acc, x| acc && x)
        }).fold(true, |acc, x| acc && x);

    return result
}


/// Kontrapunktregel 1: Sprünge müssen mit Schritten in Gegenbewegung ausgeglichen werden
fn counterpoint_1(seq: &[i32]) -> bool {
    let mut intervals = vec![];

    for i in 0..(seq.len() - 1) {
        intervals.push(interval(seq[i], seq[i + 1]));
    }

    intervals.push(interval(seq[seq.len() - 1], seq[0]));
    intervals.push(interval(seq[0], seq[1]));

    let result = intervals.windows(2).all(|pair| {
        assert!(pair.len() == 2);
        // Erstes Intervall ist ein Sprung (i > 2)
        if pair[0].abs() > 2 {
            // Zweites Intervall ist ein Schritt (i < 3) (besser 0 < i < 3)
            // Und das erste und zweite Intervall haben unterschiedliche Richtungen
            // (signum(i) * signum (j) == -1)
            let ist_schritt = pair[1].abs() < 3;
            let ist_gegenbewegung = pair[0].signum() * pair[1].signum() == -1;
            return ist_schritt && ist_gegenbewegung;
        } else {
            true
        }
    });

    result
}

/// Kontrapunktregel 2: gleiche intervalle in gleiche richtung
fn counterpoint_2(seq: &[i32]) -> bool {
    let mut intervals = vec![];

    for i in 0..(seq.len() - 1) {
        intervals.push(interval(seq[i], seq[i + 1]));
    }

    intervals.push(interval(seq[seq.len() - 1], seq[0]));

    let result = intervals.windows(2).all(|pair| {
        assert!(pair.len() == 2);
        // Erstes Intervall ist ein Sprung (i > 2)
        if pair[0] == pair[1] {
            let summe = 2 * pair[0];
            MELODIC_CONSONANTS.contains(&summe)
        } else {
            true
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::{all_windows_are_loops, x_is_loop, overlapping_windows_are_loops, counterpoint_1};

    #[test]
    fn test_all_windows_are_valid() {
        let example = [0, -5, -6, -5];
        let result = all_windows_are_loops(&example, 2, 2);
        assert!(result)
    }

    #[test]
    fn test_all_windows_are_valid2() {
        let example = [0, -5, -7];
        let result = all_windows_are_loops(&example, 2, 2);
        assert!(result)
    }

    #[test]
    fn test_x_is_loop() {
        // let examples = [[0, -5], [-5, -7], [-7, 0]];

        let examples = [[0, 1], [0, 2]];
        for example in examples {
            assert!(x_is_loop(&example), "for example {:?}", &example)
        }
    }

    fn modulus(a: i32, b: i32) -> i32 {
        ((a % b) + b) % b
    }

    #[test]
    fn test_rest_division() {
        assert_eq!(0 % 12, 0);
        assert_eq!(14 % 12, 2);
        assert_eq!(modulus(-1, 12), 11);
        assert_eq!((-1i32).rem_euclid(12), 11);
    
    }

    #[test]
    fn test_overlap_windows() {
        let example = [0, 11, 9, 8, 1, 2];
        let windows = [(6, -1)];

        assert!(overlapping_windows_are_loops(&example, &windows))
    }

    #[test]
    fn test_counterpoint_1() {
        let example = [0, 2, 9, 7];
        assert!(counterpoint_1(&example))
    }
    
}
