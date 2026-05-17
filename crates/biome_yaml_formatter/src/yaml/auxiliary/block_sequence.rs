use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockSequence, YamlBlockSequenceFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequence;
impl FormatNodeRule<YamlBlockSequence> for FormatYamlBlockSequence {
    fn fmt_fields(&self, node: &YamlBlockSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockSequenceFields { entries, .. } = node.as_fields();
        write!(f, [entries.format()])
    }
}
