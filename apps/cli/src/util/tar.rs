use std::{
    io::{self, Write},
    path::Path,
};

use tar::Builder;

pub trait MaybeAppend {
    fn maybe_append_dir_all(
        &mut self,
        path: impl AsRef<Path>,
        src_path: impl AsRef<Path>,
    ) -> io::Result<()>;

    fn maybe_append_named(
        &mut self,
        path: impl AsRef<Path>,
        name: impl AsRef<Path>,
    ) -> io::Result<()>;
}

impl<W: Write> MaybeAppend for Builder<W> {
    fn maybe_append_dir_all(
        &mut self,
        path: impl AsRef<Path>,
        src_path: impl AsRef<Path>,
    ) -> io::Result<()> {
        if src_path.as_ref().exists() {
            self.append_dir_all(path, src_path)?;
        }

        Ok(())
    }

    fn maybe_append_named(
        &mut self,
        path: impl AsRef<Path>,
        name: impl AsRef<Path>,
    ) -> io::Result<()> {
        if path.as_ref().exists() {
            self.append_path_with_name(path, name)?;
        }

        Ok(())
    }
}
