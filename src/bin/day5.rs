#![feature(iter_array_chunks, array_windows)]
#![feature(let_chains)]


fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	let almanac = Almanac::parse(&include_str!("day5.txt").replace("\r\n", "\n")).unwrap();

	let min_location = almanac.seeds.iter()
		.map(|seed| almanac.location_for_seed(*seed))
		.min()
		.unwrap();

	dbg!(min_location);

	let min_location = almanac.seed_ranges.into_iter()
		.flat_map(|range| almanac.seed_to_soil.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.soil_to_fertilizer.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.fertilizer_to_water.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.water_to_light.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.light_to_temperature.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.temperature_to_humidity.map_range_to_destination_ranges(range))
		.flat_map(|range| almanac.humidity_to_location.map_range_to_destination_ranges(range))
		.map(|range| range.start)
		.min();

	dbg!(min_location);
}


fn parse_entry(entry_str: &str) -> Option<MapEntry> {
	let mut number_it = entry_str.split_whitespace()
		.filter_map(|s| s.parse().ok());

	let destination_start = number_it.next()?;
	let source_start = number_it.next()?;
	let length = number_it.next()?;

	Some(MapEntry {
		source_range: Range::from_start_length(source_start, length),
		destination_start,
	})
}

fn parse_map(map_str: &str) -> Map {
	let mut entries = map_str.lines()
		.skip(1) // Skip the name of the map
		.map(parse_entry)
		.collect::<Option<Vec<_>>>()
		.unwrap();

	entries.sort();

	Map { entries }
}




#[derive(Debug)]
struct Almanac {
	seeds: Vec<usize>,
	seed_ranges: Vec<Range>,

	seed_to_soil: Map,
	soil_to_fertilizer: Map,
	fertilizer_to_water: Map,
	water_to_light: Map,
	light_to_temperature: Map,
	temperature_to_humidity: Map,
	humidity_to_location: Map,
}

impl Almanac {
	fn parse(almanac_str: &str) -> Option<Almanac> {
		let almanac_str = almanac_str.replace("\r\n", "\n");

		// Sections are split by empty newlines
		let mut section_it = almanac_str.split("\n\n").map(str::trim);

		let seed_str = section_it.next()?
			.strip_prefix("seeds: ")?;

		let mut seeds = seed_str.split_whitespace()
			.map(str::parse)
			.map(Result::ok)
			.collect::<Option<Vec<_>>>()?;

		let mut seed_ranges: Vec<_> = seed_str.split_whitespace()
			.map(|s| s.parse().unwrap())
			.array_chunks()
			.map(|[start, length]| Range::from_start_length(start, length))
			.collect();

		seeds.sort();
		seed_ranges.sort();

		let seed_to_soil = section_it.next().map(parse_map)?;
		let soil_to_fertilizer = section_it.next().map(parse_map)?;
		let fertilizer_to_water = section_it.next().map(parse_map)?;
		let water_to_light = section_it.next().map(parse_map)?;
		let light_to_temperature = section_it.next().map(parse_map)?;
		let temperature_to_humidity = section_it.next().map(parse_map)?;
		let humidity_to_location = section_it.next().map(parse_map)?;

		Some(Almanac {
			seeds,
			seed_ranges,

			seed_to_soil,
			soil_to_fertilizer,
			fertilizer_to_water,
			water_to_light,
			light_to_temperature,
			temperature_to_humidity,
			humidity_to_location,
		})
	}

	fn location_for_seed(&self, seed_id: usize) -> usize {
		let soil_id = self.seed_to_soil.lookup(seed_id);
		let fertilizer_id = self.soil_to_fertilizer.lookup(soil_id);
		let water_id = self.fertilizer_to_water.lookup(fertilizer_id);
		let light_id = self.water_to_light.lookup(water_id);
		let temperature_id = self.light_to_temperature.lookup(light_id);
		let humidity_id = self.temperature_to_humidity.lookup(temperature_id);
		self.humidity_to_location.lookup(humidity_id)
	}
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Range {
	start: usize,
	end: usize,
}

impl Range {
	#[allow(unused)]
	fn new(start: usize, end: usize) -> Self {
		Range {start, end}
	}

	fn from_start_length(start: usize, length: usize) -> Self {
		Range {start, end: start + length}
	}

	fn end(&self) -> usize {
		self.end
	}

	fn contains(&self, idx: usize) -> bool {
		(self.start..self.end).contains(&idx)
	}

	fn is_empty(&self) -> bool {
		self.start >= self.end
	}
}



#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct MapEntry {
	source_range: Range,
	destination_start: usize,
}

#[derive(Debug)]
struct Map {
	entries: Vec<MapEntry>,
}

impl Map {
	fn lookup(&self, src: usize) -> usize {
		// Find the last entry with source_start < src
		let Some(entry_index) = self.entries.partition_point(|e| e.source_range.start <= src).checked_sub(1)
			else {
				// If a region isn't mapped, the mapping is 1:1
				return src
			};

		// src must be >= entry.source_start here
		let entry = &self.entries[entry_index];
		let diff = src - entry.source_range.start;

		if entry.source_range.contains(src) {
			// Entry maps src, so return appropriate dest 
			entry.destination_start + diff

		} else {
			// Entry doesn't map src
			src
		}
	}

	fn entries_containing_range(&self, input_range: Range) -> &[MapEntry] {
		// Find the first entry that _may_ contain src_begin
		let entries_begin = self.entries
			.binary_search_by(|entry| {
				use std::cmp::Ordering;

				match entry.source_range.start.cmp(&input_range.start) {
					Ordering::Greater => Ordering::Greater,

					_ => {
						if entry.source_range.contains(input_range.start) {
							Ordering::Equal
						} else {
							Ordering::Less
						}
					}
				}
			})
			.unwrap_or_else(|insertion_point| insertion_point);

		// Find the entry after the one that _may_ contain src_end
		let entries_end = self.entries[entries_begin..]
			.binary_search_by(|entry| {
				use std::cmp::Ordering;

				if entry.source_range.contains(input_range.end().saturating_sub(1)) {
					Ordering::Equal
				} else {
					entry.source_range.end().cmp(&input_range.end())
				}
			})
			.map_or_else(|insertion_point| insertion_point.saturating_sub(0), |exact_match| exact_match + 1);

		&self.entries[entries_begin..entries_begin + entries_end]
	}

	// [0, 9]
	// [2, 4] [6, 10]
	// -> [0, 1] [2, 4] [5, 5] [6, 9]

	fn map_range_to_destination_ranges(&self, mut input_range: Range) -> impl Iterator<Item=Range> + '_ {
		let mut relevant_entries = self.entries_containing_range(input_range);

		let mut output_ranges = Vec::new();

		while !relevant_entries.is_empty() && !input_range.is_empty() {
			let entry = &relevant_entries[0];

			// We have an unmapped region
			if !entry.source_range.contains(input_range.start) {
				// So create a Range spanning from [input.start, range.start)
				let end_intersection = entry.source_range.start.min(input_range.end());
				let length = end_intersection - input_range.start;

				output_ranges.push(Range::from_start_length(input_range.start, length));
				input_range.start += length;
				continue
			}

			// The start of our input range is mapped, so map to destination
			let start_intersection = entry.source_range.start.max(input_range.start);
			let end_intersection = entry.source_range.end.min(input_range.end);

			let length = end_intersection - start_intersection;
			let offset = start_intersection - entry.source_range.start;

			output_ranges.push(Range::from_start_length(entry.destination_start + offset, length));
			input_range.start += length;

			// pop front
			relevant_entries = &relevant_entries[1..];
		}

		// If we're out of entries and there's still range left, its unmapped so output it verbatim
		if !input_range.is_empty() {
			output_ranges.push(input_range);
		}

		output_ranges.into_iter()
	}
}





#[test]
fn test_lookups() {
	let almanac = Almanac::parse(include_str!("day5.reference.txt")).unwrap();
	dbg!(&almanac);
	
	assert_eq!(almanac.seed_to_soil.lookup(49), 49);
	assert_eq!(almanac.seed_to_soil.lookup(98), 50);
	assert_eq!(almanac.seed_to_soil.lookup(99), 51);
	assert_eq!(almanac.seed_to_soil.lookup(50), 52);
	assert_eq!(almanac.seed_to_soil.lookup(51), 53);


	assert_eq!(almanac.seed_to_soil.lookup(79), 81);
	assert_eq!(almanac.seed_to_soil.lookup(14), 14);
	assert_eq!(almanac.seed_to_soil.lookup(13), 13);
	assert_eq!(almanac.seed_to_soil.lookup(55), 57);


	assert_eq!(almanac.soil_to_fertilizer.lookup(81), 81);
	assert_eq!(almanac.fertilizer_to_water.lookup(81), 81);
	assert_eq!(almanac.water_to_light.lookup(81), 74);
	assert_eq!(almanac.light_to_temperature.lookup(74), 78);
	assert_eq!(almanac.temperature_to_humidity.lookup(78), 78);
	assert_eq!(almanac.humidity_to_location.lookup(78), 82);

	assert_eq!(almanac.location_for_seed(14), 43);
	assert_eq!(almanac.location_for_seed(55), 86);
	assert_eq!(almanac.location_for_seed(13), 35);
}

#[test]
fn test_entries_containing_range() {
	let almanac = Almanac::parse(include_str!("day5.reference.txt")).unwrap();
	assert_eq!(almanac.seed_to_soil.entries_containing_range(Range::from_start_length(0, 100)), almanac.seed_to_soil.entries);

	assert_eq!(almanac.seed_to_soil.entries_containing_range(Range::from_start_length(100, 100)), &[]);
	assert_eq!(almanac.seed_to_soil.entries_containing_range(Range::from_start_length(90, 10)), &almanac.seed_to_soil.entries);
	assert_eq!(almanac.seed_to_soil.entries_containing_range(Range::from_start_length(90, 2)), &almanac.seed_to_soil.entries[..1]);

	// fertilizer-to-water map:
	// 0 7 -> 42
	// 7 4 -> 57
	// 11 42 -> 0
	// 53 8 -> 49

	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(100, 100)), &[]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(0, 10)), &almanac.fertilizer_to_water.entries[0..2]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(0, 11)), &almanac.fertilizer_to_water.entries[0..2]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(1, 2)), &almanac.fertilizer_to_water.entries[0..1]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(1, 7)), &almanac.fertilizer_to_water.entries[0..2]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(6, 2)), &almanac.fertilizer_to_water.entries[0..2]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(7, 2)), &almanac.fertilizer_to_water.entries[1..2]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(60, 6)), &almanac.fertilizer_to_water.entries[3..4]);
	assert_eq!(almanac.fertilizer_to_water.entries_containing_range(Range::from_start_length(61, 6)), &[]);
}

#[test]
fn test_map_range_to_destination_ranges() {
	let almanac = Almanac::parse(include_str!("day5.reference.txt")).unwrap();

	let identity_map = almanac.seed_to_soil.map_range_to_destination_ranges(Range::from_start_length(0, 150)) .collect::<Vec<_>>();
	assert_eq!(identity_map, &[
		Range::new(0, 50),
		Range::from_start_length(52, 48),
		Range::from_start_length(50, 2),
		Range::new(100, 150),
	]);

	let subrange = almanac.seed_to_soil.map_range_to_destination_ranges(Range::from_start_length(45, 10)) .collect::<Vec<_>>();
	assert_eq!(subrange, &[
		Range::new(45, 50),
		Range::from_start_length(52, 5),
	]);


	// soil-to-fertilizer map:
	// 0  x15 -> 39
	// 15 x37 -> 0
	// 52 x2  -> 37

	let combined = almanac.seed_to_soil.map_range_to_destination_ranges(Range::from_start_length(0, 150))
		.flat_map(|range| almanac.soil_to_fertilizer.map_range_to_destination_ranges(range))
		.collect::<Vec<_>>();

	assert_eq!(combined, &[
		// Range::new(0, 50),
		Range::from_start_length(39, 15),
		Range::from_start_length(0, 50-15),

		// Range::new(52, 100),
		Range::from_start_length(37, 2),
		Range::new(54, 100),

		// Range::new(50, 52),
		Range::from_start_length(50-15, 2),

		// Range::new(100, 150),
		Range::new(100, 150),
	]);
}


#[test]
fn test_min_location() {
	let almanac = Almanac::parse(include_str!("day5.reference.txt")).unwrap();

	let min_location = almanac.seed_ranges.into_iter()
		.inspect(|range| {
			println!("\nseed: {range:?}")
		})
		.flat_map(|range| almanac.seed_to_soil.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- soil: {range:?}")
		})
		.flat_map(|range| almanac.soil_to_fertilizer.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- fert: {range:?}")
		})
		.flat_map(|range| almanac.fertilizer_to_water.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- --- water: {range:?}")
		})
		.flat_map(|range| almanac.water_to_light.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- --- --- light: {range:?}")
		})
		.flat_map(|range| almanac.light_to_temperature.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- --- --- --- temp: {range:?}")
		})
		.flat_map(|range| almanac.temperature_to_humidity.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- --- --- --- --- humid: {range:?}")
		})
		.flat_map(|range| almanac.humidity_to_location.map_range_to_destination_ranges(range))
		.inspect(|range| {
			println!("--- --- --- --- --- --- --- location: {range:?}")
		})
		.map(|range| range.start)
		.min();

	assert_eq!(min_location, Some(46));
}