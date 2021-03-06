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
mod day25;
mod util;

fn ex2<IN, OUT1, OUT2, FP, F1, F2>(day: i32, parse: FP, p1: F1, p2: F2)
where
    OUT1: std::fmt::Display,
    OUT2: std::fmt::Display,
    FP: Fn(&str) -> IN,
    F1: Fn(&IN) -> OUT1,
    F2: Fn(&IN) -> OUT2,
{
    let input = util::read_input(day);
    let n0 = std::time::Instant::now();
    let input = parse(&input);

    let n1 = std::time::Instant::now();
    let r1 = p1(&input);
    let t1 = n1.elapsed().as_secs_f64() * 1000.;

    let n2 = std::time::Instant::now();
    let r2 = p2(&input);
    let t2 = n2.elapsed().as_secs_f64() * 1000.;

    let t_total = n0.elapsed().as_secs_f64() * 1000.;

    println!("Day {},\tpart 1: {} ({:.3} ms)", day, r1, t1);
    println!("       \tpart 2: {} ({:.3} ms)", r2, t2);
    println!("       \ttotal: {:.3} ms", t_total);
    println!();
}

fn main() {
    ex2(1, day01::parse, day01::part1, day01::part2);
    ex2(2, day02::parse, day02::part1, day02::part2);
    ex2(3, day03::parse, day03::part1, day03::part2);
    ex2(4, day04::parse, day04::part1, day04::part2);
    ex2(5, day05::parse, day05::part1, day05::part2);
    ex2(6, day06::parse, day06::part1, day06::part2);
    ex2(7, day07::parse, day07::part1, day07::part2);
    ex2(8, day08::parse, day08::part1, day08::part2);
    ex2(9, day09::parse, day09::part1, day09::part2);
    ex2(10, day10::parse, day10::part1, day10::part2);
    ex2(11, day11::parse, day11::part1, day11::part2);
    ex2(12, day12::parse, day12::part1, day12::part2);
    ex2(13, day13::parse, day13::part1, day13::part2);
    ex2(14, day14::parse, day14::part1, day14::part2);
    ex2(15, day15::parse, day15::part1, day15::part2);
    ex2(16, day16::parse, day16::part1, day16::part2);
    ex2(17, day17::parse, day17::part1, day17::part2);
    ex2(18, day18::parse, day18::part1, day18::part2);
    ex2(19, day19::parse, day19::part1, day19::part2);
    ex2(20, day20::parse, day20::part1, day20::part2);
    ex2(21, day21::parse, day21::part1, day21::part2);
    ex2(22, day22::parse, day22::part1, day22::part2);
    ex2(23, day23::parse, day23::part1, day23::part2);
    ex2(24, day24::parse, day24::part1, day24::part2);
    ex2(25, day25::parse, day25::part1, day25::part2);
}
