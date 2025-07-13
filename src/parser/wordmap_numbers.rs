use phf::{phf_map};

// 0,1,2,3,4,5,6,7,8,9,10,11 and so on
// Past 19, words can be constructed like "twenty one" and "six thousand five hundred and seventy seven"
// The parser should also take into account percentages and pronounce them like: "one hundred percent"
// Time could be a challenge as there are a few different formats. It could be military, or include seconds, and perhaps specify AM or PM.

/*
pub static NUMBERS: phf::Map<&'static str, &'static str> = phf_map! {

};
*/