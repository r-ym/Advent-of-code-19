use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = "input.txt";

    let input = File::open(file).unwrap();
    let csv = BufReader::new(input);

    let mut v: Vec<usize> = Vec::new();

    for line in csv.lines() {
        let mut vecc: Vec<usize> = line
            .unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        v.append(&mut vecc);
    }

    let mut vex = v.clone();

    let mut i: usize = 0;
    loop {
        match vex[i] {
            1 => {
                let i1 = vex[i + 1];
                let i2 = vex[i + 2];
                let i3 = vex[i + 3];
                let val = vex[i1] + vex[i2];
                vex[i3] = val;
                i += 4;
            }
            2 => {
                let i1 = vex[i + 1];
                let i2 = vex[i + 2];
                let i3 = vex[i + 3];
                let val = vex[i1] * vex[i2];
                vex[i3] = val;
                i += 4;
            }
            99 => break,
            _ => continue,
        }
    }

    println!("Part1 answer: {}",vex[0]);

    for noun in 0..99 {
        for verb in 0..99 {
            let input = File::open(file).unwrap();
            let csv = BufReader::new(input);
            let mut vex = v.clone();
            vex[1] = noun;
            vex[2] = verb;
            let mut i: usize = 0;
            loop {
                match vex[i] {
                    1 => {
                        let i1 = vex[i + 1];
                        let i2 = vex[i + 2];
                        let i3 = vex[i + 3];
                        let val = vex[i1] + vex[i2];
                        vex[i3] = val;
                        i += 4;
                    }
                    2 => {
                        let i1 = vex[i + 1];
                        let i2 = vex[i + 2];
                        let i3 = vex[i + 3];
                        let val = vex[i1] * vex[i2];
                        vex[i3] = val;
                        i += 4;
                    }
                    99 => break,
                    _ => continue,
                }
            }
            if vex[0] == 19690720 {
                println!("Part2 answer: {}",(noun*100+verb));
                break;
            }
        }
    }
}
