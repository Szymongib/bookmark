use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;

pub(crate) fn create_temp_file(suffix: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let time = SystemTime::now().elapsed()?.as_nanos();

    let mut temp_path = env::temp_dir();
    temp_path.push(format!("{}_{}", time, suffix));

    File::create(temp_path.clone())?;

    Ok(temp_path)
}
