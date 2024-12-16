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

    let file_contents = collect_file_contents(&args.dir, &args.ext, &args.separator, &output_path, &args.exclude)?;

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