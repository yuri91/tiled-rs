#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io;
use std::io::Read;
use std::fs;

#[derive(Deserialize, Debug)]
pub enum Orientation {
    #[serde(rename = "orthogonal")]
    Orthogonal,
    #[serde(rename = "isometric")]
    Isometric,
    #[serde(rename = "staggered")]
    Staggered,
    #[serde(rename = "hexagonal")]
    Hexagonal
}
#[derive(Deserialize, Debug)]
pub enum RenderOrder {
    #[serde(rename = "right-down")]
    RightDown,
    #[serde(rename = "right-up")]
    RightUp,
    #[serde(rename = "left-down")]
    LeftDown,
    #[serde(rename = "left-up")]
    LeftUp
}

#[derive(Deserialize, Debug)]
pub struct Layer {
    name: String,
    opacity: u32,
    visible: bool,
    data: Vec<u32>
}

#[derive(Deserialize, Debug)]
pub struct Set {
    firstgid: u32,
    name: String,
    tilewidth: u32,
    tileheight: u32,
    spacing: u32,
    margin: u32,
    tilecount: u32,
    columns: u32,
    pub image: String,
    imageheight: u32,
    imagewidth: u32
}

#[derive(Deserialize, Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tileheight: u32,
    pub tilewidth: u32,
    version: f64,
    orientation: Orientation,
    renderorder: RenderOrder,
    nextobjectid: u32,
    pub tilesets: Vec<Set>,
    layers: Vec<Layer>
}

#[derive(Debug,Clone)]
pub struct TileInfo {
    pub u: u32,
    pub v: u32,
    pub set: usize
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(serde_json::Error)
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Parse(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

impl Map {
    pub fn new(path: &str) ->Result<Map>  {
        let mut f = try!(fs::File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));
        let map: Map = try!(serde_json::from_str(&s));

        Ok(map)
    }
    pub fn get(self: &Map, i: u32, j: u32, l: u32) -> Option<TileInfo> {
        match self.layers.get(l as usize) {
            None => None,
            Some(layer) => {
                let k = i+j*self.width;
                let gid = layer.data[k as usize];
                let set = self.tilesets.iter()
                    .rev()
                    .enumerate()
                    .find(|&(_,s)| s.firstgid < gid);
                match set {
                    None => None,
                    Some((i,s)) => {
                        //println!("{} {}",gid,s.firstgid);
                        Some(TileInfo{
                        u:(gid-s.firstgid)%s.columns,
                        v:(s.imageheight/s.tileheight-1)-(gid-s.firstgid)/s.columns,
                        set:self.tilesets.len()-i-1
                    })}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;

    #[test]
    fn it_works() {
        let map: Map = Map::new("Mine.json").unwrap();
        assert!(map.version == 1.0);
    }
}
