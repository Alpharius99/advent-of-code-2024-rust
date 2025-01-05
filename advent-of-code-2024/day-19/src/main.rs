#![warn(clippy::all)]
use std::time::Instant;
use utils::get_file_contents;

struct Cfg {}

#[cfg(feature = "debug")]
impl Cfg {
    const FILE_PATH: &'static str = "sample";
}

#[cfg(not(feature = "debug"))]
impl Cfg {
    const FILE_PATH: &'static str = "input";
}

fn bench<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let t0 = Instant::now();
    let result = f(); // Call the function and store the result
    println!("time used: {:?}", Instant::now().duration_since(t0));
    result // Return the result of the function
}

fn main() {
    let (patterns, designs) = bench(|| preamble(Cfg::FILE_PATH));
    let (p1, p2) = bench(|| count(&patterns, &designs));
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn preamble(path: &str) -> (Vec<String>, Vec<String>) {
    let file_content: String = get_file_contents(path);
    let patterns: Vec<String> = file_content
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs: Vec<String> = file_content
        .lines()
        .skip(2)
        .map(|s| s.to_string())
        .collect();

    (patterns, designs)
}

fn count(patterns: &Vec<String>, designs: &Vec<String>) -> (usize, usize) {
    let mut count = 0;
    let mut count_all = 0;

    for design in designs {
        let all = count_combinations(design, patterns);
        if all > 0 {
            count += 1;
            count_all += all;
        }
    }

    (count, count_all)
}

fn count_combinations(target: &str, substrings: &Vec<String>) -> usize {
    let target_len = target.len();
    let mut dp = vec![0; target_len + 1];
    dp[0] = 1;

    for i in 0..=target_len {
        if dp[i] > 0 {
            for substr in substrings {
                if target[i..].starts_with(substr) {
                    dp[i + substr.len()] += dp[i];
                }
            }
        }
    }

    dp[target_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let (patterns, designs) = preamble("sample");
        let (p1, _) = count(&patterns, &designs);
        assert_eq!(p1, 6);
    }

    #[test]
    fn test_input_part_one() {
        let (patterns, designs) = preamble("input");
        let (p1, _) = count(&patterns, &designs);
        assert_eq!(p1, 369);
    }

    #[test]
    fn test_sample_part_two() {
        let (patterns, designs) = preamble("sample");
        let (_, p2) = count(&patterns, &designs);
        assert_eq!(p2, 16);
    }

    #[test]
    fn test_input_part_two() {
        let (patterns, designs) = preamble("input");
        let (_, p2) = count(&patterns, &designs);
        assert_eq!(p2, 761826581538190);
    }
}
