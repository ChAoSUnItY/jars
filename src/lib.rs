use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use zip::ZipArchive;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct JarOption {
    extract_targets: HashSet<String>,
    extension_targets: HashSet<String>,
}

impl JarOption {
    fn target_match(&self, qualified_target_path: &str) -> bool {
        if self.extract_targets.is_empty() {
            true
        } else {
            self.extract_targets.iter().any(|target| qualified_target_path.starts_with(target))
        }
    }

    fn ext_match(&self, qualified_target_path: &str) -> bool {
        if self.extension_targets.is_empty() {
            true
        } else {
            self.extension_targets.iter().any(|ext| qualified_target_path.ends_with(ext))
        }
    }
}

#[derive(Debug)]
pub struct JarOptionBuilder {
    extract_targets: HashSet<String>,
    extension_targets: HashSet<String>,
}

impl JarOptionBuilder {
    pub fn default() -> JarOption {
        JarOption::default()
    }

    pub fn builder() -> Self {
        Self {
            extract_targets: HashSet::new(),
            extension_targets: HashSet::new(),
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

    pub fn ext(mut self, ext: &str) -> Self {
        self.extension_targets.insert(ext.to_string());
        self
    }

    pub fn exts(mut self, exts: &Vec<String>) -> Self {
        for ext in exts {
            self.extension_targets.insert(ext.clone());
        }
        self
    }

    pub fn build(self) -> JarOption {
        JarOption {
            extract_targets: self.extract_targets,
            extension_targets: self.extension_targets,
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

        if !option.target_match(&file_path) && !option.ext_match(&file_path) {
            continue;
        }

        files.insert(file_path, file.bytes().collect::<Result<Vec<_>, _>>()?);
    }

    Ok(Jar {
        files
    })
}

/// Warning! Only tests when you have your own rt.jar, which can be copied from $JAVA_HOME/lib/rt.java
/// below java 8, for java 9 and later, do not test it since it's not possible to obtain rt.jar.
#[cfg(test)]
mod tests {
    use crate::{jar, JarOptionBuilder};

    #[test]
    fn test_rt_jar_folders() {
        let jar = jar("../sample/rt.jar", JarOptionBuilder::builder().target("java/lang").build());

        assert!(jar.is_ok());
    }
}
