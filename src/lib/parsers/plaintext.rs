use super::Pattern;

pub fn is_plaintext_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("!Name:")
}

pub fn parse_plaintext_file<S: AsRef<str>>(s: &S) -> Result<Pattern, String> {
    let s = s.as_ref();

    let mut pattern = Pattern::default();

    let mut metadata = s.lines().take_while(|x| x.starts_with('!'));

    // Process name (!Name: name)
    if let Some(name) = metadata.next() {
        // exlude the "!Name:" and remove surrounding whitespace
        let name: &str = name[6..].trim();
        pattern.name = Some(String::from(name));
    }

    // Process comments (lines starting with '!')
    for description in metadata {
        // Check for other information
        if description.starts_with("!Author:") {
            let description = description[8..].trim();
            pattern.author = Some(String::from(description));
        } else {
            // Default, this line is a description
            let description = description[1..].trim();
            // and add to earlier description lines
            if let Some(d) = pattern.description {
                pattern.description = Some(format!("{}\n{}", d, description));
            } else {
                pattern.description = Some(String::from(description));
            }
        }
    }

    let lines = s.lines().skip_while(|x| x.starts_with('!'));

    for (y, line) in lines.enumerate() {
        for (x, token) in line.chars().enumerate() {
            match token {
                // Cell is alive.
                'O' => {
                    pattern.cells.push((x as isize, y as isize));
                }
                // Cell is dead.
                '.' => {}
                a => {
                    return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
                }
            }
        }
    }

    Ok(pattern)
}
