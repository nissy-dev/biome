use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockSequenceEntry, YamlBlockSequenceEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntry;
impl FormatNodeRule<YamlBlockSequenceEntry> for FormatYamlBlockSequenceEntry {
    fn fmt_fields(&self, node: &YamlBlockSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockSequenceEntryFields { minus_token, value } = node.as_fields();

        write!(f, [minus_token.format()])?;

        if let Some(value) = value {
            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}
