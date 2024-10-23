use std::path::PathBuf;

use crate::result::Result;
// unique paths
const AVATAR_PATH: &str = "avatar.png";
const FRIENDS: &str = "friends/";
const BACKGROUND: &str = "backgrounds/";
const BADGE: &str = "badges/";
const FRAME: &str = "frames/";
const BANNER: &str = "banners/";

// Общие пути
const ALL: &str = "all/";
const CREATORS: &str = "creators/";
const EVENTS: &str = "events/";
const HOLIDAYS: &str = "holidays/";
const OLDPOINTS: &str = "oldpoints/";
const OLDSEASONS: &str = "oldseasons/";
const SEASONS: &str = "seasons/";
const SHOP: &str = "shop/";
const SUPPORT: &str = "support/";

// unique path for Badge
const ACHIEVEMENTS: &str = "achievement/s";
const ANIME: &str = "anim/e";
const CODERS: &str = "coders/";
const CODES: &str = "codes/";
const COOPS: &str = "coop/";
const HALLOWEEN: &str = "halloween/";
const IWAS: &str = "iwas/";
const LIMITED: &str = "limited/";
const MESSAGES: &str = "messages/";
const MUSIC: &str = "music/";
const NEWYEAR: &str = "newyear/";
const OLDSIDES: &str = "oldsides/";
const PAWS: &str = "paws/";
const RP: &str = "rp/";
const TOURNAMENTS: &str = "tournaments/";

// unique path for Frame
const CLASSIC: &str = "classic/";
const MINECRAFT: &str = "minecraft/";
const NEON: &str = "neon/";
const PASTEL: &str = "pastel/";

// None type
const NONE: &str = "";

// Нам будут давать что искать, к примеру 1.png, мы просто лучим путь и выдадим

macro_rules! search_in_dir {
    ($base:expr, $subdir:expr) => {
        Ok(search(&format!("{}{}", $base, $subdir), FILE_TYPE))
    };
    ($dir:expr) => {
        Ok(search($dir, FILE_TYPE))
    };
}
/// Default extension of file to search
const FILE_TYPE: &str = "png";

impl Type {
    /// Return list of file's paths by directory
    pub fn get_folder(&self) -> Result<Vec<PathBuf>> {
        match self {
            Type::File(file_type) => match file_type {
                FileType::Avatar => Ok(vec![PathBuf::from(AVATAR_PATH)]),
            },
            Type::Folder(dir_type) => match dir_type {
                DirType::Background(background_type) => match background_type {
                    BackgroundType::All => search_in_dir!(BACKGROUND, ALL),
                    BackgroundType::Creators => search_in_dir!(BACKGROUND, CREATORS),
                    BackgroundType::Events => search_in_dir!(BACKGROUND, EVENTS),
                    BackgroundType::Holidays => search_in_dir!(BACKGROUND, HOLIDAYS),
                    BackgroundType::Oldpoints => search_in_dir!(BACKGROUND, OLDPOINTS),
                    BackgroundType::Oldseasons => search_in_dir!(BACKGROUND, OLDSEASONS),
                    BackgroundType::Shop => search_in_dir!(BACKGROUND, SHOP),
                    BackgroundType::Support => search_in_dir!(BACKGROUND, SUPPORT),
                },
                DirType::Badge(badge_type) => match badge_type {
                    BadgeType::Achievements => search_in_dir!(BADGE, ACHIEVEMENTS),
                    BadgeType::All => search_in_dir!(BADGE, ALL),
                    BadgeType::Anime => search_in_dir!(BADGE, ANIME),
                    BadgeType::Coders => search_in_dir!(BADGE, CODERS),
                    BadgeType::Codes => search_in_dir!(BADGE, CODES),
                    BadgeType::Coops => search_in_dir!(BADGE, COOPS),
                    BadgeType::Creators => search_in_dir!(BADGE, CREATORS),
                    BadgeType::Events => search_in_dir!(BADGE, EVENTS),
                    BadgeType::Halloween => search_in_dir!(BADGE, HALLOWEEN),
                    BadgeType::Holidays => search_in_dir!(BADGE, HOLIDAYS),
                    BadgeType::Iwas => search_in_dir!(BADGE, IWAS),
                    BadgeType::Limited => search_in_dir!(BADGE, LIMITED),
                    BadgeType::Messages => search_in_dir!(BADGE, MESSAGES),
                    BadgeType::Music => search_in_dir!(BADGE, MUSIC),
                    BadgeType::NewYear => search_in_dir!(BADGE, NEWYEAR),
                    BadgeType::None => Ok(vec![]),
                    BadgeType::OldPoints => search_in_dir!(BADGE, OLDPOINTS),
                    BadgeType::OldSeasons => search_in_dir!(BADGE, OLDSEASONS),
                    BadgeType::OldSides => search_in_dir!(BADGE, OLDSIDES),
                    BadgeType::Paws => search_in_dir!(BADGE, PAWS),
                    BadgeType::Rp => search_in_dir!(BADGE, RP),
                    BadgeType::Shop => search_in_dir!(BADGE, SHOP),
                    BadgeType::Support => search_in_dir!(BADGE, SUPPORT),
                    BadgeType::Tournaments => search_in_dir!(BADGE, TOURNAMENTS),
                },
                DirType::Banner(banner_type) => match banner_type {
                    BannerType::All => search_in_dir!(BANNER, ALL),
                    BannerType::Creators => search_in_dir!(BANNER, CREATORS),
                    BannerType::Events => search_in_dir!(BANNER, EVENTS),
                    BannerType::Holidays => search_in_dir!(BANNER, HOLIDAYS),
                    BannerType::Limited => search_in_dir!(BANNER, LIMITED),
                    BannerType::None => Ok(vec![]),
                    BannerType::Seasons => search_in_dir!(BANNER, SEASONS),
                    BannerType::Shop => search_in_dir!(BANNER, SHOP),
                },
                DirType::Frame(frame_type) => match frame_type {
                    FrameType::Classic => search_in_dir!(FRAME, CLASSIC),
                    FrameType::Minecraft => search_in_dir!(FRAME, MINECRAFT),
                    FrameType::Neon => search_in_dir!(FRAME, NEON),
                    FrameType::None => Ok(vec![]),
                    FrameType::Pastel => search_in_dir!(FRAME, PASTEL),
                },
                DirType::Friend => search_in_dir!(FRIENDS),
            },
        }
    }
    /// Return specific file if exist in folder
    pub fn get_file(&self, name: &str) -> Result<Option<PathBuf>> {
        self.get_folder().map(|paths| {
            paths.into_iter().find(|path| {
                path.file_name()
                    .and_then(|os_str| os_str.to_str())
                    .map_or(false, |file_name| file_name == name)
            })
        })
    }
}

/// Search files in path and filtered by filetype
fn search(path: &str, filetype: &str) -> Vec<PathBuf> {
    use std::fs;
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(filetype) {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod dirtype {
    use std::path::PathBuf;

    use crate::backend::dir::search;
    #[test]
    fn search_in_dev_dir() {
        let paths = search(".", "toml");
        let comparable = PathBuf::from("./Cargo.toml");
        assert_eq!(vec![comparable], paths);
    }
}

/// Subfolders of backgrounds folder
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BackgroundType {
    All,
    Creators,
    Events,
    Holidays,
    Oldpoints,
    Oldseasons,
    Shop,
    Support,
}
impl std::fmt::Display for BackgroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundType::All => f.write_str("All"),
            BackgroundType::Creators => f.write_str("Creators"),
            BackgroundType::Events => f.write_str("Events"),
            BackgroundType::Holidays => f.write_str("Holidays"),
            BackgroundType::Oldpoints => f.write_str("Oldpoints"),
            BackgroundType::Oldseasons => f.write_str("Oldseasons"),
            BackgroundType::Shop => f.write_str("Shop"),
            BackgroundType::Support => f.write_str("Support"),
        }
    }
}
/// Subfolders of badges folder
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BadgeType {
    Achievements,
    All,
    Anime,
    Coders,
    Codes,
    Coops,
    Creators,
    Events,
    Halloween,
    Holidays,
    Iwas,
    Limited,
    Messages,
    Music,
    NewYear,
    None,
    OldPoints,
    OldSeasons,
    OldSides,
    Paws,
    Rp,
    Shop,
    Support,
    Tournaments,
}
impl std::fmt::Display for BadgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeType::Achievements => f.write_str("Achievements"),
            BadgeType::All => f.write_str("All"),
            BadgeType::Anime => f.write_str("Anime"),
            BadgeType::Coders => f.write_str("Coders"),
            BadgeType::Codes => f.write_str("Codes"),
            BadgeType::Coops => f.write_str("Coops"),
            BadgeType::Creators => f.write_str("Creators"),
            BadgeType::Events => f.write_str("Events"),
            BadgeType::Halloween => f.write_str("Halloween"),
            BadgeType::Holidays => f.write_str("Holidays"),
            BadgeType::Iwas => f.write_str("Iwas"),
            BadgeType::Limited => f.write_str("Limited"),
            BadgeType::Messages => f.write_str("Messages"),
            BadgeType::Music => f.write_str("Music"),
            BadgeType::NewYear => f.write_str("NewYear"),
            BadgeType::None => f.write_str("None"),
            BadgeType::OldPoints => f.write_str("OldPoints"),
            BadgeType::OldSeasons => f.write_str("OldSeasons"),
            BadgeType::OldSides => f.write_str("OldSides"),
            BadgeType::Paws => f.write_str("Paws"),
            BadgeType::Rp => f.write_str("Rp"),
            BadgeType::Shop => f.write_str("Shop"),
            BadgeType::Support => f.write_str("Support"),
            BadgeType::Tournaments => f.write_str("Tournaments"),
        }
    }
}
/// Subfolders of frames folder
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrameType {
    Classic,
    Minecraft,
    Neon,
    None,
    Pastel,
}
impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::Classic => f.write_str("Classic"),
            FrameType::Minecraft => f.write_str("Minecraft"),
            FrameType::Neon => f.write_str("Neon"),
            FrameType::None => f.write_str("None"),
            FrameType::Pastel => f.write_str("Pastel"),
        }
    }
}
/// Subfolders of banners folder
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BannerType {
    All,
    Creators,
    Events,
    Holidays,
    Limited,
    None,
    Seasons,
    Shop,
}
impl std::fmt::Display for BannerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BannerType::All => f.write_str("All"),
            BannerType::Creators => f.write_str("Creators"),
            BannerType::Events => f.write_str("Events"),
            BannerType::Holidays => f.write_str("Holidays"),
            BannerType::Limited => f.write_str("Limited"),
            BannerType::None => f.write_str("None"),
            BannerType::Seasons => f.write_str("Seasons"),
            BannerType::Shop => f.write_str("Shop"),
        }
    }
}
/// Where we can find some data
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirType {
    Background(BackgroundType),
    Badge(BadgeType),
    Banner(BannerType),
    Frame(FrameType),
    Friend,
}
/// Single file in the main folder
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    Avatar,
}
impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Avatar => f.write_str("Avatar"),
        }
    }
}
/// Data absorbation
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    File(FileType),
    Folder(DirType),
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Type::File(_) => "File",
            Type::Folder(_) => "Folder",
        })
    }
}
