fn get_hash(url: &str) -> u32 {
    highhash::murmur::hash32(url.as_bytes())
}

fn get_hash_with_seed(url: &str, seed: u32) -> u32 {
    highhash::murmur::hash32_with_seed(url.as_bytes(), seed)
}

fn u32_to_62(hash: u32) -> String {
    let dict = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut n = hash;
    let mut chars: Vec<char> = vec![];
    while n > 0 {
        let i = (n % 64) as usize;
        let c = dict.chars().nth(i).unwrap();
        chars.push(c);
        n /= 62;
    }
    chars.reverse();
    chars.into_iter().collect::<String>()
}

pub fn short_url(url: &str) -> String {
    let hash = get_hash(url);
    u32_to_62(hash)
}

#[allow(unused)]
pub fn short_url_with_salt(url: &str, salt: u32) -> String {
    let hash = get_hash_with_seed(url, salt);
    u32_to_62(hash)
}
