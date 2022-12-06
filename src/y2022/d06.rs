//! Day 6: Tuning Trouble

/// end of first sequence of 4 unique characters
pub fn a(input: &Vec<&str>) -> String {
    find_marker(input[0], 4).unwrap().to_string()
}

/// end of first sequence of 14 unique characters
pub fn b(input: &Vec<&str>) -> String {
    find_marker(input[0], 14).unwrap().to_string()
}

fn find_marker(msg: &str, len: usize) -> Option<usize> {
    let mut index = len;
    for window in msg.as_bytes().windows(len) {
        let mut is_repeated = false;
        'outer: for i in 0..len-1 {
            for j in i+1..len {
                if window[i] == window[j] {
                    is_repeated = true;
                    break 'outer;
                }
            }
        }
        if !is_repeated {
            return Some(index);
        }
        index += 1;
    }
    None
}

#[test]
pub fn test() {
    let input1 = vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb"];
    let input2 = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz"];
    let input3 = vec!["nppdvjthqldpwncqszvftbrmjlhg"];
    let input4 = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"];
    let input5 = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];

    assert_eq!(a(&input1), "7");
    assert_eq!(a(&input2), "5");
    assert_eq!(a(&input3), "6");
    assert_eq!(a(&input4), "10");
    assert_eq!(a(&input5), "11");

    assert_eq!(b(&input1), "19");
    assert_eq!(b(&input2), "23");
    assert_eq!(b(&input3), "23");
    assert_eq!(b(&input4), "29");
    assert_eq!(b(&input5), "26");
}
