/// TODO: docs.
pub fn parse_rle_file<S: ToString>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.to_string();
    let mut cells: Vec<(isize, isize)> = Vec::new();

    // Remove all of the lines starting with `#`
    let mut lines = s.lines().filter(|x| !x.starts_with('#'));

    let _header = match lines.next() {
        Some(v) => v,
        None => return Err(String::from("The header for this `.rle` file could not be found because there were no (uncommented) lines.")),
    };

    // TODO: process header information

    let data: String = lines.collect();
    let data = data.split('$');

    let mut y: isize = 0;
    for line in data {
        let mut amount: isize = 0;
        let mut x = 0;
        for c in line.chars() {
            match c {
                'b' | '.' => {
                    // Off state
                    if amount == 0 {
                        // Not preceded by a number
                        x += 1;
                    } else {
                        x += amount;
                        amount = 0;
                    }
                }
                'o' | 'A' => {
                    // On state
                    if amount == 0 {
                        // Not preceded by a number
                        cells.push((x, y));
                        x += 1;
                    } else {
                        for i in 0..amount {
                            cells.push((x + i, y));
                        }
                        x += amount;
                        amount = 0;
                    }
                }
                '0' => amount *= 10,
                '1' => amount = amount * 10 + 1,
                '2' => amount = amount * 10 + 2,
                '3' => amount = amount * 10 + 3,
                '4' => amount = amount * 10 + 4,
                '5' => amount = amount * 10 + 5,
                '6' => amount = amount * 10 + 6,
                '7' => amount = amount * 10 + 7,
                '8' => amount = amount * 10 + 8,
                '9' => amount = amount * 10 + 9,
                '!' => {
                    // The end of this pattern was reached
                    return Ok(cells);
                }
                unknown => {
                    return Err(format!(
                        "Unexpected character `{}` while reading data from a `.rle` file.",
                        unknown
                    ))
                }
            }
        }
        if amount != 0 {
            y += amount;
        } else {
            y += 1;
        }
    }

    Ok(cells)
}