mod parser;
mod counter;
mod tests;
mod latexparser;
mod to_markdown;
mod formatter;
use std::fs;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
fn process_md_file(input_path: &Path, output_dir: &Path) -> std::io::Result<()> {
    let mut file = fs::File::open(input_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("now check:{:?}",file);
    let new_content = to_markdown::parse_markdown(&content);

    let file_name = input_path.file_name().unwrap(); 
    let output_path = output_dir.join(file_name);

    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(new_content.as_bytes())?;

    Ok(())
}
fn main() -> std::io::Result<()> {
    // let input_folder = Path::new("old_posts");   
    // let output_folder = Path::new("_posts"); 
    let input_folder = Path::new("input_dir");   
    let output_folder = Path::new("output_dir"); 
    fs::create_dir_all(output_folder)?;
    for entry in WalkDir::new(input_folder).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            println!("Processing: {:?}", path);
            process_md_file(path, output_folder)?;
        }
    }

    Ok(())
}
