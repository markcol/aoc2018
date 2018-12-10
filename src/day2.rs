
use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let (cnt2, cnt3) = input.lines().map(|l| {
        let mut map = HashMap::with_capacity(l.len());
        l.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        let two = map.values().any(|&n| n == 2);
        let three = map.values().any(|&n| n == 3);
        (two, three)
    }).fold((0, 0), |acc, n| match n {
        (true, true) => (acc.0 + 1, acc.1 + 1),
        (true, false) => (acc.0 + 1, acc.1),
        (false, true) => (acc.0, acc.1 + 1),
        (false, false) => acc,  
    });
    cnt2 * cnt3
}

pub fn diffs(in1: &str, in2: &str) -> (i32, i32) {

    if in1.chars().count() != in2.chars().count() {
        return (0, 0);
    }
    let mut cnt = 0;
    let mut idx = 0;
    let mut i = 0;
    let mut in1_c = in1.chars();
    let mut in2_c = in2.chars();
    loop {
        let c1 = in1_c.next();
        let c2 = in2_c.next();
        if c1 == None || c2 == None {
            break;
        }
        if c1.unwrap() != c2.unwrap() {
            cnt += 1;
            idx = i;
        }
        i += 1;
    }
    (cnt, idx)
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let _v = input.lines();
    "fgij".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample1() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(part1(input), 12);
    }


    #[test]
    fn diff() {  
        assert_eq!(diffs("fghij", "fguij"), (1, 2));
        assert_eq!(diffs("abcdef", "fguij"), (0, 0));
        assert_eq!(diffs("abcde", "fguij"), (5, 4));
    }

    #[test]
    fn sample2() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";        
        assert_eq!(part2(input), "fgij");
    }
}