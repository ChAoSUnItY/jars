use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use zip::ZipArchive;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct JarOption {
    extract_targets: HashSet<String>,
}

impl JarOption {
    fn target_match(&self, qualified_target_path: &str) -> bool {
        self.extract_targets.iter().any(|target| qualified_target_path.starts_with(target))
    }
}

#[derive(Debug)]
pub struct JarOptionBuilder {
    extract_targets: HashSet<String>,
}

impl JarOptionBuilder {
    pub fn default() -> JarOption {
        JarOption::default()
    }
    
    pub fn builder() -> Self {
        Self {
            extract_targets: HashSet::new()
        }
    }
    
    pub fn keep_meta_info(mut self) -> Self {
        self.extract_targets.insert("META-INF".to_string());
        self
    }
    
    pub fn target(mut self, target: &str) -> Self {
        self.extract_targets.insert(target.to_string());
        self
    }
    
    pub fn targets(mut self, targets: &Vec<String>) -> Self {
        for target in targets {
            self.extract_targets.insert(target.clone());
        }
        self
    }
    
    pub fn build(self) -> JarOption {
        JarOption {
            extract_targets: self.extract_targets,
        }
    }
}

pub struct Jar {
    files: HashMap<String, Vec<u8>>,
}

pub fn jar<P>(path: P, option: JarOption) -> Result<Jar, Error> where P: AsRef<Path> {
    let mut files = HashMap::new();
    let mut jar_zip = File::open(path).map(ZipArchive::new)??;
    
    for i in 0..jar_zip.len() {
        let file = jar_zip.by_index(i)?;
        let file_path = match file.enclosed_name() {
            Some(file_path) => file_path.to_string_lossy().to_string(),
            None => continue,
        };
        
        if file.is_dir() {
            continue;
        }
        
        if !option.target_match(&file_path) {
            continue;
        }
        
        files.insert(file_path, file.bytes().collect::<Result<Vec<_>, _>>()?);
    }
    
    Ok(Jar{
        files
    })
}

#[cfg(test)]
mod tests {
    use crate::{jar, JarOption, JarOptionBuilder};

    #[test]
    fn test_rt_jar_folders() {
        let jar = jar("../sample/rt.jar", JarOptionBuilder::builder().target("java/lang").build());
    }
}
