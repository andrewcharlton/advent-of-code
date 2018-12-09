use std::collections::{HashMap, VecDeque};

fn main() {
    let players = 458;
    let last_marble = 72019;

    println!("Part one: {}", marble_game(players, last_marble));
    println!("Part two: {}", marble_game(players, last_marble * 100));
}

fn marble_game(players: usize, last_marble: usize) -> usize {
    let mut marbles = VecDeque::new();
    let mut scores: HashMap<usize, usize> = HashMap::new();

    marbles.push_back(0);
    let mut player = 0;

    for m in 1..last_marble + 1 {
        player = (player + 1) % players;

        if m % 23 == 0 {
            for _ in 0..6 {
                let end = marbles.pop_back().unwrap();
                marbles.push_front(end);
            }
            let end = marbles.pop_back().unwrap();
            let n = marbles.pop_back().unwrap();
            marbles.push_back(end);

            scores.insert(player, scores.get(&player).unwrap_or(&0) + m + n);
            continue;
        }

        let front = marbles.pop_front().unwrap();
        marbles.push_back(front);
        marbles.push_back(m);
    }

    *scores.values().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn marble_game_test() {
        assert_eq!(marble_game(5, 25), 32);
        assert_eq!(marble_game(10, 1618), 8317);
        assert_eq!(marble_game(13, 7999), 146373);
        assert_eq!(marble_game(17, 1104), 2764);
        assert_eq!(marble_game(21, 6111), 54718);
        assert_eq!(marble_game(30, 5807), 37305);
    }
}
