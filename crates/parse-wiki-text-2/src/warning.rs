// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Warning from the parser telling that something is not well-formed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Warning {
    /// The byte position in the wiki text where the warning ends.
    pub end: usize,

    /// An identifier for the kind of warning.
    pub message: WarningMessage,

    /// The byte position in the wiki text where the warning starts.
    pub start: usize,
}

/// Identifier for a kind of warning from the parser.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WarningMessage {
    /// List broken by definition term.
    DefinitionTermContinuation,

    /// End tag in comment.
    EndTagInComment,

    /// Invalid character.
    InvalidCharacter,

    /// Invalid heading syntax. Rewinding.
    InvalidHeadingSyntaxRewinding,

    /// Invalid link syntax.
    InvalidLinkSyntax,

    /// Invalid parameter syntax.
    InvalidParameterSyntax,

    /// Invalid tag syntax.
    InvalidTagSyntax,

    /// Missing end tag. Rewinding.
    MissingEndTagRewinding,

    /// Repeated empty line.
    RepeatedEmptyLine,

    /// Stray text in table.
    StrayTextInTable,

    /// Wiki text comes after a redirect.
    TextAfterRedirect,

    /// The end tag does not match the last start tag. Rewinding.
    UnexpectedEndTagRewinding,

    /// An end tag was found with no preceeding start tag.
    UnexpectedEndTag,

    /// Expected heading of higher level. Correcting start of heading.
    UnexpectedHeadingLevelCorrecting,

    /// A tag with an unrecognized tag name was found.
    UnrecognizedTagName,

    /// Useless text in parameter.
    UselessTextInParameter,

    /// Useless text in redirect.
    UselessTextInRedirect,
}

impl WarningMessage {
    /// Human-readable description of the warning.
    pub fn message(self) -> &'static str {
        match self {
            WarningMessage::DefinitionTermContinuation => "List broken by definition term.",
            WarningMessage::EndTagInComment => "End tag in comment.",
            WarningMessage::InvalidCharacter => "Invalid character.",
            WarningMessage::InvalidHeadingSyntaxRewinding => "Invalid heading syntax. Rewinding.",
            WarningMessage::InvalidLinkSyntax => "Invalid link syntax.",
            WarningMessage::InvalidParameterSyntax => "Invalid parameter syntax.",
            WarningMessage::InvalidTagSyntax => "Invalid tag syntax.",
            WarningMessage::MissingEndTagRewinding => "Missing end tag. Rewinding.",
            WarningMessage::RepeatedEmptyLine => "Repeated empty line.",
            WarningMessage::StrayTextInTable => "Stray text in table.",
            WarningMessage::TextAfterRedirect => "Wiki text comes after a redirect.",
            WarningMessage::UnexpectedEndTagRewinding => {
                "The end tag does not match the last start tag. Rewinding."
            }
            WarningMessage::UnexpectedEndTag => {
                "An end tag was found with no preceeding start tag."
            }
            WarningMessage::UnexpectedHeadingLevelCorrecting => {
                "Expected heading of higher level. Correcting start of heading."
            }
            WarningMessage::UnrecognizedTagName => "A tag with an unrecognized tag name was found.",
            WarningMessage::UselessTextInParameter => "Useless text in parameter.",
            WarningMessage::UselessTextInRedirect => "Useless text in redirect.",
        }
    }
}

impl fmt::Display for WarningMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.message())
    }
}
