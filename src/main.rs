use rand::Rng; // 0.8.0


fn main() {
    // Generate random number in the range [0, 99]
    let mut rng= rand::thread_rng();
    let mut v = vec![0; 10_0000usize];
    for n in v.iter_mut() {
        *n = rng.gen();
    }
    
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
}
