



pub fn main() {
	let input = include_str!("day1input.txt");

	let part1: u32 = input.lines()
		.map(part1)
		.sum();

	dbg!(part1);

	let part2: u32 = input.lines()
		.map(part2)
		.sum();

	dbg!(part2);
}

fn part1(s: &str) -> u32 {
	let first = s.chars().filter_map(|c| c.to_digit(10)).next().unwrap();
	let second = s.chars().filter_map(|c| c.to_digit(10)).next_back().unwrap();

	first * 10 + second
}

fn part2(s: &str) -> u32 {
	let first = {
		let mut s = s;
		loop {
			assert!(!s.is_empty());

			if let Some(digit) = s.chars().next().and_then(|c| c.to_digit(10)) {
				break digit
			}

			if s.starts_with("one") { break 1 }
			else if s.starts_with("two") { break 2 }
			else if s.starts_with("three") { break 3 }
			else if s.starts_with("four") { break 4 }
			else if s.starts_with("five") { break 5 }
			else if s.starts_with("six") { break 6 }
			else if s.starts_with("seven") { break 7 }
			else if s.starts_with("eight") { break 8 }
			else if s.starts_with("nine") { break 9 }

			s = &s[1..];
		}
	};

	let second = {
		let mut s = s;
		loop {
			assert!(!s.is_empty());

			if let Some(digit) = s.chars().rev().next().and_then(|c| c.to_digit(10)) {
				break digit
			}

			if s.ends_with("one") { break 1 }
			else if s.ends_with("two") { break 2 }
			else if s.ends_with("three") { break 3 }
			else if s.ends_with("four") { break 4 }
			else if s.ends_with("five") { break 5 }
			else if s.ends_with("six") { break 6 }
			else if s.ends_with("seven") { break 7 }
			else if s.ends_with("eight") { break 8 }
			else if s.ends_with("nine") { break 9 }

			s = &s[..s.len() - 1];
		}
	};

	first * 10 + second
}