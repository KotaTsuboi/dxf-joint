use crate::input::HJoint;
use crate::output_x::write_x_view;
use crate::output_z::write_z_view;
use dxf::Drawing;
use std::error::Error;

pub fn write(input: &HJoint, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    write_x_view(&mut drawing, input)?;

    write_z_view(&mut drawing, input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
