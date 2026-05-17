use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowYamlNode, YamlFlowYamlNodeFields};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowYamlNode;

impl FormatNodeRule<YamlFlowYamlNode> for FormatYamlFlowYamlNode {
    fn fmt_fields(&self, node: &YamlFlowYamlNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowYamlNodeFields {
            properties,
            content,
        } = node.as_fields();

        let has_properties = properties.iter().next().is_some();

        write!(f, [properties.format()])?;

        if let Some(content) = content {
            if has_properties {
                if f.comments().has_leading_comments(content.syntax()) {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [space()])?;
                }
            }

            write!(f, [content.format()])?;
        }

        Ok(())
    }
}
