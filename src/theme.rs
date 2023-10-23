use crate::core::style::Color;

pub struct Theme {
    pub bg: Color,

    pub tab_line_bg: Color,
    pub tab_fg: Color,
    pub tab_bg: Color,
    pub tab_selected_fg: Color,
    pub tab_selected_bg: Color,

    pub info_column_fg: Color,
    pub info_column_bg: Color,

    pub text_fg: Color,
    pub text_bg: Color,
    pub text_selected_fg: Color,
    pub text_selected_bg: Color,
    pub text_finded_fg: Color,
    pub text_finded_bg: Color,

    pub status_line_bg: Color,
    pub status_normal_mode_fg: Color,
    pub status_normal_mode_bg: Color,
    pub status_insert_mode_fg: Color,
    pub status_insert_mode_bg: Color,
    pub status_visual_mode_fg: Color,
    pub status_visual_mode_bg: Color,
    pub status_command_mode_fg: Color,
    pub status_command_mode_bg: Color,
    pub status_find_mode_fg: Color,
    pub status_find_mode_bg: Color,

    pub border_color_fg: Color,
    pub border_color_bg: Color,
}

pub const BLACK: Color = (0, 0, 0);
pub const SILVER: Color = (192, 192, 192);
pub const GRAY: Color = (128, 128, 128);
pub const LIGHT_GRAY: Color = (175, 175, 175);
pub const WHITE: Color = (255, 255, 255);
pub const RED: Color = (255, 0, 0);
pub const PURPLE: Color = (128, 0, 128);
pub const BLUE: Color = (100, 149, 237);
pub const GREEN: Color = (0, 100, 0);

pub const THEME_ONE: Theme = Theme {
    bg: SILVER,

    tab_line_bg: GRAY,
    tab_fg: WHITE,
    tab_bg: GRAY,
    tab_selected_fg: BLACK,
    tab_selected_bg: SILVER,

    info_column_fg: BLACK,
    info_column_bg: LIGHT_GRAY,

    text_fg: BLACK,
    text_bg: SILVER,
    text_selected_fg: WHITE,
    text_selected_bg: GRAY,
    text_finded_fg: RED,
    text_finded_bg: GRAY,

    status_line_bg: GRAY,
    status_normal_mode_fg: WHITE,
    status_normal_mode_bg: GREEN,
    status_insert_mode_fg: BLACK,
    status_insert_mode_bg: BLUE,
    status_visual_mode_fg: WHITE,
    status_visual_mode_bg: PURPLE,
    status_command_mode_fg: WHITE,
    status_command_mode_bg: GRAY,
    status_find_mode_fg: WHITE,
    status_find_mode_bg: GRAY,

    border_color_fg: BLACK,
    border_color_bg: SILVER,
};
