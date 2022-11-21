//! Day 21: Dirac Dice

use std::collections::HashMap;

type Pair = (usize, usize);

/// score of losing player multiplied by number of die rolls
pub fn a(input: &Vec<&str>) -> String {
    let mut pos = parse_input(input);

    const MAX_SCORE: usize = 1000;

    let mut scores = (0, 0);
    let mut rolls = 0;
    let mut is_player_1 = true;

    while scores.0 < MAX_SCORE && scores.1 < MAX_SCORE {
        let val = (rolls + 1) * 3 + 3;
        rolls += 3;
        if is_player_1 {
            pos.0 = (pos.0 - 1 + val) % 10 + 1;
            scores.0 += pos.0;
        } else {
            pos.1 = (pos.1 - 1 + val) % 10 + 1;
            scores.1 += pos.1;
        }
        is_player_1 = !is_player_1;
    }

    (if is_player_1 { scores.0 } else { scores.1 } * rolls).to_string()
}

/// number of winning universes
pub fn b(input: &Vec<&str>) -> String {
    let pos = parse_input(input);

    const MAX_SCORE: usize = 21;

    /// 3^3=27 combinations of die rolls
    const ROLLS: [Pair; 7] = [
        (3, 1),  // 1 roll of sum 3
        (4, 3),  // 3 rolls of sum 4
        (5, 6),  // 6 rolls of sum 5
        (6, 7),  // 7 rolls of sum 6
        (7, 6),  // 6 rolls of sum 5
        (8, 3),  // 3 rolls of sum 8
        (9, 1),  // 1 roll of sum 9
    ];

    /// caches (player, pos, scores) -> combinations
    type CombCache = HashMap<(bool, Pair, Pair), Pair>;

    /// determines winning combinations based on given position
    fn simulate(
        cache: &mut CombCache,
        pos: &Pair,
        scores: &Pair,
        is_player_1: bool,
    ) -> Pair {
        let cache_key = (is_player_1, *pos, *scores);
        if let Some(hit) = cache.get(&cache_key) {
            return *hit;
        }

        let mut combs = (0, 0);
        for (val, num) in ROLLS {
            let mut new_pos = *pos;
            let mut new_scores = *scores;
            if is_player_1 {
                new_pos.0 = (pos.0 - 1 + val) % 10 + 1;
                new_scores.0 += new_pos.0;
            } else {
                new_pos.1 = (pos.1 - 1 + val) % 10 + 1;
                new_scores.1 += new_pos.1;
            }
            if new_scores.0 < MAX_SCORE && new_scores.1 < MAX_SCORE {
                let res = simulate(cache, &new_pos, &new_scores, !is_player_1);
                combs.0 += num * res.0;
                combs.1 += num * res.1;
            } else {
                *(if is_player_1 { &mut combs.0 } else { &mut combs.1 }) += num;
            }
        }

        cache.insert(cache_key, combs);
        combs
    }

    let combinations = simulate(&mut CombCache::new(), &pos, &(0, 0), true);
    std::cmp::max(combinations.0, combinations.1).to_string()
}

fn parse_input(input: &Vec<&str>) -> Pair {
    let mut it = input
        .iter()
        .map(|&s| s
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap()
        );
    (
        it.next().unwrap(),
        it.next().unwrap(),
    )
}

#[test]
pub fn test() {
    let input = vec![
        "Player 1 starting position: 4",
        "Player 2 starting position: 8",
    ];

    assert_eq!(parse_input(&input), (4, 8));

    assert_eq!(a(&input), "739785");
    assert_eq!(b(&input), "444356092776315");
}
