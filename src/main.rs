const A:u128 = 6364136223846793005;
const C:u128 = 1442695040888963407;
const M:u128 = u64::MAX as u128;

const REPETICOES:u16 = 1000;

fn main() {
    println!("Hello, world!");
    let mut seed:u128 = 7;

    for _ in 0..REPETICOES {
        let next = linear_congruencial_gen(seed);
        let normalized:f64 = (next as f64)/(M as f64);
        println!("{}", normalized);
        seed = next;
    }
}


// Given a number, calculates the next one in the sequence
// using it in a loop for M times gives all the numbers
// before they start to repeat
fn linear_congruencial_gen(seed: u128) -> u128 {
    (A * seed + C) % M  as u128
}