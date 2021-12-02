use std::env;
use std::fs;
use std::iter::Iterator;
use std::path::Path;

struct RingBuffer<const SIZE: usize> {
    index: usize,
    buffer: [i32; SIZE],
}

impl<const SIZE: usize> RingBuffer<SIZE> {
    pub fn new() -> Self {
        RingBuffer {
            index: 0,
            buffer: [0; SIZE],
        }
    }

    pub fn advance(&mut self) {
        self.index = (self.index + 1) % SIZE;
    }

    pub fn set(&mut self, value: i32) {
        self.buffer[self.index] = value;
    }

    pub fn nth(&self, n: usize) -> i32 {
        self.buffer[n]
    }

    pub fn iter(&self) -> RingIterator<SIZE> {
        RingIterator {
            step: 0,
            current: self.index,
            buffer: self,
        }
    }
}

struct RingIterator<'a, const SIZE: usize> {
    step: usize,
    current: usize,
    buffer: &'a RingBuffer<SIZE>,
}

impl<'a, const SIZE: usize> Iterator for RingIterator<'a, SIZE> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 2 {
            return None;
        }

        let index = (self.current + self.step) % 3;

        self.step += 1;

        Some(self.buffer.nth(index))
    }
}

struct ReadingAnalyzer {
    buffer: RingBuffer<3>,
    last_value: i32,
    increase_count: i32,
    reading_count: i32,
}

impl ReadingAnalyzer {
    pub fn new() -> Self {
        ReadingAnalyzer {
            buffer: RingBuffer::<3>::new(),
            last_value: -1,
            increase_count: 0,
            reading_count: 0,
        }
    }

    pub fn record(&mut self, value: i32) {
        self.buffer.set(value);
        self.buffer.advance();

        self.reading_count += 1;

        if self.reading_count < 3 {
            return;
        }

        let window_value = self.buffer.iter().reduce(|a, b| a + b).unwrap();

        self.increase_count += match self.last_value {
            -1 => 0,
            _ if window_value > self.last_value => 1,
            _ => 0,
        };

        self.last_value = window_value;
    }

    pub fn get_increase_count(&self) -> i32 {
        self.increase_count
    }
}

fn main() {
    let file_path_arg = match env::args().nth(1) {
        Some(str) => str,
        None => panic!("Specify input file path"),
    };

    let current_dir = env::current_dir().unwrap();
    let file_path = Path::join(&current_dir, file_path_arg);
    let contents = fs::read_to_string(file_path).unwrap();

    let mut accumulator = String::new();
    let mut analyzer = ReadingAnalyzer::new();

    for char in contents.chars() {
        let value = match char {
            '\n' | '\r' => i32::from_str_radix(accumulator.as_str(), 10).unwrap(),
            _ => {
                accumulator.push(char);
                continue;
            }
        };

        accumulator.clear();

        analyzer.record(value);
    }

    if accumulator.len() > 0 {
        analyzer.record(i32::from_str_radix(accumulator.as_str(), 10).unwrap());
    }

    println!("Increased {} times", analyzer.get_increase_count());
}
