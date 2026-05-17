use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlBlockMapImplicitEntry, YamlBlockMapImplicitEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        if let Some(key) = key {
            write!(f, [key.format()])?;
        }

        write!(f, [colon_token.format()])?;

        if let Some(value) = value {
            if value.syntax().has_leading_newline() || f.comments().has_leading_comments(value.syntax()) {
                write!(f, [hard_line_break()])?;
            } else {
                write!(f, [space()])?;
            }

            write!(f, [value.format()])?;
        }

        Ok(())
    }
}
