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

    // 除外するファイル名
        /// 除外するファイル名
    #[arg(long, value_name = "FILENAME", default_value = "", num_args=0..)]
    pub exclude: Vec<String>,
}