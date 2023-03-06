use std::io::Error;
use jars::jar;

fn main() -> Result<(), Error> {
    let jar = jar("./sample/rt.jar")?;
    
    Ok(())
}