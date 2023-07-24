use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dxf_joint::run()?;
    Ok(())
}
