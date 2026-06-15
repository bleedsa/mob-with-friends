use bleedthorn::building::Building;

fn main() {
    let b = Building::rand(0, std::ptr::null_mut());

    println!("{b:#?}");
}
