use crate::r#match;
use r#match::Match;
use regex::{escape, Regex};
use std::fs;
use walkdir::WalkDir;

/// Iterates over files in directory and finds matches of given pattern
/// Returns found matches
pub fn find_matches<'a>(old_file_mask: &'a str, new_file_mask: &'a str) -> Vec<Match> {
    let mask_splitted = old_file_mask.split("/");
    let mut matches: Vec<Match> = vec![];

    'file_iter: for file_entry in WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let filename = file_entry
            .path()
            .display()
            .to_string()
            .split_off(2)
            .replace("\\", "/");
        let file_path = filename.split("/");
        if file_path.clone().count() != mask_splitted.clone().count() {
            continue 'file_iter;
        }
        let mut group_values: Vec<&str> = vec![];
        for (mask_block, file_block) in mask_splitted.clone().zip(file_path.clone()) {
            let reg_expr = format!(
                "^{}$",
                escape(mask_block).to_string().replace("\\*", "(.*)")
            );
            let re = Regex::new(&*reg_expr).unwrap();

            let mask_match = re.captures(file_block);
            if mask_match.is_none() {
                continue 'file_iter;
            }
            for i in mask_match.unwrap().iter().skip(1) {
                group_values.push(i.unwrap().as_str());
            }
        }
        matches.push(Match::new(filename.as_str(), new_file_mask, group_values));
    }
    matches.dedup();
    matches
}

#[test]
fn test_matches() {
    let expected = vec![Match {
        old_file_path: "tests/test_source/texts/file_2.txt".parse().unwrap(),
        new_file_path: "tests/test_source/new_texts/file_2.txt".parse().unwrap(),
    }];
    assert_eq!(
        find_matches(
            &"tests/test_source/*/*_2.txt".to_string(),
            &"tests/test_source/new_#1/#2_2.txt".to_string(),
        ),
        expected
    );

    let expected = vec![
        Match {
            old_file_path: "tests/test_source/texts/file_1.txt".parse().unwrap(),
            new_file_path: "tests/test_source/new_texts/file_1_updated.txt"
                .parse()
                .unwrap(),
        },
        Match {
            old_file_path: "tests/test_source/texts/file_2.txt".parse().unwrap(),
            new_file_path: "tests/test_source/new_texts/file_2_updated.txt"
                .parse()
                .unwrap(),
        },
    ];
    assert_eq!(
        find_matches(
            &"tests/test_source/*/*.txt".to_string(),
            &"tests/test_source/new_#1/#2_updated.txt".to_string(),
        ),
        expected
    );
}

/// Checks whether given file doesn't already exists
pub fn check_destination_availability(path: &str) -> bool {
    return fs::metadata(path).is_err();
}

#[test]
pub fn test_destination_available() {
    assert_eq!(
        check_destination_availability("tests/test_source/file_2.txt"),
        true
    );
    assert_eq!(
        check_destination_availability("tests/test_source/file_1.txt"),
        false
    );
}
