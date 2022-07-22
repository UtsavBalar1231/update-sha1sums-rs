//use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};

pub struct UpdateSha1sums {
    pub cleanup: bool,
}

impl UpdateSha1sums {
    /// Creates a new MissingBlobs builder.
    pub fn builder() -> UpdateSha1sumsBuilder {
        UpdateSha1sumsBuilder::default()
    }

    /// Searches for blobs in the given paths, and displays missing dependencies.
    pub fn run(&self, content: &str, paths: &[&str], vendor_path: &str) {
        let mut lines = content
            .lines()
            .collect::<Vec<&str>>();

        match self.cleanup {
            true => {
                println!("Cleaning up SHA1 sums");
                cleanup_sha1sums(&mut lines);
            }
            false => {
                println!("Updating SHA1 sums");
                let file_paths: Vec<PathBuf> = find_files(&paths);
                let blob_paths: Vec<&PathBuf> = file_paths
                    .iter()
                    .filter(|path| match path.extension() {
                        // Assume that valid blobs have ".so" extension.
                        Some(ext) => ext == "so",
                        None => false,
                    })
                    .collect();
                println!("Found {} blobs", blob_paths.len());
                for file in file_paths.iter() {
                    println!("{}", file.display().to_string());
                }
                for blob_path in &blob_paths {
                    println!("{}", blob_path.display().to_string());
                }

                update_sha1sums(&mut lines, blob_paths, vendor_path);
            }
        }
    }
}

/// The MissingBlobs builder.
pub struct UpdateSha1sumsBuilder {
    cleanup: bool,
}

impl Default for UpdateSha1sumsBuilder {
    fn default() -> Self {
        Self { cleanup: false }
    }
}

impl UpdateSha1sumsBuilder {
    /// Builds a UpdateSha1sums.
    pub fn build(&self) -> UpdateSha1sums {
        UpdateSha1sums {
            cleanup: self.cleanup,
        }
    }

    pub fn cleanup(mut self, cleanup: bool) -> Self {
        self.cleanup = cleanup;
        self
    }
}

fn find_files(paths: &[&str]) -> Vec<PathBuf> {
    let dirs = paths
        .iter()
        .map(Path::new)
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    print!("Searching for files in {} directories: ", dirs.len());
    for dir in dirs.iter() {
        println!("{}", dir.display().to_string());
    }

    let file_paths: Vec<PathBuf> = dirs
        .iter()
        .map(|dir| fs::read_dir(dir).expect("Could not read directory."))
        .flat_map(|read_dir| {
            read_dir.map(|dir_entry| dir_entry.expect("Could not read directory entry.").path())
        })
        .collect();

    file_paths
}

fn cleanup_sha1sums(lines: &mut Vec<&str>) {
    println!("Cleaning up");
    for (index, line) in lines.clone().iter().enumerate() {
        if line.len() == 0 || line.starts_with("#") || !line.contains("|") {
            continue;
        }

        // Remove the SHA1 hash.
        lines[index] = line.split("|").nth(0).unwrap();

        
        //lines[index] = format!("{}{}", line.split("|").next().unwrap(), "").as_str();
        println!("{}", lines[index]);
    }
}

fn update_sha1sums(lines: &mut Vec<&str>, blob_paths: Vec<&PathBuf>, vendor_path: &str) {
    for (index, line) in lines.clone().iter().enumerate() {
        // Skip empty lines
        if line.len() == 0 {
            continue;
        }

        // Check if we need to set SHA1 hash for the next files
        let mut cleanup = false;
        if line.starts_with("#") {
            cleanup = line.contains(" - ");
            continue;
        }

        if cleanup == true {
            // Remove existing SHA1 hash
            lines[index] = line.split("|").nth(0).unwrap();
            //lines[index] = format!("{}{}", line.split("|").next().unwrap(), "").as_str();

            // Remove - from start of the line
            if line.starts_with("-") {
                lines[index] = line.split("-").nth(1).unwrap();
                //lines[index] = lines[index].replace("-", "").as_str();
            }
        }
    }
}
