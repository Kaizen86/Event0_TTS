// Enum of all possible phoneme tokens
#[allow(dead_code)] // Suppress unused warning due to the upcoming use statement
enum Phonemes {
  AA0,AA1,AA2,
  AE0,AE1,AE2,
  AH0,AH1,AH2,
  AO1,AO2,
  AW1,AW2,
  AY0,AY1,AY2,
  EH0,EH1,EH2,
  ER0,ER1,ER2,
  EY1,EY2,
  IH0,IH1,IH2,
  IY0,IY1,IY2,
  L,M,N,NG,
  OW0,OW1,OW2,
  OY1,
  R,
  UH1,UH2,
  UW0,UW1,UW2,
  V,W,Y,Z
}

// Just to avoid typing "Phonemes::" over and over in the upcoming vectors
use Phonemes::*;

// Given a word, return a vector with a list of phoneme tokens
pub fn get_phonemes(word:&str) -> Vec<Phonemes> {
  // This might be a slightly stupid approach, but my attempts at enums, maps, and hashmaps have proven to either be unworkable or require sentinel values.
  match word {
    "hello" => vec![UH1,M], // wait a second where are the "H" tokens??
    _ => vec![EH0,R,AW2] // Any unrecognised words should say "error".
  }
}