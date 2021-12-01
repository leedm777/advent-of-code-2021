mod day01;
mod util;

fn main() {
    let d01 = util::file_as_numbers("./src/day01.txt");
    let d01p1 = day01::part1(&d01);
    let d01p2 = day01::part2(&d01);

    println!("Day 1, part 1: {}", d01p1);
    println!("       part 2: {}", d01p2);
}
