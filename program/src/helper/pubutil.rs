use solana_program::pubkey::Pubkey;
use std::convert::TryInto;

pub trait Boolean {
  fn xor(&self, pk: &Pubkey) -> Self;
}

impl Boolean for Pubkey {
  fn xor(&self, pk: &Pubkey) -> Self {
    let a: [u8; 32] = [1; 32];
    let b: [u8; 32] = [2; 32];
    let c: [u8; 32] = a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect::<Vec<u8>>().try_into().unwrap();
    return Pubkey::new_from_array(c);
  }
}
