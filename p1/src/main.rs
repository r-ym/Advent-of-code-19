use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_classic(mass: isize) -> isize {
    mass / 3 - 2
}

fn fuel_recur(mass: isize) -> isize {
    let amount = (mass / 3) - 2;
    if amount <= 0 {
        return 0;
    } else {
        return amount + fuel_recur(amount);
    }
}

fn main() {
    let file = "input.txt";
    let input = File::open(file).unwrap();
    let buffered = BufReader::new(input);
    let mut fuel: isize = 0;
    let mut fuel_rec: isize = 0;

    for line in buffered.lines() {
        let mass = line.unwrap().parse::<isize>().unwrap();
        fuel = fuel + fuel_classic(mass);
        fuel_rec = fuel_rec + fuel_recur(mass);
    }
    println!("Part1 = {}", fuel);
    println!("Part1 = {}", fuel_rec);
}