//! serializers

use crate::locations::MaxDepth;
use serde::Serialize;

impl Serialize for MaxDepth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}
