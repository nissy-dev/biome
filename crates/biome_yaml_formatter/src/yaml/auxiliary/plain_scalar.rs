use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlPlainScalar, YamlPlainScalarFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPlainScalar;

impl FormatNodeRule<YamlPlainScalar> for FormatYamlPlainScalar {
    fn fmt_fields(&self, node: &YamlPlainScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlPlainScalarFields { value_token } = node.as_fields();
        let value_token = value_token?;
        let value_text = value_token.text_trimmed().trim_end_matches([' ', '\t']);

        write!(
            f,
            [
                format_removed(&value_token),
                text(value_text, value_token.text_trimmed_range().start())
            ]
        )
    }
}
