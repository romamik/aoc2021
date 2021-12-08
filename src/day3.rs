use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error;

pub fn main() {
    
    let test_vec = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
    let day3_vec = read_binary_ints("input/day3.txt").unwrap();
    println!("test pt1 {0}", get_pt1_result(&test_vec));
    println!("day3 pt1 {0}", get_pt1_result(&day3_vec));
    println!("test pt2 {0}", get_pt2_result(&test_vec));
    println!("day3 pt2 {0}", get_pt2_result(&day3_vec));
}

fn get_pt2_result(values: &[u32]) -> u32 {
    
    let mcb_result = filter_values(values, FilterCriteria::MostCommonBit);
    let lcb_result = filter_values(values, FilterCriteria::LeastCommonBit);
    //println!("{} {}", mcb_result, lcb_result);
    mcb_result * lcb_result
}

enum FilterCriteria {
    MostCommonBit,
    LeastCommonBit,
}

fn filter_values(values: &[u32], criteria: FilterCriteria) -> u32 {

    let mut vec = values.to_vec();
    let mut bit = 0_u8;
    while vec.len() > 1 && bit < 32 {
        let (mcb, lcb) = get_most_least_common_bits(&vec, bit);
        let wanted_bit_value = match criteria {
            FilterCriteria::MostCommonBit => mcb,
            FilterCriteria::LeastCommonBit => lcb
        };
        vec = vec.into_iter().filter(|value| get_bit_value(*value, bit) == wanted_bit_value).collect();
        bit += 1;
        //println!("{} {} {:?}", 32-bit, wanted_bit_value, vec.iter().map(|v| format!("{:b}", &v)).collect::<Vec<String>>());
    }
    vec[0]
}
    
// // здесь всё не так, надо искать mcp и lcp на каждом шаге, среди оставшихся векторов

//     let mut vec: Vec::<u32> = values.to_vec();
//     let (mcp, lcp) = get_most_least_common_bits(values);
//     let src = mcp;
//     let mut bit = 31;
//     println!("{:b}", mcp);
//     while vec.len() > 1 {

//         let src_bit = (src & (1 << bit)) != 0;
//         vec = vec.into_iter().filter(|val| {
//             let val_bit = val & (1 << bit) != 0;
//             val_bit == src_bit
//         }).collect::<Vec<u32>>();
//         println!("{} {} {:?}", src_bit, bit, vec.iter().map(|v| format!("{:b}", &v)).collect::<Vec<String>>());
//         bit -= 1;
//     }
//     println!("{}", vec[0]);
//     0
// }

fn get_pt1_result(values: &[u32]) -> u32 {
    
    let mut mcb = 0_u32;
    let mut lcb = 0_u32;
    
    for bit in 0..32_u8 {
        let (mcb_bit, lcb_bit) = get_most_least_common_bits(values, bit);
        mcb = set_bit_value(mcb, bit, mcb_bit);
        lcb = set_bit_value(lcb, bit, lcb_bit);
    }
    mcb * lcb
}

fn get_most_least_common_bits(values: &[u32], bit: u8) -> (bool, bool) {

    let mut set_bit_count = 0_usize;
    for value in values.iter() {
        let bit_value = get_bit_value(*value, bit);
        if bit_value { set_bit_count += 1 }
    }
    let unset_bit_count = values.len() - set_bit_count;
    let mcb = set_bit_count >= unset_bit_count; // most common bit
    let lcb = set_bit_count != 0 && unset_bit_count > set_bit_count; // least common bit. if all bits are 0, then least common bit is still 0 
    (mcb, lcb)
}

fn get_bit_value(value: u32, bit: u8) -> bool {
    value & (1 << (31 - bit)) != 0
}

fn set_bit_value(value: u32, bit: u8, bit_value: bool) -> u32 {
    if bit_value { set_bit(value, bit) } else { reset_bit(value, bit) }
}

fn set_bit(value: u32, bit: u8) -> u32 {
    value | (1 << (31 - bit))
}

fn reset_bit(value: u32, bit: u8) -> u32 {
    return value & !(1 << (31 - bit))
}

fn read_binary_ints<P>(filename: P) -> Result<Vec::<u32>, Box<dyn error::Error>> 
where P: AsRef<Path> {

    let mut vec = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines {
        vec.push(u32::from_str_radix(&line?, 2)?);
    }
    Ok(vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}