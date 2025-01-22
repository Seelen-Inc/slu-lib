use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::WidgetId;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, TS)]
pub struct SettingsByWidget(HashMap<WidgetId, serde_json::Map<String, serde_json::Value>>);

impl SettingsByWidget {
    pub fn get_raw_config(
        &self,
        widget_id: &WidgetId,
    ) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.0.get(widget_id)
    }

    pub fn get_raw_config_mut(
        &mut self,
        widget_id: &WidgetId,
    ) -> Option<&mut serde_json::Map<String, serde_json::Value>> {
        self.0.get_mut(widget_id)
    }

    pub fn get_config<T: Default + for<'de> Deserialize<'de>>(&self, widget_id: &WidgetId) -> T {
        self.0
            .get(widget_id)
            .and_then(|v| serde_json::from_value(v.clone().into()).ok())
            .unwrap_or_default()
    }

    pub fn set_config<T: Serialize>(
        &mut self,
        widget_id: WidgetId,
        config: T,
    ) -> Result<(), serde_json::Error> {
        if let serde_json::Value::Object(v) = serde_json::to_value(config)? {
            self.0.insert(widget_id, v);
        }
        Ok(())
    }

    pub fn sanitize(&mut self) {
        let mut sanitized = self.0.clone();
        for (key, _) in self.0.iter() {
            if !key.is_valid() {
                sanitized.remove(key);
            }
        }
        self.0 = sanitized;
    }
}
