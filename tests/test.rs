use mmv2000;
use std::fs;
use walkdir::WalkDir;

fn copy_test_files(test_folder_name: &str) {
    let _ = fs::remove_dir_all(format!("tests/{}", test_folder_name));
    let _ = copy_dir::copy_dir("tests/test_source", format!("tests/{}", test_folder_name));
}

fn check_folder_content(test_folder_name: &str, expected_content: Vec<&str>) {
    let mut result_contain: Vec<String> = vec![];
    for entity in WalkDir::new(format!("tests/{}", test_folder_name))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        result_contain.push(entity.path().display().to_string().replace("\\", "/"));
    }
    assert_eq!(result_contain, expected_content);
}

#[test]
fn test_no_files_found() {
    copy_test_files("test_no_files_found");
    let payload  = std::panic::catch_unwind(||
                                                mmv2000::run_mmv(mmv2000::Cli {
        old_file_mask: "tests/test_no_files_found/file_2.*".to_string(),
        new_file_mask: "tests/test_no_files_found/file_2_new.#1".to_string(),
        force: false,
    }));
    assert_eq!(true, payload.is_err());
    let _ = fs::remove_dir_all(format!("tests/{}", "test_no_files_found"));
}

#[test]
fn test_file_overwrite_panic() {
    copy_test_files("test_file_overwrite_panic");
    let payload  = std::panic::catch_unwind(||
                                              mmv2000::run_mmv(mmv2000::Cli {
                                                  old_file_mask: "tests/test_file_overwrite_panic/*/file_1.txt".to_string(),
                                                  new_file_mask: "tests/test_file_overwrite_panic/file_1.txt".to_string(),
                                                  force: false,
                                              }));
    let _ = fs::remove_dir_all(format!("tests/{}", "test_file_overwrite_panic"));
    assert_eq!(true, payload.is_err());
}

#[test]
fn test_multiple_move() {
    copy_test_files("test_multiple_move");
    mmv2000::run_mmv(mmv2000::Cli {
        old_file_mask: "tests/test_multiple_move/*/file_*.*".to_string(),
        new_file_mask: "tests/test_multiple_move/#1_#2.#3".to_string(),
        force: false,
    });
    check_folder_content(
        "test_multiple_move",
        vec![
            "tests/test_multiple_move",
            "tests/test_multiple_move/file_1.txt",
            "tests/test_multiple_move/pics",
            "tests/test_multiple_move/pics_1.png",
            "tests/test_multiple_move/texts",
            "tests/test_multiple_move/texts_1.txt",
            "tests/test_multiple_move/texts_2.txt",
        ],
    );
    let _ = fs::remove_dir_all(format!("tests/{}", "test_multiple_move"));
}

#[test]
fn test_overwrite() {
    copy_test_files("test_overwrite");
    mmv2000::run_mmv(mmv2000::Cli {
        old_file_mask: "tests/test_overwrite/texts/*".to_string(),
        new_file_mask: "tests/test_overwrite/#1".to_string(),
        force: true,
    });

    let mut result_contain: Vec<String> = vec![];
    for entity in WalkDir::new("tests/test_overwrite")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        result_contain.push(entity.path().display().to_string());
    }
    check_folder_content(
        "test_overwrite",
        vec![
            "tests/test_overwrite",
            "tests/test_overwrite/file_1.txt",
            "tests/test_overwrite/file_2.txt",
            "tests/test_overwrite/pics",
            "tests/test_overwrite/pics/file_1.png",
            "tests/test_overwrite/texts",
        ],
    );
    let _ = fs::remove_dir_all(format!("tests/{}", "test_overwrite"));
}
