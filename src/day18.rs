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

fn explode_or_split(p: &mut Part) -> bool {
    // we traverse our tree and remember last occured single as single_before
    // when we find pair at depth 4 we remember it's values and replace it with 0, and stop remembering single_before
    // after that we continue traverse and remember first occured single as single_after

    // this can be refactored as struct with methods instead of messy multiple mutable ref arguments
    // also it is possible to get rid of recursion and stop iterating better

    fn visit<'a>(
        part: &'a mut Part,
        has_split: &mut bool,
        exploded_part: &mut Option<[usize; 2]>,
        single_before: &mut Option<&'a mut usize>,
        single_after: &mut Option<&'a mut usize>,
        depth: usize,
    ) {
        let has_exploded = matches!(&exploded_part, Some(_));
        if *has_split || (has_exploded && matches!(&single_after, Some(_))) {
            return;
        }

        match part {
            Part::Single(v) if *v >= 10 => {
                *has_split = true;
                let v0 = *v / 2;
                let v1 = *v - v0;
                *part = Part::Pair([Box::new(Part::Single(v0)), Box::new(Part::Single(v1))]);
            }
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
                    has_split,
                    exploded_part,
                    single_before,
                    single_after,
                    depth + 1,
                )
            }),
        }
    }
    let mut has_split = false;
    let mut single_before = None;
    let mut single_after = None;
    let mut exploded_part = None;
    visit(
        p,
        &mut has_split,
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
    has_split || has_exploded
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

pub fn main() {
    test_parse();
    test_explode();
    test_explode_or_split();
    test_add_and_reduce();
    test_add_list();

    let mut p = PairParser::parse("[[[[[9,8],1],2],3],4]");
    if let Part::Single(ref mut v) = *p {
        *v = 10;
    }
    println!("{:?}", p);
    explode_or_split(&mut p);
    println!("{:?}", p);

    test_parse();
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
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )
    ];
    for test in tests {
        assert_eq!(format!("{:?}", add_list(&parse_input(test.0))), test.1);
    }
}

fn test_add_and_reduce() {
    let a = PairParser::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = PairParser::parse("[1,1]");
    let mut c = add(&*a, &*b);
    reduce(&mut c);
    assert_eq!(format!("{:?}", c), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
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
        explode_or_split(&mut p);
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
