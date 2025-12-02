use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("../../d/2")?;
    let data = data.trim_end();
    let mut bad: Vec<i64> = Vec::new();

    for chunk in data.split(',') {
        let parts: Vec<&str> = chunk.split('-').collect();
        if let [start, end] = parts.as_slice() {
            let low: i64 = start.trim().parse()?;
            let high: i64 = end.trim().parse()?;

            for num in low..=high {
                let num_str = num.to_string();
                let length = num_str.len();

                for pattern_len in 1..=length / 2 {
                    if length % pattern_len == 0 {
                        let pattern = &num_str[..pattern_len];
                        let repetitions = length / pattern_len;
                        if pattern.repeat(repetitions) == num_str {
                            bad.push(num);
                            break;
                        }
                    }
                }
            }
        }
    }

    let sum: i64 = bad.iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}
