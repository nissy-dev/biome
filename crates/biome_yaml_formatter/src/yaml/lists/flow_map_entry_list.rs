use crate::prelude::*;
use biome_formatter::separated::TrailingSeparator;
use biome_yaml_syntax::YamlFlowMapEntryList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapEntryList;

impl FormatRule<YamlFlowMapEntryList> for FormatYamlFlowMapEntryList {
    type Context = YamlFormatContext;

    fn fmt(&self, node: &YamlFlowMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();

        for formatted in node.format_separated(",", TrailingSeparator::Omit) {
            join.entry(formatted.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
