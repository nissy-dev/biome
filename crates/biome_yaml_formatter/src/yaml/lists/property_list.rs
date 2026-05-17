use crate::prelude::*;
use biome_yaml_syntax::YamlPropertyList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPropertyList;
impl FormatRule<YamlPropertyList> for FormatYamlPropertyList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlPropertyList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_space();

        for property in node.iter() {
            join.entry(property.syntax(), &property.format());
        }

        join.finish()
    }
}
