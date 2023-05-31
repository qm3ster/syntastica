use std::fs;

use anyhow::Result;

use crate::schema::Group;

pub fn write() -> Result<()> {
    let toml_path = crate::WORKSPACE_DIR.join("syntastica-parsers/Cargo.toml");
    let mut toml = fs::read_to_string(&toml_path)?;

    if let Some((preserve, _)) = toml.split_once(super::TOML_AUTOGEN_HEADER) {
        toml.truncate(preserve.len());
    }
    toml += super::TOML_AUTOGEN_HEADER;

    toml += super::TOML_FEATURES_HEAD;
    toml += &super::parsers_toml_feature(Group::Some, true);
    toml += super::TOML_FEATURES_MOST;
    toml += &super::parsers_toml_feature(Group::Most, true);
    toml += super::TOML_FEATURES_ALL;
    toml += &super::parsers_toml_feature(Group::All, true);

    toml += super::TOML_FEATURES_DOCS;

    super::parsers_toml_deps(&mut toml, false);

    fs::write(&toml_path, toml)?;

    Ok(())
}
