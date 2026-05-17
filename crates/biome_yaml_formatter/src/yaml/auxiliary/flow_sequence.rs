use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlFlowSequence, YamlFlowSequenceFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequence;

impl FormatNodeRule<YamlFlowSequence> for FormatYamlFlowSequence {
    fn fmt_fields(&self, node: &YamlFlowSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowSequenceFields {
            l_brack_token,
            entries,
            r_brack_token,
        } = node.as_fields();

        let should_expand = f.comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [
                l_brack_token.format(),
                group(&soft_block_indent(&format_args![
                    entries.format(),
                    format_dangling_comments(node.syntax()),
                ]))
                .should_expand(should_expand),
                r_brack_token.format(),
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowSequence,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
