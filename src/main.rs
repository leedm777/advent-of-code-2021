mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod util;

fn ex<T, T2, F1, F2>(day: i32, input: &T, p1: F1, p2: F2)
where
    F1: Fn(&T) -> T2,
    F2: Fn(&T) -> T2,
    T2: std::fmt::Display,
{
    let n1 = std::time::Instant::now();
    let r1 = p1(input);
    let t1 = n1.elapsed().as_secs_f64() * 1000.;

    let n2 = std::time::Instant::now();
    let r2 = p2(input);
    let t2 = n2.elapsed().as_secs_f64() * 1000.;

    println!("Day {},\tpart 1: {} ({:.3} ms)", day, r1, t1);
    println!("       \tpart 2: {} ({:.3} ms)", r2, t2);
}

fn ex2<IN, OUT, FP, F1, F2>(day: i32, parse: FP, p1: F1, p2: F2)
where
    OUT: std::fmt::Display,
    FP: Fn(&str) -> IN,
    F1: Fn(&IN) -> OUT,
    F2: Fn(&IN) -> OUT,
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
    ex(
        1,
        &util::file_as_numbers("./src/day01.txt"),
        day01::part1,
        day01::part2,
    );

    ex(
        2,
        &util::file_as_strings("./src/day02.txt"),
        day02::part1,
        day02::part2,
    );

    ex2(3, day03::parse, day03::part1, day03::part2);
    ex2(4, day04::parse, day04::part1, day04::part2);
    ex2(5, day05::parse, day05::part1, day05::part2);
    ex2(6, day06::parse, day06::part1, day06::part2);
    ex2(7, day07::parse, day07::part1, day07::part2);
    ex2(8, day08::parse, day08::part1, day08::part2);
}
