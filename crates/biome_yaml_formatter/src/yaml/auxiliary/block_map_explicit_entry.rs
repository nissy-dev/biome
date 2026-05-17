use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlBlockMapExplicitEntry, YamlBlockMapExplicitEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapExplicitEntry;
impl FormatNodeRule<YamlBlockMapExplicitEntry> for FormatYamlBlockMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapExplicitEntryFields {
            question_mark_token,
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [question_mark_token.format()])?;

        if let Some(key) = key {
            write!(f, [space(), key.format()])?;
        }

        if let Some(colon_token) = colon_token {
            write!(f, [colon_token.format()])?;
        }

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
