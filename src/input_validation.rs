use regex::Regex;

/// Checks pattern for absence of ** and replace-groups for validity
/// Panics in case of an error
pub fn validate_patterns(mask_from: &str, mask_to: &str) {
    if mask_from.contains("**") {
        panic!("Mask can't contain 2 '*' symlbols in a row");
    }
    let group_cnt = mask_from.matches("*").count();
    let replace_find_regex = Regex::new(r"#(\d+)").unwrap();
    for group in replace_find_regex.find_iter(mask_to) {
        let group_id = &group.to_owned().as_str()[1..].parse::<i32>().unwrap();
        match *group_id {
            cond if cond <= 0 => panic!("Group id cannot be less than 1"),
            cond if cond > group_cnt as i32 => {
                panic!("Group id cannot be larger than number of * in from_mask")
            }
            _ => {}
        }
    }
}

#[test]
#[should_panic]
fn test_2_asterisks() {
    let mut asterisks_path = "path/file_**.txt".to_string();
    let mut usual_path = "path/file_#1.txt".to_string();
    validate_patterns(&mut asterisks_path, &mut usual_path);
}

#[test]
#[should_panic]
fn test_zero_group_id() {
    let mut mask_from = "path/file_*.txt";
    let mut mask_to = "path/file_#0.txt";
    validate_patterns(&mut mask_from, &mut mask_to);
}

/// Removes extra slashes and makes them forward-slashes
pub fn normalize_mask_slashes(mask: &mut String) {
    *mask = mask.replace("\\", "/");
    let multiple_slash_regex = Regex::new(r"/+/").unwrap();
    *mask = multiple_slash_regex.replace_all(mask, "/").parse().unwrap();
}

#[test]
fn test_normalize_mask_slashes() {
    let mut backslash_path = "path\\to\\file".to_string();
    normalize_mask_slashes(&mut backslash_path);
    assert_eq!(backslash_path, "path/to/file");

    let mut multiple_slash_path = "path/to//file\\\\but//////////longer".to_string();
    normalize_mask_slashes(&mut multiple_slash_path);
    assert_eq!(multiple_slash_path, "path/to/file/but/longer");

    let mut backslash_path = "path\\to\\file".to_string();
    normalize_mask_slashes(&mut backslash_path);
    assert_eq!(backslash_path, "path/to/file");
}
