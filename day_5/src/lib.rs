static INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Debug)]
struct Range {
    dest_start: u64,
    source_start: u64,
    size: u64,
}

impl Range {
    fn dest(&self) -> std::ops::Range<u64> {
        self.dest_start..self.dest_start + self.size
    }

    fn source(&self) -> std::ops::Range<u64> {
        self.source_start..self.source_start + self.size
    }
}

#[derive(Clone, Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn make_from_input(input: &str) -> Self {
        let mut ranges = Vec::new();
        for line in input.lines() {
            if line.chars().next().is_some_and(|x| !x.is_ascii_digit()) {
                continue;
            }

            let mut range = line.split_whitespace();
            let destination = range.next().unwrap().parse().unwrap();
            let source = range.next().unwrap().parse().unwrap();
            let size = range.next().unwrap().parse().unwrap();
            ranges.push(Range {
                dest_start: destination,
                source_start: source,
                size,
            })
        }

        Map { ranges }
    }

    fn destination_from_source(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.source().contains(&source) {
                return range.dest_start + (source - range.source_start);
            }
        }

        source
    }

    fn fill_out(&mut self, min: u64, max: u64) {
        self.ranges.sort_by_key(|x| x.source_start);

        let mut extras = Vec::new();
        for window in self.ranges.windows(2) {
            let first = window[0];
            let second = window[1];

            if first.source_start + first.size < second.source_start {
                let start = first.source_start + first.size;
                let end = second.source_start;
                let size = end - start;

                extras.push(Range {
                    source_start: start,
                    size,
                    dest_start: start,
                })
            }
        }

        if self.ranges[0].source_start > min {
            let start = min;
            let end = self.ranges[0].source_start;
            let size = end - start;
            extras.push(Range {
                source_start: start,
                size,
                dest_start: start,
            })
        }

        let l = &self.ranges[self.ranges.len() - 1];
        if l.source_start + l.size < max {
            let start = l.source_start + l.size;
            let end = max;
            let size = end - start;
            extras.push(Range {
                source_start: start,
                size,
                dest_start: start,
            })
        }

        self.ranges.append(&mut extras);
    }
}

fn make_maps_from_input(input: &str) -> Vec<Map> {
    let mut maps = Vec::new();

    let mut seeds_and_maps = input.split("\n\n");
    seeds_and_maps.next();

    for map in seeds_and_maps {
        maps.push(Map::make_from_input(map));
    }

    maps
}

fn make_seeds_from_input(input: &str) -> Vec<u64> {
    let seed_line = input.lines().next().unwrap();

    seed_line
        .split_whitespace()
        .map(|x| x.parse())
        .flatten()
        .collect()
}

#[test]
fn task_1() {
    let seeds = make_seeds_from_input(INPUT);
    let maps = make_maps_from_input(INPUT);

    let mut sequence = seeds.clone();

    for map in maps.iter() {
        sequence
            .iter_mut()
            .for_each(|x| *x = map.destination_from_source(*x));
    }

    dbg!(&sequence);
    dbg!(sequence.iter().min());
}

fn ranges_overlap(
    a: &std::ops::Range<u64>,
    b: &std::ops::Range<u64>,
) -> Option<std::ops::Range<u64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

struct Seeds {
    seeds: Vec<std::ops::Range<u64>>,
}

impl Seeds {
    fn make_from_input(input: &str) -> Self {
        let seed_line = input.lines().next().unwrap();

        let numbers: Vec<u64> = seed_line
            .split_whitespace()
            .map(|x| x.parse())
            .flatten()
            .collect();

        Self {
            seeds: numbers.chunks_exact(2).map(|x| x[0]..x[0] + x[1]).collect(),
        }
    }
}

fn find_seed_number_range_corresponding(
    maps: &[Map],
    destination_range: std::ops::Range<u64>,
    seeds: &Seeds,
) -> Option<u64> {
    let Some((last, rest)) = maps.split_last() else {
        // check the seeds
        for seed in seeds.seeds.iter() {
            if let Some(overlap) = ranges_overlap(seed, &destination_range) {
                return Some(overlap.start);
            }
        }

        return None;
    };

    for range in last.ranges.iter() {
        if let Some(overlap) = ranges_overlap(&range.dest(), &destination_range) {
            let start = overlap.start - range.dest_start + range.source_start;
            let size = overlap.end - overlap.start;
            let overlapping_source_range = start..start + size;

            // dbg!(
            //     &range,
            //     &destination_range,
            //     &overlap,
            //     &overlapping_source_range
            // );

            if let Some(result) =
                find_seed_number_range_corresponding(rest, overlapping_source_range, seeds)
            {
                return Some(result);
            }
        }
    }

    return None;
}

#[test]
fn task_2() {
    let mut maps = make_maps_from_input(INPUT);

    let max = maps
        .iter()
        .map(|x| {
            x.ranges
                .iter()
                .map(|x| x.source_start.min(x.dest_start))
                .max()
        })
        .flatten()
        .max()
        .unwrap();

    let mut seeds = Seeds::make_from_input(INPUT);
    seeds.seeds.sort_by_key(|x| x.start);

    let max = max.max(seeds.seeds.iter().map(|x| x.end).max().unwrap());
    let min = 0;

    maps.iter_mut().for_each(|x| x.fill_out(min, max));

    for map in maps.iter_mut() {
        map.ranges.sort_by_key(|x| x.dest_start);
    }

    let (last, split_maps) = maps.split_last().unwrap();

    for range in last.ranges.iter() {
        if let Some(seed) = find_seed_number_range_corresponding(split_maps, range.source(), &seeds)
        {
            dbg!(seed);
            let location = maps
                .iter()
                .fold(seed, |acc, v| v.destination_from_source(acc));
            dbg!(location);

            return;
        }
    }

    panic!("couldn't find solution")
}
