use std::io::Write as _;
use std::path::Path;

use crate::error::Result;
use crate::aef::AefParser;
use crate::util;

use super::super::args::{Cli, TokensFormat};
use super::super::reports::{self, TokensReport};

impl Cli {
    pub(super) fn run_tokens(&self, path: &Path, format: TokensFormat) -> Result<()> {
        let data = Self::read_aef_bytes(path)?;
        let aef = AefParser::new().parse(&data)?;
        let script_hash = aef.script_hash();
        let script_hash_le = util::format_hash(&script_hash);
        let script_hash_be = util::format_hash_be(&script_hash);

        if aef.method_tokens.is_empty() {
            match format {
                TokensFormat::Text => {
                    self.write_stdout(|out| writeln!(out, "(no method tokens)"))?;
                }
                TokensFormat::Json => {
                    let report = TokensReport {
                        file: path.display().to_string(),
                        script_hash_le,
                        script_hash_be,
                        method_tokens: Vec::new(),
                        warnings: Vec::new(),
                    };
                    self.print_json(&report)?;
                }
            }
            return Ok(());
        }

        match format {
            TokensFormat::Text => {
                self.write_stdout(|out| {
                    for (index, token) in aef.method_tokens.iter().enumerate() {
                        writeln!(out, "{}", reports::format_method_token_line(index, token))?;
                    }
                    Ok(())
                })?;
            }
            TokensFormat::Json => {
                let tokens = aef
                    .method_tokens
                    .iter()
                    .map(reports::build_method_token_report)
                    .collect::<Vec<_>>();
                let report = TokensReport {
                    file: path.display().to_string(),
                    script_hash_le,
                    script_hash_be,
                    warnings: reports::collect_warnings(&tokens),
                    method_tokens: tokens,
                };
                self.print_json(&report)?;
            }
        }

        Ok(())
    }
}
