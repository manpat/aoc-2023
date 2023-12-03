use common::*;


fn main() {
	std::env::set_var("RUST_BACKTRACE", "FULL");

	let map = parse_map(include_str!("day3input.txt"));

	let part_1: u32 = map.part_numbers.iter()
		.map(|n| n.value)
		.sum();

	dbg!(part_1);

	let part_2: u32 = map.symbols.iter()
		.filter(|sym| sym.ch == '*')
		.filter_map(|sym| get_exactly_two_adjacent_values(sym.pos, &map.part_numbers))
		.map(|(a, b)| a * b) // gear ratio
		.sum();

	dbg!(part_2);
}


fn parse_map(document: &str) -> Map {
	let mut map = Map::default();

	// Find symbols
	for (y, line_str) in document.lines().enumerate() {
		let mut search_start = 0;

		while let Some(found_index) = line_str[search_start..].find(|c: char| c != '.' && c.is_ascii_punctuation()) {
			let x = found_index + search_start;

			map.symbols.push(Symbol {
				pos: Vec2i::new(x as i32, y as i32),
				ch: line_str.chars().nth(x).unwrap(),
			});

			search_start += found_index + 1;
		}
	}

	// Find numbers
	for (y, line_str) in document.lines().enumerate() {
		let mut search_start = 0;

		while let Some(found_index) = line_str[search_start..].find(|c: char| c != '.' && c.is_ascii_digit()) {
			let number_start_index = found_index + search_start;
			let pos = Vec2i::new(number_start_index as i32, y as i32);

			let number_search_str = &line_str[number_start_index..];

			let number_str = number_search_str.split_once(|c: char| !c.is_ascii_digit())
				.map_or(number_search_str, |(s, _)| s);

			let span = number_str.len();

			let number = Number {
				pos,
				value: number_str.parse().unwrap(),
				span: span as i32,
			};

			if number_has_adjacent_symbol(&number, &map.symbols) {
				map.part_numbers.push(number);
			}

			search_start += found_index + span;
		}
	}

	map
}


fn get_exactly_two_adjacent_values(pos: Vec2i, numbers: &[Number]) -> Option<(u32, u32)> {
	let numbers = {
		let start_index = numbers.partition_point(|num| num.pos.y < pos.y - 1);
		let end_index = numbers[start_index..].partition_point(|num| num.pos.y <= pos.y + 1);
		&numbers[start_index..start_index + end_index]
	};

	let mut it = numbers.iter().filter(|num| num.bounds().contains(pos)).map(|num| num.value);
	let result = (it.next()?, it.next()?);

	if it.next().is_none() {
		Some(result)
	} else {
		None
	}
}

fn number_has_adjacent_symbol(number: &Number, symbols: &[Symbol]) -> bool {
	let bounds = number.bounds();

	let symbol_scope = {
		let start_index = symbols.partition_point(|sym| cmp_vec2i(sym.pos, bounds.min).is_lt());
		let end_index = symbols[start_index..].partition_point(|sym| cmp_vec2i(sym.pos, bounds.max).is_le());
		&symbols[start_index..start_index + end_index]
	};

	symbol_scope.iter()
		.any(move |sym| bounds.contains(sym.pos))
}



use std::cmp::Ordering;

fn cmp_vec2i(a: Vec2i, b: Vec2i) -> Ordering {
	a.y.cmp(&b.y).then(a.x.cmp(&b.x))
}

struct Bounds {
	min: Vec2i,
	max: Vec2i,
}

impl Bounds {
	fn contains(&self, pos: Vec2i) -> bool {
		// both bounds are inclusive
		(self.min.x..=self.max.x).contains(&pos.x)
		&& (self.min.y..=self.max.y).contains(&pos.y)
	}
}


#[derive(Debug, Default)]
struct Map {
	symbols: Vec<Symbol>,
	part_numbers: Vec<Number>,
}

#[derive(Debug)]
struct Number {
	pos: Vec2i,
	span: i32,

	value: u32,
}

impl Number {
	fn bounds(&self) -> Bounds {
		Bounds {
			min: self.pos - Vec2i::splat(1),
			max: self.pos + Vec2i::new(self.span, 1),
		}
	}
}

#[derive(Debug)]
struct Symbol {
	pos: Vec2i,
	ch: char,
}
