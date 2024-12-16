use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

pub fn collect_file_contents(
    dir_path: &Path,
    ext: &str,
    separator: &str,
    output_path: &PathBuf,
) -> Result<String, Box<dyn Error>> {
    let mut all_contents = String::new();

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let subdir_contents = collect_file_contents(&path, ext, separator, output_path)?;
            all_contents.push_str(&subdir_contents);
            continue;
        }
        if path.is_file() && path.extension().map_or(false, |e| e == ext) {
            let content = read_file_content(&path)?;
            let filename = path.file_name().and_then(|name| name.to_str()).unwrap_or("unknown");

            let separator_line = separator.replace("{filename}", filename);
            all_contents.push_str(&separator_line);
            all_contents.push_str("\n");
            if output_path == &PathBuf::from("output.md") || output_path.extension().map_or(false, |e| e == "md") {
                all_contents.push_str(&format!("``` {}\n", ext));
            }
            all_contents.push_str("\n");
             // ファイル内容
            all_contents.push_str(&content);
            all_contents.push_str("\n");
             // Markdownコードブロックの終了
            if output_path == &PathBuf::from("output.md") || output_path.extension().map_or(false, |e| e == "md") {
                all_contents.push_str("```\n");
            }
        }
    }

    Ok(all_contents)
}


fn read_file_content(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}