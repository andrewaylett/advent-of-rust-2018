mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

pub fn day01() {
    day01::star_one("");
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    macro_rules! test_solve {
        ($day:ident, $one:expr, $two:expr) => {
            #[test]
            fn $day() {
                use $day::{star_one, star_two};

                let input = load_file(&format!("{}.txt", stringify!($day)));

                assert_eq!(star_one(&input), $one);
                assert_eq!(star_two(&input), $two);
            }
        }
    }

    test_solve!(day01, 1, 1);
    test_solve!(day02, 1, 1);
    test_solve!(day03, 1, 1);
    test_solve!(day04, 1, 1);
    test_solve!(day05, 1, 1);
    test_solve!(day06, 1, 1);
    test_solve!(day07, 1, 1);
    test_solve!(day08, 1, 1);
    test_solve!(day09, 1, 1);
    test_solve!(day10, 1, 1);
    test_solve!(day11, 1, 1);
    test_solve!(day12, 1, 1);
    test_solve!(day13, 1, 1);
    test_solve!(day14, 1, 1);
    test_solve!(day15, 1, 1);
    test_solve!(day16, 1, 1);
    test_solve!(day17, 1, 1);
    test_solve!(day18, 1, 1);
    test_solve!(day19, 1, 1);
    test_solve!(day20, 1, 1);
    test_solve!(day21, 1, 1);
    test_solve!(day22, 1, 1);
    test_solve!(day23, 1, 1);
    test_solve!(day24, 1, 1);
}
