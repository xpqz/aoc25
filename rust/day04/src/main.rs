use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn read_grid(path: &str) -> std::io::Result<Grid<char>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut width = 0usize;
    let mut height = 0usize;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            width = line.len();
        }

        data.extend(line.chars());
        height += 1;
    }

    Ok(Grid {
        data,
        width,
        height,
    })
}

fn pred(grid: &Grid<char>, x: usize, y: usize) -> bool {
    if grid.data[grid.idx(x, y)] != '@' {
        return false;
    }

    let mut count = 0;
    let width = grid.width as isize;
    let height = grid.height as isize;
    let x = x as isize;
    let y = y as isize;

    for dy in -1..=1 {
        for dx in -1..=1 {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                continue;
            }

            let idx = grid.idx(nx as usize, ny as usize);
            if grid.data[idx] != '@' {
                continue;
            }

            count += 1;
            if count > 4 {
                return false;
            }
        }
    }

    true
}

fn apply<F>(grid: &Grid<char>, mut f: F) -> Grid<bool>
where
    F: FnMut(&Grid<char>, usize, usize) -> bool,
{
    let mut out = Grid {
        data: vec![false; grid.data.len()],
        width: grid.width,
        height: grid.height,
    };

    let w = grid.width;
    let h = grid.height;

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            out.data[idx] = f(grid, x, y);
        }
    }

    out
}

fn prune_until_stable(grid: &mut Grid<char>) -> usize {
    let mut iterations = 0;

    loop {
        let mask = apply(grid, pred);
        let mut changed = false;

        for (idx, &remove) in mask.data.iter().enumerate() {
            if remove && grid.data[idx] == '@' {
                grid.data[idx] = '.';
                changed = true;
            }
        }

        if !changed {
            break;
        }

        iterations += 1;
    }

    iterations
}

fn count_char(grid: &Grid<char>, target: char) -> usize {
    grid.data.iter().filter(|&&c| c == target).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = read_grid("../../d/4")?;

    let initial_at = count_char(&grid, '@');

    let _iterations = prune_until_stable(&mut grid);

    let final_at = count_char(&grid, '@');
    let removed = initial_at.saturating_sub(final_at);

    println!("removed {removed} '@' (initial {initial_at} -> final {final_at})");
    // println!("{grid}");

    Ok(())
}
