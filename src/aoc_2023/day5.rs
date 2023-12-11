use std::fs::read_to_string;
use std::str::Lines;
use std::io::Result;
use std::ops::Range;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/aoc_2023/day5/inputs.txt";

#[derive(Debug)]
struct Almanach {
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    _name: String,
    ranges: Vec<SeedRange>
}

#[derive(Debug)]
struct SeedRange {
    destination_start: u64,
    source_start: u64,
    step_range: u64,
}

impl Almanach {
    fn push_range_map(&mut self, seed_range: SeedRange) {
        let index_map = self.maps.len()-1;
        self.maps[index_map].push_range(seed_range);
    }
}

impl Map {
    fn push_range(&mut self, seed_range: SeedRange) {
        self.ranges.push(seed_range);
    }
}

pub fn day5_main() -> Result<(u64, u64)> {
    let (seeds, almanach) = parse_file( read_to_string(FILE_PATH)?.lines());
    let mut location_e1 = u64::MAX;
    let mut location_e2 = u64::MAX;
    let mut seeds_e2 = vec![];

    for (indx, _) in seeds.iter().enumerate() {
        if indx != seeds.len()-1 && indx % 2 == 0 {seeds_e2.push(Range{start: seeds[indx], end: seeds[indx] + seeds[indx+1]})}
    }

    let now = Instant::now();
    for range in &seeds_e2 {
        for seed in range.clone() {
            let mut location = seed;
            for map in &almanach.maps {
                location = search_seed_in_map(location, map);
            }

            if location < location_e2 {location_e2 =  location}
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    for seed in seeds {
        let mut location = seed;
        for map in &almanach.maps {
            location = search_seed_in_map(location, map);
        }

        if location < location_e1 {location_e1 =  location}
    }

    Ok((location_e1, location_e2))
}

/*Return (seeds to find, array of Map*/
fn parse_file(lines: Lines) -> (Vec<u64>, Almanach) {
    let mut almanach = Almanach{maps: vec![]};
    let mut seeds = vec![];

    for line in lines {
        if line == "" || line == " " {
            continue;
        } else if line.contains("seeds") {
            seeds = extract_seeds(line)
        } else if line.contains("map") {
            almanach.maps.push(Map {_name : line.split_whitespace().next().unwrap_or(" ").to_string(), ranges: vec![]});
        } else {
            almanach.push_range_map(extract_range(line));
        }
    }

    (seeds, almanach)
}

fn extract_seeds(line:&str) -> Vec<u64> {
    line.split(':').last().unwrap().trim().split_whitespace().map(|number| number.parse::<u64>().unwrap()).collect()
}

fn extract_range(line:&str) -> SeedRange {
    let mut destination_range = 0;
    let mut source_range = 0;
    let mut step_range = 0;

    for (indx, number) in line.trim().split_whitespace().enumerate() {
        match indx {
            0 => destination_range = number.parse::<u64>().unwrap(),
            1 => source_range = number.parse::<u64>().unwrap(),
            2 => step_range = number.parse::<u64>().unwrap(),
            _ => {}
        }
    }

    SeedRange { destination_start: destination_range, source_start: source_range, step_range}
}

fn search_seed_in_map(seed:u64, map:&Map) -> u64{
    let mut location = seed;

    for range in &map.ranges {
        if location >= range.source_start && location < range.source_start + range.step_range {
            location = range.destination_start + (location - range.source_start);
            break;
        }
    }

    location
}