use rand::Rng;

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let id: String = (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..=chars.len());
            chars[idx] as char
        })
        .collect();
    id
}
