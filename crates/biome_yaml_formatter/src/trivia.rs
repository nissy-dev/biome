use crate::FormatYamlSyntaxToken;
use crate::prelude::YamlFormatContext;
use crate::YamlFormatter;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Format, FormatResult};
use biome_yaml_syntax::YamlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a YamlSyntaxToken,
}

pub(crate) fn format_removed(token: &YamlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl Format<YamlFormatContext> for FormatRemoved<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        FormatYamlSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) fn on_skipped(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_removed(token, f)
}
