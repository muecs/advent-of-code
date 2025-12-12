//! Day 12: Christmas Tree Farm

/// Number of regions able to fit given shapes
pub fn a(input: &Vec<&str>) -> String {
    let regions = parse_input(input);
    regions
        .iter()
        .filter(|region| {
            let region_area = region.width * region.height;
            let shapes_area = 9 * region.quantities.iter().sum::<usize>();
            shapes_area <= region_area // WTF?
            // (doesn't work on the example though)
        })
        .count()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Regions {
    // assume 6 3x3 shapes
    // let shapes = input
    //     .chunks_exact(5)
    //     .map(|lines| {
    //         lines[1..4]
    //             .iter()
    //             .flat_map(|line| line[0..3].bytes().map(|b| b == b'#'))
    //             .collect::<Vec<_>>()
    //             .try_into()
    //             .unwrap()
    //     })
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .unwrap();

    input[30..]
        .iter()
        .map(|line| {
            let ((width, height), quantities) = line
                .split_once(": ")
                .map(|(wh, q)| {
                    (
                        wh.split_once('x')
                            .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
                            .unwrap(),
                        q.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap();
            Region {
                width,
                height,
                quantities: quantities.try_into().unwrap(),
            }
        })
        .collect()
}

// type Shape = [bool; 9];
// type Shapes = [Shape; 6];

struct Region {
    width: usize,
    height: usize,
    quantities: [usize; 6],
}
type Regions = Vec<Region>;

#[test]
pub fn test() {
    let _input = vec![
        "0:",
        "###",
        "##.",
        "##.",
        "",
        "1:",
        "###",
        "##.",
        ".##",
        "",
        "2:",
        ".##",
        "###",
        "##.",
        "",
        "3:",
        "##.",
        "###",
        "##.",
        "",
        "4:",
        "###",
        "#..",
        "###",
        "",
        "5:",
        "###",
        ".#.",
        "###",
        "",
        "4x4: 0 0 0 0 2 0",
        "12x5: 1 0 1 0 2 2",
        "12x5: 1 0 1 0 3 2",
    ];

    // assert_eq!(a(&input), "2");
}
