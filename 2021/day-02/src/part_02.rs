use std::{iter::Iterator, ops::Add, str::Lines};

#[derive(Debug)]
struct Vector(
    i32, // Forward
    i32, // Depth
    i32, // Aim
);

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let aim = self.2 + other.2;
        let depth_change = other.0 * aim;

        Vector(self.0 + other.0, self.1 + depth_change, aim)
    }
}

struct MovementStream<'a> {
    lines: Lines<'a>,
}

impl<'a> MovementStream<'a> {
    pub fn new(raw_input: &'a str) -> Self {
        MovementStream {
            lines: raw_input.lines(),
        }
    }
}

impl<'a> Iterator for MovementStream<'a> {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(line) => {
                let mut parts = line.split_whitespace();

                match (parts.nth(0), parts.nth(0)) {
                    (Some(dir), Some(mag)) => Some(match (dir, mag.parse::<i32>()) {
                        ("forward", Ok(value)) => Vector(value, 0, 0),
                        ("up", Ok(value)) => Vector(0, 0, -1 * value),
                        ("down", Ok(value)) => Vector(0, 0, value),
                        _ => panic!("Parse error"),
                    }),
                    _ => panic!("Parse error"),
                }
            }
            None => None,
        }
    }
}

#[test]
fn part_02() {
    let input = include_str!("easy.txt");
    let stream = MovementStream::new(input);
    let position = stream.reduce(|l, r| l + r).unwrap();

    assert_eq!(position.0 * position.1, 900);

    let input = include_str!("input.txt");
    let stream = MovementStream::new(input);
    let position = stream.reduce(|l, r| l + r).unwrap();

    assert_eq!(position.0 * position.1, 1903644897);
}
