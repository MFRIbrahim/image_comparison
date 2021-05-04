# image_comparison

A multithreaded CLI tool for comparing a ton of images in multiple directories to detect duplicates or very similar images. The tool uses the dHash perceptual hash algorithm implemented in the [img_hash](https://github.com/abonander/img_hash) crate to compute image hashes and compare image similarity. The result is a "hash_map.json" file that has the following structure:  
- key: image hash
- value: list of absolute paths to the images that hash to the same image hash value


# Usage

To use this tool build it with cargo. Simply [install Rust](https://www.rust-lang.org/tools/install), clone the repository and run ```cargo build --release``` within the repository folder. To run the tool simply use 
```
./target/release/image_comparison <path>...
```  
, where multiple paths are separated with spaces. A description  of the tool can also be accessed in the command line through the usage of the help flag ```-h``` or ```--help```.
