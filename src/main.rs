use clap::{App, Arg};
use dashmap::DashMap;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_NUMBER: usize = num_cpus::get() - 1;
}

fn main() {
    let matches = App::new("Image Comparison Tool")
        .author("MFRIbrahim")
        .about(
"The Image Comparison Tool crates a JSON file \"hash_map.json\" in the directory of the binary that
groups all images in the given image directories that are duplicates of or very similar to each
other. The image directories can have arbitrary many nested subdirectories with arbitrary files. The
Tool will find all images in the directory trees. The JSON file has the following key value pairs:
    key:    image hash
    value:  list of absolute paths to the images that hash to the same image hash value",
        )
        .arg(
            Arg::with_name("path")
                .help(
"The absolute or relative paths to the image directories to use. Multiple paths need to be separated
with a space."
                )
                .required(true)
                .multiple(true)
        )
        .get_matches();

    let img_dir_path = matches.values_of_os("path").unwrap();
    let path_list = image_comparison::create_path_list(img_dir_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing the directory files: {}", err);
        process::exit(1);
    });

    let shared_hash_map: Arc<DashMap<String, Vec<PathBuf>>> = Arc::new(DashMap::new());
    let mut pool = scoped_threadpool::Pool::new(*THREAD_NUMBER as u32);
    pool.scoped(|scope| {
        for chunk in (&path_list[..]).chunks(path_list.len() / *THREAD_NUMBER) {
            let shared_hash_map = Arc::clone(&shared_hash_map);
            scope.execute(move || image_comparison::fill_hash_map(chunk, shared_hash_map));
        } 
    });
    (shared_hash_map).retain(|_, v| v.len() > 1);

    image_comparison::write_to_json(&*shared_hash_map).unwrap_or_else(|err| {
        eprintln!("Problem saving results to a json file: {}", err);
        process::exit(1);
    });

    let len = shared_hash_map.len();
    println!("{} groups of duplicates or very similar images were found.", len);
}
