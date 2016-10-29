extern crate tiled;

fn main() {
    let map = tiled::Map::new("Mine.json");
    println!("{:?}",&map);
}
