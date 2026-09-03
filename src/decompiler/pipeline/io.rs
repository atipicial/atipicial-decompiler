use std::fs;
use std::path::Path;

use crate::disassembler::DisassemblyOutput;
use crate::error::{AefError, Result};
use crate::manifest::ContractManifest;

use super::super::{Decompilation, OutputFormat, MAX_AEF_FILE_SIZE};
use super::Decompiler;

fn read_aef_file(path: &Path) -> Result<Vec<u8>> {
    let size = fs::metadata(path)?.len();
    if size > MAX_AEF_FILE_SIZE {
        return Err(AefError::FileTooLarge {
            size,
            max: MAX_AEF_FILE_SIZE,
        }
        .into());
    }
    Ok(fs::read(path)?)
}

impl Decompiler {
    pub(super) fn io_decompile_file<P: AsRef<Path>>(&self, path: P) -> Result<Decompilation> {
        let data = read_aef_file(path.as_ref())?;
        self.decompile_bytes(&data)
    }

    pub(super) fn io_disassemble_file<P: AsRef<Path>>(&self, path: P) -> Result<DisassemblyOutput> {
        let data = read_aef_file(path.as_ref())?;
        self.disassemble_bytes(&data)
    }

    pub(super) fn io_decompile_file_with_manifest<P, Q>(
        &self,
        aef_path: P,
        manifest_path: Option<Q>,
        output_format: OutputFormat,
    ) -> Result<Decompilation>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let aef_path = aef_path.as_ref();
        let data = read_aef_file(aef_path)?;
        let manifest = match manifest_path {
            Some(path) => Some(ContractManifest::from_file(path)?),
            None => None,
        };
        self.decompile_bytes_with_manifest(&data, manifest, output_format)
    }
}
