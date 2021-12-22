use std::fmt;

#[derive(Clone)]
enum Part {
    Single(usize),
    Pair(Pair),
}

type Pair = [Box<Part>; 2];

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Single(v) => write!(f, "{}", v),
            Part::Pair(pair) => write!(f, "[{:?},{:?}]", pair[0], pair[1]),
        }
    }
}

fn unwrap_single(p: &Part) -> usize {
    match *p {
        Part::Single(v) => v,
        _ => panic!("not single"),
    }
}

fn explode(p: &mut Part) -> bool {
    // we traverse our tree and remember last occured single as single_before
    // when we find pair at depth 4 we remember it's values and replace it with 0, and stop remembering single_before
    // after that we continue traverse and remember first occured single as single_after

    // this can be refactored as struct with methods instead of messy multiple mutable ref arguments
    // also it is possible to get rid of recursion and stop iterating better

    fn visit<'a>(
        part: &'a mut Part,
        exploded_part: &mut Option<[usize; 2]>,
        single_before: &mut Option<&'a mut usize>,
        single_after: &mut Option<&'a mut usize>,
        depth: usize,
    ) {
        if matches!((&exploded_part, &single_after), (Some(_), Some(_))) {
            return;
        }

        match part {
            Part::Single(ref mut v) => match (&exploded_part, &single_after) {
                (None, _) => *single_before = Some(v),
                (Some(_), None) => *single_after = Some(v),
                _ => (),
            },
            Part::Pair(pair) if depth == 4 && matches!(exploded_part, None) => {
                *exploded_part = Some([unwrap_single(&pair[0]), unwrap_single(&pair[1])]);
                *part = Part::Single(0);
            }
            Part::Pair(pair) => pair.iter_mut().for_each(|p| {
                visit(
                    &mut *p,
                    exploded_part,
                    single_before,
                    single_after,
                    depth + 1,
                )
            }),
        }
    }
    let mut single_before = None;
    let mut single_after = None;
    let mut exploded_part = None;
    visit(
        p,
        &mut exploded_part,
        &mut single_before,
        &mut single_after,
        0,
    );
    let has_exploded = if let Some(exploded_part) = exploded_part {
        if let Some(single_before) = single_before {
            *single_before += exploded_part[0];
        }
        if let Some(single_after) = single_after {
            *single_after += exploded_part[1];
        }
        true
    } else {
        false
    };
    has_exploded
}

fn split(p: &mut Part) -> bool {
    fn visit<'a>(part: &'a mut Part, has_split: &mut bool) {
        if *has_split {
            return;
        }

        match part {
            Part::Single(v) => {
                if *v >= 10 {
                    *has_split = true;
                    let v0 = *v / 2;
                    let v1 = *v - v0;
                    *part = Part::Pair([Box::new(Part::Single(v0)), Box::new(Part::Single(v1))]);
                }
            }
            Part::Pair(pair) => pair.iter_mut().for_each(|p| visit(&mut *p, has_split)),
        }
    }
    let mut has_split = false;
    visit(p, &mut has_split);
    has_split
}

fn explode_or_split(p: &mut Part) -> bool {
    explode(p) || split(p)
}

fn add(a: &Part, b: &Part) -> Part {
    Part::Pair([Box::new(a.clone()), Box::new(b.clone())])
}

fn reduce(p: &mut Part) {
    loop {
        if !explode_or_split(p) {
            break;
        }
    }
}

fn add_list(input: &[Box<Part>]) -> Part {
    let mut it = input.iter();
    let mut a = *(it.next().unwrap()).clone();
    for b in it {
        a = add(&a, &*b);
        reduce(&mut a);
    }
    a
}

fn calc_magnitude(a: &Part) -> usize {
    match a {
        Part::Single(v) => *v,
        Part::Pair(pair) => 3 * calc_magnitude(&pair[0]) + 2 * calc_magnitude(&pair[1]),
    }
}

fn add_list_calc_magnitude(input: &[Box<Part>]) -> usize {
    calc_magnitude(&add_list(input))
}

fn find_max_magnitude_sum(input: &[Box<Part>]) -> usize {
    let mut max_mag = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                let a = &input[i];
                let b = &input[j];
                let mut c = add(a, b);
                reduce(&mut c);
                let mag = calc_magnitude(&c);
                max_mag = std::cmp::max(mag, max_mag);
            }
        }
    }
    max_mag
}

pub fn main() {
    test_parse();
    test_explode();
    test_explode_or_split();
    test_add_and_reduce();
    test_add_list();
    test_calc_magnitude();
    test_add_list_calc_magnitude();
    test_find_max_magnitude_sum();

    let input = parse_input(&std::fs::read_to_string("input/day18.txt").unwrap());
    println!("day18 pt1 {}", add_list_calc_magnitude(&input));
    println!("day18 pt2 {}", find_max_magnitude_sum(&input));
}

fn test_find_max_magnitude_sum() {
    let s = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    let input = parse_input(s);
    assert_eq!(find_max_magnitude_sum(&input), 3993);
}

fn test_add_list_calc_magnitude() {
    let s = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    let input = parse_input(s);
    assert_eq!(add_list_calc_magnitude(&input), 4140);
}

fn test_calc_magnitude() {
    let tests = [
        ("[9,1]", 29),
        ("[[9,1],[1,9]]", 129),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
        (
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
            3993,
        ),
    ];
    for test in tests.iter() {
        assert_eq!(
            calc_magnitude(&PairParser::parse(test.0)),
            test.1,
            "{}",
            test.0
        );
    }
}

fn test_add_list() {
    let tests = [
        (
            "[1,1]
            [2,2]
            [3,3]
            [4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        ),
        (
            "[1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        ),
        (
            "[1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]
            [6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ),
        (
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ),
        (
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        ),
    ];
    for test in tests {
        assert_eq!(format!("{:?}", add_list(&parse_input(test.0))), test.1);
    }
}

fn test_add_and_reduce() {
    let tests = [
        (
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ),
        (
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
        ),
    ];

    for test in tests.iter() {
        let a = PairParser::parse(test.0);
        let b = PairParser::parse(test.1);
        let mut c = add(&*a, &*b);
        reduce(&mut c);
        assert_eq!(format!("{:?}", c), test.2);
    }
}

fn test_explode_or_split() {
    let tests = [
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    ];

    let mut prev = None;
    for &s in tests.iter() {
        if let Some(prev) = prev {
            let mut p = PairParser::parse(prev);
            explode_or_split(&mut p);
            assert_eq!(format!("{:?}", p), s);
        }
        prev = Some(s);
    }
}

fn test_explode() {
    let tests = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    for test in tests.iter() {
        let mut p = PairParser::parse(test.0);
        explode(&mut p);
        assert_eq!(format!("{:?}", p), test.1);
    }
}

fn test_parse() {
    let tests = [
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
        "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        "[  [[[1  ,  3],    [5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
    ];
    for test in tests {
        let pair = PairParser::parse(test);
        let s: String = test.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(format!("{:?}", pair), s);
    }
}
struct PairParser {
    chars: Vec<char>,
    pos: usize,
}

impl PairParser {
    fn parse(s: &str) -> Box<Part> {
        let chars = s.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
        let mut parser = PairParser { chars, pos: 0 };
        parser.parse_part()
    }

    fn cur(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_part(&mut self) -> Box<Part> {
        match self.cur() {
            Some('[') => self.parse_pair(),
            Some(c) if c.is_digit(10) => self.parse_single(),
            Some(c) => panic!("unexpected char {}", c),
            None => panic!("unexpected end"),
        }
    }

    fn parse_single(&mut self) -> Box<Part> {
        let mut result: usize = 0;
        while let Some(digit) = self.cur().map(|c| c.to_digit(10)).flatten() {
            result *= 10;
            result += digit as usize;
            self.advance();
        }
        Box::new(Part::Single(result))
    }

    fn parse_pair(&mut self) -> Box<Part> {
        assert_eq!(self.cur(), Some('['));
        self.advance();
        let a = self.parse_part();
        assert_eq!(self.cur(), Some(','));
        self.advance();
        let b = self.parse_part();
        assert_eq!(self.cur(), Some(']'));
        self.advance();
        Box::new(Part::Pair([a, b]))
    }
}

fn parse_input(s: &str) -> Vec<Box<Part>> {
    s.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| PairParser::parse(s))
        .collect()
}
