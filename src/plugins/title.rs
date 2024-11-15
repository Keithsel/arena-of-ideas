use super::*;

pub struct TitlePlugin;

impl TitlePlugin {
    pub fn add_tiles(world: &mut World) {
        Tile::new(Side::Left, |ui, world| {
            ui.vertical_centered_justified(|ui| {
                if Button::click("Play").ui(ui).clicked() {
                    GameState::GameStart.proceed_to_target(world);
                }
                if Button::click("Settings").ui(ui).clicked() {
                    Tile::new(Side::Left, |ui, world| {
                        title("Settings", ui);
                        ui.vertical_centered_justified(|ui| {
                            if Button::click("Video").ui(ui).clicked() {
                                SettingsPlugin::add_tile_video(world);
                            }
                            if Button::click("Audio").ui(ui).clicked() {
                                SettingsPlugin::add_tile_audio(world);
                            }
                            if Button::click("Profile").ui(ui).clicked() {
                                ProfilePlugin::add_tile_settings(world);
                            }
                        });
                    })
                    .min_space(egui::vec2(200.0, 0.0))
                    .with_id("Settings".into())
                    .push(world);
                }
                if QuestPlugin::new_available()
                    && Button::click("New Quests")
                        .cstr("New Quests".cstr_rainbow())
                        .color(CYAN, ui)
                        .ui(ui)
                        .clicked()
                {
                    GameState::Quests.proceed_to_target(world);
                }
                if MetaPlugin::can_balance_vote()
                    && Button::click("Vote to earn credits")
                        .color(YELLOW, ui)
                        .ui(ui)
                        .clicked()
                {
                    MetaPlugin::set_mode(MetaMode::Balancing, world);
                    GameState::Meta.proceed_to_target(world);
                }
            });
        })
        .min_space(egui::vec2(200.0, 0.0))
        .pinned()
        .push(world);
        Tile::new(Side::Bottom, |ui, _| {
            ui.horizontal_centered(|ui| {
                if Button::click("Discord")
                    .icon(Icon::Discord)
                    .ui(ui)
                    .clicked()
                {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab("https://discord.gg/c3UT58M9wb"));
                }
                if Button::click("Youtube")
                    .icon(Icon::Youtube)
                    .ui(ui)
                    .clicked()
                {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab("https://www.youtube.com/@makscee"));
                }
                if Button::click("Github").icon(Icon::Github).ui(ui).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::same_tab(
                        "https://github.com/makscee/arena-of-ideas/releases",
                    ));
                }
                if Button::click("Patreon")
                    .icon(Icon::Patreon)
                    .ui(ui)
                    .clicked()
                {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab("https://www.patreon.com/makscee"));
                }
            });
        })
        .pinned()
        .transparent()
        .no_expand()
        .push(world);
    }
}
