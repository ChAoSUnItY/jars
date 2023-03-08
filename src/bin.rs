use std::io::Error;
use jars::{jar, JarOptionBuilder};

fn main() -> Result<(), Error> {
    let jar = jar("./sample/rt.jar", JarOptionBuilder::builder().target("java/lang").build())?;
    
    Ok(())
}