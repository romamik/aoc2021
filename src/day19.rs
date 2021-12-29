/*
Matrix and vector layout
[[00, 01, 02, 03],       [0,
 [10, 11, 12, 13],        1,
 [20, 21, 22, 23]]        2]

X - face right, Y - face up, Z - from viewer

view in Z directtion:
Y
|  Z
| /
|/
*------X

view in Y direction:
   Y
  /
 /
*------X
|
|
|
Z

view in X directon:
Z
|  X
| /
|/
*------Y

*/

use std::{
    collections::{HashMap, HashSet},
    fmt,
    ops::{Index, IndexMut},
};

use Axis::*;

type Input = HashMap<String, Vec<Vector>>;

const MIN_PAIR: usize = 12;

#[derive(Clone, Copy)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    fn all() -> [Axis; 3] {
        [X, Y, Z]
    }
}

#[derive(PartialEq, Clone, Hash, Eq)]
struct Vector([i32; 3]);

impl Vector {
    fn negate(&self) -> Vector {
        Vector([-self[X], -self[Y], -self[Z]])
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Index<Axis> for Vector {
    type Output = i32;

    fn index(&self, axis: Axis) -> &i32 {
        &self.0[axis as usize]
    }
}

impl IndexMut<Axis> for Vector {
    fn index_mut(&mut self, axis: Axis) -> &mut i32 {
        &mut self.0[axis as usize]
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Matrix([[i32; 4]; 3]);

impl Matrix {
    fn identity() -> Matrix {
        Matrix([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0]])
    }

    fn rotate90cw(axis: Axis) -> Matrix {
        match axis {
            X => Matrix([[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, 0, 0]]),
            Y => Matrix([[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0]]),
            Z => Matrix([[0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 1, 0]]),
        }
    }

    fn rotate_n_90cw(axis: Axis, n: u32) -> Matrix {
        let mut m = Self::identity();
        for _ in 0..n {
            m = m.mul(&Self::rotate90cw(axis));
        }
        m
    }

    fn translate(v: &Vector) -> Matrix {
        Matrix([[1, 0, 0, v[X]], [0, 1, 0, v[Y]], [0, 0, 1, v[Z]]])
    }

    fn all_orientations() -> Vec<Matrix> {
        let mut result = vec![];

        for m0 in [
            Matrix::identity(),
            Matrix::rotate90cw(X),
            Matrix::rotate90cw(Y),
        ] {
            for flip_dir in 0..2 {
                let m1 = m0.mul(&Matrix::rotate_n_90cw(Y, flip_dir * 2));
                for rot_z in 0..4 {
                    result.push(m1.mul(&Matrix::rotate_n_90cw(Z, rot_z)));
                }
            }
        }

        result
    }

    fn mul(&self, right: &Matrix) -> Matrix {
        let b = &right.0;

        let v0 = self.apply4(b[0][0], b[1][0], b[2][0], 0);
        let v1 = self.apply4(b[0][1], b[1][1], b[2][1], 0);
        let v2 = self.apply4(b[0][2], b[1][2], b[2][2], 0);
        let v3 = self.apply4(b[0][3], b[1][3], b[2][3], 1);

        assert_eq!([v0[3], v1[3], v2[3], v3[3]], [0, 0, 0, 1]);

        Matrix([
            [v0[0], v1[0], v2[0], v3[0]],
            [v0[1], v1[1], v2[1], v3[1]],
            [v0[2], v1[2], v2[2], v3[2]],
        ])
    }

    fn apply(&self, v: &Vector) -> Vector {
        let v = &v.0;
        let r = self.apply4(v[0], v[1], v[2], 1);
        Vector([r[0], r[1], r[2]])
    }

    fn apply4(&self, v0: i32, v1: i32, v2: i32, v3: i32) -> [i32; 4] {
        let m = &self.0;
        let m0 = &m[0];
        let m1 = &m[1];
        let m2 = &m[2];

        [
            m0[0] * v0 + m0[1] * v1 + m0[2] * v2 + m0[3] * v3,
            m1[0] * v0 + m1[1] * v1 + m1[2] * v2 + m1[3] * v3,
            m2[0] * v0 + m2[1] * v1 + m2[2] * v2 + m2[3] * v3,
            v3,
        ]
    }

    fn extract_translate(&self) -> Vector {
        Vector([self.0[0][3], self.0[1][3], self.0[2][3]])
    }
}

#[derive(Clone)]
struct Scanner(HashSet<Vector>);

impl Scanner {
    fn from_vec(v: &[Vector]) -> Scanner {
        Scanner(v.iter().cloned().collect())
    }

    fn apply_transform(&self, transform: &Matrix) -> Scanner {
        Scanner(self.0.iter().map(|v| transform.apply(v)).collect())
    }

    fn count_common(&self, other: &Scanner, other_transform: &Matrix) -> usize {
        other
            .0
            .iter()
            .cloned()
            .filter(|v| self.0.contains(&other_transform.apply(v)))
            .count()
    }
}

fn try_align(scanner0: &Scanner, scanner1: &Scanner) -> Option<Matrix> {
    // align scanner0 and scanner1 on all positions and try all oriatations in every position
    for pos0 in scanner0.0.iter() {
        for pos1 in scanner1.0.iter() {
            let pre_orient = Matrix::translate(&pos1.negate());
            let post_orient = Matrix::translate(&pos0);
            for orientation in Matrix::all_orientations() {
                let transform = post_orient.mul(&orientation).mul(&pre_orient);
                let count = scanner0.count_common(scanner1, &transform);
                if count >= MIN_PAIR {
                    return Some(transform);
                }
            }
        }
    }
    None
}

fn solve_both_parts(scanners: &[Scanner]) -> (usize, usize) {
    let mut knowns = vec![scanners[0].clone()];
    let mut known_positions = vec![Vector([0, 0, 0])];
    let mut known_queue = vec![knowns[0].clone()];
    let mut unknowns = scanners[1..].iter().collect::<Vec<_>>();

    while unknowns.len() > 0 {
        let known = known_queue.pop().unwrap();
        unknowns.retain(|&unknown| {
            if let Some(transform) = try_align(&known, unknown) {
                let transformed_unknown = unknown.apply_transform(&transform);
                knowns.push(transformed_unknown.clone());
                known_queue.push(transformed_unknown);
                known_positions.push(transform.extract_translate());
                false
            } else {
                true
            }
        })
    }
    let count = Scanner(knowns.iter().map(|s| s.0.clone()).flatten().collect())
        .0
        .len();

    let mut max_dist = 0;
    for i in 0..known_positions.len() {
        for j in i + 1..known_positions.len() {
            let a = &known_positions[i];
            let b = &known_positions[j];
            let dist = ((a[X] - b[X]).abs() + (a[Y] - b[Y]).abs() + (a[Z] - b[Z]).abs()) as usize;
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    (count, max_dist)
}

pub fn main() {
    let input = parse_input(&std::fs::read_to_string("input/day19.txt").unwrap());
    test_matrices();
    test_all_orientations();
    test_matrix_translate();
    test0(input.get("test0").unwrap());
    test1(input.get("test1").unwrap());
    println!("tests ok");

    let mut day19_scanners = vec![];
    let mut i = 0;
    let day19_input = input.get("day19").unwrap();
    while let Some(vec) = day19_input.get(&format!("scanner {}", i)) {
        day19_scanners.push(Scanner::from_vec(vec));
        i += 1;
    }
    println!("day19 pt1, pt2: {:?}", solve_both_parts(&day19_scanners));
}

fn test1(test1_input: &Input) {
    let scanners = (0..=4)
        .map(|n| test1_input.get(&format!("scanner {}", n)).unwrap())
        .map(|v| Scanner::from_vec(v))
        .collect::<Vec<_>>();

    // try align scanners 0 and 1
    let align_result = try_align(&scanners[0], &scanners[1]).unwrap();
    let transformed_1 = scanners[1].apply_transform(&align_result);
    assert_eq!(
        try_align(&scanners[0], &transformed_1),
        Some(Matrix::identity())
    );
    assert_eq!(align_result.extract_translate().0, [68, -1246, -43]);

    let align_result = try_align(&transformed_1, &scanners[4]).unwrap();
    assert_eq!(align_result.extract_translate().0, [-20, -1133, 1061]);

    assert_eq!(solve_both_parts(&scanners), (79, 3621));
}

fn test0(test0_input: &Input) {
    // all given scanners are same but in different orientations
    let first = test0_input.get("scanner 0-0").unwrap();
    for (_, vecs) in test0_input.iter() {
        let found = Matrix::all_orientations().iter().any(|m| {
            vecs.iter().zip(first.iter()).all(|(v, first_v)| {
                let transformed = m.apply(v);
                transformed == *first_v
            })
        });
        assert!(found);
    }
}

fn test_matrix_translate() {
    let v = Vector([1, 2, 3]);
    assert_eq!(
        Matrix::translate(&Vector([1, 1, 1])).apply(&v),
        Vector([2, 3, 4])
    );

    let m = Matrix::translate(&Vector([1, 2, 3]));
    let m = m.mul(&Matrix::translate(&Vector([-1, -2, -3])));
    assert_eq!(m, Matrix::identity());

    // move x+1, then rotate cw
    let m1 = Matrix::rotate90cw(Z).mul(&Matrix::translate(&Vector([1, 0, 0])));
    // rotate cw, then move y-1
    let m2 = Matrix::translate(&Vector([0, -1, 0])).mul(&Matrix::rotate90cw(Z));
    assert_eq!(m1, m2);
}

fn test_all_orientations() {
    // should be 24 transforms: facing positive or negative x, y, or z, and considering any of four directions "up" from that facing.
    let mut all = Matrix::all_orientations();
    assert_eq!(all.len(), 24);

    // they should not duplicate
    all.dedup();
    assert_eq!(all.len(), 24);

    // transformed vectors should not duplicate
    let mut all_v = all
        .iter()
        .map(|m| m.apply(&Vector([1, 2, 3])))
        .collect::<Vec<_>>();
    all_v.dedup();
    assert_eq!(all_v.len(), 24);

    // unit vectors facing along axis should not change when rotating along this axis, so one of the axis aligned unit vectors should transform to only 6 unique vectors
    let mut total_unit_count = 0;
    for unit in [Vector([1, 0, 0]), Vector([0, 1, 0]), Vector([0, 0, 1])] {
        let mut all_unit_transforms = all.iter().map(|m| m.apply(&unit)).collect::<Vec<_>>();
        all_unit_transforms.dedup();
        total_unit_count += all_unit_transforms.len();
    }
    assert_eq!(total_unit_count, 24 + 24 + 6);
}

fn test_matrices() {
    let vec = Vector([1, 2, 3]);
    assert_eq!(Matrix::identity().apply(&vec), vec);
    assert_eq!(Matrix::rotate90cw(X).apply(&vec), Vector([1, 3, -2]));
    assert_eq!(Matrix::rotate90cw(Y).apply(&vec), Vector([-3, 2, 1]));
    assert_eq!(Matrix::rotate90cw(Z).apply(&vec), Vector([2, -1, 3]));

    for axis in Axis::all() {
        let mut v = vec.clone();
        for _ in 0..4 {
            let v1 = Matrix::rotate90cw(axis).apply(&v);
            assert_eq!(v[axis], v1[axis]); // rotation around axis should not change this vector value for this axis
            v = v1;
        }
        assert_eq!(vec, v); // after four rotations vector should be the same
    }

    // test that apply rotate90cw n times is same as rotate_n_90cw
    for axis in Axis::all() {
        for n in 0..5 {
            let mut v = vec.clone();
            for _ in 0..n {
                v = Matrix::rotate90cw(axis).apply(&v);
            }
            assert_eq!(v, Matrix::rotate_n_90cw(axis, n).apply(&vec));
        }
    }

    // test that consequtive rotating is same as multiplying matrices and that applying resulting transform
    let mut v = vec.clone();
    let mut m = Matrix::identity();
    for axis in Axis::all() {
        /*
        note on order of operations:
            v = m * v_init
            v_next = op * v
            v_next = op * m * v_init
            v_next = (op * m) * v_init;
            v_next = m_next * v_init
            m_next = op * m
         */
        let op = Matrix::rotate90cw(axis);
        m = op.mul(&m);
        v = op.apply(&v);
    }
    assert_eq!(v, m.apply(&vec));
}

fn parse_input(s: &str) -> HashMap<String, Input> {
    let mut result = HashMap::new();
    let mut lines = s.split("\n").map(|s| s.trim());
    while let Some(input_name) = lines.next() {
        let mut input = HashMap::new();
        while let Some(scanner) = lines.next() {
            if scanner.len() == 0 {
                break;
            }
            let scanner_name = &scanner[4..scanner.len() - 4];
            let mut scanner_vecs = vec![];
            while let Some(vec) = lines.next() {
                if vec.len() == 0 {
                    break;
                }
                let vec = Vector(
                    <[i32; 3]>::try_from(
                        vec.split(',')
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                    .unwrap(),
                );
                scanner_vecs.push(vec);
            }
            input.insert(scanner_name.to_string(), scanner_vecs);
        }
        result.insert(input_name.to_string(), input);
    }
    result
}
