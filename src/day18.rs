use std::fmt;

#[derive(Clone)]
enum Part {
    Single(usize),
    Pair(Pair),
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Single(v) => write!(f, "{}", v),
            Part::Pair(pair) => write!(f, "{:?}", pair),
        }
    }
}

#[derive(Clone)]
struct Pair(Box<[Part; 2]>);

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.0[0], self.0[1])
    }
}


fn add(a: &Pair, b: &Pair) -> Pair {
    Pair(Box::new([Part::Pair(a.clone()), Part::Pair(b.clone())]))
}

fn walk(a: &Pair) {

    fn visit(p: &Part, depth: usize) {
        match p {
            Part::Single(v) => println!("{} d:{}", v, depth),
            Part::Pair(pair) => {
                visit(&pair.0[0], depth + 1);
                visit(&pair.0[1], depth + 1);
            }
        }
    }

    visit(&a.0[0], 0);
    visit(&a.0[1], 0);
}

pub fn main() {
    test_parse();

    let pair: Pair = PairParser::parse("[[1,2],3]");
    walk(&pair);
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
        let pair: Pair = PairParser::parse(test);
        let s: String = test.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(format!("{:?}", pair), s);
    }
}
struct PairParser {
    chars: Vec<char>,
    pos: usize,
}

impl PairParser {
    fn parse(s: &str) -> Pair {
        let chars = s.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
        let mut parser = PairParser { chars, pos: 0 };
        if let Part::Pair(pair) = parser.parse_pair() {
            pair
        } else {
            panic!("not a pair")
        }
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

    fn parse_part(&mut self) -> Part {
        match self.cur() {
            Some('[') => self.parse_pair(),
            Some(c) if c.is_digit(10) => self.parse_single(),
            Some(c) => panic!("unexpected char {}", c),
            None => panic!("unexpected end"),
        }
    }

    fn parse_single(&mut self) -> Part {
        let mut result: usize = 0;
        while let Some(digit) = self.cur().map(|c| c.to_digit(10)).flatten() {
            result *= 10;
            result += digit as usize;
            self.advance();
        }
        Part::Single(result)
    }

    fn parse_pair(&mut self) -> Part {
        assert_eq!(self.cur(), Some('['));
        self.advance();
        let a = self.parse_part();
        assert_eq!(self.cur(), Some(','));
        self.advance();
        let b = self.parse_part();
        assert_eq!(self.cur(), Some(']'));
        self.advance();
        Part::Pair(Pair(Box::new([a, b])))
    }
}
