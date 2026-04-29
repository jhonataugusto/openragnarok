mod baby_job;
mod item_info;
mod item_name;
mod item_resource;
mod job_identity;
mod map_sky_data;
mod skill_information;
mod skill_requirements;
mod skill_tree;

use std::hash::Hash;

use encoding_rs::EUC_KR;
use hashbrown::{HashMap, HashSet};
use korangar_loaders::FileLoader;
use mlua::Lua;

pub use self::baby_job::IsBabyJob;
pub use self::item_info::ItemInfo;
pub use self::item_name::{ItemName, ItemNameKey};
pub use self::item_resource::{ItemResource, ItemResourceKey};
pub use self::job_identity::JobIdentity;
pub use self::map_sky_data::MapSkyData;
pub use self::skill_tree::SkillTreeLayout;
use crate::loaders::GameFileLoader;
pub use crate::world::library::skill_information::SkillListInformation;
pub use crate::world::library::skill_requirements::{SkillListKey, SkillListRequirements};

pub struct Library {
    job_identity_table: <JobIdentity as Table>::Storage,
    item_info_table: <ItemInfo as Table>::Storage,
    map_sky_data_table: <MapSkyData as Table>::Storage,
    skill_information_table: <SkillListInformation as Table>::Storage,
    skill_requirements_table: <SkillListRequirements as Table>::Storage,
    skill_tree_table: <SkillTreeLayout as Table>::Storage,
    baby_job_table: <IsBabyJob as Table>::Storage,
    visual_animation_files: HashSet<String>,
    weapon_sprite_names: HashMap<u32, String>,
}

impl Library {
    pub fn new(game_file_loader: &GameFileLoader) -> mlua::Result<Self> {
        let job_identity_table = JobIdentity::load(game_file_loader)?;
        let item_info_table = ItemInfo::load(game_file_loader)?;
        let map_sky_data_table = MapSkyData::load(game_file_loader)?;
        let skill_information_table = SkillListInformation::load(game_file_loader)?;
        let skill_requirements_table = SkillListRequirements::load(game_file_loader)?;
        let skill_tree_table = SkillTreeLayout::load(game_file_loader)?;
        let baby_job_table = IsBabyJob::load(game_file_loader)?;
        let visual_animation_files = game_file_loader
            .get_files_with_extension(&[".spr", ".act"])
            .into_iter()
            .filter_map(normalize_visual_animation_file_path)
            .collect();
        let weapon_sprite_names = load_weapon_sprite_names(game_file_loader);

        Ok(Self {
            job_identity_table,
            item_info_table,
            map_sky_data_table,
            skill_information_table,
            skill_requirements_table,
            skill_tree_table,
            baby_job_table,
            visual_animation_files,
            weapon_sprite_names,
        })
    }

    #[inline(always)]
    pub fn get<T: Table>(&self, key: T::Key<'_>) -> &T {
        T::get(self, key)
    }

    pub fn has_visual_sprite_file(&self, file_path: &str) -> bool {
        self.visual_animation_files.contains(&format!("{file_path}.spr").to_lowercase())
            && self.visual_animation_files.contains(&format!("{file_path}.act").to_lowercase())
    }

    pub fn weapon_sprite_name(&self, weapon: u32) -> Option<&str> {
        self.weapon_sprite_names.get(&weapon).map(String::as_str)
    }

    #[cfg(test)]
    pub(crate) fn test_new(visual_animation_files: HashSet<String>, weapon_sprite_names: HashMap<u32, String>) -> Self {
        Self {
            job_identity_table: HashMap::new(),
            item_info_table: HashMap::new(),
            map_sky_data_table: HashMap::new(),
            skill_information_table: HashMap::new(),
            skill_requirements_table: HashMap::new(),
            skill_tree_table: HashMap::new(),
            baby_job_table: HashMap::new(),
            visual_animation_files,
            weapon_sprite_names,
        }
    }
}

fn load_weapon_sprite_names(game_file_loader: &GameFileLoader) -> HashMap<u32, String> {
    const WEAPON_TABLE_FILES: &[&str] = &[
        "data\\luafiles514\\lua files\\datainfo\\WeaponTable.lub",
        "data\\luafiles514\\lua files\\datainfo\\WeaponTable_F.lub",
    ];

    if WEAPON_TABLE_FILES.iter().any(|file| game_file_loader.get(file).is_err()) {
        return HashMap::new();
    }

    let Ok(state) = Lua::load_from_game_files(game_file_loader, WEAPON_TABLE_FILES) else {
        return HashMap::new();
    };

    let globals = state.globals();
    let Ok(weapon_name_table) = globals.get::<mlua::Table>("WeaponNameTable") else {
        return HashMap::new();
    };

    let mut result = HashMap::new();
    for (weapon_id, weapon_name) in weapon_name_table.pairs::<u32, String>().flatten() {
        result.insert(weapon_id, normalize_weapon_sprite_name(weapon_name));
    }

    if let Ok(expansion_weapon_ids) = globals.get::<mlua::Table>("Expansion_Weapon_IDs") {
        for (weapon_id, real_weapon_id) in expansion_weapon_ids.pairs::<u32, u32>().flatten() {
            if let Some(weapon_name) = result.get(&real_weapon_id).cloned() {
                result.insert(weapon_id, weapon_name);
            }
        }
    }

    result.compact()
}

fn normalize_weapon_sprite_name(weapon_name: String) -> String {
    fix_encoding(weapon_name).trim_start_matches('_').to_owned()
}

fn normalize_visual_animation_file_path(file_path: String) -> Option<String> {
    const SPRITE_PREFIX: &str = "data\\sprite\\";

    let file_path = file_path.to_lowercase();
    let file_path = file_path.strip_prefix(SPRITE_PREFIX).unwrap_or(&file_path);

    (file_path.starts_with("인간족\\") || file_path.starts_with("방패\\")).then(|| file_path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_animation_file_paths_are_stored_without_loader_prefix() {
        assert_eq!(
            normalize_visual_animation_file_path("data\\sprite\\인간족\\초보자\\초보자_남_단검.spr".to_owned()),
            Some("인간족\\초보자\\초보자_남_단검.spr".to_owned())
        );
    }

    #[test]
    fn visual_animation_file_paths_keep_legacy_relative_paths() {
        assert_eq!(
            normalize_visual_animation_file_path("방패\\검사\\검사_남_28901_방패.act".to_owned()),
            Some("방패\\검사\\검사_남_28901_방패.act".to_owned())
        );
    }

    #[test]
    fn visual_animation_file_paths_ignore_non_visual_sprite_files() {
        assert_eq!(
            normalize_visual_animation_file_path("data\\sprite\\npc\\missing.spr".to_owned()),
            None
        );
    }

    #[test]
    fn library_resolves_existing_visual_sprite_pairs() {
        let library = Library::test_new(
            HashSet::from_iter([
                "인간족\\초보자\\초보자_남_단검.spr".to_owned(),
                "인간족\\초보자\\초보자_남_단검.act".to_owned(),
            ]),
            HashMap::new(),
        );

        assert!(library.has_visual_sprite_file("인간족\\초보자\\초보자_남_단검"));
    }
}

/// Trait for compacting a hash map after it is completely populated.
trait HashMapExt {
    /// Compact the hash map, possibly by creating a second one.
    fn compact(self) -> Self;
}

impl<K, V> HashMapExt for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn compact(self) -> Self {
        HashMap::from_iter(self)
    }
}

trait LuaExt: Sized {
    fn load_from_game_files(game_file_loader: &GameFileLoader, files: &[&str]) -> mlua::Result<Self>;
}

impl LuaExt for Lua {
    fn load_from_game_files(game_file_loader: &GameFileLoader, files: &[&str]) -> mlua::Result<Self> {
        let state = Lua::new();

        for file in files {
            let data = game_file_loader
                .get(file)
                .unwrap_or_else(|_| panic!("failed to open lua file {}", file));

            state.load(&data).exec()?;
        }

        Ok(state)
    }
}

/// Trait for data that can be stored in a table and retrieved using a key.
pub trait Table {
    type Key<'a>;
    type Storage;

    fn load(game_file_loader: &GameFileLoader) -> mlua::Result<Self::Storage>;

    fn try_get<'a, 'b>(library: &'a Library, key: Self::Key<'b>) -> Option<&'a Self>
    where
        Self: Sized;

    fn get<'a, 'b>(library: &'a Library, key: Self::Key<'b>) -> &'a Self
    where
        Self: Sized;
}

fn fix_encoding(broken: String) -> String {
    let bytes: Vec<u8> = broken.chars().map(|char| char as u8).collect();
    match EUC_KR.decode_without_bom_handling_and_without_replacement(&bytes) {
        None => broken.to_string(),
        Some(char) => char.to_string(),
    }
}
