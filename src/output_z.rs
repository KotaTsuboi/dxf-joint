use crate::input::HJoint;
use crate::output_util::*;
use dxf::Drawing;
use std::error::Error;

static GAP_BETWEEN_VIEW: f64 = 1000.0;

fn write_base(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let b = input.section().b();
    let margin = 1000.0;
    let gap = 10.0;
    let layer = input.layer_name().base();

    write_line(
        drawing,
        -margin,
        GAP_BETWEEN_VIEW,
        -gap / 2.0,
        GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -margin,
        b + GAP_BETWEEN_VIEW,
        -gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        margin,
        GAP_BETWEEN_VIEW,
        gap / 2.0,
        GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        margin,
        b + GAP_BETWEEN_VIEW,
        gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;

    write_line(
        drawing,
        gap / 2.0,
        GAP_BETWEEN_VIEW,
        gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -gap / 2.0,
        GAP_BETWEEN_VIEW,
        -gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;

    Ok(())
}

fn write_outer_plate(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let b = input.section().b();
    let l = input.flange().outer_plate().l();
    let layer = input.layer_name().plate();

    Ok(())
}

fn write_flange_bolt(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let is_staggered = input.flange().bolt().is_staggered();
    let pc = if is_staggered { 45.0 } else { 60.0 };
    let e = 40.0;
    let gap = 10.0;
    let x0 = gap / 2.0 + e;
    let nf = input.flange().bolt().nf();
    let layer = input.layer_name().bolt();

    for i in 0..nf {
        let x = x0 + i as f64 * pc;
    }

    Ok(())
}

pub fn write_z_view(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    write_base(drawing, input)?;

    write_outer_plate(drawing, input)?;

    write_flange_bolt(drawing, input)?;

    Ok(())
}
