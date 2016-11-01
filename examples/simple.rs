extern crate tiled;

fn main() {
    let map = tiled::Map::new("Mine.json");
    println!("{:?}",&map);
    if let Ok(m) = map {
        let aTileLayers = m.get(0,0);
        println!("{:?}",&aTileLayers);
    }
}
