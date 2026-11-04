use std::hash::{DefaultHasher, Hash as _, Hasher as _};

fn device_uuid(name: &str) -> [u8; 16] {
    let mut uuid = [0; 16];
    for (salt, half) in uuid.chunks_exact_mut(8).enumerate() {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        salt.hash(&mut hasher);
        half.copy_from_slice(&hasher.finish().to_be_bytes());
    }
    uuid
}