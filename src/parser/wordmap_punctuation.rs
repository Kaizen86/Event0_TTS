use phf::{phf_map};

pub static PUNCTUATION: phf::Map<&'static str, &'static str> = phf_map! {
  "." => "D;AA1;T",
  ".." => "D;AA1;T;D;AA1;T",
  "/" => "S;L;AE2;SH",
  "~" => "T;IH1;L;D;AH0"
};