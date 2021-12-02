mod day01;
mod day02;
mod util;

fn ex<T, F1, F2>(day: i32, input: &T, p1: F1, p2: F2)
where
    F1: Fn(&T) -> i32,
    F2: Fn(&T) -> i32,
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
    )
}
