use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;

const ALL_SEGMENTS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

/*
Segment - segment of 7-segment display, named "a" to "g".
Signal - set of turned on segments - for example "cf", or "acdeg"
Mapping - mapping of segments to segments. Mapping can be represented as string of length 0 to 7. For example "cbd" - means that "a" is mapped to "c", "b" to "b" and "c" to

We have
    valid_signals: digits 0 to 9 rendered on 7-segment display.
    input_signals: set of Signals that should be mapped using some Mapping so that each becomes one of the valid_signals
    test_signals: set of Signals we should map using valid mapping

fn possible_valid_signals(input_signal, mapping)
    function to find to which of valid_signals input_signal can be mapped using mapping.
    if mapping is complete, i.e. defines mapping for every segment - it's obvious: just apply mapping and find matches in valid_signals
    if mapping is incomplete, we should map only segments that have defined mapping and then find valid_signals that have mapped segments turned on or off same as in input_signal
    One more thing to consider is that input_signal can possibly be mapped to valid_signal if they have the same length

fn find_mapping (solve pt2)
    function to find mapping that makes all input_signals to be mapped to one of valid_signals
    we just test all possible mappings to see if they map each input_signal to exactly one valid_signal
    but we do not generate all possible complete mappings, we start with shorter incomplete mappings and produce more complete mappings only for mappings that possibly can map input_signals to valid_signals
*/

type InputLine = [Vec<Signal>; 2];
type Input = Vec<InputLine>;

type Segment = char;

#[derive(Clone)]
struct Signal(String, HashSet<Segment>);

impl Signal {
    fn new(s: &str) -> Signal {
        let hash_set: HashSet<Segment> = s.chars().collect();
        let mut vec: Vec<Segment> = hash_set.iter().cloned().collect();
        vec.sort();
        let name: String = vec.iter().collect();
        Signal(name, hash_set)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_segment_on(&self, segment: Segment) -> bool {
        self.1.get(&segment).is_some()
    }

    fn has_all_segments_on(&self, signal: &Signal) -> bool {
        for segment in signal.1.iter() {
            if !self.is_segment_on(*segment) {
                return false;
            }
        }
        true
    }

    fn map(&self, mapping: &HashMap<Segment, Segment>) -> Signal {
        let mut mapped = String::new();
        for segment in self.1.iter() {
            if let Some(mapped_segment) = mapping.get(&segment) {
                mapped.push(*mapped_segment);
            }
        }
        Signal::new(&mapped)
    }
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Mapping {
    name: String,
    map: HashMap<Segment, Segment>,
    map_back: HashMap<Segment, Segment>,
}

impl Mapping {
    fn new() -> Mapping {
        Mapping {
            name: String::new(),
            map: HashMap::new(),
            map_back: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.name.len()
    }

    fn has_segment_mapping(&self, to: Segment) -> bool {
        self.map_back.get(&to).is_some()
    }

    fn add_segment_mapping(&mut self, to: Segment) {
        if self.has_segment_mapping(to) {
            panic!()
        }
        let from: char = ALL_SEGMENTS[self.len()];
        self.map.insert(from, to);
        self.map_back.insert(to, from);
        self.name.push(to);
        //println!("add {} {:?}", to, self);
    }

    fn remove_last_segment_mapping(&mut self) {
        let to = self.name.pop().unwrap();
        let from = *self.map_back.get(&to).unwrap();
        self.map.remove(&from);
        self.map_back.remove(&to);
        //println!("remove {} {:?}", to, self);
    }
}

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} map: {:?} map_back: {:?}",
            self.name, self.map, self.map_back
        )
    }
}

fn solve_pt1(valid_signals: &Vec<Signal>, input: &Input) -> i32 {
    let mut count = 0;
    for input_line in input.iter() {
        for test_signal in input_line[1].iter() {
            let possible_valid_signal_count = valid_signals
                .iter()
                .filter(|valid_signal| valid_signal.len() == test_signal.len())
                .count();
            if possible_valid_signal_count == 1 {
                count += 1;
            }
        }
    }
    count
}

fn find_possible_valid_signals<'a>(
    valid_signals: &'a Vec<Signal>,
    signal: &'a Signal,
    mapping: &'a Mapping,
) -> Vec<&'a Signal> {
    // it is possible to check not only turned on segments, but also turned off segments
    let mut possible_valid_signals = Vec::new();
    for valid_signal in valid_signals {
        if valid_signal.len() == signal.len() {
            let mapped = signal.map(&mapping.map);
            if valid_signal.has_all_segments_on(&mapped) {
                let mapped_back = valid_signal.map(&mapping.map_back);
                if signal.has_all_segments_on(&mapped_back) {
                    possible_valid_signals.push(valid_signal);
                }
            }
        }
    }
    possible_valid_signals
}

fn solve_pt2_line(valid_signals: &Vec<Signal>, input_line: &InputLine) -> i32 {
    fn recursive_search(
        valid_signals: &Vec<Signal>,
        mapping: &mut Mapping,
        input_signals: &Vec<Signal>,
    ) -> bool {
        for input_signal in input_signals {
            let possible_count = find_possible_valid_signals(valid_signals, input_signal, mapping)
                .iter()
                .count();
            if possible_count == 0 {
                return false;
            }
        }
        if mapping.len() == ALL_SEGMENTS.len() {
            return true;
        }

        for segment in ALL_SEGMENTS.iter().cloned() {
            if !mapping.has_segment_mapping(segment) {
                mapping.add_segment_mapping(segment);
                if recursive_search(valid_signals, mapping, input_signals) {
                    return true;
                }
                mapping.remove_last_segment_mapping();
            }
        }

        false
    }

    let mut mapping = Mapping::new();
    let mut combined_input = input_line[0].clone();
    combined_input.extend(input_line[1].clone());
    recursive_search(valid_signals, &mut mapping, &combined_input);
    let mut result = 0;
    //println!("{:?}", mapping);
    for test_signal in &input_line[1] {
        let mapped = find_possible_valid_signals(valid_signals, test_signal, &mapping);
        if mapped.len() != 1 {
            panic!();
        }
        let mapped = mapped[0];
        let mapped = valid_signals.iter().position(|s| s.0 == mapped.0).unwrap();
        result *= 10;
        result += mapped;
        //println!("{:?} => {:?}", test_signal, mapped);
    }

    result as i32
}

fn solve_pt2(valid_signals: &Vec<Signal>, input: &Input) -> i32 {
    let mut result = 0;
    for input_line in input.iter() {
        result += solve_pt2_line(valid_signals, input_line);
        //println!("{}", line_result);
    }
    result
}

pub fn main() {
    let valid_signals: Vec<Signal> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .map(|s| Signal::new(s))
    .collect();

    let test_input = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";
    let test_input = parse_input(test_input);
    let day8_input = parse_input(&fs::read_to_string("input/day8.txt").unwrap());
    println!("test pt1 {:?}", solve_pt1(&valid_signals, &test_input));
    println!("day8 pt1 {:?}", solve_pt1(&valid_signals, &day8_input));
    println!("test pt2 {:?}", solve_pt2(&valid_signals, &test_input));
    println!("day8 pt2 {:?}", solve_pt2(&valid_signals, &day8_input));
}

fn parse_input(s: &str) -> Input {
    s.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| parse_input_line(s))
        .collect::<Vec<_>>()
}

fn parse_input_line(s: &str) -> InputLine {
    InputLine::try_from(
        s.split("|")
            .map(|s| {
                s.split_whitespace()
                    .map(|s| Signal::new(s))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
