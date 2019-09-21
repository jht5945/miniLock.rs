
// Implemention miniLock: https://github.com/kaepora/miniLock/

// miniLock file format:
// miniLock magic bytes (8 bytes)
// Header length in bytes (4 bytes, little-endian)
// Header bytes
// Ciphertext bytes

// Magic: miniLock
pub const MINI_LOCK_MAGIC_BYTES: [u8; 8] = [0x6d, 0x69, 0x6e, 0x69, 0x4c, 0x6f, 0x63, 0x6b];

pub struct MiniLock {
    pub header_bytes: Vec<u8>,
    pub ciphertext_bytes: Vec<u8>,
}

impl MiniLock {
    // TODO ...
}

pub struct MiniLockHeaderDecryptInfoFileInfo {
    pub file_key: String,
    pub file_nonce: String,
    pub file_hash: String,
}

pub struct MiniLockHeaderDecryptInfo {
    pub sender_id: String,
    pub recipient_id: String,
    pub file_info: MiniLockHeaderDecryptInfoFileInfo,
}

pub struct MiniLockHeader {
    pub version: i32,
    pub ephemeral: String, // base64 key pair
    pub decrypt_info: MiniLockHeaderDecryptInfo,
}
