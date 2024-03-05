use std::collections::HashMap;

use num_complex::Complex;

advent_of_code::solution!(18);

fn solve_2(instructions: &[(String, f64)], dirs: &HashMap<String, Complex<f64>>) -> f64 {
    let mut vs: Vec<Complex<f64>> = Vec::new();
    for (dir, dist) in instructions {
        let v = dirs.get(dir).unwrap() * dist;
        vs.push(v);
    }

    let mut sum_r: f64 = 0.0;
    let mut sum_i: f64 = 0.0;
    for v in &mut vs {
        sum_r += v.re;
        v.re = sum_r;
        sum_i += v.im;
        v.im = sum_i;
    }

    0.0
}

fn solve(instructions: &[(String, f64)], dirs: &HashMap<String, Complex<f64>>) -> f64 {
    let mut vs: Vec<Complex<f64>> = Vec::new();
    for (dir, dist) in instructions {
        let v = dirs.get(dir).unwrap() * dist;
        vs.push(v);
    }

    let mut sum_r: f64 = 0.0;
    let mut vertical_distance = 0.0;
    for v in &vs {
        sum_r += v.re;
        vertical_distance += sum_r * v.im;
    }

    let horizontal_distance = vs
        .iter()
        .map(|v: &Complex<f64>| {
            println!("{}", v.norm().to_string());
            v.norm()
        })
        .sum::<f64>()
        / 2.0
        + 1.0;

    println!(
        "Vertical distance: {},Horizontal distance: {}",
        vertical_distance.abs(),
        horizontal_distance
    );
    vertical_distance.abs() + horizontal_distance
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ws: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        ws.push(line.split_whitespace().map(|s| s.to_string()).collect());
    }

    let mut dirs: HashMap<String, Complex<f64>> = HashMap::new();
    dirs.insert("R".to_string(), Complex::new(0.0, 1.0));
    dirs.insert("L".to_string(), Complex::new(0.0, -1.0));
    dirs.insert("U".to_string(), Complex::new(-1.0, 0.0));
    dirs.insert("D".to_string(), Complex::new(1.0, 0.0));

    // Part 1
    let instructions: Vec<(String, f64)> = ws
        .iter()
        .map(|w| (w[0].clone(), w[1].parse::<f64>().unwrap()))
        .collect();
    let total_distance = solve(&instructions, &dirs);
    Some(total_distance as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ws: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        ws.push(line.split_whitespace().map(|s| s.to_string()).collect());
    }

    let mut dirs: HashMap<String, Complex<f64>> = HashMap::new();
    dirs.insert("R".to_string(), Complex::new(0.0, 1.0));
    dirs.insert("L".to_string(), Complex::new(0.0, -1.0));
    dirs.insert("U".to_string(), Complex::new(-1.0, 0.0));
    dirs.insert("D".to_string(), Complex::new(1.0, 0.0));

    // Part 2
    let mut table: HashMap<String, String> = HashMap::new();
    table.insert("0".to_string(), "R".to_string());
    table.insert("1".to_string(), "D".to_string());
    table.insert("2".to_string(), "L".to_string());
    table.insert("3".to_string(), "U".to_string());
    let instructions: Vec<(String, f64)> = ws
        .iter()
        .map(|w| {
            (
                table[&w[2][7..8]].to_string(),
                i64::from_str_radix(&w[2][2..7], 16).unwrap() as f64,
            )
        })
        .collect();
    let total_distance = solve(&instructions, &dirs);
    Some(total_distance as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
