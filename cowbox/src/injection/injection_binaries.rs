use super::injection_binary::InjectionBinary;
use std::io::Result;
use std::path::Path;

pub struct InjectionBinaries {
    injection_binaries: &'static [InjectionBinary],
}

impl InjectionBinaries {
    pub const fn new(injection_binaries: &'static [InjectionBinary]) -> Self {
        Self { injection_binaries }
    }

    pub fn update<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        for ib in self.injection_binaries {
            ib.update(&dir)?;
        }

        Ok(())
    }

    pub fn preferred(&self) -> &'static InjectionBinary {
        self.injection_binaries.get(0).expect("at least one injection binary should be defined")
    }
}