use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static VALID_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("Hello, world!");

    // kleine Sekunde, große Sekunde, reine Quarte
    let allowed_intervals = [1, 2, 5];

    let number_of_notes = 8;

    let root = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut current_iteration = 0;


    fn calculate_next(tones: &[i32; 8], iteration: usize) {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    // kleine Sekunde, große Sekunde, reine Quarte
    let allowed_intervals = [1, 2, 5];

        for sign in [-1, 1] {
            for interval in allowed_intervals {
                let previous_tone = tones[iteration - 1];
                let x: i32 = previous_tone +  (sign * interval);
                let next_note = x.rem_euclid(12);

                // print_is_valid_interval(previous_tone, next_note);
                
                // neue note in einen klon der tonreihe einfügen.
                let mut klon = tones.clone();
                klon[iteration] = next_note;

                // prüfen, ob bis hier hin gültig
                // zum beispiel keine Töne doppelt.
                
                if check(&klon, iteration) && check_last(&klon, iteration) && iteration < 7 {
                    // dump("☑", &klon, iteration + 1);
                    calculate_next(&klon, iteration + 1)
                    // beginne nächste iteration mit diesem klon.
                } else {
                    // dump("☒", &klon, iteration + 1)
                }

                if iteration == 7 {
                    if check(&klon, iteration) && check_last(&klon, iteration){
                        dump("☑", &klon, iteration + 1);
                        VALID_COUNT.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }
    }

    calculate_next(&root, 1);
    println!("Valid sequences found: {}", VALID_COUNT.load(Ordering::SeqCst) )
}

fn print_is_valid_interval(first: i32, second: i32)  {
    let interval = (first - second).abs();
    println!("Interval {} - {}, differenz: {}", tonhöhen[first as usize], tonhöhen[second as usize], interval);
    if !(interval == 2 || interval == 1 || interval == 5 || interval == 11 || interval == 10 || interval == 7) {
        panic!()
    }
}


const tonhöhen: [&str; 12] = ["C", "D♭", "D", "E♭", "E", "F", "G♭", "G", "A♭", "A", "B", "H"];

fn dump<'a, I>(checkmark: &str, tones: I, iteration: usize) where I: IntoIterator<Item = &'a i32>  {

    

    let output = tones.into_iter()
    .take(iteration)
    .map(|tone| tonhöhen[*tone as usize].to_string())
    // .map(|item| format!("{}", item))
    .collect::<Vec<String>>()
    .join(" ");
    
    println!("{} {}: {}", 
    checkmark,
    CALL_COUNT.load(Ordering::SeqCst),
    output)
}

// Liefert true, wenn keine duplikate enthalten sind.
fn check<'a, I>(tones: I, iteration: usize) -> bool where I: IntoIterator<Item = &'a i32> {
    let mut histogram = [0; 12];
    for note in tones.into_iter().take(iteration) {
        
        histogram[*note as usize] = histogram[*note as usize] + 1
    }


    histogram.into_iter().all(|item| item < 2)
}

// Liefert true, wenn keine duplikate enthalten sind.
fn check_last<'a, I>(tones: I, iteration: usize) -> bool where I: IntoIterator<Item = &'a i32> {
    if iteration == 7 {
        *tones.into_iter().last().unwrap() == 0
    } else {
        true
    }
}