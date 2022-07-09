use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let mut benchmark = Benchmark::new();
    for _ in 0..10000 {
        // Increment number of simulation counter by 1
        benchmark.number_of_games += 1;
        let doors: Vec<Door> = Door::random_door_options();

        // Choose a random door as our gues
        let choosen_door = (0..3).choose(&mut rng).unwrap();

        // Calculate our correct guessing chance before revelaing a door
        if doors[choosen_door].has_prize == true {
            benchmark.correct_guesses_before_changing += 1;
        }

        // Find which doors donot have prize and isn't our chosen door
        let mut not_prize_doors: Vec<i32> = Vec::new();
        for i in &doors {
            if i.has_prize == false && i.door_number != choosen_door as i32 + 1 {
                not_prize_doors.push(i.door_number - 1);
            }
        }

        //Select and  Open a random not prize door
        let random_door = not_prize_doors.choose(&mut rng).unwrap();

        // Calculate our correct guessing chance after revelaing a door
        for i in doors {
            if i.door_number != *random_door + 1
                && i.door_number != choosen_door as i32 + 1
                && i.has_prize == true
            {
                benchmark.correct_guesses_after_changing += 1;

                break;
            }
        }
    }
    println!("Number of games {}", benchmark.number_of_games);
    println!(
        "Correct guess before changing {:.5}",
        benchmark.correct_guesses_before_changing as f32 / benchmark.number_of_games as f32
    );
    println!(
        "Correct guess before after {}",
        benchmark.correct_guesses_after_changing as f32 / benchmark.number_of_games as f32
    );
}

struct Door {
    has_prize: bool,
    door_number: i32,
    has_opened: bool,
}

impl Door {
    fn random_door_options() -> Vec<Door> {
        let mut rng = rand::thread_rng();

        let mut y: Vec<Door> = Vec::new();

        for i in 1..=3 {
            y.push(Door {
                has_prize: false,
                door_number: i,
                has_opened: false,
            })
        }
        y[(0..3).choose(&mut rng).unwrap()].has_prize = true;

        y
    }
}

struct Benchmark {
    number_of_games: i32,
    correct_guesses_before_changing: i32,
    correct_guesses_after_changing: i32,
}
impl Benchmark {
    fn new() -> Benchmark {
        Benchmark {
            number_of_games: 0,
            correct_guesses_before_changing: 0,
            correct_guesses_after_changing: 0,
        }
    }
}
