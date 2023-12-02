

fn main() {
	let input = include_str!("day2input.txt");

	let part1: u32 = input.lines()
		.map(to_game)
		.filter(|Game{cubes, ..}| {
			cubes.red <= 12
			&& cubes.green <= 13
			&& cubes.blue <= 14
		})
		.map(|game| game.id)
		.sum();

	dbg!(part1);


	let part2: u32 = input.lines()
		.map(to_game)
		.map(|Game{cubes, ..}| {
			cubes.red * cubes.green * cubes.blue
		})
		.sum();

	dbg!(part2);
}


#[derive(Default, Debug)]
struct Hand {
	red: u32,
	green: u32,
	blue: u32,
}

#[derive(Debug)]
struct Game {
	id: u32,
	cubes: Hand,
}

fn to_game(line: &str) -> Game {
	let (id_str, cubes_str) = line.split_once(':').unwrap();

	Game {
		id: id_str.strip_prefix("Game ").unwrap().parse().unwrap(),
		cubes: cubes_str.split(';')
			.map(to_hand)
			.reduce(|a, b| Hand {
				red: a.red.max(b.red),
				green: a.green.max(b.green),
				blue: a.blue.max(b.blue),
			})
			.unwrap(),
	}
}

fn to_hand(s: &str) -> Hand {
	let mut hand = Hand::default();

	for cubes_str in s.split(',').map(str::trim) {
		match cubes_str.split_once(' ').unwrap() {
			(n, "red") => hand.red = n.parse().unwrap(),
			(n, "green") => hand.green = n.parse().unwrap(),
			(n, "blue") => hand.blue = n.parse().unwrap(),
			_ => unreachable!(),
		}
	}

	hand
}
