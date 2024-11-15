use spacetimedb_sdk::{table::TableWithPrimaryKey, Table};

use super::*;

pub const HOME_DIR: &str = ".aoi";
pub fn home_dir_path() -> PathBuf {
    let mut path = home::home_dir().unwrap();
    path.push(HOME_DIR);
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Login), Self::login)
            .init_resource::<LoginData>();
    }
}

#[derive(Resource, Default)]
pub struct LoginData {
    name_field: String,
    pass_field: String,
    pub identity_player: Option<TPlayer>,
}

impl LoginPlugin {
    fn login(world: &mut World) {
        let co = ConnectOption::get(world);
        let mut identity_user = None;
        if let Some(player) = cn()
            .db
            .player()
            .iter()
            .find(|u| u.identities.contains(&co.identity))
        {
            if currently_fulfilling() == GameOption::ForceLogin {
                Self::complete(Some(player.clone()), world);
            }
            identity_user = Some(player);
        }
        world.insert_resource(LoginData {
            name_field: default(),
            pass_field: default(),
            identity_player: identity_user,
        });
    }
    pub fn complete(player: Option<TPlayer>, world: &mut World) {
        let player = player.unwrap_or_else(|| {
            world
                .resource::<LoginData>()
                .identity_player
                .clone()
                .unwrap()
        });
        LoginOption { player }.save(world);
        StdbQuery::subscribe(StdbQuery::queries_game(), move |world| {
            GameAssets::cache_tables();
            GameState::proceed(world);
            cn().db.trade().on_insert(|e, r| {
                let id = r.id;
                match &e.event {
                    spacetimedb_sdk::Event::Reducer(e) => {
                        if matches!(e.reducer, Reducer::OpenLootbox(..)) {
                            OperationsPlugin::add(move |world| {
                                Trade::open(id, &egui_context(world).unwrap());
                            });
                        }
                    }
                    _ => {}
                }
            });
            cn().db.wallet().on_update(|_, before, after| {
                let delta = after.amount - before.amount;
                let delta_txt = if delta > 0 {
                    format!("+{delta}")
                } else {
                    delta.to_string()
                };
                AudioPlugin::queue_sound(SoundEffect::Coin);
                Notification::new(
                    "Credits "
                        .cstr_c(YELLOW)
                        .push(delta_txt.cstr_c(VISIBLE_LIGHT))
                        .take(),
                )
                .push_op();
            });
            cn().db.quest().on_insert(|_, d| {
                let text = "New Quest\n".cstr().push(d.cstr()).take();
                Notification::new(text).push_op();
            });
            cn().db.quest().on_update(|_, before, after| {
                let before = before.clone();
                let after = after.clone();
                OperationsPlugin::add(move |world| {
                    if before.complete && after.complete {
                        return;
                    }
                    if before.counter < after.counter {
                        ShopPlugin::maybe_queue_notification(
                            "Quest Progress:\n"
                                .cstr_c(VISIBLE_BRIGHT)
                                .push(after.cstr())
                                .take(),
                            world,
                        )
                    }
                    if !before.complete && after.complete {
                        ShopPlugin::maybe_queue_notification(
                            "Quest Complete!\n"
                                .cstr_c(VISIBLE_BRIGHT)
                                .push(after.cstr())
                                .take(),
                            world,
                        )
                    }
                });
            });
        });
    }
    pub fn login_ui(ui: &mut Ui, world: &mut World) {
        center_window("login", ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                let mut ld = world.resource_mut::<LoginData>();
                if let Some(player) = ld.identity_player.clone() {
                    format!("Login as {}", player.name)
                        .cstr_cs(VISIBLE_LIGHT, CstrStyle::Heading2)
                        .label(ui);
                    if Button::click("Login").ui(ui).clicked() {
                        let _ = cn().reducers.login_by_identity();
                    }
                    br(ui);
                    if Button::click("Logout").gray(ui).ui(ui).clicked() {
                        ld.identity_player = None;
                    }
                } else {
                    let mut ld = world.resource_mut::<LoginData>();
                    "Register"
                        .cstr_cs(VISIBLE_LIGHT, CstrStyle::Heading)
                        .label(ui);
                    if Button::click("New Player").ui(ui).clicked() {
                        let _ = cn().reducers.register_empty();
                    }
                    br(ui);
                    "Login".cstr_cs(VISIBLE_LIGHT, CstrStyle::Heading).label(ui);
                    Input::new("name").ui_string(&mut ld.name_field, ui);
                    Input::new("password")
                        .password()
                        .ui_string(&mut ld.pass_field, ui);
                    if Button::click("Submit").ui(ui).clicked() {
                        let _ = crate::login::login(
                            &cn().reducers,
                            ld.name_field.clone(),
                            ld.pass_field.clone(),
                        );
                    }
                }
            });
        });
    }
}
