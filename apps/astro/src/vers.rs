use crate::{models::MediaWiki, PATCH_NOTES_URL, VER_REGEX};
use anyhow::Result;
use itertools::Itertools;
use modhost::GameVersion;
use parse_wiki_text_2::{Configuration, Node, TableNode};

pub async fn get_astro_versions() -> Result<Vec<GameVersion>> {
    let res = reqwest::get(PATCH_NOTES_URL).await?.text().await?;

    let txt = serde_xml_rs::from_str::<MediaWiki>(&res)?
        .page
        .revision
        .text;

    let parsed = Configuration::default().parse(&txt)?;

    let tables = parsed
        .nodes
        .iter()
        .filter_map(|v| match v.clone() {
            Node::Table {
                attributes,
                captions,
                end,
                rows,
                start,
            } => Some(TableNode {
                attributes,
                captions,
                end,
                rows,
                start,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    let cells = tables
        .iter()
        .flat_map(|v| v.rows.iter().flat_map(|v| v.cells.clone()))
        .collect::<Vec<_>>();

    let cell_texts = cells
        .iter()
        .flat_map(|v| {
            v.content.iter().filter_map(|v| match v {
                Node::Text {
                    end: _,
                    start: _,
                    value,
                } => Some(value),
                _ => None,
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    let nums = cell_texts
        .iter()
        .filter(|v| VER_REGEX.is_match(v))
        .cloned()
        .collect::<Vec<_>>();

    let semvers = nums
        .iter()
        .map(|v| lenient_semver::parse(v).unwrap())
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let fmt_semvers = semvers
        .iter()
        .map(|v| format!("{}.{}.{}.{}", v.major, v.minor, v.patch, v.build))
        .collect::<Vec<_>>();

    Ok(fmt_semvers
        .iter()
        .map(|v| GameVersion {
            id: v.clone(),
            beta: false,
        })
        .collect())
}
