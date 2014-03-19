// Everything that varies very little between two or more architecures.

#[cfg(target_arch = "x86")]
#[cfg(target_arch = "x86_64")]
pub mod x86;
