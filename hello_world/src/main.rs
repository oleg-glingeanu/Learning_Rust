use rand::Rng;

#[derive(Debug)]
enum EventType {
    Mens,
    Womens,
}

struct LongJumper {
    name: String,
    event_type: EventType,
    personal_best: i32,
    seasonal_best: i32,
    num_jumps: u32,
}

impl LongJumper {
    fn new(name: &str, event_type: EventType, personal_best: i32) -> Self {
        LongJumper {
            name: name.to_owned(),
            event_type: event_type,
            personal_best: personal_best,
            seasonal_best: 0,
            num_jumps: 0,
        }
    }

    fn record_jump(&mut self, jump: i32) {
        self.num_jumps += 1;
        if jump > self.personal_best {
            self.personal_best = jump;
        }
        if jump > self.seasonal_best {
            self.personal_best = jump;
        }
    }
}

struct Stats;

impl Stats {
    fn record_jump(long_jumper: &mut LongJumper, jump: i32) {
        long_jumper.record_jump(jump)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    println!("============ START ============");

    let mut jumper1: LongJumper = LongJumper::new("Oleg", EventType::Mens, rand.gen_range(0..4));
    let mut jumper2: LongJumper = LongJumper::new("Alice", EventType::Womens, rand.gen_range(0..4));
    let mut jumper3: LongJumper = LongJumper::new("John", EventType::Mens, rand.gen_range(0..4));

    Stats::record_jump(&mut jumper1, 10);
    Stats::record_jump(&mut jumper1, 7);
    Stats::record_jump(&mut jumper2, 11);
    Stats::record_jump(&mut jumper2, 6);
    Stats::record_jump(&mut jumper3, 1);
    Stats::record_jump(&mut jumper3, 10);

    let jumpers: Vec<LongJumper> = vec![jumper1, jumper2, jumper3];
    let personal_best_jumpers: Vec<&LongJumper> = jumpers
        .iter()
        .filter(|jumper| jumper.seasonal_best == jumper.seasonal_best)
        .collect();

    for jumper in personal_best_jumpers {
        println!(
            "Name: {}, Event Type: {:?}, Personal Best: {}, Season Best: {}, Number of Jumps: {}",
            jumper.name,
            jumper.event_type,
            jumper.personal_best,
            jumper.seasonal_best,
            jumper.num_jumps
        );
    }
}
