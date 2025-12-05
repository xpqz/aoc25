use range_collections::range_set::RangeSetRange;
use range_collections::RangeSet2;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_data(path: &str) -> io::Result<(RangeSet2<i64>, Vec<i64>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ranges = RangeSet2::<i64>::empty();
    let mut cands = Vec::<i64>::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let parts: Vec<&str> = line.split('-').collect();

        if let [start, end] = parts.as_slice() {
            let low = match start.trim().parse::<i64>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let high = match end.trim().parse::<i64>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            ranges |= RangeSet2::from(low..(high + 1));
        } else {
            let cand = match parts[0].trim().parse::<i64>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            cands.push(cand);
        }
    }

    Ok((ranges, cands))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ranges, cands) = read_data("../../d/5")?;

    let count = cands.iter().filter(|c| ranges.contains(c)).count();
    println!("Part 1: {}", count);

    let total: i64 = ranges
        .iter()
        .map(|r| match r {
            RangeSetRange::Range(range) => range.end - range.start,
            _ => 0,
        })
        .sum();
    println!("Part 2: {}", total);

    Ok(())
}
