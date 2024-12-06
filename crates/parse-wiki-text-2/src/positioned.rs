// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

macro_rules! impl_positioned {
    ($type:tt) => {
        impl crate::Positioned for crate::$type {
            fn end(&self) -> usize {
                self.end
            }

            fn start(&self) -> usize {
                self.start
            }
        }
    };
}

impl_positioned!(DefinitionListItem);
impl_positioned!(ListItem);
impl_positioned!(Parameter);
impl_positioned!(TableCaption);
impl_positioned!(TableCell);
impl_positioned!(TableRow);

impl crate::Positioned for crate::Node {
    fn end(&self) -> usize {
        match *self {
            crate::Node::Bold { end, .. } => end,
            crate::Node::BoldItalic { end, .. } => end,
            crate::Node::Category { end, .. } => end,
            crate::Node::CharacterEntity { end, .. } => end,
            crate::Node::Comment { end, .. } => end,
            crate::Node::DefinitionList { end, .. } => end,
            crate::Node::EndTag { end, .. } => end,
            crate::Node::ExternalLink { end, .. } => end,
            crate::Node::Heading { end, .. } => end,
            crate::Node::HorizontalDivider { end, .. } => end,
            crate::Node::Image { end, .. } => end,
            crate::Node::Italic { end, .. } => end,
            crate::Node::Link { end, .. } => end,
            crate::Node::MagicWord { end, .. } => end,
            crate::Node::OrderedList { end, .. } => end,
            crate::Node::ParagraphBreak { end, .. } => end,
            crate::Node::Parameter { end, .. } => end,
            crate::Node::Preformatted { end, .. } => end,
            crate::Node::Redirect { end, .. } => end,
            crate::Node::StartTag { end, .. } => end,
            crate::Node::Table { end, .. } => end,
            crate::Node::Tag { end, .. } => end,
            crate::Node::Template { end, .. } => end,
            crate::Node::Text { end, .. } => end,
            crate::Node::UnorderedList { end, .. } => end,
        }
    }

    fn start(&self) -> usize {
        match *self {
            crate::Node::Bold { start, .. } => start,
            crate::Node::BoldItalic { start, .. } => start,
            crate::Node::Category { start, .. } => start,
            crate::Node::CharacterEntity { start, .. } => start,
            crate::Node::Comment { start, .. } => start,
            crate::Node::DefinitionList { start, .. } => start,
            crate::Node::EndTag { start, .. } => start,
            crate::Node::ExternalLink { start, .. } => start,
            crate::Node::Heading { start, .. } => start,
            crate::Node::HorizontalDivider { start, .. } => start,
            crate::Node::Image { start, .. } => start,
            crate::Node::Italic { start, .. } => start,
            crate::Node::Link { start, .. } => start,
            crate::Node::MagicWord { start, .. } => start,
            crate::Node::OrderedList { start, .. } => start,
            crate::Node::ParagraphBreak { start, .. } => start,
            crate::Node::Parameter { start, .. } => start,
            crate::Node::Preformatted { start, .. } => start,
            crate::Node::Redirect { start, .. } => start,
            crate::Node::StartTag { start, .. } => start,
            crate::Node::Table { start, .. } => start,
            crate::Node::Tag { start, .. } => start,
            crate::Node::Template { start, .. } => start,
            crate::Node::Text { start, .. } => start,
            crate::Node::UnorderedList { start, .. } => start,
        }
    }
}
