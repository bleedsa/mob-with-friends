use bleedthorn::{building::BuildingAttrs};

fn main() {
    let b = BuildingAttrs::rand();

    println!("{b:#?}");
}
