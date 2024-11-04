//! State of app

use std::path::PathBuf;

use crate::ui::Plates;

/// App state
#[derive(Default)]
pub struct State {
    /// Path to avatar
    pub avatar_path: Option<PathBuf>,
    /// Selected folder with friend's images
    pub current_friend_folder_path: Option<PathBuf>,
    /// Current right app's side
    pub current_plate: Plates,
}

/// App actions
#[derive(Debug, Clone)]
pub enum Message {
    // Pick sections
    /// Try to pick avatar img
    PickAvatar,
    // Try to pick avatars from folder
    PickFriends,

    /// Try to fetch latest assets from github
    ForceAssetsReload,
    // Selector sections
    /// Select Background
    SelectBackground,
    /// Select Badge
    SelectBadge,
    /// Select Frame
    SelectFrame,
    /// Select Banner
    SelectBanner,

    /// Switch window to Icons plate
    SwitchWindowPin,
    /// Switch window to Banner plate
    SwitchWindowBanner,
    /// Switch window to Background plate
    SwitchWindowBackground,
    /// Switch window to Frames plate
    SwitchWindowFrame,
    /// Switch window to About plate
    SwitchWindowAbout,
}
