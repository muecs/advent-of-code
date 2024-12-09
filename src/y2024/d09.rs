//! Day 9: Disk Fragmenter

/// compaction checksum of fragmented file IDs
pub fn a(input: &Vec<&str>) -> String {
    let disk = parse_input(input);
    let mut checksum = 0;
    let mut block = 0;
    let mut back = disk.len() - 1;
    let mut back_left = disk[back];
    for front in 0..disk.len() {
        if front >= back {
            break;
        }
        for _ in 0..disk[front] {
            let id = if front % 2 == 0 {
                front // file
            } else {
                // free space; take file(s) from back
                while back_left == 0 {
                    back -= 2;
                    back_left = disk[back];
                }
                back_left -= 1;
                back
            } / 2;
            checksum += block * id;
            block += 1;
        }
    }
    for _ in 0..back_left {
        let id = back / 2;
        checksum += block * id;
        block += 1;
    }
    checksum.to_string()
}

/// compaction checksum of completely moved file IDs
pub fn b(input: &Vec<&str>) -> String {
    let disk = parse_input(input);
    let mut spaces = disk
        .windows(2)
        .step_by(2)
        .scan(0, |offset, sizes| {
            *offset += sizes[0] as usize; // space taken by file
            let space = (sizes[1], *offset);
            *offset += sizes[1] as usize; // free space
            Some(space)
        })
        .collect::<Vec<_>>();

    // NOTE: first file (ID=0) doesn't contribute to checksum
    (2..disk.len())
        .rev()
        .step_by(2)
        .map(|back| {
            // try to move right-most file
            let id = back / 2;
            for (size, offset) in spaces.iter_mut().take(id) {
                // find left-most space
                if *size >= disk[back] {
                    let checksum = id * (*offset..*offset + disk[back] as usize).sum::<usize>();
                    *size -= disk[back];
                    *offset += disk[back] as usize;
                    return checksum;
                }
            }

            // if we didn't find a space leave the file where it is
            let offset = disk.iter().take(back).map(|size| *size as usize).sum::<usize>();
            id * (offset..offset + disk[back] as usize).sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<u8> {
    input
        .first()
        .unwrap()
        .bytes()
        .map(|b| match b {
            b'0'..=b'9' => b - b'0',
            _ => unreachable!(),
        })
        .collect()
}

#[test]
pub fn test() {
    let input = vec!["2333133121414131402"];

    assert_eq!(a(&input), "1928");
    assert_eq!(b(&input), "2858");
}
