use std::io::{BufRead, BufReader};

fn go(x: usize, y: usize, to: char) -> Option<(usize, usize)> {
    if x == 0 && to == 'L' {
        return None;
    }
    if x == 6 && to == 'R' {
        return None;
    }
    if y == 0 && to == 'U' {
        return None;
    }
    if y == 6 && to == 'D' {
        return None;
    }
    let next_x = match to {
        'U' | 'D' => x,
        'L' => x - 1,
        'R' => x + 1,
        _ => unreachable!(),
    };
    let next_y = match to {
        'L' | 'R' => y,
        'U' => y - 1,
        'D' => y + 1,
        _ => unreachable!(),
    };
    Some((next_x, next_y))
}

fn dfs(seq: &Vec<char>, map: &mut [[bool; 7]; 7], i: i32, x: usize, y: usize) -> usize {
    if x == 0 && y == 6 {
        if i == 47 {
            return 1;
        }
        return 0;
    }
    if i == 47 {
        return 0;
    }
    let mut count = 0;
    let mut tos = vec![];
    let c = seq[(i + 1) as usize];
    for to in ['U', 'D', 'L', 'R'] {
        if c != '?' && c != to {
            continue;
        }
        let Some((next_x, next_y)) = go(x, y, to) else {
            continue;
        };
        if map[next_x][next_y] {
            continue;
        }
        tos.push(to);
    }
    if tos.len() == 2 {
        let dirs = (tos[0], tos[1]);
        if dirs == ('U', 'D') || dirs == ('L', 'R') {
            return 0;
        }
    }
    for to in tos {
        let (next_x, next_y) = go(x, y, to).unwrap();
        map[next_x][next_y] = true;
        count += dfs(seq, map, i + 1, next_x, next_y);
        map[next_x][next_y] = false;
    }
    count
}

fn solution(input: &str) -> usize {
    let mut map = [[false; 7]; 7];
    map[0][0] = true;

    dfs(&input.chars().collect(), &mut map, -1, 0, 0)
}

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let answer = solution(&line);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_paths() {
        assert_eq!(
            solution("????????????????????????????????????????????????"),
            88418
        );
    }

    #[test]
    fn test_given_example() {
        assert_eq!(
            solution("??????R??????U??????????????????????????LD????D?"),
            201
        );
    }

    #[test]
    fn test_case_20() {
        assert_eq!(
            solution("???????????????????????????????????????????????D"),
            45647
        );
    }
}
