// Enum of all possible phoneme tokens
#[allow(dead_code)] // Suppress unused warning due to the upcoming use statement
#[derive(Debug)] // Allow debug prints
pub enum Phonemes {
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

// Given a phoneme token, return a random filename
pub fn get_filenames(phoneme:&Phonemes) -> Vec<&str> { // -> Option<&&str> {
  let names = match phoneme {
    AA0 => vec!["cannot", "cannot2"],
    AA1 => vec!["LibriVox", "are", "dot"],
    AA2 => vec!["argumentative", "paradox", "particularly", "volunteer"],
    AE0 => vec!["abstractions", "accept", "accepted", "spasmodic"],
    AE1 => vec!["Chapter", "Chapter2", "Traveller", "matter"],
    AE2 => vec!["Mathematical3", "mathematical", "mathematical2", "mathematicians"],
    AH0 => vec!["Machine", "The", "a", "recording"],
    AH1 => vec!["f", "f2", "public"],
    AH2 => vec!["Everyone", "humbu", "unaccountable", "understand"],
    AO1 => vec!["All", "For", "recordin", "recordings"],
    AO2 => vec!["laboratory", "therefore", "transitory", "uniform"],
    AW1 => vec!["Our2", "expounding", "founded", "our"],
    AW2 => vec!["anyhow", "however", "however2"],
    AY0 => vec!["Psychologist", "Psychologist2", "idea", "ideas"],
    AY1 => vec!["Time", "Time2", "by", "by2"],
    AY2 => vec!["Scientific", "recognized", "recondite", "scientific2"],
    EH0 => vec!["embraced", "existence", "existence2", "object"],
    EH1 => vec!["chairs", "incandescent", "recondite"],
    EH2 => vec!["investigations", "represent", "represent2", "representations"],
    ER0 => vec!["Chapter", "Traveller"],
    ER1 => vec!["burned", "earnestness", "luxurious", "universally"],
    ER2 => vec!["controvert", "framework"],
    EY1 => vec!["H", "domain", "rey", "information"],
    EY2 => vec!["Yesterday", "animated", "yesterday2"],
    IH0 => vec!["LibriVox", "LibriVox2", "recording"],
    IH1 => vec!["This", "is", "visit", "volunteer"],
    IH2 => vec!["atmosphere", "forefinger", "incandescent"],
    IY0 => vec!["brightly", "lilies", "radiance", "usually"],
    IY1 => vec!["G", "Machine", "Machine2", "please"],
    IY2 => vec!["reassured"],
    L   => vec!["LibriVox", "LibriVox2"],
    M   => vec!["Machine", "Time", "domain", "more"],
    N   => vec!["1", "Machine", "in"],
    NG  => vec!["expoundin", "recordin", "recording2", "recordings"],
    OW0 => vec!["Plat", "follow", "follow2"],
    OW1 => vec!["ver", "roams", "s"],
    OW2 => vec!["almost", "almost2", "anecdote", "elbows"],
    OY1 => vec!["adroitly", "pointed", "points", "voyage"],
    R   => vec!["LibriVox", "LibriVox2", "recording"],
    UH1 => vec!["Duration", "could", "looking", "put"],
    UH2 => vec!["verlook", "verlooked"],
    UW0 => vec!["continued"],
    UW1 => vec!["t", "to2", "usually"],
    UW2 => vec!["universally"],
    V   => vec!["LibriVox"],
    W   => vec!["1", "Wells", "was", "will"],
    Y   => vec!["You", "convenient", "universally", "usually"],
    Z   => vec!["is"]
  };
  names
  //names.choose(&mut rand::thread_rng())
}