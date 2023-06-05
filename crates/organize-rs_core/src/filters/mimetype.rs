#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;

use crate::filters::Filter;

#[cfg_attr(feature = "cli", derive(Args))]
#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct MimeTypeArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    mime: Vec<String>,
}

impl Filter for MimeTypeArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        // TODO Support also  top level media type e.g. `image`
        // TODO to identify all images, for now the unit test
        // TODO is on `should_fail` for that
        let Ok(Some(file_kind)) = infer::get_from_path(entry.path()) else { return false };

        let file_mime_type = match file_kind.mime_type().parse::<mime::Mime>() {
            Ok(it) => it,
            Err(err) => {
                eprintln!(
                    "couldn't determine mimetype of {}: {err}",
                    entry.path().display()
                );
                return false;
            }
        };

        self.mime
            .iter()
            .map(|f| f.parse::<mime::Mime>())
            .filter_map(|r| r.map_err(|err| println!("{err}")).ok())
            .any(|f| f == file_mime_type)
    }
}
