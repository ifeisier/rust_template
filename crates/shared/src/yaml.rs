//! yaml 工具

use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;

/// 从 yaml 文件加载配置, 并转到对应的实例.
pub fn from_yaml_file<T: DeserializeOwned>(path: &str) -> Result<T> {
    let file = File::open(path)?;
    let options = serde_yaml::from_reader(BufReader::new(file))?;
    Ok(options)
}
