use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    stdr_sjoint::run()?;
    Ok(())
}
