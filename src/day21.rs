use std::collections::HashMap;

struct Rules {
    rolls: usize,
    dice_sides: usize,
    board_size: usize,
    max_score: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PlayerState {
    pos: usize,
    score: usize,
    is_current: bool,
}

type GameState = [PlayerState; 2];

fn make_next_state(state: &GameState, dice_sum: usize, rules: &Rules) -> GameState {
    fn make_next_player_state(state: &PlayerState, dice_sum: usize, rules: &Rules) -> PlayerState {
        if state.is_current {
            let pos = (state.pos + dice_sum) % rules.board_size;
            PlayerState {
                pos: pos,
                score: state.score + pos + 1,
                is_current: !state.is_current,
            }
        } else {
            PlayerState {
                pos: state.pos,
                score: state.score,
                is_current: !state.is_current,
            }
        }
    }

    [
        make_next_player_state(&state[0], dice_sum, rules),
        make_next_player_state(&state[1], dice_sum, rules),
    ]
}

fn get_winning_player(state: &GameState, rules: &Rules) -> Option<usize> {
    for n in 0..state.len() {
        if state[n].score >= rules.max_score {
            return Some(n);
        }
    }
    None
}

fn play_game_determenistic_dice(pos1: usize, pos2: usize, rules: &Rules) -> usize {
    let mut state = [
        PlayerState {
            is_current: true,
            pos: pos1 - 1,
            score: 0,
        },
        PlayerState {
            is_current: false,
            pos: pos2 - 1,
            score: 0,
        },
    ];
    let mut roll_count = 0_usize;
    while get_winning_player(&state, rules) == None {
        let mut dice_sum = 0;
        for _ in 0..3 {
            dice_sum += roll_count % rules.dice_sides + 1;
            roll_count += 1;
        }
        state = make_next_state(&state, dice_sum, rules);
        //println!("{:?}", state);
    }
    let winning_player = get_winning_player(&state, rules).unwrap();
    let other_player = (winning_player + 1) % 2;
    let other_score = state[other_player].score;
    other_score * roll_count
}

fn make_all_dice_rolls(rules: &Rules) -> HashMap<usize, usize> {
    fn make_rolls(roll: usize, sum: usize, results: &mut HashMap<usize, usize>, rules: &Rules) {
        if roll == rules.rolls {
            let count = results.get(&sum).unwrap_or(&0) + 1;
            results.insert(sum, count);
        } else {
            for side in 1..=rules.dice_sides {
                make_rolls(roll + 1, sum + side, results, rules);
            }
        }
    }
    let mut results = HashMap::new();
    make_rolls(0, 0, &mut results, rules);
    results
}

fn play_game_quantum_dice(pos1: usize, pos2: usize, rules: &Rules) -> [usize; 2] {
    let all_rolls = make_all_dice_rolls(rules);
    //println!("{:?}", all_rolls);
    let state = [
        PlayerState {
            pos: pos1 - 1,
            score: 0,
            is_current: true,
        },
        PlayerState {
            pos: pos2 - 1,
            score: 0,
            is_current: false,
        },
    ];

    let mut states: HashMap<GameState, usize> = HashMap::new();
    states.insert(state, 1);
    let mut wins = [0; 2];

    while states.len() > 0 {
        let mut new_states = HashMap::new();
        for (dice_sum, dice_sum_count) in all_rolls.iter() {
            for (state, state_count) in states.iter() {
                let next_state = make_next_state(state, *dice_sum, rules);
                if let Some(winning_player) = get_winning_player(&next_state, rules) {
                    wins[winning_player] += state_count * dice_sum_count;
                } else {
                    let new_count =
                        new_states.get(&next_state).unwrap_or(&0) + state_count * dice_sum_count;
                    new_states.insert(next_state, new_count);
                }
            }
        }
        states = new_states;
    }

    wins
}

pub fn main() {
    let rules0 = Rules {
        board_size: 10,
        dice_sides: 100,
        max_score: 1000,
        rolls: 3,
    };
    let rules1 = Rules {
        board_size: 10,
        dice_sides: 3,
        max_score: 21,
        rolls: 3,
    };
    assert_eq!(739785, play_game_determenistic_dice(4, 8, &rules0));
    assert_eq!([444356092776315, 341960390180808], play_game_quantum_dice(4, 8, &rules1));
    println!("tests ok");
    println!("day 21 pt1 {:?}", play_game_determenistic_dice(7, 8, &rules0));
    println!("day 21 pt2 {:?}", play_game_quantum_dice(7, 8, &rules1).iter().max().unwrap());
}
