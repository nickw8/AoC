use day_05::part1::process as part1;
use day_05::part2::process as part2;

use miette::Context;

fn main() -> miette::Result<()> {
    let file1 = include_str!("../../input1.txt");
    let result1 = part1(file1).context("process part 1")?;
    println!("part 1 results: {}", result1);

    let file2 = include_str!("../../input2.txt");
    let result2 = part2(file2).context("process part 2")?;
    println!("part 2 results: {}", result2);

    Ok(())
}
