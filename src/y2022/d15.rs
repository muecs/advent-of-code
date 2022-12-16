//! Day 15: Beacon Exclusion Zone

use std::collections::BTreeSet;

/// number of positions excluded to contain a beacon
pub fn a(input: &Vec<&str>) -> String {
    let report = parse_input(input);
    let exclusion_count = count_beacon_exclusion(&report, 2000000);
    exclusion_count.to_string()
}

/// encoded coordinates of hidden beacon
pub fn b(input: &Vec<&str>) -> String {
    let report = parse_input(input);
    let beacon = find_hidden_beacon(&report, 4000000);
    (beacon.0 * 4000000 + beacon.1).to_string()
}

fn parse_input(input: &Vec<&str>) -> Report {
    input
        .iter()
        .map(|line| line[12..].split_once(": closest beacon is at x=").unwrap())
        .map(|pair| {
            (
                pair.0.split_once(", y=").unwrap(),
                pair.1.split_once(", y=").unwrap(),
            )
        })
        .map(|(sensor, beacon)| {
            (
                (
                    sensor.0.parse().unwrap(),
                    sensor.1.parse().unwrap(),
                ),
                (
                    beacon.0.parse().unwrap(),
                    beacon.1.parse().unwrap(),
                ),
            )
        })
        .map(|(sensor, beacon)| SensorData {
            sensor, beacon, radius: distance(&sensor, &beacon) })
        .collect()
}

/// calculates Manhattan Distance between two points
fn distance(p1: &Point, p2: &Point) -> isize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as isize
}

/// determines intersections of row with sensor area (beacon exclusion zone)
fn get_sensor_intersections(report: &Report, row: isize) -> Vec<(isize, bool)> {
    let mut intersections = Vec::with_capacity(report.len() * 2);
    for data in report {
        let range = data.radius - (data.sensor.1 - row).abs();
        if range >= 0 {
            intersections.push((data.sensor.0 - range, true));  // start
            intersections.push((data.sensor.0 + range, false));  // end
        }
    }
    intersections.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    intersections
}

fn count_beacon_exclusion(report: &Report, row: isize) -> usize {
    let intersections = get_sensor_intersections(report, row);

    let mut exclusion_count = 0usize;
    let mut overlaps = 0usize;
    let mut start = isize::MAX;

    for (x, inside) in &intersections {
        if *inside {
            if overlaps == 0 {
                start = *x;
            }
            overlaps += 1;
        } else {
            overlaps -= 1;
            if overlaps == 0 {
                exclusion_count += (*x - start + 1) as usize;
            }
        }
    }

    let beacons_in_row = report
        .iter()
        .filter_map(|data| data.beacon.1.eq(&row).then(|| data.beacon))
        .collect::<BTreeSet<_>>()
        .len();

    exclusion_count - beacons_in_row
}

fn find_hidden_beacon(report: &Report, max_coord: isize) -> Point {
    for row in 0..=max_coord {  // (this is slow)
        let intersections = get_sensor_intersections(report, row);

        let mut overlaps = 0usize;
        let mut end = isize::MAX;

        for (x, inside) in &intersections {
            if *inside {
                if overlaps == 0 && end < *x - 1 {
                    return (x - 1, row);
                }
                overlaps += 1;
            } else {
                overlaps -= 1;
                if overlaps == 0 {
                    end = *x;
                }
            }
        }
    }
    
    unreachable!("failed to find hidden beacon");
}

type Point = (isize, isize);
struct SensorData {
    sensor: Point,
    beacon: Point,
    radius: isize,
}
type Report = Vec<SensorData>;

#[test]
pub fn test() {
    let input = vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    let report = parse_input(&input);

    assert_eq!(count_beacon_exclusion(&report, 10), 26);
    assert_eq!(find_hidden_beacon(&report, 20), (14, 11));

    // assert_eq!(a(&input), "26");
    // assert_eq!(b(&input), "56000011");
}
