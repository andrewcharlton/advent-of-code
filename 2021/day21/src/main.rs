use std::collections::HashMap;

fn main() {
    println!("Part one: {}", deterministic(8, 5));
}

fn deterministic(x: u64, y: u64) -> u64 {
    let mut scores = (0, 0);
    let mut pos = (x - 1, y - 1); // 0-indexed to make the arithmetic easier.

    let mut die = 0;
    let mut rolls = 0;

    let mut player = 0;

    loop {
        let mut roll = 0;
        for _ in 0..3 {
            die = 1 + (die % 100);
            rolls += 1;
            roll += die;
        }

        if player == 0 {
            pos.0 = (pos.0 + roll) % 10;
            scores.0 += 1 + pos.0;
            if scores.0 >= 1000 {
                return scores.1 * rolls;
            }
        } else {
            pos.1 = (pos.1 + roll) % 10;
            scores.1 += 1 + pos.1;
            if scores.1 >= 1000 {
                return scores.0 * rolls;
            }
        }

        player = (player + 1) % 2;
    }
}

type Player interface {

fn dirac(x: u64, y: u64) -> u64 {
    let mut player0: HashMap<(u64, u64), u64> = HashMap::new(); // (pos, score): routes to that position
    player0.insert((x, 0), 1);
    let mut player0_wins = 0;

    let mut player1: HashMap<(u64, u64), u64> = HashMap::new();
    player1.insert((y, 0), 1);
    let mut player1_wins = 0;

    let rolls = HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

    let mut current_player = 0;

    while player0.len() > 0 && player1.len() > 0 {
        // Advance all the possible scores/positions of the map.
        let mut new
        if current_player == 0 {
        }

        current_player = (current_player + 1) % 2;
    }

    if player0_wins > player1_wins {
        player0_wins
    } else {
        player1_wins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(deterministic(4, 8), 739785, "Deterministic");
    }
}
