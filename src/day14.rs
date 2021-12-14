use std::collections::HashMap;

type Rules = HashMap<[char; 2], char>;
#[derive(Debug)]
struct Input {
    template: String,
    rules: Rules,
}

fn insert(template: &str, rules: &Rules) -> String {
    let mut result = String::new();
    let mut iter = template.chars();
    let mut prev_char = iter.next().unwrap();
    for cur_char in iter {
        result.push(prev_char);
        if let Some(&to_insert) = rules.get(&[prev_char, cur_char]) {
            result.push(to_insert);
        }
        prev_char = cur_char;
    }
    result.push(prev_char);
    result
}

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in s.chars() {
        let count = *result.get(&c).unwrap_or(&0) + 1;
        result.insert(c, count);
    }
    result
}

fn find_min_max(char_count: &HashMap<char, usize>) -> [Option::<(char, usize)>; 2] {
    let mut min = None;
    let mut max = None;
    for (&ch, &count) in char_count.iter() {
        match min {
            None => { min = Some((ch, count)); }
            Some(p) if p.1 > count => { min = Some((ch, count)); }
            _ => (),
        }
        match max {
            None => { max = Some((ch, count)); }
            Some(p) if p.1 < count => { max = Some((ch, count)); }
            _ => (),
        }
    }
    [min, max]
}

fn naive_solve(input: &Input, steps: usize) -> usize {
    let mut s = input.template.clone();
    for _ in 0..steps {
        s = insert(&s, &input.rules);
    }
    let char_count = count_chars(&s);
    let [min, max] = find_min_max(&char_count);
    max.unwrap().1 - min.unwrap().1
}

fn better_solve(input: &Input, steps: usize) -> usize {

    fn insert_pair(hash_map: &mut HashMap::<[char;2], usize>, pair: &[char;2], count: usize) {
        let count = hash_map.get(pair).unwrap_or(&0) + count;
        hash_map.insert(*pair, count);
    }

    let mut rules = HashMap::new();

    for (&from, &to_insert) in input.rules.iter() {
        let to0 = [from[0], to_insert];
        let to1 = [to_insert, from[1]];
        rules.insert(from, [to0, to1]);
    }
    
    let mut pairs = HashMap::new();
    let mut iter = input.template.chars();
    let mut prev_char = iter.next().unwrap();
    for cur_char in iter {
        insert_pair(&mut pairs, &[prev_char, cur_char], 1);
        prev_char = cur_char;
    }
    insert_pair(&mut pairs, &[prev_char, '!'], 1); // fake pair that will never expand, now can just count first chars of the pairs
    
    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for (&pair, &count) in pairs.iter() {
            if let Some([p0, p1]) = rules.get(&pair) {
                insert_pair(&mut new_pairs, p0, count);
                insert_pair(&mut new_pairs, p1, count);
            }
            else {
                insert_pair(&mut new_pairs, &pair, count);
            }
        }
        pairs = new_pairs;
    }

    let mut char_count = HashMap::new();
    for ([ch, _], count) in pairs.iter() {
        let count = char_count.get(ch).unwrap_or(&0) + count;
        char_count.insert(*ch, count);
    }

    let [min, max] = find_min_max(&char_count);
    max.unwrap().1 - min.unwrap().1
}

fn naive_solve_pt1(input: &Input) -> usize {
    naive_solve(input, 10)
}

fn solve_pt1(input: &Input) -> usize {
    better_solve(input, 10)
}

fn solve_pt2(input: &Input) -> usize {
    better_solve(input, 40)
}

fn test1(input: &Input) {
    let rules = &input.rules;
    let s = insert(&input.template, rules);
    assert_eq!(s, "NCNBCHB");
    let s = insert(&s, rules);
    assert_eq!(s, "NBCCNBBBCBHCB");
    let s = insert(&s, rules);
    assert_eq!(s, "NBBBCNCCNBBNBNBBCHBHHBCHB");
    let s = insert(&s, rules);
    assert_eq!(s, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    let s = insert(&s, rules);
    assert_eq!(s.len(), 97);
    let mut s = s;
    for _ in 0..5 {
        s = insert(&s, rules);
    }
    let char_count = count_chars(&s);
    assert_eq!(char_count.get(&'B'), Some(&1749));
    assert_eq!(char_count.get(&'C'), Some(&298));
    assert_eq!(char_count.get(&'H'), Some(&161));
    assert_eq!(char_count.get(&'N'), Some(&865));
    assert_eq!(naive_solve_pt1(input), 1588);
    assert_eq!(solve_pt1(input), 1588);
    assert_eq!(solve_pt2(input), 2188189693529);
}

pub fn main() {
    let input = read_input("input/day14.txt");

    test1(input.get("test1").unwrap());

    let day14_input = input.get("day14").unwrap();
    println!("day14 pt1 {}", solve_pt1(day14_input));
    println!("day14 pt2 {}", solve_pt2(day14_input));
    
}

fn read_input(filename: &str) -> HashMap<String, Input> {
    let s = std::fs::read_to_string(filename).unwrap();
    let mut lines = s.split("\n").map(|s| s.trim()).collect::<Vec<_>>();
    lines.reverse();
    let mut result = HashMap::new();
    while let Some(name) = lines.pop() {
        if name.len() == 0 {
            continue;
        }
        let template = lines.pop().unwrap();
        assert_eq!(lines.pop(), Some(""));
        let mut rules = Rules::new();
        while let Some(line) = lines.pop() {
            if line.len() == 0 {
                break;
            }
            let mut rule_iter = line.split("->").map(|s| s.trim());
            let from = <[char; 2]>::try_from(rule_iter.next().unwrap().chars().collect::<Vec<_>>())
                .unwrap();
            let to = <[char; 1]>::try_from(rule_iter.next().unwrap().chars().collect::<Vec<_>>())
                .unwrap()[0];
            rules.insert(from, to);
        }
        result.insert(
            name.to_string(),
            Input {
                template: template.to_string(),
                rules,
            },
        );
    }
    result
}
