use regex::Regex;

#[derive(Debug)]
struct Mul(usize, usize);

impl Mul {
    fn result(&self) -> usize {
        self.0 * self.1
    }
}

fn main() {
    //     let input = "
    // xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    // ";
    let input = include_str!("../../../inputs/day-3.txt");

    let mul_re = Regex::new(r"mul\((?<v1>[0-9]{1,3})\,(?<v2>[0-9]{1,3})\)").unwrap();
    let res = input
        .trim()
        .lines()
        .map(|line| {
            mul_re
                .captures_iter(line)
                .map(|caps| {
                    let v1 = caps.name("v1").unwrap().as_str().parse::<usize>().unwrap();
                    let v2 = caps.name("v2").unwrap().as_str().parse::<usize>().unwrap();
                    Mul(v1, v2)
                })
                .map(|elem| elem.result())
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_1() {
        assert!(true)
    }
}
