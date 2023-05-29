//! filesystem related

#[cfg(unix)]
use std::os::unix::fs::symlink;

use std::{fs::FileType, path::Path};

/// Copy the existing directory `src` to the target path `dst`
pub(crate) fn copy_dir_to<A>(src: A, dst: A) -> std::io::Result<()>
where
    A: AsRef<Path>,
{
    if !dst.as_ref().is_dir() {
        std::fs::create_dir(dst.as_ref())?;
    }

    for entry_result in src.as_ref().read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(
            &entry.path(),
            &file_type,
            &dst.as_ref().join(entry.file_name()),
        )?;
    }

    Ok(())
}

pub(crate) fn copy_to<A>(src: A, src_type: &FileType, dst: A) -> std::io::Result<()>
where
    A: AsRef<Path>,
{
    if src_type.is_file() {
        std::fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.as_ref().read_link()?;
        symlink(target, dst.as_ref())?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not deal with file_type: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

#[cfg(not(unix))]
pub(crate) fn symlink<P, Q>(src: P, _dst: Q) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("can't copy symbolic linke: {}", src.as_ref().display()),
    ))
}
