//! State of app

use std::path::PathBuf;

use iced::widget::combo_box;

use crate::backend::dir::{BackgroundType, BadgeType, BannerType, FrameType};

/// App state
#[derive(Default)]
pub struct State {
    /// Path to avatar
    pub avatar_path: Option<PathBuf>,
    // lists state
    pub background: combo_box::State<BackgroundType>,
    pub badge: combo_box::State<BadgeType>,
    pub frame: combo_box::State<FrameType>,
    pub banner: combo_box::State<BannerType>,
    // list action
    pub select_background: Option<BackgroundType>,
    pub select_badge: Option<BadgeType>,
    pub select_frame: Option<FrameType>,
    pub select_banner: Option<BannerType>,
}

/// App actions
#[derive(Debug, Clone)]
pub enum Message {
    /// Try to pick avatar img
    PickAvatar,
    // Selectors
    /// Select Background
    SelectBackground,
    /// Select Badge
    SelectBadge,
    /// Select Frame
    SelectFrame,
    /// Select Banner
    SelectBanner,
}
