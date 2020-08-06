use crate::aead_impl::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    AesGcm128 = 0x0001,
    AesGcm256 = 0x0002,
    ChaCha20Poly1305 = 0x0003,
}

impl From<u16> for Mode {
    fn from(x: u16) -> Mode {
        match x {
            0x0001 => Mode::AesGcm128,
            0x0002 => Mode::AesGcm256,
            0x0003 => Mode::ChaCha20Poly1305,
            _ => panic!("Unknown AEAD Mode {}", x),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    OpenError,
    InvalidConfig,
    InvalidNonce,
}

pub(crate) trait AeadTrait {
    fn new() -> Self
    where
        Self: Sized;
    fn seal(&self, key: &[u8], nonce: &[u8], aad: &[u8], plain_txt: &[u8]) -> Result<Vec<u8>, Error>;
    fn open(
        &self,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        cipher_txt: &[u8],
    ) -> Result<Vec<u8>, Error>;
    fn get_key_length(&self) -> usize;
    fn get_nonce_length(&self) -> usize;
}

pub struct Aead {
    aead: Box<dyn AeadTrait>,
}

fn get_aead_object(mode: Mode) -> Box<dyn AeadTrait> {
    match mode {
        Mode::AesGcm128 => Box::new(AesGcm128::new()),
        Mode::AesGcm256 => Box::new(AesGcm256::new()),
        Mode::ChaCha20Poly1305 => Box::new(ChaCha20Poly1305::new()),
    }
}

impl Aead {
    pub fn new(mode: Mode) -> Self {
        Self {
            aead: get_aead_object(mode),
        }
    }
    pub fn get_nk(&self) -> usize {
        self.aead.get_key_length()
    }
    pub fn get_nn(&self) -> usize {
        self.aead.get_nonce_length()
    }
    pub fn seal(&self, key: &[u8], nonce: &[u8], aad: &[u8], plain_txt: &[u8]) -> Result<Vec<u8>, Error> {
        self.aead.seal(key, nonce, aad, plain_txt)
    }
    pub fn open(
        &self,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        cipher_txt: &[u8],
    ) -> Result<Vec<u8>, Error> {
        self.aead.open(key, nonce, aad, cipher_txt)
    }
}
