use droprate::{FairlyRandomTable, ProbabilityTable, RandomTable};

use std::collections::HashMap;

fn gen_string_table() -> RandomTable<String> {
    let outcomes: HashMap<String, f64> = [
        (String::from("first"), 1f64),
        (String::from("second"), 1f64),
        (String::from("third"), 1f64),
    ]
    .iter()
    .cloned()
    .collect();
    RandomTable::from_map(outcomes)
}

fn gen_fairly_random_string_table() -> FairlyRandomTable<String> {
    let outcomes: HashMap<String, f64> = [
        (String::from("first"), 1f64),
        (String::from("second"), 1f64),
        (String::from("third"), 1f64),
    ]
    .iter()
    .cloned()
    .collect();
    FairlyRandomTable::from_map(outcomes)
}

#[test]
fn empty_table() {
    let table = RandomTable::<String>::new();
    assert_eq!(table.count(), 0);
}

#[test]
fn populated_table() {
    let outcomes: HashMap<String, f64> = [
        (String::from("first"), 1f64),
        (String::from("second"), 1f64),
        (String::from("third"), 1f64),
    ]
    .iter()
    .cloned()
    .collect();

    let mut table = RandomTable::from_map(outcomes);
    assert_eq!(table.count(), 3);

    let fail_string = String::from("fail");

    println!("trial 1: {}", table.random().unwrap_or(fail_string.clone()));
    println!("trial 2: {}", table.random().unwrap_or(fail_string.clone()));
    println!("trial 3: {}", table.random().unwrap_or(fail_string.clone()));
    println!("trial 4: {}", table.random().unwrap_or(fail_string.clone()));
    println!("trial 5: {}", table.random().unwrap_or(fail_string.clone()));
    println!("trial 6: {}", table.random().unwrap_or(fail_string.clone()));
}

#[test]
fn seems_always_valid() {
    let mut table = gen_string_table();
    for _ in 0..10000 {
        assert!(table.random().is_ok());
    }
}

// TODO: report longest distance between two items of the same
// TODO: report longest streak of same items in a row

fn random_odds(table: &mut ProbabilityTable<String>, num_cycles: u32) {
    let keys = table.keys();
    let mut stats = HashMap::<&String, u64>::new();

    for k in &keys {
        stats.insert(k, 0);
    }

    for _ in 0..num_cycles {
        let r = table.random().unwrap();
        let val = stats.get_mut(&r).unwrap();
        *val += 1;
    }

    for pair in stats {
        println!("{}: {}", pair.0, pair.1 as f64 / num_cycles as f64);
    }
}

#[test]
fn report_random_probability() {
    let num_cycles = 10000u32;
    random_odds(&mut gen_string_table(), num_cycles);
}

#[test]
fn report_reactive_probability() {
    let num_cycles = 10000u32;
    random_odds(&mut gen_fairly_random_string_table(), num_cycles);
}
