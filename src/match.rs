use crate::fs_utils::check_destination_availability;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
/// Contains two paths - source path and path to move
pub struct Match {
    /// Old path to file: this file will be moved
    pub old_file_path: String,
    /// New path to file: file will be moved to this path
    pub new_file_path: String,
}

#[derive(Debug)]
/// Contains status of file move
pub enum MoveStatus {
    /// File destination already existed, so it was overwritten
    MovedNotOverwritten,
    /// File destination didn't exist
    MovedOverwritten,
}

impl Match {
    /// Creates Match by source path, mask for a new file and Vec of values to replace in mask
    pub fn new(old_file_path: &str, new_file_mask: &str, group_values: Vec<&str>) -> Self {
        let mut new_file_path = new_file_mask.to_string();
        for i in 0..group_values.len() {
            new_file_path =
                new_file_path.replace(format!("#{}", i + 1).as_str(), &*group_values[i]);
        }
        Match {
            old_file_path: old_file_path.to_string(),
            new_file_path: new_file_path,
        }
    }

    /// Executes move from old path to new, returns Result, containing move status or an error
    pub fn execute_move(&self) -> Result<MoveStatus, std::io::Error> {
        if self.new_file_path.contains("/") {
            let dir = &self.new_file_path[..self.new_file_path.rfind("/").unwrap()];
            fs::create_dir_all(dir)?;
        }
        let file_existed = !check_destination_availability(&self.new_file_path);
        fs::rename(&self.old_file_path, &self.new_file_path)?;
        return Ok(if file_existed {
            MoveStatus::MovedOverwritten
        } else {
            MoveStatus::MovedNotOverwritten
        });
    }
}

#[test]
fn test_match_constructor() {
    let x = Match {
        old_file_path: "".to_string(),
        new_file_path: "1/@/fdfd".to_string(),
    };
    let y = Match::new("", "#1/#3/#2", vec!["1", "fdfd", "@"]);
    assert_eq!(x, y);
}
