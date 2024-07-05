use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn salt(n: usize) -> String {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.sample(Alphanumeric) as char).collect()
}

#[cfg(test)]
mod tests {
    use super::salt;

    #[test]
    fn generate_salt() {
        let salt: String = salt(12);
        assert_eq!(salt.len(), 12, "Salt length is less than 12 characters");
    }

    #[test]
    fn two_salts_are_different() {
        let salt1: String = salt(12);
        let salt2: String = salt(12);
        assert_ne!(salt1, salt2, "Two salts are the same");
    }
}
