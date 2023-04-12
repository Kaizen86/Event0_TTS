use phf::{phf_map};

//  error[E0425]: cannot find value `AO0` in this scope
//  error[E0425]: cannot find value `AW0` in this scope
//  error[E0425]: cannot find value `EY0` in this scope
//  error[E0425]: cannot find value `OY0` in this scope
//  error[E0425]: cannot find value `OY2` in this scope
//  error[E0425]: cannot find value `UH0` in this scope
//  error[E0425]: cannot find value `ZH` in this scope

pub static TECHNOBABBLE: phf::Map<&'static str, &'static str> = phf_map! {
  // DISTRO, UNIX, SSH, LS, and so on
};