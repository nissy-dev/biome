use crate::prelude::*;
use crate::yaml::auxiliary::flow_map_implicit_entry::format_flow_map_entry;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST;
use biome_yaml_syntax::{YamlFlowMapExplicitEntry, YamlFlowMapExplicitEntryFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapExplicitEntry;

impl FormatNodeRule<YamlFlowMapExplicitEntry> for FormatYamlFlowMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapExplicitEntryFields {
            question_mark_token,
            key,
            colon_token,
            value,
        } = node.as_fields();

        // Prettier omits explicit `?` in flow mappings but keeps it in flow sequences.
        if let Ok(question_mark_token) = question_mark_token {
            let is_in_flow_mapping = node
                .syntax()
                .parent()
                .is_some_and(|parent| parent.kind() == YAML_FLOW_MAP_ENTRY_LIST);

            if is_in_flow_mapping {
                format_removed(&question_mark_token).fmt(f)?;
            } else {
                question_mark_token.format().fmt(f)?;
                write!(f, [space()])?;
            }
        }

        format_flow_map_entry(f, key, colon_token, value)
    }
}
