#[derive(Clone)]
struct Img {
    size_x: usize,
    size_y: usize,
    data: Vec<bool>,
    on_outside: bool,
}

impl Img {
    fn new(size_x: usize, size_y: usize) -> Img {
        Img {
            size_x,
            size_y,
            data: vec![false; size_x * size_y],
            on_outside: false,
        }
    }

    fn from_str(s: &str) -> Img {
        let lines = s
            .split("\n")
            .map(|s| s.trim())
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("unexpected char"),
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|l| l.len() > 0)
            .collect::<Vec<_>>();

        let size_y = lines.len();
        let size_x = if size_y > 0 { lines[0].len() } else { 0 };

        lines.iter().for_each(|l| assert_eq!(l.len(), size_x));

        Img {
            size_x,
            size_y,
            data: lines.iter().flatten().cloned().collect(),
            on_outside: false,
        }
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.size_x as isize || y >= self.size_y as isize {
            self.on_outside
        } else {
            self.data[x as usize + y as usize * self.size_x]
        }
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        assert!(x < self.size_x && y < self.size_y);
        self.data[x + y * self.size_x] = v;
    }

    fn count_on(&self) -> usize {
        self.data.iter().cloned().filter(|&v| v).count()
    }
}

impl std::fmt::Debug for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                write!(
                    f,
                    "{}",
                    if self.get(x as isize, y as isize) {
                        "#"
                    } else {
                        "."
                    }
                )?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

fn get_enhance_offset(img: &Img, x: isize, y: isize) -> usize {
    let mut result = 0;
    for y in y - 1..=y + 1 {
        for x in x - 1..=x + 1 {
            let on = img.get(x, y);
            result = result << 1;
            if on {
                result = result | 1
            }
        }
    }
    result
}

fn enhance_step(img: &Img, enhance: &Img) -> Img {
    assert_eq!((enhance.size_x, enhance.size_y), (512, 1));
    let mut result = Img::new(img.size_x + 2, img.size_y + 2);

    for y in 0..result.size_y {
        for x in 0..result.size_x {
            let off = get_enhance_offset(img, x as isize - 1, y as isize - 1);
            let on = enhance.get(off as isize, 0);
            result.set(x, y, on);
        }
    }

    result.on_outside = enhance.get(get_enhance_offset(img, -2, -2) as isize, 0);

    result
}

fn count_on_after_enhance(img: &Img, enhance: &Img, steps: usize) -> usize {
    let mut img = (*img).clone();
    for _ in 0..steps {
        img = enhance_step(&img, enhance);
    }
    img.count_on()
}

pub fn main() {
    test();
    println!("tests ok");

    let day20_input: Vec<_> = std::fs::read_to_string("input/day20.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    let day20_enhance = Img::from_str(&day20_input[0]);
    let day20_img = Img::from_str(&day20_input[1]);

    println!(
        "day20 pt1 {}",
        count_on_after_enhance(&day20_img, &day20_enhance, 2)
    );
    println!(
        "day20 pt2 {}",
        count_on_after_enhance(&day20_img, &day20_enhance, 50)
    );
}

fn test() {
    let img = Img::from_str(
        "
        #..#.
        #....
        ##..#
        ..#..
        ..###
        ",
    );
    let enhance = Img::from_str(&concat!(
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
        "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
        ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
        ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
        ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
        "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
        "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
    ));

    assert_eq!(get_enhance_offset(&img, 2, 2), 0b000100010);
    assert_eq!(count_on_after_enhance(&img, &enhance, 2), 35);
    assert_eq!(count_on_after_enhance(&img, &enhance, 50), 3351);
}
