# Advent of Code 2024
My solutions to AoC2024 with RUST [Advent of Code 2024](https://adventofcode.com/2024). Hopefully they are not too horrendous (I'm still a begineer 🥺). All except the first 2 days are proudly written at 5am each day to hopefully fix my sleep schedule 🥰. Probably a huge waste of time considering the solve time per day. At least its fun so far.

## Repo Structure
Solution for each day is contained within its own folder as a seperate Rust project. Answers for the example and my inputs are at the bottom of each `lib.rs` file (or in `main.rs` if there is no `lib.rs` file).

## Benchmarks
I will be benchmarking each day with the release version by using the `time` command on terminal.
### Process
Build release version with `cargo build --release`.
Then time the code multiple times with `time target/release/day_n`.
### Specs
MacBook Air M1

## Notes
| **Day** | **Benchmark (approximate)** | **Solve Time** | **Notes** |
|:---:|:---:|:---:|:---:|
|[1](day01)| 0.012s | >24h | Just simple operations between two lists.
|[2](day02)| 0.013s | 11:44:11 | Part 1 was straightforward but I brute forced Part 2.
|[3](day03)| 0.012s | 01:36:44 | Don't know how to use regex so I parsed mul(num,num) manually. The multiple conditions are slightly ugly 😷. At least from my research I was told this would make it faster :>
|[4](day04)| 0.014s | 01:57:26 | Feel kinda bad for using another library to find the diagonals but it made things a lot simpler for Part 1.
|[5](day05)| 0.022s | 02:10:56 | Learnt about `HashSet` and that I can use custom ordering rules with `sort_by`. I was going to just randomize the order 🫣.
|[6](day06)| 0.059s | 02:14:30 | The code is getting longer and harder to read 😰 but the nested matches were fun to write. Part 2 was another brute force method but I don't see any other way to do it (but then again what do I know). Without threading, it takes 5s plus (forgot the time 👺) with `cargo run` on debug. I had to relearn `Arc` cause its been a while since I read the rustbook.
|[7](day07)| 0.122s | 01:20:01 | More brute force solutions 🤡!!! I tried to remove combinations that start with the same operators if they pass the target value but I could not get this to be faster than just calculating everything. Something to do with iterating through all the combinations and matching the operators offsets the time reduction 😑. Also used `multi_cartesian_product` from itertools to generate all permutations with replacement cause my brain at 5am can not handle that 🫠.
|[8](day08)| 0.011s | 00:59:47 | Surprisingly simple today. Finally beat top guy on the private leaderboard (4th this time). Sub 1h today but decided to write this `README` file so no sleep today either...
|[9](day09)|
|[10](day10)|
|[11](day11)|
|[12](day12)|
|[13](day13)|
|[14](day14)|
|[15](day15)|
|[16](day16)|
|[17](day17)|
|[18](day18)|
|[19](day19)|
|[20](day20)|
|[21](day21)|
|[22](day22)|
|[23](day23)|
|[24](day24)|
|[25](day25)|
