use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct HJoint {
    pub section: Section,
    pub bolt: Bolt,
    pub flange: Flange,
    pub web: Web,
    #[serde(default)]
    pub layer_name: LayerName,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Section {
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Bolt {
    pub diameter: u32,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Flange {
    pub bolt: FlangeBolt,
    pub gauge: Gauge,
    pub outer_plate: OuterPlate,
    pub inner_plate: InnerPlate,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct FlangeBolt {
    pub nf: u32,
    pub mf: u32,
    pub is_staggered: bool,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Gauge {
    pub g1: f64,
    pub g2: f64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct OuterPlate {
    pub t: f64,
    pub l: f64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct InnerPlate {
    pub t: f64,
    pub b: f64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Web {
    pub bolt: WebBolt,
    pub plate: WebPlate,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct WebBolt {
    pub mw: u32,
    pub nw: u32,
    pub pc: f64,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct WebPlate {
    pub t: f64,
    pub b: f64,
    pub l: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LayerName {
    pub base: String,
    pub bolt: String,
    pub plate: String,
}

impl Default for LayerName {
    fn default() -> Self {
        LayerName {
            base: "S母材".to_string(),
            bolt: "Sボルト".to_string(),
            plate: "Sプレート".to_string(),
        }
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
