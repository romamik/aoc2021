use std::{collections::HashMap, hash::Hash};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    hall: Vec<char>, // '.' - empty, 'A', 'B' - etc amphypod
    rooms: Vec<Vec<char>>,
    room_coords: Vec<usize>,
}

fn is_apod_char(c: char) -> bool {
    return c.is_ascii_alphabetic() && c.is_ascii_uppercase();
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn is_room_has_only_home_type(
    room_n: usize,
    state: &State,
    home_room: &HashMap<char, usize>,
) -> bool {
    let room = &state.rooms[room_n];
    !room
        .iter()
        .any(|&c| c != '.' && *home_room.get(&c).unwrap() != room_n)
}

fn find_possible_positions_in_hall(room_n: usize, state: &State) -> Vec<usize> {
    let mut possible_pos = Vec::new();
    let room_x = state.room_coords[room_n];
    for d in [-1_isize, 1] {
        let mut x = room_x;
        while x > 0 && x < state.hall.len() - 1 {
            x = (x as isize + d) as usize;
            match state.hall[x] {
                '.' => {
                    if state.room_coords.iter().position(|&p| p == x).is_none() {
                        possible_pos.push(x);
                    }
                }
                _ => break,
            }
        }
    }
    possible_pos
}

fn can_move_in_hall(state: &State, from: usize, to: usize) -> bool {
    let (from, to) = if to > from { (from + 1, to) } else { (to, from - 1) };
    state.hall[from..=to].iter().all(|&c| c == '.')
}

fn get_top_apod_in_room(room_n: usize, state: &State) -> Option<(char, usize)> {
    let room = &state.rooms[room_n];
    let pos = room.iter().cloned().position(is_apod_char);
    pos.map(|p| (room[p], p))
}

fn get_top_free_pos_in_room(room_n: usize, state: &State) -> Option<usize> {
    let room = &state.rooms[room_n];
    let pos = room.iter().rev().cloned().position(|c| !is_apod_char(c));
    pos.map(|p| room.len() - p - 1)
}

fn is_room_full_of_home_type(
    room_n: usize,
    state: &State,
    home_room: &HashMap<char, usize>,
) -> bool {
    let room = &state.rooms[room_n];
    !room
        .iter()
        .cloned()
        .any(|c| home_room.get(&c).map(|&r| r != room_n).unwrap_or(true))
}

fn is_state_final(state: &State, home_room: &HashMap<char, usize>) -> bool {
    for room_n in 0..state.rooms.len() {
        if !is_room_full_of_home_type(room_n, state, home_room) {
            return false;
        }
    }
    true
}

fn make_next_states(
    state: &State,
    move_cost: &HashMap<char, usize>,
    home_room: &HashMap<char, usize>,
) -> Vec<(State, usize)> {
    let mut next_states = Vec::new();
    // move every apod in hall to its home room
    for x in 0..state.hall.len() {
        let apod = state.hall[x];
        if is_apod_char(apod) {
            let unit_cost = *move_cost.get(&apod).unwrap();
            let room_n = *home_room.get(&apod).unwrap();
            if is_room_has_only_home_type(room_n, state, home_room) {
                let room_x = state.room_coords[room_n];
                if let Some(y) = get_top_free_pos_in_room(room_n, state) {
                    if can_move_in_hall(state, x, room_x) {
                        let cost = unit_cost * ((y + 1) + abs_diff(x, room_x));
                        let mut new_state = (*state).clone();
                        new_state.hall[x] = '.';
                        new_state.rooms[room_n][y] = apod;
                        next_states.push((new_state, cost));
                    }
                }
            }
        }
    }
    // move top apod from every room to every possible position in hall
    for room_n in 0..state.rooms.len() {
        if !is_room_has_only_home_type(room_n, state, home_room) {
            let room_x = state.room_coords[room_n];
            if let Some((top_apod, top_apod_y)) = get_top_apod_in_room(room_n, state) {
                let unit_cost = *move_cost.get(&top_apod).unwrap();
                for x in find_possible_positions_in_hall(room_n, state) {
                    let cost = unit_cost * ((top_apod_y + 1) + abs_diff(x, room_x));
                    let mut new_state = (*state).clone();
                    new_state.hall[x] = top_apod;
                    new_state.rooms[room_n][top_apod_y] = '.';
                    next_states.push((new_state, cost));
                }
            }
        }
    }
    next_states
}

fn find_arrange_cost(
    state: &State,
    move_cost: &HashMap<char, usize>,
    home_room: &HashMap<char, usize>,
) -> usize {
    let mut queue = Vec::new();
    let mut visited = HashMap::new();
    visited.insert((*state).clone(), (0_usize, None));
    queue.push(((*state).clone(), 0_usize));
    loop {
        let (state, cost) = queue.pop().unwrap();
        if is_state_final(&state, home_room) {
            // let mut s = Some(state.clone());
            // while let Some(state) = s {
            //     println!("{:?}", state);
            //     s = visited
            //         .get(&state)
            //         .map(|(cost, prev_state)| prev_state.clone())
            //         .flatten();
            // }

            return cost;
        }
        for (next_state, cost_to_next_state) in make_next_states(&state, move_cost, home_room) {
            let next_cost = cost + cost_to_next_state;
            let existing_cost = visited.get(&next_state).map(|(cost, _)| *cost);
            if existing_cost
                .map(|existing_cost| existing_cost > next_cost)
                .unwrap_or(true)
            {
                visited.insert(next_state.clone(), (next_cost, Some(state.clone())));
                queue.push((next_state, next_cost));
                queue.sort_by_key(|(_, cost)| std::cmp::Reverse(*cost));
            }
        }
    }
}

pub fn main() {
    let move_cost: HashMap<char, usize> = [('A', 1), ('B', 10), ('C', 100), ('D', 1000)]
        .into_iter()
        .collect();
    let home_room: HashMap<char, usize> = [('A', 0), ('B', 1), ('C', 2), ('D', 3)]
        .into_iter()
        .collect();

    let input = parse_input(&std::fs::read_to_string("input/day23.txt").unwrap());
    let test_input = input.get("test").unwrap();
    let day23_input = input.get("day23").unwrap();
    assert_eq!(12521, find_arrange_cost(test_input, &move_cost, &home_room));
    println!("tests ok");
    println!("day23 pt1 {}", find_arrange_cost(day23_input, &move_cost, &home_room));
}

fn parse_input(s: &str) -> HashMap<String, State> {
    s.split("\n\n")
        .map(|s| {
            let lines: Vec<_> = s.split("\n").collect();
            let name = lines[0].to_string();
            let line0 = lines[1];
            let hall_size = line0.len() - 2;
            assert_eq!(
                line0,
                std::iter::repeat('#')
                    .take(hall_size + 2)
                    .collect::<String>()
            );
            let line1 = lines[2];
            assert_eq!(
                line1,
                format!(
                    "#{}#",
                    std::iter::repeat('.').take(hall_size).collect::<String>()
                )
            );
            let line2 = lines[3];
            let mut room_coords = Vec::new();
            line2.chars().enumerate().for_each(|(i, c)| match c {
                ' ' | '#' => {}
                c if is_apod_char(c) => room_coords.push(i - 1),
                _ => panic!("unexpected char {} in line {}", c, line2),
            });
            let mut rooms = vec![Vec::new(); room_coords.len()];
            lines[3..].iter().enumerate().for_each(|(line_n, line)| {
                line.chars().enumerate().for_each(|(i, c)| match c {
                    ' ' | '#' => {}
                    c if is_apod_char(c) => {
                        let room_n = room_coords.iter().position(|&pos| pos + 1 == i).unwrap();
                        assert_eq!(rooms[room_n].len(), line_n);
                        rooms[room_n].push(c);
                    }
                    _ => panic!("unexpected char {} in line {}", c, line2),
                });
            });
            assert!(!rooms.iter().any(|room| room.len() != rooms[0].len()));
            (
                name,
                State {
                    hall: std::iter::repeat('.').take(hall_size).collect(),
                    rooms,
                    room_coords,
                },
            )
        })
        .collect()
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            std::iter::repeat('#')
                .take(self.hall.len() + 2)
                .collect::<String>()
        )?;
        writeln!(f, "#{}#", self.hall.iter().collect::<String>())?;
        let room_size = self.rooms[0].len();
        let min_room_coord = *self.room_coords.first().unwrap();
        let max_room_coord = *self.room_coords.last().unwrap();
        for y in 0..room_size {
            for x in 0..=self.hall.len() + 1 {
                let c = if let Some(room_n) = self.room_coords.iter().position(|&pos| pos + 1 == x)
                {
                    self.rooms[room_n][y]
                } else if y == 0
                    || self
                        .room_coords
                        .iter()
                        .position(|&pos| pos == x || pos + 2 == x)
                        .is_some()
                {
                    '#'
                } else {
                    ' '
                };
                write!(f, "{}", c)?
            }
            writeln!(f, "")?
        }
        writeln!(
            f,
            "{}{}",
            std::iter::repeat(' ')
                .take(min_room_coord)
                .collect::<String>(),
            std::iter::repeat('#')
                .take(max_room_coord - min_room_coord + 3)
                .collect::<String>()
        )
    }
}
