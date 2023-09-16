use std::io::{BufRead, BufReader};

// fn count_map(map: &[[bool; 7]; 7]) -> usize {
//     let mut count = 0;
//     for i in 0..7 {
//         for j in 0..7 {
//             if map[i][j] {
//                 count += 1;
//             }
//         }
//     }
//     count
// }

fn go(x: i32, y: i32, to: char) -> Option<(i32, i32)> {
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
    if next_x >= 0 && next_x < 7 && next_y >= 0 && next_y < 7 {
        Some((next_x, next_y))
    } else {
        None
    }
}

fn dfs(map: &mut [[bool; 7]; 7], i: i32, x: i32, y: i32) -> usize {
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
    for to in ['U', 'D', 'L', 'R'] {
        let Some((next_x, next_y)) = go(x, y, to) else {
            continue;
        };
        if map[next_x as usize][next_y as usize] {
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
        map[next_x as usize][next_y as usize] = true;
        count += dfs(map, i + 1, next_x, next_y);
        map[next_x as usize][next_y as usize] = false;
    }
    count
}

fn solution(input: &str) -> usize {
    let mut map = [[false; 7]; 7];
    map[0][0] = true;
    dfs(&mut map, -1, 0, 0)
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
    fn test_solution() {
        assert_eq!(
            solution("??????R??????U??????????????????????????LD????D?"),
            201
        );
    }
}
