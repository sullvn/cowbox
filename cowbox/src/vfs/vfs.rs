use reflink::reflink_or_copy;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir, remove_dir_all, File, ReadDir};
use std::hash::{Hash, Hasher};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;

/// TODO: Mirror `fs`, `File`, and/or `OpenOption`
///       API
trait FileSystem {
    ///
    /// create file
    ///
    /// Per `std::fs::File::create`:
    ///
    /// > Opens a file in write-only mode.
    /// >
    /// > This function will create a file if
    /// > it does not exist, and will
    /// > truncate it if it does.
    /// >
    /// > Depending on the platform, this
    /// > function may fail if the full
    /// > directory path does not exist.
    ///
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<File>;

    fn read<P: AsRef<Path>>(&mut self, path: P) -> Result<File>;
    fn update<P: AsRef<Path>>(&mut self, path: P) -> Result<File>;
    fn delete<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<File>;
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<ReadDir>;
    fn delete_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
}

type PathBufRaw = PathBuf;
type PathBufOverlay = PathBuf;

enum FileOverlay {
    Directory,
    File,
    Removed,
    SymbolicLink(PathBufOverlay),
}

struct FS {
    overlay_map: HashMap<PathBuf, FileOverlay>,
    temp_dir: PathBuf,
}

fn hash_64<T: Hash>(x: &T) -> u64 {
    let mut h = DefaultHasher::new();
    x.hash(&mut h);
    h.finish()
}

impl FS {
    fn new() -> Result<Self> {
        let pid = process::id();
        let temp_dir = env::temp_dir().join(format!("cowbox-{pid:x}"));
        create_dir(temp_dir)?;

        Ok(Self {
            overlay_map: HashMap::new(),
            temp_dir,
        })
    }

    fn overlay_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        // WARNING - Hash is a hack instead of finding a
        // collision-free filename format / file organization
        let hash = hash_64(&path.as_ref());
        self.temp_dir.join(format!("{hash:x}"))
    }
}

impl Drop for FS {
    fn drop(&mut self) {
        // Explicitly avoid panic
        remove_dir_all(self.temp_dir).unwrap_or_default()
    }
}

impl FileSystem for FS {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        //! TODO: Check semantics
        let p = self.overlay_path(path);

        // TODO: Check directory exists
        let file = File::create(p)?;
        self.overlay_map
            .insert(path.as_ref().to_path_buf(), FileOverlay::File);

        Ok(file)
    }

    fn read<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        //! TODO: Check semantics
        let has_overlay = self.overlay_map.contains_key(&path.as_ref().to_path_buf());
        if has_overlay {
            File::open(self.overlay_path(path))
        } else {
            File::open(path)
        }
    }

    fn update<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        //! TODO: Check semantics
        let overlay_path = self.overlay_path(path);
        let has_overlay = self.overlay_map.contains_key(&path.as_ref().to_path_buf());

        if !has_overlay {
            reflink_or_copy(path, overlay_path)?;
        }

        self.overlay_map
            .insert(path.as_ref().to_path_buf(), FileOverlay::File);
        File::options().write(true).open(overlay_path)
    }

    fn delete<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        //! TODO: Check semantics
        // TODO: Check file exists, is not directory
        self.overlay_map
            .insert(path.as_ref().to_path_buf(), FileOverlay::Removed);
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<ReadDir>;
    fn delete_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
}
