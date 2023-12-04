


fn main() {
	let cards = include_str!("day4.txt").lines()
		.map(to_card)
		.collect::<Vec<_>>();

	let total_score: u32 = cards.iter()
		.map(card_score)
		.sum();

	dbg!(total_score);

	dbg!(evaluate_total_scratchcards(&cards));
}


#[derive(Debug)]
struct Card {
	winning: Vec<u32>,
	have: Vec<u32>,
}

fn to_card(line: &str) -> Card {
	let line = line.split_once(':').unwrap().1;
	let (winning_str, have_str) = line.split_once('|').unwrap();
	let mut winning: Vec<_> = winning_str.trim().split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();

	let mut have: Vec<_> = have_str.trim().split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();

	winning.sort();
	have.sort();

	Card { winning, have }
}

fn calculate_num_matches(card: &Card) -> u32 {
	card.have.iter()
		.filter(|have| card.winning.binary_search(have).is_ok())
		.count() as u32
}

fn card_score(card: &Card) -> u32 {
	match calculate_num_matches(card) {
		0 => 0,
		num_matches => 1 << (num_matches - 1),
	}
}

fn evaluate_total_scratchcards(cards: &[Card]) -> usize {
	let mut num_card_copies = vec![1; cards.len()];

	for (index, card) in cards.iter().enumerate() {
		let matched_card_copies = num_card_copies[index];
		let num_matches = calculate_num_matches(card);

		for successive_card_copies in &mut num_card_copies[index+1 .. index+num_matches as usize+1] {
			*successive_card_copies += matched_card_copies;
		}
	}

	num_card_copies.into_iter().sum()
}