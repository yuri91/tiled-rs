extern crate tiled;

fn main() {
    let map = tiled::Map::new("Mine.json");
    println!("{:?}",&map);
    if let Ok(m) = map {
        let a_tile_layers = m.get(0,0);
        println!("{:?}",&a_tile_layers);
    }
}
