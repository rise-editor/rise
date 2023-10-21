pub type Color = (u8, u8, u8);

pub struct Theme {
    pub tab_line_bg: Color,
    pub tab_fg: Color,
    pub tab_bg: Color,
    pub tab_selected_fg: Color,
    pub tab_selected_bg: Color,

    pub info_column_fg: Color,
    pub info_column_bg: Color,

    pub bg: Color,
    pub text_fg: Color,
    pub text_bg: Color,
    pub text_selected_fg: Color,
    pub text_selected_bg: Color,

    pub status_line_bg: Color,
    pub status_normal_mode_fg: Color,
    pub status_normal_mode_bg: Color,
    pub status_insert_mode_fg: Color,
    pub status_insert_mode_bg: Color,
    pub status_visual_mode_fg: Color,
    pub status_visual_mode_bg: Color,
}

pub const BLACK: Color = (0, 0, 0);
pub const SILVER: Color = (192, 192, 192);
pub const GRAY: Color = (128, 128, 128);
pub const LIGHT_GRAY: Color = (175, 175, 175);
pub const WHITE: Color = (255, 255, 255);
pub const PURPLE: Color = (128, 0, 128);
pub const BLUE: Color = (100, 149, 237);
pub const GREEN: Color = (0, 100, 0);

pub const THEME_ONE: Theme = Theme {
    tab_line_bg: GRAY,
    tab_fg: WHITE,
    tab_bg: GRAY,
    tab_selected_fg: WHITE,
    tab_selected_bg: SILVER,

    info_column_fg: BLACK,
    info_column_bg: LIGHT_GRAY,

    bg: SILVER,
    text_fg: BLACK,
    text_bg: SILVER,
    text_selected_fg: WHITE,
    text_selected_bg: GRAY,

    status_line_bg: GRAY,
    status_normal_mode_fg: WHITE,
    status_normal_mode_bg: GREEN,
    status_insert_mode_fg: BLACK,
    status_insert_mode_bg: BLUE,
    status_visual_mode_fg: WHITE,
    status_visual_mode_bg: PURPLE,
};
