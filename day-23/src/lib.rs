use std::collections::{HashMap, VecDeque};

pub fn process_part1(input: &str) -> String {
    let mut cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut current_cup = *cups.first().unwrap();
    (0..100).for_each(|_| {
        // rotate so that current cup is the first cup
        let current_index = cups.iter().position(|&x| x == current_cup).unwrap();
        cups.rotate_left(current_index);

        // The crab picks up the three cups that are immediately clockwise of the current cup.
        // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let crab = vec![cups.remove(1), cups.remove(1), cups.remove(1)];

        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        // If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up.
        // If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
        let mut destination_cup = current_cup - 1;
        while !cups.contains(&destination_cup) {
            if destination_cup == 0 {
                destination_cup = 10;
            }
            destination_cup -= 1;
        }
        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
        let destination_index = cups.iter().position(|&x| x == destination_cup).unwrap() + 1;
        crab.into_iter().enumerate().for_each(|(i, cup)| {
            cups.insert(destination_index + i, cup);
        });

        // The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
        let current_index = cups.iter().position(|&x| x == current_cup).unwrap();
        current_cup = cups[(current_index + 1) % 9];
    });
    let current_index = cups.iter().position(|&x| x == current_cup).unwrap() + 1;
    cups.rotate_left(current_index);
    cups.pop();
    let mut answer = String::new();
    cups.into_iter()
        .for_each(|cup| answer.push_str(&cup.to_string()));
    answer
}

// pub fn process_part2(input: &str) -> String {
//     let mut cups: Vec<usize> = input
//         .chars()
//         .map(|c| c.to_digit(10).unwrap() as usize)
//         .collect();
//     cups.reserve_exact(1_000_000);
//     cups.extend(10..1_000_001);
//     let mut current_cup = *cups.first().unwrap();
//     (0..10_000_000).for_each(|run| {
//         if run % 1_000 == 0 {
//             println!("{run}");
//         }
//         // rotate so that current cup has at least three to the right of it
//         let mut current_index = cups.iter().position(|&x| x == current_cup).unwrap();
//         if current_index > 999_996 {
//             cups.rotate_left(3);
//             current_index -= 3;
//         }

//         // The crab picks up the three cups that are immediately clockwise of the current cup.
//         // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
//         // let crab = vec![cups.remove(1), cups.remove(1), cups.remove(1)];

//         let crab: Vec<_> = cups
//             .splice(current_index..(current_index + 3), vec![])
//             .collect();

//         // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
//         // If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up.
//         // If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
//         let mut destination_cup = current_cup - 1;
//         if destination_cup == 0 {
//             destination_cup = 1_000_000;
//         }
//         while crab.contains(&destination_cup) {
//             destination_cup -= 1;
//             if destination_cup == 0 {
//                 destination_cup = 1_000_000;
//             }
//         }
//         // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
//         let destination_index = cups.iter().position(|&x| x == destination_cup).unwrap() + 1;
//         cups.splice(destination_index..destination_index, crab);

//         // The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
//         if destination_index < current_index {
//             current_index += 4;
//         } else {
//             current_index += 1;
//         }
//         current_index %= 999_999;
//         current_cup = cups[current_index];
//     });
//     let current_index = cups.iter().position(|&x| x == current_cup).unwrap();
//     cups.rotate_left(current_index);
//     (cups[1] * cups[2]).to_string()
// }

pub fn process_part2(input: &str) -> String {
    let mut cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let max_cup = 1_000_000;
    let iterations = 10_000_000;

    for i in cups.len() + 1..=max_cup {
        cups.push(i);
    }

    let mut next_cup = vec![0; max_cup + 1];

    for i in 0..max_cup {
        next_cup[cups[i]] = cups[(i + 1) % max_cup];
    }

    let mut current_cup = cups[0];

    for _ in 0..iterations {
        let cup1 = next_cup[current_cup];
        let cup2 = next_cup[cup1];
        let cup3 = next_cup[cup2];

        let mut destination_cup = current_cup;
        loop {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = max_cup;
            }
            if destination_cup != cup1 && destination_cup != cup2 && destination_cup != cup3 {
                break;
            }
        }

        next_cup[current_cup] = next_cup[cup3];
        let after_destination = next_cup[destination_cup];
        next_cup[destination_cup] = cup1;
        next_cup[cup3] = after_destination;

        current_cup = next_cup[current_cup];
    }

    let cup1 = next_cup[1];
    let cup2 = next_cup[cup1];
    let result = cup1 * cup2;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "389125467";
        let result = process_part1(input);
        // assert_eq!(result, "92658374");
        assert_eq!(result, "67384529");
    }

    #[test]
    fn part2() {
        let input = "389125467";
        let result = process_part2(input);
        assert_eq!(result, "149245887792");
    }
}
