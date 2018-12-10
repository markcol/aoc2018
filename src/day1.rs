use fnv::FnvHashSet;

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[isize]) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[isize]) -> isize {
    let max_iters = 100_000;
    let mut iters = 0;
    let mut freq = 0;
    let mut set = FnvHashSet::default();
    set.insert(freq);
    loop {
        for v in input {
            freq += v;
            if !set.insert(freq) {
                return freq;
            }
        }
        iters += 1;
        if iters > max_iters {
            return isize::max_value();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    #[test]
    fn sample1() {
        assert_eq!(part1(&vec![1, -1]), 0);
        assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
        assert_eq!(part1(&vec![1, 1, 1]), 3);
        assert_eq!(part1(&vec![1, 1, -2]), 0);
        assert_eq!(part1(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&vec![1, -1]), 0);
        assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
        assert_eq!(part2(&vec![1, 2, 3, 4, 5]), isize::max_value());
    }
}