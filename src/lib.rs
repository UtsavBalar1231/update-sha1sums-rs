use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

pub struct UpdateSha1sums {
    pub cleanup: bool,
}

impl UpdateSha1sums {
    pub fn builder() -> UpdateSha1sumsBuilder {
        UpdateSha1sumsBuilder::default()
    }

    pub fn run(&self, content: String, vendor_path: &str) {
        let mut lines: Vec<String> = content.lines().map(|x| x.to_string()).collect();

        match self.cleanup {
            true => {
                cleanup_sha1sums(&mut lines);
            }
            false => {
                update_sha1sums(&mut lines, vendor_path);
            }
        }
    }
}

pub struct UpdateSha1sumsBuilder {
    cleanup: bool,
}

impl Default for UpdateSha1sumsBuilder {
    fn default() -> Self {
        Self { cleanup: false }
    }
}

impl UpdateSha1sumsBuilder {
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

fn cleanup_sha1sums(lines: &mut Vec<String>) {
    for (index, line) in lines.clone().iter().enumerate() {
        // Skip empty lines, comments and lines with no SHA1 hash
        if line.len() == 0 || line.starts_with("#") || !line.contains("|") {
            continue;
        }

        lines[index] = line.split("|").nth(0).unwrap().to_string();
    }

    // Write the new file
    write_file(lines);
}

fn update_sha1sums(lines: &mut Vec<String>, vendor_path: &str) {
    let mut needsha1 = false;

    // TODO: This is a hack to get the path to the vendor directory.
    // TODO: Remove Clone usage when it's possible to use &str instead of String.
    for (index, line) in lines.clone().iter().enumerate() {
        // Skip empty lines
        if line.len() == 0 {
            continue;
        }

        // Check if we need to set SHA1 hash for the next files
        #[allow(unused_assignments)]
        if line.starts_with("#") {
            needsha1 = line.contains(" - ");
            continue;
        }

        if needsha1 {
            // Remove existing SHA1 hash
            lines[index] = line.split("|").nth(0).unwrap().to_string();

            let mut filepath = lines[index]
                .split(";")
                .nth(0)
                .unwrap()
                .split(":")
                .last()
                .unwrap();

            // Remove - from start of the line
            if filepath.starts_with("-") {
                filepath = &filepath[1..];
            }

            // TODO: Find an optimized implementation to do this

            // Open the file and get the SHA1 hash
            let mut file: File = std::fs::File::open(PathBuf::from(vendor_path).join(filepath)).expect("Failed to read file");

            let mut hasher = Sha1::new();
            io::copy(&mut file, &mut hasher).expect("Failed to read file");

            let sha1_hash: String = hasher
                .finalize()
                .into_iter()
                .map(|x| format!("{:02x}", x))
                .collect();

            // Add SHA1 hash to the file
            lines[index] = format!("{}|{}", lines[index], sha1_hash);
        }
    }

    // Write the new file
    write_file(lines);
}

fn write_file(lines: &[String]) {
    let mut file = BufWriter::new(File::create("proprietary-files.txt").unwrap());
    for line in lines {
        file.write_all(line.as_bytes())
            .expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
    }
}
