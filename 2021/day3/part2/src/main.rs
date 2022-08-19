#![feature(array_zip)]

const WORD_LEN: usize = 12;

fn main() {
    let input = include_str!("../../input.txt");

    let (num_words, sums) = input
        .lines()
        .map(|word| word.as_bytes())
        .map(|word| word.try_into().unwrap())
        .map(|word: [u8; WORD_LEN]| word.map(|b| b - b'0'))
        .map(|word| word.map(u16::from))
        .map(|word| (1u16, word))
        .reduce(|(count, sum), (_, word)| (count + 1, sum.zip(word).map(|(s, b)| s + b)))
        .unwrap();

    let half_num_words = num_words / 2;

    let gamma = sums.map(|s| s > half_num_words);

    let gamma = gamma.iter().fold(0, |num, &b| (num << 1) | b as u32);
    let epsilon = !gamma & 0x0FFF;

    println!("{}", gamma);

    println!("{:?}", gamma * epsilon);

    /*
    let epsilon = !gamma & 0x0FFF;

    println!("{}", gamma as u32 * epsilon as u32);
    */
}

