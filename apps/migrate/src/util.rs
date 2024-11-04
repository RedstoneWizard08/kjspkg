use std::{
    future::Future,
    io::{self, Write},
    path::Path,
};

use async_trait::async_trait;
use tar::Builder;

#[async_trait]
pub trait AsyncFilterMap<T>
where
    Self: Send,
{
    async fn filter_map_async<'a, O, Fut, C, F>(self, cx: &'a C, func: F) -> Vec<O>
    where
        O: Send + Sync,
        T: Send + Sync,
        C: Send + Sync + 'a,
        Fut: Future<Output = Option<O>> + Send + 'a,
        F: Fn(&'a C, T) -> Fut + Send + 'a;
}

#[async_trait]
impl<T> AsyncFilterMap<T> for Vec<T>
where
    Self: Send,
{
    async fn filter_map_async<'a, O, Fut, C, F>(self, cx: &'a C, func: F) -> Vec<O>
    where
        O: Send + Sync,
        T: Send + Sync,
        C: Send + Sync + 'a,
        Fut: Future<Output = Option<O>> + Send + 'a,
        F: Fn(&'a C, T) -> Fut + Send + 'a,
    {
        let mut out = Vec::new();

        for item in self {
            if let Some(it) = func(cx, item).await {
                out.push(it);
            }
        }

        out
    }
}

pub trait MaybeAppendDirAll {
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

impl<W: Write> MaybeAppendDirAll for Builder<W> {
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
