struct Race {
    time: i32,
    record: i32,
}

fn calc_winning(race: Race) -> Vec<i32> {
    let mut winning_times = vec![];
    for i in 0..race.time {
        let distance = i * (race.time - i);
        if distance > race.record {
            winning_times.push(i);
        }
    }
    winning_times
}

struct RaceV2 {
    time: u64,
    record: u64,
}

fn calc_winning_v2(race: RaceV2) -> Vec<u64> {
    let mut winning_times = vec![];
    for i in 0..race.time {
        let distance = i.checked_mul(race.time - i).unwrap_or(0); // Use checked_mul to prevent overflow
        if distance > race.record { 
            winning_times.push(i); 
        }
    }
    winning_times
}

fn main() {
    let race1 = Race {
        time: 50,
        record: 242,
    };
    let race2 = Race {
        time: 74,
        record: 1017
    };
    let race3 = Race {
        time: 86,
        record: 1691,
    };
    let race4 = Race {
        time: 85,
        record: 1252,
    };

    let mut races: Vec<Race> = vec![];
    races.push(race1);
    races.push(race2);
    races.push(race3);
    races.push(race4);
    
    let mut product = 1;

    let final_race = RaceV2 {
        time: 50748685,
        record: 242101716911252,
    };
    
    for race in races {
        let result = calc_winning(race);
        product = product * result.len();
        println!("Product: {}", product);
    }
    let final_result = calc_winning_v2(final_race);
    println!("final _race: {}", final_result.len());

}

