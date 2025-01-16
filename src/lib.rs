pub mod error;
pub mod handlers;
pub mod rect;
pub mod state;
pub mod system_state;

pub use error::SeelenLibError;

#[test]
fn generate_schemas() {
    use state::{AppConfig, IconPack, Placeholder, Settings, Theme, WegItem, WindowManagerLayout};

    fn write_schema<T>(path: &str)
    where
        T: schemars::JsonSchema,
    {
        let schema = schemars::schema_for!(T);
        std::fs::write(path, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
    }

    std::fs::create_dir_all("./gen/schemas").unwrap();
    write_schema::<Settings>("./gen/schemas/settings.schema.json");
    write_schema::<Placeholder>("./gen/schemas/placeholder.schema.json");
    write_schema::<Theme>("./gen/schemas/theme.schema.json");
    write_schema::<WindowManagerLayout>("./gen/schemas/layout.schema.json");
    write_schema::<Vec<AppConfig>>("./gen/schemas/settings_by_app.schema.json");
    write_schema::<Vec<WegItem>>("./gen/schemas/weg_items.schema.json");
    write_schema::<IconPack>("./gen/schemas/icon_pack.schema.json");

    handlers::SeelenEvent::generate_ts_file("./src/handlers/events.ts");
    handlers::SeelenCommand::generate_ts_file("./src/handlers/commands.ts");
}
