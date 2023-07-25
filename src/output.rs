use crate::input::HJoint;
use dxf::{
    entities::{Circle, DimensionBase, Entity, EntityType, Line, OrdinateDimension, Text},
    enums::{HorizontalTextJustification, VerticalTextJustification},
    tables::{DimStyle, Layer},
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let base_layer = Layer {
        name: input.layer_name().base(),
        color: Color::from_index(4),
        ..Default::default()
    };

    drawing.add_layer(base_layer);

    let bolt_layer = Layer {
        name: input.layer_name().bolt(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(bolt_layer);

    let plate_layer = Layer {
        name: input.layer_name().plate(),
        color: Color::from_index(1),
        ..Default::default()
    };

    drawing.add_layer(plate_layer);

    Ok(())
}

fn write_line(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let line = Line {
        p1: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        p2: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
        ..Default::default()
    };
    let mut line = Entity::new(dxf::entities::EntityType::Line(line));
    line.common.layer = layer.to_string();
    drawing.add_entity(line);
    Ok(())
}

fn write_circle(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let circle = Circle {
        center: Point { x, y, z: 0.0 },
        radius: r,
        ..Default::default()
    };

    let mut circle = Entity::new(dxf::entities::EntityType::Circle(circle));

    circle.common.layer = layer.to_string();

    drawing.add_entity(circle);

    Ok(())
}

fn write_dimension(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: String,
    is_vertical: bool,
) -> Result<(), Box<dyn Error>> {
    let dim_style = DimStyle {
        name: "mydim".to_string(),
        dimensioning_text_height: 1000.0,
        dimensioning_arrow_size: 500.0,
        dimension_extension_line_offset: 2000.0,
        ..Default::default()
    };

    drawing.add_dim_style(dim_style);

    let gap = 5000.0;

    let dimension_base = if is_vertical {
        DimensionBase {
            definition_point_1: Point {
                x: (x1 + x2) / 2.0 - gap,
                y: (y1 + y2) / 2.0,
                z: 0.0,
            },
            text_mid_point: Point {
                x: (x1 + x2) / 2.0 - gap,
                y: (y1 + y2) / 2.0,
                z: 0.0,
            },
            dimension_style_name: "mydim".to_string(),
            text_rotation_angle: 270.0,
            ..Default::default()
        }
    } else {
        DimensionBase {
            definition_point_1: Point {
                x: (x1 + x2) / 2.0,
                y: (y1 + y2) / 2.0 - gap,
                z: 0.0,
            },
            text_mid_point: Point {
                x: (x1 + x2) / 2.0,
                y: (y1 + y2) / 2.0 - gap,
                z: 0.0,
            },
            dimension_style_name: "mydim".to_string(),
            ..Default::default()
        }
    };

    let dimension = OrdinateDimension {
        dimension_base,
        definition_point_2: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        definition_point_3: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
    };

    let mut dimension = Entity::new(dxf::entities::EntityType::OrdinateDimension(dimension));

    dimension.common.layer = layer;

    drawing.add_entity(dimension);

    Ok(())
}

fn write_base(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let sec = input.section();
    let gap = 10.0;
    let margin = 1000.0;

    let coords = [
        ((-gap / 2.0, 0.0), (-gap / 2.0, sec.h())),
        ((gap / 2.0, 0.0), (gap / 2.0, sec.h())),
        ((-margin, 0.0), (-gap / 2.0, 0.0)),
        ((gap / 2.0, 0.0), (margin, 0.0)),
        ((-margin, sec.tf()), (-gap / 2.0, sec.tf())),
        ((gap / 2.0, sec.tf()), (margin, sec.tf())),
        (
            (-margin, sec.h() - sec.tf()),
            (-gap / 2.0, sec.h() - sec.tf()),
        ),
        (
            (gap / 2.0, sec.h() - sec.tf()),
            (margin, sec.h() - sec.tf()),
        ),
        ((-margin, sec.h()), (-gap / 2.0, sec.h())),
        ((gap / 2.0, sec.h()), (margin, sec.h())),
    ];

    for (p1, p2) in coords {
        write_line(drawing, p1.0, p1.1, p2.0, p2.1, &input.layer_name().base())?;
    }

    Ok(())
}

fn write_outer_plate(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let sec = input.section();

    let h = input.section().h();
    let t = input.flange().outer_plate().t();
    let l = input.flange().outer_plate().l();
    let layer = input.layer_name().plate();

    write_line(drawing, -l / 2.0, h + t, l / 2.0, h + t, &layer)?;
    write_line(drawing, -l / 2.0, h, l / 2.0, h, &layer)?;
    write_line(drawing, -l / 2.0, h, -l / 2.0, h + t, &layer)?;
    write_line(drawing, l / 2.0, h, l / 2.0, h + t, &layer)?;

    write_line(drawing, -l / 2.0, -t, l / 2.0, -t, &layer)?;
    write_line(drawing, -l / 2.0, 0.0, l / 2.0, 0.0, &layer)?;
    write_line(drawing, -l / 2.0, 0.0, -l / 2.0, -t, &layer)?;
    write_line(drawing, l / 2.0, 0.0, l / 2.0, -t, &layer)?;

    Ok(())
}

fn write_inner_plate(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let sec = input.section();

    let h = input.section().h();
    let tf = input.section().tf();
    let t = input.flange().inner_plate().t();
    let l = input.flange().outer_plate().l();
    let layer = input.layer_name().plate();

    write_line(drawing, -l / 2.0, h - tf - t, l / 2.0, h - tf - t, &layer)?;
    write_line(drawing, -l / 2.0, h - tf, l / 2.0, h - tf, &layer)?;
    write_line(drawing, -l / 2.0, h - tf, -l / 2.0, h - tf - t, &layer)?;
    write_line(drawing, l / 2.0, h - tf, l / 2.0, h - tf - t, &layer)?;

    write_line(drawing, -l / 2.0, tf + t, l / 2.0, tf + t, &layer)?;
    write_line(drawing, -l / 2.0, tf, l / 2.0, tf, &layer)?;
    write_line(drawing, -l / 2.0, tf, -l / 2.0, tf + t, &layer)?;
    write_line(drawing, l / 2.0, tf, l / 2.0, tf + t, &layer)?;

    Ok(())
}

fn write_web_plate(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let sec = input.section();

    let h = input.section().h();
    let l = input.web().plate().l();
    let b = input.web().plate().b();
    let layer = input.layer_name().plate();

    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 - b / 2.0,
        l / 2.0,
        h / 2.0 - b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 + b / 2.0,
        l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 - b / 2.0,
        -l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        l / 2.0,
        h / 2.0 - b / 2.0,
        l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;

    Ok(())
}

fn write_web_bolt(drawing: &mut Drawing, input: &HJoint) -> Result<(), Box<dyn Error>> {
    let gap = 10.0;
    let layer = input.layer_name().plate();
    let h = input.section().h();
    let mw = input.web().bolt().mw();
    let nw = input.web().bolt().nw();
    let pc = input.web().bolt().pc();
    let e = 40.0;
    let r = input.bolt().diameter() as f64 / 2.0 + 1.0;
    let g = 60.0;

    let c = pc * (mw - 1) as f64;
    let mut x = gap / 2.0 + e;
    for i in 0..nw {
        let mut y = h / 2.0 - c / 2.0;
        for j in 0..mw {
            write_circle(drawing, x, y, r, &layer);
            write_circle(drawing, -x, y, r, &layer);
            y += pc;
        }
        x += g;
    }
    Ok(())
}

pub fn write(input: HJoint, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_base(&mut drawing, &input)?;

    write_outer_plate(&mut drawing, &input)?;

    write_inner_plate(&mut drawing, &input)?;

    write_web_plate(&mut drawing, &input)?;

    write_web_bolt(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
