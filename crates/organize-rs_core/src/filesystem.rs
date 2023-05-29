//! filesystem related

// #[cfg(unix)]
// use std::os::unix::fs::symlink;

use std::{
    fs::{remove_dir, remove_file, rename},
    path::Path,
};

fn already_exists<A>(dst: &A) -> std::io::Result<()>
where
    A: AsRef<Path>,
{
    if !dst.as_ref().exists() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("file already exists: {}", dst.as_ref().display()),
        ))
    }
}

pub(crate) fn move_to_trash<A>(src: A) -> std::io::Result<()>
where
    A: AsRef<Path>,
{
    let src_type = src.as_ref().metadata()?.file_type();

    if src_type.is_file() {
        if let Err(err) = trash::delete(src.as_ref()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("can not trash file: {}, {err}", src.as_ref().display()),
            ));
        };
    } else if src_type.is_dir() {
        if let Err(err) = trash::delete_all(src.as_ref()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("can not trash directory: {}, {err}", src.as_ref().display()),
            ));
        };
    } else if src_type.is_symlink() {
        if let Err(err) = trash::delete(src.as_ref()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("can not trash symlink: {}, {err}", src.as_ref().display()),
            ));
        };
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not trash: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

pub(crate) fn remove_irrecoverably<A>(src: A) -> std::io::Result<()>
where
    A: AsRef<Path>,
{
    let src_type = src.as_ref().metadata()?.file_type();

    if src_type.is_file() {
        remove_file(src)?;
    } else if src_type.is_dir() {
        // TODO: How to treat `remove_dir_all()`,
        // TODO e.g. if `dir` still has children
        remove_dir(src)?;
    } else if src_type.is_symlink() {
        if let Err(err) = trash::delete(src.as_ref()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("can not remove symlink: {}, {err}", src.as_ref().display()),
            ));
        };
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not remove: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

/// wrapper around rename_to for convenience and clarity
pub(crate) fn move_to<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    rename_to(src, dst)
}

pub(crate) fn rename_to<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    let src_type = src.as_ref().metadata()?.file_type();

    if src_type.is_file() {
        rename(src, dst)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not move/rename: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

pub(crate) fn symlink_to<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    let src_type = src.as_ref().metadata()?.file_type();

    if src_type.is_file() {
        symlink_file(src, dst)?;
    } else if src_type.is_dir() {
        symlink_dir(src, dst)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not symlink: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

pub(crate) fn copy_to<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    let src_type = src.as_ref().metadata()?.file_type();

    if src_type.is_file() {
        std::fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("can not deal with file_type: {}", src.as_ref().display()),
        ));
    }
    Ok(())
}

/// Copy the existing directory `src` to the target path `dst`
pub(crate) fn copy_dir_to<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    if !dst.as_ref().is_dir() {
        std::fs::create_dir(dst.as_ref())?;
    }

    for entry_result in src.as_ref().read_dir()? {
        let entry = entry_result?;
        copy_to(&entry.path(), &dst.as_ref().join(entry.file_name()))?;
    }

    Ok(())
}

/// Create a directory symlink to the given src with the given link name.
/// taken from: https://github.com/Byron/jwalk/blob/0079deb9faed6be48e77676494351f06411db5de/tests/util/mod.rs#L174
/// Copyright (c) 2019 Jesse Grosjean
pub fn symlink_dir<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    #[cfg(windows)]
    fn imp(src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::os::windows::fs::symlink_dir;
        symlink_dir(src, dst)
    }

    #[cfg(unix)]
    fn imp(src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::os::unix::fs::symlink;
        symlink(src, dst)
    }

    imp(src.as_ref(), dst.as_ref()).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "failed to symlink directory {} with target {}: {}",
                src.as_ref().display(),
                dst.as_ref().display(),
                e
            ),
        )
    })
}

/// Create a file symlink to the given src with the given link name.
/// taken from https://github.com/Byron/jwalk/blob/0079deb9faed6be48e77676494351f06411db5de/tests/util/mod.rs#L147
/// Copyright (c) 2019 Jesse Grosjean
pub fn symlink_file<A, D>(src: A, dst: D) -> std::io::Result<()>
where
    A: AsRef<Path>,
    D: AsRef<Path>,
{
    // TODO: Conflict handling here
    already_exists(&dst)?;

    #[cfg(windows)]
    fn imp(src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::os::windows::fs::symlink_file;
        symlink_file(src, dst)
    }

    #[cfg(unix)]
    fn imp(src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::os::unix::fs::symlink;
        symlink(src, dst)
    }

    imp(src.as_ref(), dst.as_ref()).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "failed to symlink file {} with target {}: {}",
                src.as_ref().display(),
                dst.as_ref().display(),
                e
            ),
        )
    })
}
