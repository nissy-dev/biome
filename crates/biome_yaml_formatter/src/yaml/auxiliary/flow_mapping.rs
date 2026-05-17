use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowMapping, YamlFlowMappingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapping;

impl FormatNodeRule<YamlFlowMapping> for FormatYamlFlowMapping {
    fn fmt_fields(&self, node: &YamlFlowMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowMappingFields {
            l_curly_token,
            entries,
            r_curly_token,
        } = node.as_fields();

        let should_expand = f.comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                space(),
                entries.format(),
                format_dangling_comments(node.syntax()),
                space(),
                r_curly_token.format(),
            ])
            .should_expand(should_expand),]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowMapping,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
