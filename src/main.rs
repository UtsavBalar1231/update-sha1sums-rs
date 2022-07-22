use clap::{crate_description, crate_name, crate_version, App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};
use update_sha1sums_rs::UpdateSha1sums;

static DEVICE: &str = "alioth";
static VENDOR: &str = "xiaomi";
static FILENAME: &str = "proprietary-files.txt";

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("cleanup")
                .short("c")
                .long("cleanup")
                .help("Remove SHA1 hash from all files"),
        )
        .get_matches();

    let vendor_path: String = format!(
        "{}{}{}{}{}",
        "../../../vendor/", VENDOR, "/", DEVICE, "/proprietary/"
    );
    let contents: String = BufReader::new(File::open(FILENAME).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let cleanup = matches.is_present("cleanup");

    UpdateSha1sums::builder()
        .cleanup(cleanup)
        .build()
        .run(contents, vendor_path.as_str());
}
