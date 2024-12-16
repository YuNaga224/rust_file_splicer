#### processing.rs
``` rs

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
```
#### main.rs
``` rs

use clap::Parser;
use std::error::Error;
use colored::Colorize;
use crate::args::Args;
use crate::file_utils::{determine_output_path, create_output_writer, };
use crate::processing::collect_file_contents;
use std::io::Write;

mod args;
mod file_utils;
mod processing;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let output_path = determine_output_path(&args.output)?;
    let mut output_writer = create_output_writer(&output_path)?;

    let file_contents = collect_file_contents(&args.dir, &args.ext, &args.separator, &output_path)?;

    if file_contents.is_empty() {
        println!("{}", "指定された条件に一致するファイルが見つかりませんでした。".yellow());
        return Ok(());
    }

    write_output(&mut output_writer, file_contents)?;

    println!("{}", format!("ファイル結合が完了しました。\n出力先: {}", output_path.display()).green());
    Ok(())
}


fn write_output(writer: &mut Box<dyn Write>, content: String) -> Result<(), Box<dyn Error>> {
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}
```
#### file_utils.rs
``` rs

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::ffi::OsStr;

pub fn determine_output_path(output_arg: &Option<PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    match output_arg {
        Some(path) => {
            if path.is_dir() {
                Ok(path.join("output.md"))
             // --outputでディレクトリが指定さていなかった場合(ファイル名のみの場合)、spliced_folderを作成する
            } else if path.parent().is_none()  || path.parent() == Some(Path::new("")){
                let spliced_dir = PathBuf::from("spliced_folder");
                fs::create_dir_all(&spliced_dir)?;
                Ok(spliced_dir.join(path))
            }
            else {
                Ok(path.clone())
            }
        }
        None => {
            let spliced_dir = PathBuf::from("spliced_folder");
            fs::create_dir_all(&spliced_dir)?;
            Ok(spliced_dir.join("output.md"))
        }
    }
}

pub fn create_output_writer(output_path: &PathBuf) -> Result<Box<dyn Write>, Box<dyn Error>> {
    if output_path == &PathBuf::from("output.md"){
        let file_path = create_numbered_file_path(output_path)?;
        let file = File::create(file_path)?;
        Ok(Box::new(BufWriter::new(file)))
    }else{
        let file_path = create_numbered_file_path(output_path)?;
        let file = File::create(file_path)?;
        Ok(Box::new(BufWriter::new(file)))
    }
}


pub fn create_numbered_file_path(file_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let mut new_path = file_path.to_path_buf();
    let original_filename = file_path.file_stem().and_then(OsStr::to_str).unwrap_or("output");
    let original_extension = file_path.extension().and_then(OsStr::to_str).unwrap_or("md");
    let parent_dir = file_path.parent().unwrap_or(Path::new("."));

    let mut version = 1;
    loop {
        let file_name = if version == 1 {
            format!("{}.{}", original_filename, original_extension)
        }else {
          format!("{}-v{}.{}", original_filename, version, original_extension)
        };
        new_path = parent_dir.join(file_name);
        if !new_path.exists() {
            break;
        }
        version += 1;
    }
    Ok(new_path)
}
```
#### args.rs
``` rs

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 探索対象のディレクトリパス
    #[arg(short, long, value_name = "PATH")]
    pub dir: PathBuf,

    /// 探索対象の拡張子
    #[arg(short, long, value_name = "EXTENSION")]
    pub ext: String,

    /// 出力ファイルパス (省略時は output.md)
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// ファイル区切り文字 (省略時は Markdown のコードブロック)
    #[arg(long, value_name = "STRING", default_value = "#### {filename}")]
    pub separator: String,
}
```
