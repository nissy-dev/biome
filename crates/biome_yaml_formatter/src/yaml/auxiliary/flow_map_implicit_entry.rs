use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowMapImplicitEntry, YamlFlowMapImplicitEntryFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapImplicitEntry;

impl FormatNodeRule<YamlFlowMapImplicitEntry> for FormatYamlFlowMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        format_flow_map_entry(f, key, colon_token, value)
    }
}

pub(super) fn format_flow_map_entry(
    f: &mut YamlFormatter,
    key: Option<impl IntoFormat<YamlFormatContext>>,
    colon_token: Option<biome_yaml_syntax::YamlSyntaxToken>,
    value: Option<impl IntoFormat<YamlFormatContext>>,
) -> FormatResult<()> {
    if let Some(key) = key {
        key.into_format().fmt(f)?;
    }

    if let Some(colon_token) = colon_token {
        write!(f, [colon_token.format(), space(),])?;
        if let Some(value) = value {
            value.into_format().fmt(f)?;
        }
    }

    Ok(())
}
