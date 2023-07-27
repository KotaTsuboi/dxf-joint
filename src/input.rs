use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize, CopyGetters, Debug)]
pub struct HJoint {
    #[getset(get_copy = "pub")]
    section: Section,
    #[getset(get_copy = "pub")]
    bolt: Bolt,
    #[getset(get_copy = "pub")]
    flange: Flange,
    #[getset(get_copy = "pub")]
    web: Web,
    layer_name: Option<LayerName>,
}

impl HJoint {
    pub fn layer_name(&self) -> LayerName {
        self.layer_name.clone().unwrap_or(LayerName {
            base: Some("S母材".to_string()),
            bolt: Some("Sボルト".to_string()),
            plate: Some("Sプレート".to_string()),
        })
    }
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct Section {
    #[getset(get_copy = "pub")]
    h: f64,
    #[getset(get_copy = "pub")]
    b: f64,
    #[getset(get_copy = "pub")]
    tw: f64,
    #[getset(get_copy = "pub")]
    tf: f64,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct Bolt {
    #[getset(get_copy = "pub")]
    diameter: u32,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct Flange {
    #[getset(get_copy = "pub")]
    bolt: FlangeBolt,
    #[getset(get_copy = "pub")]
    gauge: Gauge,
    #[getset(get_copy = "pub")]
    outer_plate: OuterPlate,
    #[getset(get_copy = "pub")]
    inner_plate: InnerPlate,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct FlangeBolt {
    #[getset(get_copy = "pub")]
    nf: u32,
    #[getset(get_copy = "pub")]
    mf: u32,
    #[getset(get_copy = "pub")]
    is_staggered: bool,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct Gauge {
    #[getset(get_copy = "pub")]
    g1: f64,
    g2: Option<f64>,
}

impl Gauge {
    pub fn g2(&self) -> f64 {
        self.g2.unwrap_or(40.0)
    }
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct OuterPlate {
    #[getset(get_copy = "pub")]
    t: f64,
    #[getset(get_copy = "pub")]
    l: f64,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct InnerPlate {
    #[getset(get_copy = "pub")]
    t: f64,
    #[getset(get_copy = "pub")]
    b: f64,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct Web {
    #[getset(get_copy = "pub")]
    bolt: WebBolt,
    #[getset(get_copy = "pub")]
    plate: WebPlate,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct WebBolt {
    #[getset(get_copy = "pub")]
    mw: u32,
    #[getset(get_copy = "pub")]
    nw: u32,
    #[getset(get_copy = "pub")]
    pc: f64,
}

#[derive(Deserialize, Clone, Copy, CopyGetters, Debug)]
pub struct WebPlate {
    #[getset(get_copy = "pub")]
    t: f64,
    #[getset(get_copy = "pub")]
    b: f64,
    #[getset(get_copy = "pub")]
    l: f64,
}

#[derive(Deserialize, Clone, Getters, CopyGetters, Debug)]
pub struct LayerName {
    base: Option<String>,
    bolt: Option<String>,
    plate: Option<String>,
}

impl LayerName {
    pub fn base(&self) -> String {
        self.base.clone().unwrap_or_else(|| "S母材".to_string())
    }

    pub fn bolt(&self) -> String {
        self.bolt.clone().unwrap_or_else(|| "Sボルト".to_string())
    }

    pub fn plate(&self) -> String {
        self.plate
            .clone()
            .unwrap_or_else(|| "Sプレート".to_string())
    }
}

fn read_file(path: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<HJoint, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<HJoint, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
