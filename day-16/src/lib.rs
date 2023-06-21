use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, (fields, _your_ticket, nearby_tickets)) = parse_input(input).unwrap();
    let all_ranges: Vec<RangeInclusive<usize>> = fields
        .iter()
        .flat_map(|f| [f.ranges.0.clone(), f.ranges.1.clone()].into_iter())
        .collect();
    let ticket_errors: Vec<usize> = nearby_tickets
        .into_iter()
        .flat_map(|v| v.values.into_iter())
        .filter(|val| all_ranges.iter().all(|range| !range.contains(val)))
        .collect();
    ticket_errors.into_iter().sum::<usize>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (fields, your_ticket, nearby_tickets)) = parse_input(input).unwrap();
    let valid_tickets = get_valid_tickets(&fields, nearby_tickets);
    let det_ticket = determine_fields(your_ticket, valid_tickets, fields);
    let result = det_ticket
        .fields
        .into_iter()
        .filter_map(|(name, value)| {
            if name.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .product::<usize>();
    result.to_string()
}

fn get_valid_tickets(fields: &[Field], nearby_tickets: Vec<Ticket>) -> Vec<Ticket> {
    let all_ranges: Vec<RangeInclusive<usize>> = fields
        .iter()
        .flat_map(|f| [f.ranges.0.clone(), f.ranges.1.clone()].into_iter())
        .collect();
    nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .values
                .iter()
                .all(|val| !all_ranges.iter().all(|range| !range.contains(val)))
        })
        .collect()
}

fn determine_fields(
    your_ticket: Ticket,
    valid_tickets: Vec<Ticket>,
    fields: Vec<Field>,
) -> DeterminedTicket {
    let len = your_ticket.values.len();
    let mut options = vec![HashSet::<_>::from_iter(fields.iter().cloned()); len];
    valid_tickets.into_iter().for_each(|ticket| {
        ticket.values.into_iter().enumerate().for_each(|(i, val)| {
            options[i].retain(|f| f.contains(val));
        })
    });
    let mut singles: HashSet<_> = options
        .iter()
        .filter_map(|s| {
            if s.len() == 1 {
                Some(s.iter().take(1).cloned())
            } else {
                None
            }
        })
        .flatten()
        .collect();

    while singles.len() < len {
        options.iter_mut().for_each(|s| {
            if s.len() > 1 {
                s.retain(|f| !singles.contains(f));
            }
        });
        singles = options
            .iter()
            .filter_map(|s| {
                if s.len() == 1 {
                    Some(s.iter().take(1).cloned())
                } else {
                    None
                }
            })
            .flatten()
            .collect();
    }

    let determined_fields = options
        .into_iter()
        .map(|s| s.into_iter().last().unwrap().name)
        .zip(your_ticket.values.into_iter())
        .collect();

    DeterminedTicket {
        fields: determined_fields,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field<'a> {
    name: &'a str,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl<'a> Field<'a> {
    fn contains(&self, n: usize) -> bool {
        self.ranges.0.contains(&n) || self.ranges.1.contains(&n)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DeterminedTicket<'a> {
    fields: Vec<(&'a str, usize)>,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Field>, Ticket, Vec<Ticket>)> {
    let (input, fields_input) = take_until("\n\n")(input)?;
    let (_, fields) = separated_list1(tag("\n"), parse_field)(fields_input)?;
    let (input, _) = tag("\n\nyour ticket:\n")(input)?;
    let (input, your_ticket) = parse_ticket(input)?;
    let (input, _) = tag("\n\nnearby tickets:\n")(input)?;
    let (input, nearby_tickets) = separated_list1(tag("\n"), parse_ticket)(input)?;
    Ok((input, (fields, your_ticket, nearby_tickets)))
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    let (input, name) = take_until(": ")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, range1) = parse_range(input)?;
    let (input, _) = tag(" or ")(input)?;
    let (input, range2) = parse_range(input)?;
    Ok((
        input,
        Field {
            name,
            ranges: (range1, range2),
        },
    ))
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, start) = digit1(input)?;
    let start = start.parse().unwrap();
    let (input, _) = tag("-")(input)?;
    let (input, end) = digit1(input)?;
    let end = end.parse().unwrap();
    Ok((input, start..=end))
}

fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
    let (input, values) = separated_list1(tag(","), digit1)(input)?;
    let values = values.into_iter().map(|s| s.parse().unwrap()).collect();
    Ok((input, Ticket { values }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let result = process_part1(input);
        assert_eq!(result, "71");
    }

    #[test]
    fn part2_valid() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (_, (fields, _your_ticket, nearby_tickets)) = parse_input(input).unwrap();
        let valid_tickets = get_valid_tickets(&fields, nearby_tickets);
        assert_eq!(
            valid_tickets,
            vec![Ticket {
                values: vec![7, 3, 47]
            }]
        );
    }

    #[test]
    fn part2_determine() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let (_, (fields, your_ticket, nearby_tickets)) = parse_input(input).unwrap();
        let valid_tickets = get_valid_tickets(&fields, nearby_tickets);
        let det_ticket = determine_fields(your_ticket, valid_tickets, fields);
        assert_eq!(
            det_ticket,
            DeterminedTicket {
                fields: vec![("row", 11), ("class", 12), ("seat", 13)]
            }
        );
    }
}
