mod day1;
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
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    #[test]
    fn solve_day1() {
        use day1::{star_one, star_two};

        let input = load_file("day1.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day2() {
        use day2::{star_one, star_two};

        let input = load_file("day2.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day3() {
        use day3::{star_one, star_two};

        let input = load_file("day3.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day4() {
        use day4::{star_one, star_two};

        let input = load_file("day4.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day5() {
        use day5::{star_one, star_two};

        let input = load_file("day5.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day6() {
        use day6::{star_one, star_two};

        let input = load_file("day6.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day7() {
        use day7::{star_one, star_two};

        let input = load_file("day7.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day8() {
        use day8::{star_one, star_two};

        let input = load_file("day8.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day9() {
        use day9::{star_one, star_two};

        let input = load_file("day9.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day10() {
        use day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day11() {
        use day11::{star_one, star_two};

        let input = load_file("day11.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day12() {
        use day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day13() {
        use day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day14() {
        use day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day15() {
        use day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day16() {
        use day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day17() {
        use day17::{star_one, star_two};

        let input = load_file("day17.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day18() {
        use day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day19() {
        use day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day20() {
        use day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day21() {
        use day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day22() {
        use day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day23() {
        use day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
    #[test]
    fn solve_day24() {
        use day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}