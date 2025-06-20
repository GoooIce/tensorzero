use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Generates a random UUID version 4.
pub fn generate_uuidv4() -> String {
    Uuid::new_v4().to_string()
}

/// Calculates the SHA256 hash of a byte slice and returns it as a lowercase hex string.
pub fn _sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module (utils)

    #[test]
    fn test_generate_uuidv4_format() {
        let uuid = generate_uuidv4();
        // Basic format check (e.g., length, hyphens)
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
        // More robust check could use regex or the uuid crate's parse method
        assert!(Uuid::parse_str(&uuid).is_ok());
    }

    #[test]
    fn test_sha256_hex_known_value() {
        // Test against a known SHA256 hash
        // echo -n "hello world" | sha256sum
        // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        let input = b"hello world";
        let expected_output = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(_sha256_hex(input), expected_output);
    }

     #[test]
    fn test_sha256_hex_empty_string() {
        // echo -n "" | sha256sum
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let input = b"";
        let expected_output = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(_sha256_hex(input), expected_output);
    }
} 