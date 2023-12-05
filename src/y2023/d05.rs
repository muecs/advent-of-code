//! Day 5: If You Give A Seed A Fertilizer

/// lowest location number for single seeds
pub fn a(input: &Vec<&str>) -> String {
    let (seeds, mappings) = parse_input(input);
    seeds
        .iter()
        .map(|seed| {
            find_min_location(
                &Range {
                    start: *seed,
                    length: 1,
                },
                &mappings,
            )
        })
        .min()
        .unwrap()
        .to_string()
}

/// lowest location number for seed ranges
pub fn b(input: &Vec<&str>) -> String {
    let (seed_ranges, mappings) = parse_input(input);
    seed_ranges
        .chunks_exact(2)
        .map(|range| {
            find_min_location(
                &Range {
                    start: range[0],
                    length: range[1],
                },
                &mappings,
            )
        })
        .min()
        .unwrap()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> (Seeds, Mappings) {
    let mut it = input.iter();
    let seeds = it.next().unwrap()[7..]
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Seeds>();
    it.next();
    let mut mappings = Mappings::new();
    while it.next().is_some() {
        let mut mapping: Mapping = it
            .by_ref()
            .map_while(|s| {
                (!s.is_empty()).then(|| s.split(' ').map(|n| n.parse::<usize>().unwrap()).collect())
            })
            .collect();
        mapping.sort_unstable_by_key(|range| range.src);
        mappings.push(mapping);
    }
    (seeds, mappings)
}

fn map_number(value: usize, mapping: &Mapping) -> usize {
    mapping
        .iter()
        .find_map(|range| range.try_map(value))
        .unwrap_or(value)
}

fn map_range(mut range: Range, mapping: &Mapping) -> Ranges {
    let mut mapped_ranges = Ranges::new();
    for mapping_range in mapping {
        if range.start < mapping_range.src {
            if range.start + range.length < mapping_range.src {
                // starts and ends before, no mapping
                mapped_ranges.push(range);
                range.length = 0;
                break;
            } else {
                // overlaps start
                let new_length = mapping_range.src - range.start;
                mapped_ranges.push(Range {
                    start: range.start,
                    length: new_length,
                });
                range.start = mapping_range.src;
                range.length -= new_length;
                if range.length == 0 {
                    break;
                }
            }
        }
        let mapping_range_end = mapping_range.src + mapping_range.len;
        if range.start >= mapping_range.src && range.start < mapping_range_end {
            let mapped_start = mapping_range.try_map(range.start).unwrap();
            if range.start + range.length <= mapping_range_end {
                // completely included
                mapped_ranges.push(Range {
                    start: mapped_start,
                    length: range.length,
                });
                range.length = 0;
                break;
            } else {
                // overlaps end
                let new_length = mapping_range_end - range.start;
                mapped_ranges.push(Range {
                    start: mapped_start,
                    length: new_length,
                });
                range.start = mapping_range_end;
                range.length -= new_length;
                if range.length == 0 {
                    break;
                }
            }
        }
    }
    if range.length > 0 {
        // starts after, no mapping
        mapped_ranges.push(range);
    }
    mapped_ranges
}

fn find_min_location(range: &Range, mappings: &[Mapping]) -> usize {
    assert_ne!(range.length, 0);
    if range.length == 1 {
        mappings
            .iter()
            .fold(range.start, |value, mapping| map_number(value, mapping))
    } else if let Some(mapping) = mappings.first() {
        map_range(*range, mapping)
            .iter()
            .map(|range| find_min_location(range, &mappings[1..]))
            .min()
            .unwrap()
    } else {
        range.start
    }
}

type Seeds = Vec<usize>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Range {
    start: usize,
    length: usize,
}
type Ranges = Vec<Range>;

struct MappingRange {
    dest: usize,
    src: usize,
    len: usize,
}
type Mapping = Vec<MappingRange>;
type Mappings = Vec<Mapping>;

impl MappingRange {
    fn try_map(&self, value: usize) -> Option<usize> {
        (value >= self.src && value < self.src + self.len).then(|| self.dest + value - self.src)
    }
}

impl FromIterator<usize> for MappingRange {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        MappingRange {
            dest: it.next().unwrap(),
            src: it.next().unwrap(),
            len: it.next().unwrap(),
        }
    }
}

#[test]
pub fn test() {
    let range = MappingRange {
        dest: 50,
        src: 98,
        len: 2,
    };
    assert_eq!(range.try_map(97), None);
    assert_eq!(range.try_map(98), Some(50));
    assert_eq!(range.try_map(99), Some(51));
    assert_eq!(range.try_map(100), None);

    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    let (seeds, mappings) = parse_input(&input);
    assert_eq!(seeds, vec![79, 14, 55, 13]);
    assert_eq!(mappings.len(), 7);
    assert_eq!(mappings[0].len(), 2);
    assert_eq!(mappings[1].len(), 3);
    assert_eq!(map_number(79, &mappings[0]), 81);
    assert_eq!(map_number(14, &mappings[0]), 14);
    assert_eq!(map_number(55, &mappings[0]), 57);
    assert_eq!(map_number(13, &mappings[0]), 13);
    assert_eq!(
        map_range(
            Range {
                start: 40,
                length: 5
            },
            &mappings[0]
        ),
        vec![Range {
            start: 40,
            length: 5
        }]
    );
    assert_eq!(
        map_range(
            Range {
                start: 40,
                length: 70
            },
            &mappings[0]
        ),
        vec![
            Range {
                start: 40,
                length: 10
            },
            Range {
                start: 52,
                length: 48
            },
            Range {
                start: 50,
                length: 2
            },
            Range {
                start: 100,
                length: 10
            },
        ]
    );

    assert_eq!(a(&input), "35");
    assert_eq!(b(&input), "46");
}
