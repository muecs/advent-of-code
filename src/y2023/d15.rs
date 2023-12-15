//! Day 15: Lens Library

/// sum of hashes
pub fn a(input: &Vec<&str>) -> String {
    input[0]
        .split(',')
        .map(|s| calculate_hash(s))
        .fold(0usize, |acc, n| acc + n as usize)
        .to_string()
}

/// sum of focusing power of each lens
pub fn b(input: &Vec<&str>) -> String {
    let mut boxes = vec![Vec::<(&str, u8)>::new(); 256];
    input[0].split(',').for_each(|s| {
        let (label, focal_length) = if s.ends_with('-') {
            (&s[0..s.len() - 1], 0u8)
        } else {
            (&s[0..s.len() - 2], s[s.len() - 1..].parse().unwrap())
        };

        let hash = calculate_hash(label) as usize;
        let curr_box = &mut boxes[hash];
        let found = curr_box
            .iter()
            .enumerate()
            .find_map(|(i, (this_label, _))| (this_label == &label).then_some(i));

        if focal_length == 0 {
            // remove lens from box if exists
            if let Some(idx) = found {
                curr_box.remove(idx);
            }
        } else if let Some(idx) = found {
            // replace lens
            curr_box[idx].1 = focal_length;
        } else {
            // add lens to box
            curr_box.push((label, focal_length));
        }
    });

    let mut sum = 0;
    for (box_idx, curr_box) in boxes.iter().enumerate() {
        for (lens_idx, (_, focal_length)) in curr_box.iter().enumerate() {
            sum += (box_idx + 1) * (lens_idx + 1) * *focal_length as usize;
        }
    }
    sum.to_string()
}

fn calculate_hash(s: &str) -> u8 {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    s.bytes()
        .fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

#[test]
pub fn test() {
    let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

    assert_eq!(calculate_hash("HASH"), 52);
    assert_eq!(calculate_hash("rn=1"), 30);
    assert_eq!(calculate_hash("cm-"), 253);

    assert_eq!(a(&input), "1320");
    assert_eq!(b(&input), "145");
}
