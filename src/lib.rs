use dashmap::DashMap;
use img_hash::HasherConfig;
use serde::ser::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use clap::OsValues;

pub fn write_to_json<T: ?Sized + Serialize>(obj: &T) -> Result<(), anyhow::Error> {
    let content = serde_json::to_string_pretty(obj)?;
    fs::write("hash_map.json", content)?;

    Ok(())
}

pub fn create_path_list(img_dir_paths: OsValues) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut path_list: Vec<PathBuf> = Vec::new();
    
    for img_dir_path in img_dir_paths {
        let mut dir_stack = vec![fs::canonicalize(img_dir_path).unwrap()];
        loop {
            let current_dir = match dir_stack.pop() {
                Some(dir) => dir,
                None => break,
            };
            for file in fs::read_dir(&current_dir)? {
                let file_path = file?.path();
                let meta = fs::metadata(&file_path)?;
                if meta.is_dir() {
                    dir_stack.push(file_path);
                } else {
                    path_list.push(file_path);
                }
            }
        }
    }

    Ok(path_list)
}

pub fn fill_hash_map(path_list: &[PathBuf], hash_map: Arc<DashMap<String, Vec<PathBuf>>>) {
    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>().to_hasher();

    for file_path in path_list {
        let img = match image::open(file_path) {
            Ok(img) => img,
            Err(_) => continue,
        };
        let img_hash = hasher.hash_image(&img).to_base64();
        hash_map
            .entry(img_hash)
            .or_insert_with(Vec::new)
            .push(file_path.to_path_buf());
    }
}
