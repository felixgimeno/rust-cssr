extern crate nalgebra;
use nalgebra::{DVector};
	
fn main() {
    println!("Hello, world!");
    
}

fn get_n_gram_id(previous: u32, new_letter: u32, alphabet_size: u32) -> u32 {
	alphabet_size*previous + new_letter
	}

fn chi_square(a: &DVector<f32>, b: &DVector<f32>, alpha_thresh: f32) -> bool {
	let small : f32 = 0.0001;
	let c : DVector<f32> = a-b;
	let d : DVector<f32> = c.component_mul(&c);
	let e : DVector<f32> = d.component_div(&b.map(|x| if x > small {x} else {small}));
	let f : f32 = (e.iter()).sum();
	f <= alpha_thresh
}
