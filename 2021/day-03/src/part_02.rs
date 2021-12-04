use std::{fs::read_link, str::Lines};

struct LifeSupport<const BITS: usize> {
    oxygen: Diagnostic<BITS>,
    scrubber: Diagnostic<BITS>,
}

impl<const BITS: usize> LifeSupport<BITS> {
    pub fn measure(&self) -> (u32, u32) {
        (self.oxygen.0, self.scrubber.0)
    }

    pub fn reduce(
        mut readings: Vec<&Diagnostic<BITS>>,
        test: fn(u32, u32) -> bool,
    ) -> Diagnostic<BITS> {
        for i in 0..BITS {
            let channel_sum = readings.iter().fold(0u32, |l, r| l + r.nth_bit(i));
            let channel_value = match test(channel_sum, readings.len() as u32) {
                true => 1,
                false => 0,
            };

            readings = readings
                .into_iter()
                .filter(|diagnostic| diagnostic.nth_bit(i) == channel_value)
                .collect();

            if readings.len() == 1 {
                break;
            }
        }

        **readings.get(0).unwrap()
    }
}

impl<const BITS: usize> From<DiagnosticStream<'_, BITS>> for LifeSupport<BITS> {
    fn from(diagnostic_stream: DiagnosticStream<BITS>) -> Self {
        let readings: Vec<Diagnostic<BITS>> = diagnostic_stream.collect();

        let oxygen =
            LifeSupport::<BITS>::reduce(readings.iter().collect(), |channel_sum, reading_count| {
                channel_sum * 2 >= reading_count
            });

        let scrubber =
            LifeSupport::<BITS>::reduce(readings.iter().collect(), |channel_sum, reading_count| {
                channel_sum * 2 < reading_count
            });

        LifeSupport { oxygen, scrubber }
    }
}

#[derive(Clone, Copy, Debug)]
struct Diagnostic<const BITS: usize>(u32);

impl<const BITS: usize> Diagnostic<BITS> {
    pub fn nth_bit(&self, index: usize) -> u32 {
        self.0 >> (u32::BITS as usize - (index + 1 + u32::BITS as usize - BITS)) & 0b1
    }
}

struct DiagnosticStream<'a, const BITS: usize> {
    lines: Lines<'a>,
}

impl<'a, const BITS: usize> DiagnosticStream<'a, BITS> {
    pub fn new(raw_input: &'a str) -> Self {
        DiagnosticStream {
            lines: raw_input.lines(),
        }
    }
}

impl<'a, const BITS: usize> Iterator for DiagnosticStream<'a, BITS> {
    type Item = Diagnostic<BITS>;

    fn next(&mut self) -> Option<Diagnostic<BITS>> {
        match self.lines.next() {
            Some(str) => Some(Diagnostic::<BITS>(u32::from_str_radix(str, 2).unwrap())),
            None => None,
        }
    }
}

#[test]
fn part_02() {
    let input = include_str!("easy.txt");
    let life_support = LifeSupport::from(DiagnosticStream::<5>::new(input));
    let (oxygen, scrubber) = life_support.measure();

    assert_eq!(oxygen * scrubber, 230);

    let input = include_str!("input.txt");
    let life_support = LifeSupport::from(DiagnosticStream::<12>::new(input));
    let (oxygen, scrubber) = life_support.measure();

    assert_eq!(oxygen * scrubber, 2555739);
}
