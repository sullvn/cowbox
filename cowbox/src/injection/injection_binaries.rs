use std::io::Result;
use std::path::Path;

use super::injection_binary::InjectionBinary;

/// Multiple `InjectionBinary` definitions
/// for a single platform
pub struct InjectionBinaries {
    injection_binaries: &'static [InjectionBinary],
}

impl InjectionBinaries {
    ///
    /// Define all injection binaries for
    /// the platform
    ///
    /// NOTE: First binary will be the
    /// "preferred" one. See [`Self::preferred()`]
    /// for more details.
    ///
    pub const fn new(injection_binaries: &'static [InjectionBinary]) -> Self {
        Self { injection_binaries }
    }

    /// Update all defined binaries.
    ///
    /// TODO: Investigate sharing work
    /// between individual binary updates.
    ///
    pub fn update<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        for ib in self.injection_binaries {
            ib.update(&dir)?;
        }

        Ok(())
    }

    /// Preferred injection binary to provide
    /// the OS, if there are multiple.
    ///
    /// At the moment, this is used on Windows
    /// in order to properly handle injection
    /// into both x86 and x86_64 programs. The
    /// provided DLL has to the same
    /// architecture as the `cowbox` program.
    ///
    pub fn preferred(&self) -> &'static InjectionBinary {
        self.injection_binaries.get(0).expect("at least one injection binary should be defined")
    }
}
