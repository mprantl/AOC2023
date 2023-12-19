use std::{fs::File, io::Read};

struct Card {
    card_number: String,
    winning_numbers: Vec<u8>,
    playing_numbers: Vec<u8>,
}

fn parse_input_file(filename: &str) -> Vec<Card> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let mut cards: Vec<Card> = Vec::new();
    for line in contents.lines() {
        let mut card_number = String::new();
        let mut winning_numbers: Vec<u8> = Vec::new();
        let mut playing_numbers: Vec<u8> = Vec::new();
        let mut card_number_done = false;
        let mut winning_numbers_done = false;
        for word in line.split_whitespace() {
            if word == "Card" {
                continue;
            }
            if word == ":" {
                continue;
            }
            if word == "|" {
                winning_numbers_done = true;
                continue;
            }
            if !card_number_done {
                card_number.push_str(word);
                card_number_done = true;
            } else if !winning_numbers_done {
                winning_numbers.push(word.parse::<u8>().unwrap());
            } else {
                playing_numbers.push(word.parse::<u8>().unwrap());
            }
        }
        cards.push(Card {
            card_number,
            winning_numbers,
            playing_numbers,
        });
    }
    cards
}

fn main() {
    let cards = parse_input_file("input-smoll.txt");
    for card in cards {
        println!("Card number: {}", card.card_number);
        println!("Winning numbers: {:?}", card.winning_numbers);
        println!("Playing numbers: {:?}", card.playing_numbers);
    }
}
