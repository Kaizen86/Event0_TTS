mod wordmap_english;

// Given a phoneme token, return a random filename
pub fn get_filenames(phoneme:&str) -> Vec<&str> { // -> Option<&&str> {
  let names = match phoneme {
    "AA0" => vec!["cannot", "cannot2"],
    "AA1" => vec!["LibriVox", "are", "dot"],
    "AA2" => vec!["argumentative", "paradox", "particularly", "volunteer"],
    "AE0" => vec!["abstractions", "accept", "accepted", "spasmodic"],
    "AE1" => vec!["Chapter", "Chapter2", "Traveller", "matter"],
    "AE2" => vec!["Mathematical3", "mathematical", "mathematical2", "mathematicians"],
    "AH0" => vec!["Machine", "The", "a", "recording"],
    "AH1" => vec!["of", "of2", "public"],
    "AH2" => vec!["Everyone", "humbug", "unaccountable", "understand"],
    "AO1" => vec!["All", "For", "recording", "recordings"],
    "AO2" => vec!["laboratory", "therefore", "transitory", "uniform"],
    "AW1" => vec!["Our2", "expounding", "founded", "our"],
    "AW2" => vec!["anyhow", "however", "however2"],
    "AY0" => vec!["Psychologist", "Psychologist2", "idea", "ideas"],
    "AY1" => vec!["Time", "Time2", "by", "by2"],
    "AY2" => vec!["Scientific", "recognized", "recondite", "scientific2"],
    "B"   => vec!["LibriVox", "LibriVox2", "LibriVox3", "public"],
    "CH"  => vec!["Chapter", "H"],
    "D"   => vec!["domain"],
    "DH"  => vec!["This", "This2"],
    "EH0" => vec!["embraced", "existence", "existence2", "object"],
    "EH1" => vec!["chairs", "incandescent", "recondite"],
    "EH2" => vec!["investigations", "represent", "represent2", "representations"],
    "ER0" => vec!["Chapter", "Traveller"],
    "ER1" => vec!["burned", "earnestness", "luxurious", "universally"],
    "ER2" => vec!["controvert", "framework"],
    "EY1" => vec!["H", "domain", "grey", "information"],
    "EY2" => vec!["Yesterday", "animated", "yesterday2"],
    "F"   => vec!["information"],
    "G"   => vec!["glasses", "grey", "luxurious"],
    "HH"  => vec!["His2", "his"],
    "IH0" => vec!["LibriVox", "LibriVox2", "recording"],
    "IH1" => vec!["This", "is", "visit", "volunteer"],
    "IH2" => vec!["atmosphere", "forefinger", "incandescent"],
    "IY0" => vec!["brightly", "lilies", "radiance", "usually"],
    "IY1" => vec!["G", "Machine", "Machine2", "please"],
    "IY2" => vec!["reassured"],
    "JH"  => vec!["G", "Psychologist", "geometry"],
    "K"   => vec!["LibriVox", "recording"],
    "L"   => vec!["LibriVox", "LibriVox2"],
    "M"   => vec!["Machine", "Time", "domain", "more"],
    "N"   => vec!["1", "Machine", "in"],
    "NG"  => vec!["expounding", "recording", "recording2", "recordings"],
    "OW0" => vec!["Plato", "follow", "follow2"],
    "OW1" => vec!["over", "roams", "so"],
    "OW2" => vec!["almost", "almost2", "anecdote", "elbows"],
    "OY1" => vec!["adroitly", "pointed", "points", "voyage"],
    "P"   => vec!["Chapter", "please", "public"],
    "R"   => vec!["LibriVox", "LibriVox2", "recording"],
    "S"   => vec!["LibriVox", "This"],
    "SH"  => vec!["Machine", "Machine2", "information"],
    "T"   => vec!["Time", "to"],
    "TH"  => vec!["Smith", "thing", "thought", "thought2"],
    "UH1" => vec!["Duration", "could", "looking", "put"],
    "UH2" => vec!["overlook", "overlooked"],
    "UW0" => vec!["continued"],
    "UW1" => vec!["to", "to2", "usually"],
    "UW2" => vec!["universally"],
    "V"   => vec!["LibriVox"],
    "W"   => vec!["1", "Wells", "was", "will"],
    "Y"   => vec!["You", "convenient", "universally", "usually"],
    "Z"   => vec!["is", "please", "recordings"],
    "ZH"  => vec!["luxurious", "precision", "usually"],
    &_  => panic!("Wordlist contains unknown phoneme token {:?}", phoneme)
  };
  names
  //names.choose(&mut rand::thread_rng())
}

pub fn get_phonemes(word: &str) -> Vec<&str> {
  match wordmap_english::WORDMAP_ENGLISH.get(word).cloned() {
    Some(result) => result.split(";").collect::<Vec<&str>>(),
    None => vec![]
  }
}