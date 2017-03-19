

fn main() {
    println!("Hello, world!");

}

fn get_n_gram_id(previous: u32, new_letter: u32, alphabet_size: u32) -> u32 {
    alphabet_size * previous + new_letter
}

fn chi_square(a: Vec<f32>, b: Vec<f32>, alpha_thresh: f32) -> bool {
    let small: f32 = 0.0001;
    let mut sum: f32 = 0f32;
    match a.len() == b.len() {
        true => {
            for i in 0usize..a.len() {
                let diff = a[i] - b[i];
                let sqr = diff * diff;
                let divi = sqr / if b[i] > small { b[i] } else { small };
                sum += divi;
            }
            sum <= alpha_thresh
        }
        _ => panic!(),
    }
}
