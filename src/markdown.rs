use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

/*
Returns content of given file as a string.

# Arguments
`path` - The path to the file to read.

# Returns
The content of the file as a string.

# Errors
If the file cannot be opened or read, an `io::Error` is returned.

TODO:
- Implement custom file size limits
- Implement parallelization
- Implement progress bars
- Implement deduplication
- Migrate to Polars
    - Save original directory structure in dataframe
*/

fn read_file_content(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

/* Extracts markdown files from a directory recursively.

# Arguments
`dir` - The directory to extract markdown files from.

# Returns
A `HashMap` containing the file names as keys and the file contents as values.

# Errors
If the directory cannot be read or if a file cannot be read, an `io::Error` is returned.


TODO:

- Implement custom file size limits
- Implement parallelization
- Implement progress bars
- Implement deduplication
- Migrate to Polars
- Save original directory structure in dataframe
pub fn extract_markdown_files(dir: &PathBuf) -> io::Result<HashMap<String, String>> {
    extract_markdown_files_recursive(dir)
}
*/

pub fn extract_markdown_files_recursive(dir: &PathBuf) -> io::Result<HashMap<String, String>> {
    /*
    FILE SIZE LIMITS
    default:
        - 10 MB per file
        - 100 MB total
    TODO:
        - implement custom limits
    */
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
    const MAX_TOTAL_SIZE: u64 = 100 * 1024 * 1024;
    let mut total_size = 0u64;

    /*
    EXTRACTION LOGIC
    TODO:
    - migrate HashMap to Polars
    - save original directory structure in dataframe
    - parallelize extraction
    - add progress bars
    - deduplication?
    */
    let mut markdown_files = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = extract_markdown_files_recursive(&path)?;
            for (name, content) in sub_files {
                let content_len = content.len() as u64;
                if total_size + content_len > MAX_TOTAL_SIZE {
                    eprintln!("Reached total size limit. Stopping file processing.");
                    return Ok(markdown_files);
                }
                markdown_files.insert(name, content);
                total_size += content_len;
            }
        } else if path.extension() == Some(std::ffi::OsStr::new("md")) {
            let file_size = entry.metadata()?.len();

            if file_size > MAX_FILE_SIZE {
                eprintln!("Skipping large file: {:?} ({} bytes)", path, file_size);
                continue;
            }

            if total_size + file_size > MAX_TOTAL_SIZE {
                eprintln!("Reached total size limit. Stopping file processing.");
                return Ok(markdown_files);
            }

            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let content = read_file_content(&path)?;

            markdown_files.insert(file_name, content);
            total_size += file_size;
        }
    }

    Ok(markdown_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn setup_test_dir() -> PathBuf {
        // Map test directory
        let test_dir = Path::new("tests/tmp");

        // Create test directory if it doesn't exist
        if !test_dir.exists() {
            fs::create_dir_all(test_dir).unwrap();
        }

        // Map test markdown files
        let small_md_file = test_dir.join("small.md");
        let large_md_file = test_dir.join("large.md");

        // Write a small file (within limits)
        if !small_md_file.exists() {
            File::create(&small_md_file)
                .unwrap()
                .write_all(b"Hello, Markdown!")
                .unwrap();
        }

        // Write a large file (exceeding limits)
        if !large_md_file.exists() {
            let mut large_file = File::create(&large_md_file).unwrap();
            for _ in 0..(15 * 1024 * 1024) {
                // ~15 MB
                large_file.write_all(b"A").unwrap();
            }
        }
        test_dir.to_path_buf()
    }

    #[test]
    fn test_extract_markdown_files_recursive() {
        let test_dir = setup_test_dir();

        // Test extraction of markdown files
        let result = extract_markdown_files_recursive(&test_dir).unwrap();

        // Check that we only have the small file
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("small.md"));
        assert_eq!(result["small.md"], "Hello, Markdown!");
    }
}
