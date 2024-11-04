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
*/

fn extract_markdown_files_common(
    dir: &PathBuf,
    allow_recursion: bool,
) -> io::Result<HashMap<String, String>> {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
    const MAX_TOTAL_SIZE: u64 = 100 * 1024 * 1024; // 100 MB

    let mut total_size = 0u64;
    let mut markdown_files = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        match (path.is_dir(), allow_recursion) {
            (true, true) => {
                // Recursively extract files from the directory
                let sub_files = extract_markdown_files_common(&path, true)?;
                for (name, content) in sub_files {
                    let content_len = content.len() as u64;
                    if total_size + content_len > MAX_TOTAL_SIZE {
                        eprintln!("Reached total size limit. Stopping file processing.");
                        return Ok(markdown_files);
                    }
                    markdown_files.insert(name, content);
                    total_size += content_len;
                }
            }
            (true, false) => {
                // Stop processing if a directory is found and recursion is not allowed
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Error: target is a directory, use recursion",
                ));
            }
            (false, _) => {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
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
            }
        }
    }

    Ok(markdown_files)
}

pub fn extract_markdown_files_recursive(dir: &PathBuf) -> io::Result<HashMap<String, String>> {
    extract_markdown_files_common(dir, true)
}

pub fn extract_markdown_files_non_recursive(dir: &PathBuf) -> io::Result<HashMap<String, String>> {
    extract_markdown_files_common(dir, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn setup_test_path_buffer() -> PathBuf {
        // Map test directory
        let test_path_buffer = Path::new("tests/tmp");

        // Create test directory if it doesn't exist
        if !test_path_buffer.exists() {
            fs::create_dir_all(test_path_buffer).unwrap();
            fs::create_dir_all(test_path_buffer.join("dir")).unwrap();
        }

        // Map test markdown files
        let small_md_file = test_path_buffer.join("small.md");
        let large_md_file = test_path_buffer.join("large.md");
        let dir_md_file = test_path_buffer.join("dir").join("dir.md");

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

        // Write a small file (within limits) in dir
        if !dir_md_file.exists() {
            File::create(&dir_md_file)
                .unwrap()
                .write_all(b"Hello, Markdown in the directory!")
                .unwrap();
        }
        test_path_buffer.to_path_buf()
    }

    #[test]
    fn test_extract_markdown_files() {
        // Create path buffer
        let test_path_buffer = setup_test_path_buffer();

        // Test extraction of markdown files
        let result = extract_markdown_files_recursive(&test_path_buffer).unwrap();

        // Check that we only have the two small files
        assert_eq!(result.len(), 3);
        assert!(result.contains_key("small.md"));
        assert!(result.contains_key("dir.md"));
        assert_eq!(result["small.md"], "Hello, Markdown!");
        assert_eq!(result["dir.md"], "Hello, Markdown in the directory!");
    }

    #[test]
    fn test_extract_markdown_files_non_recursive_error() {
        // Create path buffer
        let test_path_buffer = setup_test_path_buffer();

        // Test non-recursive extraction of markdown files, with a directory as target
        let result = extract_markdown_files_non_recursive(&test_path_buffer).unwrap_err();

        // Check the output for the error message
        assert_eq!(
            result.to_string(),
            "Error: target is a directory, use recursion"
        );
    }
}
