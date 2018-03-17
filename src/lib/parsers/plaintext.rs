/// TODO: docs.
pub fn is_plaintext_file<S: ToString>(s: &S) -> bool {
    s.to_string().starts_with("!Name:")
}

/// TODO: docs.
pub fn parse_plaintext_file<S: ToString>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    unimplemented!()
}

// pub struct Parser;

// impl Parser {
//     /// Check whether the given file (read to string) is a plaintext file.
//     /// A file is a plaintext file if it starts with `!Name: `
//     pub fn is_plaintext_file<S>(s: S) -> bool
//     where
//         S: ToString,
//     {
//         let s = s.to_string();
//         let mut lines = s.lines();

//         if let Some(text) = lines.next() {
//             text.starts_with("!Name: ")
//         } else {
//             false
//         }
//     }

//     /// Parse a plaintext file to a `Vec<(isize, isize)>`.
//     pub fn parse_plaintext_file<S>(s: S) -> Result<super::Pattern, String>
//     where
//         S: ToString,
//     {
//         let s = s.to_string();
//         let lines = s.lines().skip_while(|x: &&str| x.starts_with('!'));

//         let mut pattern = super::Pattern::empty();

//         let mut y: isize = 0;
//         for line in lines {
//             for (x, token) in line.chars().enumerate() {
//                 match token {
//                     'O' => {
//                         // Cell is alive here
//                         pattern.cells.push((x as isize, y));
//                     }
//                     '.' => {
//                         // Cell is dead here
//                     }
//                     a => {
//                         return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
//                     }
//                 }
//             }
//             y += 1;
//         }

//         Ok(pattern)
//     }
// }
