use std::{sync::atomic::{AtomicUsize, Ordering}, usize, time::{Instant, Duration}};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static VALID_COUNT: AtomicUsize = AtomicUsize::new(0);

static VALID_INTERVALS: [i32; 6] = [1, 2, 5, 7, 10, 11];
static LOOP_LENGTH: usize = 8;
static WINDOWS: [usize; 3] = [2, 4, 5];
//static WINDOWS: [usize; 1] = [5];

// GUT!
// static VALID_INTERVALS: [i32; 6] = [1, 2, 5, 7, 10, 11];
// static LOOP_LENGTH: usize = 6;
// static WINDOWS: [usize; 1] = [5];

fn main() {
    println!("Generating sequences");

    let root = vec![0];
    let start = Instant::now();

    fn calculate_next(tones: &[i32], iteration: usize) {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        // let durchlauf =CALL_COUNT.load(Ordering::SeqCst);
        // if durchlauf % 100 == 0 {
        //     println!("Durchlauf: {}, Iteration: {}", durchlauf, iteration);
        // }
        
        if iteration == LOOP_LENGTH {
            if check_all_rules(tones) {
                dump("☑", tones);
                VALID_COUNT.fetch_add(1, Ordering::SeqCst);                    
            } else {
                // dump("☒", tones);
            }

            return
        }

        for interval in -11..11 {
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
        "{} valid sequences found in {:?} time",
        VALID_COUNT.load(Ordering::SeqCst),
        duration
    )
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
    some_windows_are_loops(seq, &WINDOWS)
    //&& counterpoint_1(seq)
}

// Intervall mit Richtung (Vorzeichen)
fn interval(first: i32, second: i32) -> i32 {
    first - second
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
        .map(|size| ring.windows(size))
        .map(|mut w| w.all(x_is_loop))
        .all(|result| result);

    // if !result {
    //     println!("all_windows_are_loops failed: tones {:?}, min {}, max {}", &tones, min, max);
    //     // dump("->", tones)
    // }

    result
}

fn some_windows_are_loops(seq: &[i32], windows: &[usize]) -> bool {
    let mut ring = vec![];
    // max window muss kleiner gleich tones.len() sein.
    // falls window größer ist, dann ist das equivalent zu max % tones.len()
    ring.extend_from_slice(seq);
    ring.extend_from_slice(seq);

    let result = windows.into_iter()
        .map(|size| ring.windows(*size))
        .map(|mut w| w.all(x_is_loop))
        .all(|result| result);

    // if !result {
    //     println!("all_windows_are_loops failed: tones {:?}, min {}, max {}", &tones, min, max);
    //     // dump("->", tones)
    // }

    result
}

/// Kontrapunktregel: Sprünge müssen mit Schritten in Gegenbewegung ausgeglichen werden
fn counterpoint_1(seq: &[i32]) -> bool {
    let mut intervals = vec![];

    for i in 0..(seq.len() - 1) {
        intervals.push(interval(seq[i], seq[i + 1]));
    }
    
    intervals.push(interval(seq[seq.len() - 1], seq[0]));

    intervals.windows(2).all(|pair| {
        assert!(pair.len() == 2);
        // Erstes Intervall ist ein Sprung (i > 2)
        if pair[0].abs() > 2 {
            // Zweites Intervall ist ein Schritt (i < 3) (besser 0 < i < 3)
            // Und das erste und zweite Intervall haben unterschiedliche Richtungen
            // (signum(i) * signum (j) == -1)
            return (pair[1].abs() < 3) && (pair[0].signum() * pair[1].signum() == -1)
        } else {
            true
        }
    });

    false
}

#[cfg(test)]
mod tests {
    use crate::{all_windows_are_loops, x_is_loop};

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

        let examples = [[0, -5], [-5, -7]];
        for example in examples {
            assert!(x_is_loop(&example), "for example {:?}", &example)
        }
    }
}
 