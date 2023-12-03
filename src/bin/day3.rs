use common::*;


fn main() {
	std::env::set_var("RUST_BACKTRACE", "FULL");

	let map = parse_map(include_str!("day3input.txt"));

	let part_1: u32 = map.numbers.iter()
		.filter(|n| n.is_part)
		.map(|n| n.value)
		.sum();

	dbg!(part_1);
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

			map.numbers.push(Number {
				pos,
				value: number_str.parse().unwrap(),
				span: span as i32,
				is_part: span_has_adjacent_symbol(pos, span as i32, &map.symbols),
			});

			search_start += found_index + span;
		}
	}

	map
}


fn span_has_adjacent_symbol(pos: Vec2i, span: i32, symbols: &[Symbol]) -> bool {
	let min_bounds = pos - Vec2i::splat(1);
	let max_bounds = pos + Vec2i::new(span, 1);

	let symbol_scope = {
		let start_index = symbols.partition_point(|sym| cmp_vec2i(sym.pos, min_bounds).is_lt());
		let end_index = symbols[start_index..].partition_point(|sym| cmp_vec2i(sym.pos, max_bounds).is_le());
		&symbols[start_index..start_index + end_index]
	};

	symbol_scope.iter()
		.any(move |sym| is_point_in_bounds(sym.pos, min_bounds, max_bounds))
}

fn is_point_in_bounds(pos: Vec2i, min_bounds: Vec2i, max_bounds: Vec2i) -> bool {
	// both bounds are inclusive
	(min_bounds.x..=max_bounds.x).contains(&pos.x)
	&& (min_bounds.y..=max_bounds.y).contains(&pos.y)
}


use std::cmp::Ordering;

fn cmp_vec2i(a: Vec2i, b: Vec2i) -> Ordering {
	a.y.cmp(&b.y).then(a.x.cmp(&b.x))
}



#[derive(Debug, Default)]
struct Map {
	symbols: Vec<Symbol>,
	numbers: Vec<Number>,
}

#[derive(Debug)]
struct Number {
	pos: Vec2i,
	span: i32,

	value: u32,
	is_part: bool,
}

#[derive(Debug)]
struct Symbol {
	pos: Vec2i,
	ch: char,
}