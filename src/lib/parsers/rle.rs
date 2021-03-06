use super::*;

pub fn parse_rle_file<S: ToString>(s: &S) -> Result<Pattern, String> {
    let s = s.to_string();
    let mut pattern = Pattern::default();

    // Metadata
    let metadata = s.lines().take_while(|x| x.starts_with('#'));

    for line in metadata {
        let mut linedata = line.chars().skip(1);
        match linedata.next() {
            Some('N') => {
                // Name
                let name: String = linedata.collect();
                let name = name.trim();
                if !name.is_empty() {
                    pattern.name = Some(String::from(name));
                }
            }
            Some('C') | Some('c') => {
                // Comment or description
                let description: String = linedata.collect();
                let description = description.trim();
                if let Some(d) = pattern.description {
                    pattern.description = Some(format!("{}\n{}", d, description));
                } else {
                    pattern.description = Some(String::from(description));
                }
            }
            Some('O') => {
                // Author
                let author: String = linedata.collect();
                let author = author.trim();
                pattern.author = Some(String::from(author));
            }
            Some(unknown_char) => {
                return Err(format!(
                    "Unknown combination #{} in metadata of .rle file.",
                    unknown_char
                ));
            }
            None => {}
        }
    }

    // Remove all of the lines starting with `#`
    let mut lines = s.lines().skip_while(|x| x.starts_with('#'));

    // x = m, y = n
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
                        pattern.cells.push((x, y));
                        x += 1;
                    } else {
                        for i in 0..amount {
                            pattern.cells.push((x + i, y));
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
                    return Ok(pattern);
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

    Ok(pattern)
}
