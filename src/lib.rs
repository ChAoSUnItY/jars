//! `jars` is a simple utility library allows users to extract jar files on file system based on given
//! extraction rules.
//! ## Usage
//! 
//! ```rs
//! import jars::{jar, JarOptionBuilder};
//! 
//! let jar = jars::jar("sample/rt.jar", JarOptionBuilder::default())?;
//! 
//! for (file_path, content) in jar.files {
//! // ...
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use zip::ZipArchive;

/// An option that indicates the extraction behaviour used in [jar].
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
            let extension = qualified_target_path.rsplit_once(".");
            
            if let Some((_, extension)) = extension {
                self.extension_targets.iter().any(|ext| {
                    extension.ends_with(ext)
                })
            } else {
                false
            }
        }
    }
}

/// A simple option builder for [JarOption] to build in a easy way.
#[derive(Debug)]
pub struct JarOptionBuilder {
    extract_targets: HashSet<String>,
    extension_targets: HashSet<String>,
}

impl JarOptionBuilder {
    /// Creates a [JarOption] which allows any file extraction by default.
    pub fn default() -> JarOption {
        JarOption::default()
    }

    /// Creates a [JarOptionBuilder] to build up extraction options.
    pub fn builder() -> Self {
        Self {
            extract_targets: HashSet::new(),
            extension_targets: HashSet::new(),
        }
    }

    /// Keeps `META-INF` folder on extraction.
    pub fn keep_meta_info(mut self) -> Self {
        self.extract_targets.insert("META-INF".to_string());
        self
    }

    /// Filters extraction target with providing target path. Note that [jar] extracts all files when
    /// there's no extraction target specified.
    /// 
    /// # Example
    /// 
    /// ```rs
    /// JarOptionBuilder::builder().target("java/lang").build();
    /// ```
    pub fn target(mut self, target: &str) -> Self {
        self.extract_targets.insert(target.to_string());
        self
    }

    /// Filters multiple extraction targets with providing target path. Note that [jar] extracts all 
    /// files when there's no extraction target specified.
    /// 
    /// # Example
    /// 
    /// ```rs
    /// JarOptionBuilder::builder().targets(vec!["java/lang"]).build();
    /// ```
    pub fn targets(mut self, targets: &Vec<&str>) -> Self {
        for target in targets {
            self.extract_targets.insert(target.to_string());
        }
        self
    }

    /// Filters extraction targets with providing file extension. Note that [jar] extracts all 
    /// files when there's no extraction target specified.
    ///
    /// # Example
    /// 
    /// ```rs
    /// JarOptionBuilder::builder().ext("java/lang").build();
    /// ```
    pub fn ext(mut self, ext: &str) -> Self {
        self.extension_targets.insert(ext.to_string());
        self
    }

    /// Filters multiple extraction targets with providing file extension. Note that [jar] extracts 
    /// all files when there's no extraction target specified.
    ///
    /// # Example
    ///
    /// ```rs
    /// JarOptionBuilder::builder().exts(vec!["java/lang"]).build();
    /// ```
    pub fn exts(mut self, exts: &Vec<&str>) -> Self {
        for ext in exts {
            self.extension_targets.insert(ext.to_string());
        }
        self
    }

    /// Finalize current [JarOptionBuilder] and construct a [JarOption] from current builder.
    pub fn build(self) -> JarOption {
        JarOption {
            extract_targets: self.extract_targets,
            extension_targets: self.extension_targets,
        }
    }
}

/// Simple [Jar] data representation stores files with a single [HashMap], key of files are full
/// qualified path while entry of files are read data in vector of u8.
pub struct Jar {
    pub files: HashMap<String, Vec<u8>>,
}

/// Extracts a jar file from given parameter `path`. The extraction behaviour is defined by parameter
/// `option` which can build from [JarOptionBuilder::default] with all defaulted options, or 
/// [JarOptionBuilder::builder] with multiple options provided.
/// 
/// # Example
/// 
/// ```rs
/// let jar = jar("sample/rt.jar", JarOptionBuilder::default())?;
/// ```
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
