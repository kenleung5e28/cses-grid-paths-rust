use std::io::{BufRead, BufReader};

fn solution(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let a: i32 = split.next().unwrap().parse().unwrap();
    let b: i32 = split.next().unwrap().parse().unwrap();
    let answer = solution(a, b);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(3, 4), 7);
    }
}
