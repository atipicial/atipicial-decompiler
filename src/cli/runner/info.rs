use std::path::Path;

use crate::error::Result;
use crate::aef::AefParser;

use super::super::args::{Cli, InfoFormat};

mod json;
mod text;

impl Cli {
    pub(super) fn run_info(&self, path: &Path, format: InfoFormat) -> Result<()> {
        let data = Self::read_aef_bytes(path)?;
        let aef = AefParser::new().parse(&data)?;
        let manifest = self.load_manifest(path)?;
        let manifest_path = self.resolve_manifest_path(path);

        match format {
            InfoFormat::Text => {
                self.print_info_text(path, &aef, manifest.as_ref(), manifest_path.as_ref())
            }
            InfoFormat::Json => {
                self.print_info_json(path, &aef, manifest.as_ref(), manifest_path.as_ref())
            }
        }
    }
}
