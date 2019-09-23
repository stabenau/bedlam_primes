extern crate typenum;
use bit_array::BitArray;
use typenum::{Unsigned, U65536};
use rand::Rng;

fn primes_bv() -> BitArray::<u32, U65536> {
    let mut bv = BitArray::<u32, U65536>::from_elem(true);

    // Neither 0 nor 1 are prime
    bv.set(0, false);
    bv.set(1, false);

    for i in 2.. 1 + (U65536::to_usize() as f64).sqrt() as usize {
        // if i is a prime
        if bv[i] {
            // Mark all multiples of i as non-prime (any multiples below i * i
            // will have been marked as non-prime previously)
            for j in i.. {
                if i * j >= U65536::to_usize() {
                    break;
                }
                bv.set(i * j, false)
            }
        }
    }
    bv
}

fn is_prime( u: u32, bitmap: &BitArray::<u32, U65536> ) -> bool {
            return bitmap.iter()
                .enumerate()
                .filter( |&(_x,is_prime)| is_prime )
                .find( |(x,_is_prime)| u % ( *x as u32 ) == 0 )
                .is_none();
}

// print primes with 32 bits
fn print_primes( how_many:i32, bitmap: &BitArray::<u32, U65536> ) {
    // generate a random u32 with high bit set to one and low bit set to one
    // go through all the primes in bitmap and test divisibility
    // if all is well, add to hash
    let mut rng = rand::thread_rng();

    // an unbiased integer over the entire range:
    let mut counter = 0;
    while counter < how_many {
        let mut u: u32 = rng.gen();
        u |= 1;
        u |= 1 <<31;
        if is_prime( u, bitmap ) {
            println!( "{}", u );
            counter +=1;
        }
    }
}

fn main() {
    println!("Hello, world!");
    let primes = primes_bv();
    print_primes( 1000, &primes );
}
