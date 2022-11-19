//! Day 19: Beacon Scanner

use std::collections::{BTreeMap, VecDeque};

type Point = (i32, i32, i32);
type Points = Vec<Point>;

struct PointCloud {
    points: Points,
    distances: Vec<(usize, usize, i32)>,
}
type PointClouds = Vec<PointCloud>;

type Matrix = [i32; 9];  // 3x3
type Transformation = (&'static Matrix, Point);
type Transformations = BTreeMap<usize, BTreeMap<usize, Transformation>>;

const ROTATION_MATRICES: [Matrix; 24] = [
    [-1, 0, 0, 0, -1, 0, 0, 0, 1],
    [-1, 0, 0, 0, 0, -1, 0, -1, 0],
    [-1, 0, 0, 0, 0, 1, 0, 1, 0],
    [-1, 0, 0, 0, 1, 0, 0, 0, -1],
    [0, -1, 0, -1, 0, 0, 0, 0, -1],
    [0, -1, 0, 0, 0, -1, 1, 0, 0],
    [0, -1, 0, 0, 0, 1, -1, 0, 0],
    [0, -1, 0, 1, 0, 0, 0, 0, 1],
    [0, 0, -1, -1, 0, 0, 0, 1, 0],
    [0, 0, -1, 0, -1, 0, -1, 0, 0],
    [0, 0, -1, 0, 1, 0, 1, 0, 0],
    [0, 0, -1, 1, 0, 0, 0, -1, 0],
    [0, 0, 1, -1, 0, 0, 0, -1, 0],
    [0, 0, 1, 0, -1, 0, 1, 0, 0],
    [0, 0, 1, 0, 1, 0, -1, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, -1, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, -1, -1, 0, 0],
    [0, 1, 0, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 0, -1],
    [1, 0, 0, 0, -1, 0, 0, 0, -1],
    [1, 0, 0, 0, 0, -1, 0, 1, 0],
    [1, 0, 0, 0, 0, 1, 0, -1, 0],
    [1, 0, 0, 0, 1, 0, 0, 0, 1],
];

/// number of unique points considering overlaps
pub fn a(input: &Vec<&str>) -> String {
    let point_clouds = parse_input(input);
    let transformations = find_transformations(&point_clouds);

    let mut transformed_points = point_clouds[0].points.clone();
    for i in 1..point_clouds.len() {
        transformed_points.append(&mut apply_transformations(
            &point_clouds[i].points,
            &transformations, 
            &i
        ));
    }
    transformed_points.sort_unstable();
    transformed_points.dedup();
    
    transformed_points.len().to_string()
}

/// maximum Manhattan distance between point cloud origins
pub fn b(input: &Vec<&str>) -> String {
    let point_clouds = parse_input(input);
    let transformations = find_transformations(&point_clouds);

    let mut origins = vec![(0, 0, 0)];
    for i in 1..point_clouds.len() {
        origins.append(&mut apply_transformations(
            &vec![(0, 0, 0)],
            &transformations, 
            &i
        ));
    }

    let mut max_dist = 0;
    for i in 0..origins.len()-1 {
        for j in i+1..origins.len() {
            let diff = translation(&origins[i], &origins[j]);
            let dist = diff.0.abs() + diff.1.abs() + diff.2.abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    max_dist.to_string()
}

fn parse_input(input: &Vec<&str>) -> PointClouds {
    let mut point_clouds = PointClouds::new();
    let mut points = Points::new();
    for &line in input {
        if line.starts_with("--- scanner") {
            if !points.is_empty() {
                let distances = distances(&points);
                point_clouds.push(PointCloud { points, distances });
                points = Points::new();
            }
        } else if line.is_empty() {
            continue;
        } else {
            let mut it = line.split(",").map(|s| s.parse::<i32>().unwrap());
            points.push((
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ))
        }
    }
    if !points.is_empty() {
        let distances = distances(&points);
        point_clouds.push(PointCloud { points, distances });
    }
    point_clouds
}

/// Calculates squared distance between two points
fn distance_sq(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

/// Calculates sorted, squared distances between all points
fn distances(points: &Points) -> Vec<(usize, usize, i32)> {
    let mut distances = Vec::new();
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            distances.push((i, j, distance_sq(&points[i], &points[j])));
        }
    }
    distances.sort_unstable_by(|a, b| a.2.cmp(&b.2));
    distances
}

/// Determines number occurring most often
fn mode(numbers: &Vec<usize>) -> Option<usize> {
    let mut counts = BTreeMap::new();
    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

/// Applies a rotation matrix to a point
fn rotate(m: &Matrix, p: &Point) -> Point {
    (
        m[0] * p.0 + m[1] * p.1 + m[2] * p.2,
        m[3] * p.0 + m[4] * p.1 + m[5] * p.2,
        m[6] * p.0 + m[7] * p.1 + m[8] * p.2,
    )
}

/// Translates a point by a vector
fn translate(p: &Point, v: &Point) -> Point {
    (
        p.0 + v.0,
        p.1 + v.1,
        p.2 + v.2,
    )
}

/// Calculates vector to translate one point into another
fn translation(p1: &Point, p2: &Point) -> Point {
    (
        p2.0 - p1.0,
        p2.1 - p1.1,
        p2.2 - p1.2,
    )
}

/// Determines rotation and translation required to transform points from source
/// to target
fn find_transformation(source: &[&Point], target: &[&Point]) -> (&'static Matrix, Point) {
    assert!(source.len() >= 2);
    assert!(target.len() >= 2);
    for matrix in &ROTATION_MATRICES {
        let v1 = translation(&rotate(matrix, source[0]), target[0]);
        let v2 = translation(&rotate(matrix, source[1]), target[1]);
        if v1 == v2 {
            return (matrix, v1)
        }
    }
    unreachable!("No rotation matrix found.");
}

fn find_transformations(point_clouds: &PointClouds) -> Transformations {
    let mut transformations = Transformations::new();

    for i in 0..point_clouds.len()-1 {
        for j in i+1..point_clouds.len() {
            // find matching point distances
            let mut candidates: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for (a, b, dist) in &point_clouds[i].distances {
                let result = point_clouds[j]
                    .distances
                    .binary_search_by(|x| x.2.cmp(&dist));
                if let Ok(idx) = result {
                    // point pairs a/b and c/d match
                    let (c, d, _) = point_clouds[j].distances[idx];
                    for k in [a, b] {
                        candidates
                            .entry(*k)
                            .or_insert(Vec::new())
                            .append(&mut vec![c, d]);
                    }
                }
            }
            if candidates.len() >= 12 {
                let pairs = candidates
                    .iter()
                    .map(|item| (*item.0, mode(item.1).unwrap()))
                    .collect::<Vec<_>>();

                let points_i = pairs
                    .iter()
                    .take(2)
                    .map(|(a, _)| &point_clouds[i].points[*a])
                    .collect::<Vec<_>>();
                let points_j = pairs
                    .iter()
                    .take(2)
                    .map(|(_, b)| &point_clouds[j].points[*b])
                    .collect::<Vec<_>>();
                
                transformations
                    .entry(i)
                    .or_insert(BTreeMap::new())
                    .insert(j, find_transformation(&points_i, &points_j));
                transformations
                    .entry(j)
                    .or_insert(BTreeMap::new())
                    .insert(i, find_transformation(&points_j, &points_i));

                #[cfg(test)]
                println!("groups {i} and {j} - {} pairs", candidates.len());
            }
        }
    }

    #[cfg(test)]
    {
        println!("Transformation graph before trimming:");
        transformations
            .iter()
            .for_each(|(k, v)| println!(
                "  {} -> {:?}",
                k,
                v.keys().collect::<Vec<_>>(),
            ));
    }

    let mut visited = Vec::new();
    let mut pending = VecDeque::from([0usize]);
    while let Some(node) = pending.pop_front() {
        visited.push(node);
        let adjacent_nodes = transformations[&node]
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for adj in &adjacent_nodes {
            if !visited.contains(adj) {
                pending.push_back(*adj);
            }
        }
        transformations
            .entry(node)
            .and_modify(|e| for v in &visited {
                if e.contains_key(v) {
                    e.retain(|k, _| *k == *v);
                    break;
                }
            });
    }

    #[cfg(test)]
    {
        println!("Transformation graph after trimming:");
        transformations
            .iter()
            .for_each(|(k, v)| println!(
                "  {} -> {:?}",
                k,
                v.keys().collect::<Vec<_>>(),
            ));
    }

    transformations
}

fn apply_transformations(
    points: &Points,
    transformations: &Transformations,
    source: &usize
) -> Points {
    assert_ne!(*source, 0);
    let (target, (m, v)) = transformations[source].iter().next().unwrap();
    let transformed_points = points
        .iter()
        .map(|p| translate(&rotate(m, p), v))
        .collect::<Vec<_>>();
    if *target == 0 {
        transformed_points
    } else {
        apply_transformations(&transformed_points, transformations, target)
    }
}

#[test]
pub fn test() {
    let input = vec![
        "--- scanner 0 ---",
        "404,-588,-901",
        "528,-643,409",
        "-838,591,734",
        "390,-675,-793",
        "-537,-823,-458",
        "-485,-357,347",
        "-345,-311,381",
        "-661,-816,-575",
        "-876,649,763",
        "-618,-824,-621",
        "553,345,-567",
        "474,580,667",
        "-447,-329,318",
        "-584,868,-557",
        "544,-627,-890",
        "564,392,-477",
        "455,729,728",
        "-892,524,684",
        "-689,845,-530",
        "423,-701,434",
        "7,-33,-71",
        "630,319,-379",
        "443,580,662",
        "-789,900,-551",
        "459,-707,401",
        "",
        "--- scanner 1 ---",
        "686,422,578",
        "605,423,415",
        "515,917,-361",
        "-336,658,858",
        "95,138,22",
        "-476,619,847",
        "-340,-569,-846",
        "567,-361,727",
        "-460,603,-452",
        "669,-402,600",
        "729,430,532",
        "-500,-761,534",
        "-322,571,750",
        "-466,-666,-811",
        "-429,-592,574",
        "-355,545,-477",
        "703,-491,-529",
        "-328,-685,520",
        "413,935,-424",
        "-391,539,-444",
        "586,-435,557",
        "-364,-763,-893",
        "807,-499,-711",
        "755,-354,-619",
        "553,889,-390",
        "",
        "--- scanner 2 ---",
        "649,640,665",
        "682,-795,504",
        "-784,533,-524",
        "-644,584,-595",
        "-588,-843,648",
        "-30,6,44",
        "-674,560,763",
        "500,723,-460",
        "609,671,-379",
        "-555,-800,653",
        "-675,-892,-343",
        "697,-426,-610",
        "578,704,681",
        "493,664,-388",
        "-671,-858,530",
        "-667,343,800",
        "571,-461,-707",
        "-138,-166,112",
        "-889,563,-600",
        "646,-828,498",
        "640,759,510",
        "-630,509,768",
        "-681,-892,-333",
        "673,-379,-804",
        "-742,-814,-386",
        "577,-820,562",
        "",
        "--- scanner 3 ---",
        "-589,542,597",
        "605,-692,669",
        "-500,565,-823",
        "-660,373,557",
        "-458,-679,-417",
        "-488,449,543",
        "-626,468,-788",
        "338,-750,-386",
        "528,-832,-391",
        "562,-778,733",
        "-938,-730,414",
        "543,643,-506",
        "-524,371,-870",
        "407,773,750",
        "-104,29,83",
        "378,-903,-323",
        "-778,-728,485",
        "426,699,580",
        "-438,-605,-362",
        "-469,-447,-387",
        "509,732,623",
        "647,635,-688",
        "-868,-804,481",
        "614,-800,639",
        "595,780,-596",
        "",
        "--- scanner 4 ---",
        "727,592,562",
        "-293,-554,779",
        "441,611,-461",
        "-714,465,-776",
        "-743,427,-804",
        "-660,-479,-426",
        "832,-632,460",
        "927,-485,-438",
        "408,393,-506",
        "466,436,-512",
        "110,16,151",
        "-258,-428,682",
        "-393,719,612",
        "-211,-452,876",
        "808,-476,-593",
        "-575,615,604",
        "-485,667,467",
        "-680,325,-822",
        "-627,-443,-432",
        "872,-547,-609",
        "833,512,582",
        "807,604,487",
        "839,-516,451",
        "891,-625,532",
        "-652,-548,-490",
        "30,-46,-14",
    ];

    assert_eq!(a(&input), "79");
    assert_eq!(b(&input), "3621");
}
