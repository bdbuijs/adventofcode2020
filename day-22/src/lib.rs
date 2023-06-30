use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::char as nomchar,
    character::complete::{digit1, newline},
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, (mut player1, mut player2)) = parse_input(input).unwrap();
    let mut game_over = false;
    while !game_over {
        let (card1, card2) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        match card1.cmp(&card2) {
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Less => {
                player2.push_back(card2);
                player2.push_back(card1);
            }
            std::cmp::Ordering::Greater => {
                player1.push_back(card1);
                player1.push_back(card2);
            }
        }
        game_over = player1.is_empty() || player2.is_empty();
    }
    let mut winner = match player1.len() {
        0 => player2,
        _ => player1,
    };
    winner.push_back(0);
    let score: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum();
    score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (player1, player2)) = parse_input(input).unwrap();
    let (_, mut winner) = recursive_game(player1, player2);
    winner.push_back(0);
    let score: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum();
    score.to_string()
}

fn recursive_game(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
) -> (usize, VecDeque<usize>) {
    // (winner, winner's deck)
    let mut game_history: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    loop {
        let game_state = (
            player1.iter().copied().collect(),
            player2.iter().copied().collect(),
        );
        if game_history.contains(&game_state) {
            break (1, player1);
        }
        game_history.insert(game_state);
        let (card1, card2) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        if player1.len() >= card1 && player2.len() >= card2 {
            // play recursive game!
            let new_player1 = player1.iter().copied().take(card1).collect();
            let new_player2 = player2.iter().copied().take(card2).collect();
            let (player, _) = recursive_game(new_player1, new_player2);
            match player {
                1 => {
                    player1.push_back(card1);
                    player1.push_back(card2);
                }
                2 => {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
                _ => unreachable!(),
            }
        } else {
            // play normal round
            match card1.cmp(&card2) {
                std::cmp::Ordering::Equal => unreachable!(),
                std::cmp::Ordering::Less => {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
                std::cmp::Ordering::Greater => {
                    player1.push_back(card1);
                    player1.push_back(card2);
                }
            }
            if player1.is_empty() {
                break (2, player2);
            } else if player2.is_empty() {
                break (1, player1);
            }
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (VecDeque<usize>, VecDeque<usize>)> {
    let (input, lines) = separated_list1(pair(newline, newline), parse_player)(input)?;
    let mut it = lines.into_iter();
    let player1 = it.next().unwrap();
    let player2 = it.next().unwrap();
    Ok((input, (player1, player2)))
}

fn parse_player(input: &str) -> IResult<&str, VecDeque<usize>> {
    let (input, _id) = delimited(tag("Player "), digit1, pair(nomchar(':'), newline))(input)?;
    let (input, deck_strs) = separated_list1(newline, digit1)(input)?;
    let deck = deck_strs.into_iter().map(|s| s.parse().unwrap()).collect();
    Ok((input, deck))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let result = process_part1(input);
        assert_eq!(result, "306");
    }

    #[test]
    fn part2() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let result = process_part2(input);
        assert_eq!(result, "291");
    }
}
