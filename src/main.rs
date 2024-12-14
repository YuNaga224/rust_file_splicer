use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::error::Error;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 探索対象のディレクトリパス
    #[arg(short, long, value_name = "PATH")]
    dir: PathBuf,

    /// 探索対象の拡張子
    #[arg(short, long, value_name = "EXTENSION")]
    ext: String,

    /// 出力ファイルパス (省略時は標準出力)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// ファイル区切り文字
    #[arg(long, value_name = "STRING", default_value = "==== FILE: {filename} ====")]
    separator: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let output_writer = create_output_writer(&args.output)?;

    let file_contents = collect_file_contents(&args.dir, &args.ext, &args.separator)?;

     if file_contents.is_empty() {
        println!("{}", "指定された条件に一致するファイルが見つかりませんでした。".yellow());
        return Ok(());
    }

    write_output(output_writer, file_contents)?;

    println!("{}", "ファイル結合が完了しました。".green());
    Ok(())
}

fn create_output_writer(output_path: &Option<PathBuf>) -> Result<Box<dyn Write>, Box<dyn Error>> {
    match output_path {
        Some(path) => {
            let file = File::create(path)?;
            Ok(Box::new(BufWriter::new(file)))
        }
        None => Ok(Box::new(io::stdout())),
    }
}

fn collect_file_contents(
    dir_path: &Path,
    ext: &str,
    separator: &str,
) -> Result<String, Box<dyn Error>> {
    let mut all_contents = String::new();

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let subdir_contents = collect_file_contents(&path, ext, separator)?;
            all_contents.push_str(&subdir_contents);
            continue;
        }
        if path.is_file() && path.extension().map_or(false, |e| e == ext) {
            let content = read_file_content(&path)?;
            let filename = path.file_name().and_then(|name| name.to_str()).unwrap_or("unknown");
            let separator_line = separator.replace("{filename}", filename);
            all_contents.push_str(&separator_line);
            all_contents.push_str("\n");
            all_contents.push_str(&content);
            all_contents.push_str("\n");
        }
    }

    Ok(all_contents)
}


fn read_file_content(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}


fn write_output(mut writer: Box<dyn Write>, content: String) -> Result<(), Box<dyn Error>> {
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}