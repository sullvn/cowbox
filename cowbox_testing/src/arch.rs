use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum Arch {
    X86,
    X86_64,
}

impl Arch {
    pub fn from_target() -> Self {
        if cfg!(target_arch = "x86") {
            Self::X86
        } else if cfg!(target_arch = "x86_64") {
            Self::X86_64
        } else {
            unimplemented!("Unsupported architecture")
        }
    }

    pub fn options() -> impl Iterator<Item = Self> {
        [Self::X86, Self::X86_64].into_iter()
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86 => f.write_str("x86"),
            Self::X86_64 => f.write_str("x86_64"),
        }
    }
}
