use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlDocument, YamlDocumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocument;

impl FormatNodeRule<YamlDocument> for FormatYamlDocument {
    fn fmt_fields(&self, node: &YamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlDocumentFields {
            bom_token,
            directives,
            dashdashdash_token,
            node: document_node,
            dotdotdot_token,
        } = node.as_fields();

        let has_dash_marker = dashdashdash_token.is_some();
        let has_document_node = document_node.is_some();
        let has_end_marker = dotdotdot_token.is_some();

        write!(f, [bom_token.format(), directives.format(), dashdashdash_token.format()])?;

        if has_dash_marker && has_document_node {
            write!(f, [hard_line_break()])?;
        }

        write!(f, [document_node.format()])?;

        if has_end_marker {
            if has_document_node {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [dotdotdot_token.format()])?;
        }

        Ok(())
    }
}
