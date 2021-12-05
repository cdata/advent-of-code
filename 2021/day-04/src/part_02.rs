use std::collections::BTreeMap;

struct BingoBoard {
    rows: [u8; 5],
    cols: [u8; 5],
    values: BTreeMap<u32, (usize, usize)>,
    winning_number: Option<u32>,
}

impl BingoBoard {
    pub fn new(mut data: Vec<u32>) -> Self {
        let mut values = BTreeMap::new();

        for row in (0..5).rev() {
            for col in (0..5).rev() {
                values.insert(data.pop().unwrap(), (row, col));
            }
        }

        BingoBoard {
            rows: [0; 5],
            cols: [0; 5],
            values,
            winning_number: None,
        }
    }

    pub fn mark(&mut self, number: u32) -> bool {
        if let Some((_, (x, y))) = self.values.remove_entry(&number) {
            self.rows[y] += 1;
            self.cols[x] += 1;

            if self.rows[y] == 5 || self.cols[x] == 5 {
                self.winning_number = Some(number);
            }
        }

        self.is_winner()
    }

    pub fn is_winner(&self) -> bool {
        match self.winning_number {
            Some(_) => true,
            None => false,
        }
    }

    pub fn score(&self) -> u32 {
        match self.winning_number {
            Some(number) => self.values.keys().sum::<u32>() * number,
            None => 0,
        }
    }
}

struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn new(raw_input: &str) -> Self {
        let input = String::from(raw_input);
        let mut parts = input.split("\n\n");

        BingoGame {
            numbers: parts
                .nth(0)
                .unwrap()
                .split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .collect(),
            boards: parts
                .map(|part| {
                    BingoBoard::new(
                        part.split_whitespace()
                            .map(|value| value.parse::<u32>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl Iterator for BingoGame {
    type Item = BingoBoard;

    fn next(&mut self) -> Option<Self::Item> {
        let boards = &mut self.boards;

        for (number_index, number) in self.numbers.iter().enumerate() {
            for (board_index, board) in boards.iter_mut().enumerate() {
                if board.mark(*number) {
                    self.numbers = Vec::from(self.numbers.split_at(number_index).1);
                    return Some(boards.remove(board_index));
                }
            }
        }

        None
    }
}

#[test]
fn part_02() {
    let input = include_str!("easy.txt");
    let mut game = BingoGame::new(input);
    let last_winner = game.last().unwrap();

    assert_eq!(last_winner.score(), 1924);

    let input = include_str!("input.txt");
    let mut game = BingoGame::new(input);
    let last_winner = game.last().unwrap();

    assert_eq!(last_winner.score(), 8468);
}
