# Advent of Code 2024
My solutions to AoC2024 with RUST [Advent of Code 2024](https://adventofcode.com/2024). Hopefully they are not too horrendous (I'm still a begineer 🥺). All except the first 2 days are proudly written at 5am each day to hopefully fix my sleep schedule 🥰. Probably a huge waste of time considering the solve time per day. Also will be quite reliant on Reddit for help but I will try to limit to hints and only take their approach (not copy-paste code) if I am really stumped.

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
|[9](day09)| 0.280s | 01:41:28 | Very straight-forward again today. The time is a lot higher than the others but I don't know how to run it in parallel since each computation relies on the previous ones. Sub 1s is good enough :3
|[10](day10)| 0.014s | 01:03:29 | Another easy day. Using recursion simplifies the path finding and part2 is just counting the times the recursive function reached 9. The solve time could have been at least 10 minutes faster if I didn't write `break` instead of `continue`. Again, sub 1s so I won't bother multi-threading this.
|[11](day11)| 0.020s | 01:15:30 | This one needed memory/caching previous values because the values will take up 30+ Gb of data (found out from experience lol). Of course I had no idea how to do such a thing and almost tried writing the data to a file. I had to use reddit to figure out that I needed to cache with HashMap.
|[12](day12)| 0.027s | 02:07:55 | I initially had no idea how to seperate the different regions of each plant. Reddit (my savior) pointed me towards BFS (breadth first search). It sounds fancy but it was just recursively checking for neighboring plants of the same type.
|[13](day13)| 0.012s | 01:47:10 | Today's problem was quite nice since it was simply solving a linear system of equations. Conviniently, I was looking at solving these systems with matrices for my maths module the day before. Regardless, it still took a while because as always, I confused myself with the matrix notation (even though it was a 2x2 matrix). Luckily, all of the systems had only one unique solution which I found very odd considering the numerous annoying edge cases so far in this event. Part 2 was just Part 1 with larger numbers which was no problem for the matrix solution. Very glad that I didn't try to brute force the solution for Part 1 by just adding :3.
|[14](day14)| 0.104s | 01:47:29 | Part 1 was the usual simulation stuff but Part 2 was really confusing. I did not know what the christmas tree that I was supposed to find should look like. I ended up finding an answer by looking for when the bots had no overlaps but I was told that some people's tree did have overlap (unlucky). I also tried adding up bots with at least one neighbor and setting the tree condition as when this sum is over a certain threshold, but this requies simulating all 101*103 time steps which was a lot slower. The resulting tree does look very satisfying though [tree.txt](day14/tree.txt).
|[15](day15)| 0.015s | 02:20:54 | Very slow today since I haven't slept yet (too much Valo). Moving the boxes was just recursively checking if the next box is able to move. Part 2 was a lot more confusing for me because I was lazy and wanted to keep the same `shift` function that only moves one part. It was not possible because moving the 2-width boxes vertically depends on both parts, so I reluctantly made a seperate function for vertical moves. Horizontal movement is still the same. Watching the boves move is oddly satisfying.
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
