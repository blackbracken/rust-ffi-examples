use randomizer::algorithms::mersenne_twister::MersenneTwister;

fn main() {
    let mut randomized = MersenneTwister::new(1234);

    println!("Hello, world! {}", randomized.next_u32());
}
