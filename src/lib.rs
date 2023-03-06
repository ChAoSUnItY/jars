use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use zip::ZipArchive;

pub struct JarOption {
    
}

pub struct Jar {
    class_files: HashMap<String, Vec<u8>>,
}

pub fn jar<P>(path: P) -> Result<Jar, Error> where P: AsRef<Path> {
    let mut class_files = HashMap::new();
    let mut jar_zip = File::open(path).map(ZipArchive::new)??;
    
    for i in 0..jar_zip.len() {
        let file = jar_zip.by_index(i)?;
        let file_path = match file.enclosed_name() {
            Some(file_path) => file_path,
            None => continue,
        };
        
        if file.is_dir() {
            continue;
        }
        
        println!("{:?}", file_path);
        
        class_files.insert(file_path.to_string_lossy().to_string(), file.bytes().collect::<Result<Vec<_>, _>>()?);
    }
    
    Ok(Jar{
        class_files
    })
}

#[cfg(test)]
mod tests {
    use crate::jar;

    #[test]
    fn test_rt_jar_folders() {
        let jar = jar("../sample/rt.jar");
    }
}
