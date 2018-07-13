// TODO: tests.
// extern crate game_of_life;
// use game_of_life::parsers::life_105::*;

// #[test]
// fn test_life_105_is_life_105_file() {
//     assert!(is_life_105_file(&"#Life 1.05\n#D Name: Some name"));
//     assert!(!is_life_105_file(&"#Life 1.06\n#D Name: No name"));
// }

// #[test]
// fn test_life_105_correct_file() {
//     let file = "#Life 1.05
// #D Name: B-52 bomber
// #D Author: Noam Elkies
// #D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
// #D  glider every 52 generations.
// #D www.conwaylife.com/wiki/index.php?title=B-52_bomber
// #N
// #P -19 -10
// .**
// .**.................*
// ...................*.*............*.*
// ....................*............*
// **.......**.......................*..*
// **.*.....**.......................*.*.*
// ...*.......................*.......*..*
// ...*.......................**.......**
// *..*.................**.....*
// .**..................*
// .....................***
// ....................................**
// ....................................**
// .**
// *..*
// *.*.*................*.*....**.....**
// .*..*.................**....**.....**.*
// .....*............*...*...............*
// ..*.*............*.*..................*
// ..................*................*..*
// ....................................**
// ";
//     assert!(parse_life_105_file(&file).is_ok())
// }

// #[test]
// fn test_life_105_incorrect_file() {
//     let file = "#Life 1.05
// #D Name: B-52 bomber
// #D Author: Noam Elkies
// #D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
// #D  glider every 52 generations.
// #D www.conwaylife.com/wiki/index.php?title=B-52_bomber
// #N
// #P -19 -10
// .**
// .**.................*
// ...................*.*............*.*
// ....................*............*
// **.......**.......................*..*
// **.*.....**.......................*.*.*
// ...*.......................*.......*..*
// ...*.....wrong char......**.......**
// *..*.................**.....*
// .**..................*
// .....................***
// ....................................**
// ....................................**
// .**
// *..*
// *.*.*................*.*....**.....**
// .*..*.................**....**.....**.*
// .....*............*...*...............*
// ..*.*............*.*..................*
// ..................*................*..*
// ....................................**
// ";
//     assert!(parse_life_105_file(&file).is_err());

//     let file = "#Life 1.05
// #D Name: B-52 bomber
// #D Author: Noam Elkies
// #D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
// #D  glider every 52 generations.
// #D www.conwaylife.com/wiki/index.php?title=B-52_bomber
// #N
// #P no x no y
// .**
// .**.................*
// ...................*.*............*.*
// ....................*............*
// **.......**.......................*..*
// **.*.....**.......................*.*.*
// ...*.......................*.......*..*
// ...*.......................**.......**
// *..*.................**.....*
// .**..................*
// .....................***
// ....................................**
// ....................................**
// .**
// *..*
// *.*.*................*.*....**.....**
// .*..*.................**....**.....**.*
// .....*............*...*...............*
// ..*.*............*.*..................*
// ..................*................*..*
// ....................................**
// ";
//     assert!(parse_life_105_file(&file).is_err());
// }
