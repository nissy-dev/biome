use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowInBlockNode, YamlFlowInBlockNodeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowInBlockNode;

impl FormatNodeRule<YamlFlowInBlockNode> for FormatYamlFlowInBlockNode {
    fn fmt_fields(&self, node: &YamlFlowInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowInBlockNodeFields {
            flow_start_token,
            flow,
            flow_end_token,
        } = node.as_fields();

        write!(
            f,
            [
                flow_start_token.format(),
                flow?.format(),
                flow_end_token.format(),
            ]
        )
    }
}
