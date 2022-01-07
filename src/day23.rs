use std::collections::HashMap;

struct State {
    hall: Vec<char>, // '.' - empty, 'A', 'B' - etc amphypod
    rooms: Vec<Vec<char>>,
    room_coords: Vec<usize>,
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

fn find_next_states(
    state: &State,
    move_cost: &HashMap<char, usize>,
    home_room: &HashMap<char, usize>,
) {
    for room_n in 0..state.rooms.len() {
        if !is_room_has_only_home_type(room_n, state, home_room) {
            // try move out top apod to any reachable position in hall
            let room_x = state.room_coords[room_n];
            let mut possible_pos = Vec::new();
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
            println!("{} {:?}", room_n, possible_pos);
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
    find_next_states(test_input, &move_cost, &home_room);
    println!(
        "{:#?}",test_input
        
    );
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
                c if c.is_alphabetic() && c.is_uppercase() => room_coords.push(i - 1),
                _ => panic!("unexpected char {} in line {}", c, line2),
            });
            let mut rooms = vec![Vec::new(); room_coords.len()];
            lines[3..].iter().enumerate().for_each(|(line_n, line)| {
                line.chars().enumerate().for_each(|(i, c)| match c {
                    ' ' | '#' => {}
                    c if c.is_alphabetic() && c.is_uppercase() => {
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
