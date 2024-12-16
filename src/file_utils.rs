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