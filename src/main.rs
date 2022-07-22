use clap::{crate_description, crate_name, crate_version, App, Arg};
use std::cell::RefCell;
use std::fs;
use update_sha1sums_rs::UpdateSha1sums;

const DEVICE: &str = "alioth";
const VENDOR: &str = "xiaomi";
const FILENAME: &str = "proprietary-files.txt";
thread_local! {
    static NEED_SHA1: RefCell<bool> = RefCell::new(false);
}

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
        "../../../vendor/", VENDOR, "/", DEVICE, "/proprietary"
    );
    let contents = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let paths = contents.lines().collect::<Vec<&str>>();
    let cleanup = matches.is_present("cleanup");

    UpdateSha1sums::builder()
        .cleanup(cleanup)
        .build()
        .run(contents.as_str(), &paths, vendor_path.as_str());
}
