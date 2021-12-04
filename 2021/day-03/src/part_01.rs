use std::str::Lines;

struct PowerMeter<const BITS: usize> {
    reading_count: u32,
    bit_counts: [u32; BITS],
    epsilon_mask: u32,
}

impl<const BITS: usize> PowerMeter<BITS> {
    pub fn new() -> Self {
        PowerMeter {
            reading_count: 0,
            bit_counts: [0; BITS],
            epsilon_mask: (u32::MAX >> (u32::BITS as usize - BITS)) as u32,
        }
    }

    pub fn read(&mut self, diagnostic: Diagnostic<BITS>) {
        for i in 0..BITS {
            self.bit_counts[i] += diagnostic.nth_bit(i) as u32;
        }
        self.reading_count += 1;
    }

    pub fn measure(&self) -> (u32, u32) {
        let half_reading_count = self.reading_count / 2;
        let mut gamma = 0u32;

        for i in 0..BITS {
            let channel = self.bit_counts[i] / half_reading_count;
            gamma = gamma | (channel << (BITS - (i + 1)));
        }

        (gamma, self.epsilon_mask ^ gamma)
    }
}

impl<const BITS: usize> From<DiagnosticStream<'_, BITS>> for PowerMeter<BITS> {
    fn from(diagnostic_stream: DiagnosticStream<BITS>) -> Self {
        let mut meter = PowerMeter::new();

        for diagnostic in diagnostic_stream {
            meter.read(diagnostic);
        }

        meter
    }
}

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
fn part_01() {
    let input = include_str!("easy.txt");
    let meter = PowerMeter::from(DiagnosticStream::<5>::new(input));

    let (gamma, epsilon) = meter.measure();

    assert_eq!(gamma * epsilon, 198);

    let input = include_str!("input.txt");
    let meter = PowerMeter::from(DiagnosticStream::<12>::new(input));

    let (gamma, epsilon) = meter.measure();

    assert_eq!(gamma * epsilon, 3985686);
}
