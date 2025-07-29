macro_rules! define_hotkey_actions {
    (
        $($field:ident$(($arg:ty))? $(= $shortcut:expr)?),*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TS)]
        #[serde(rename_all = "snake_case")]
        pub enum SluHotkeyAction {
            $(
                $field$(($arg))?,
            )*
        }

        impl SluShortcutsSettings {
            fn _default_shortcuts() -> Vec<SluHotkey> {
                vec![
                    $($(
                        SluHotkey::new(SluHotkeyAction::$field).keys($shortcut),
                    )?)*
                ]
            }
        }
    };
}

define_hotkey_actions! {
    ToggleLauncher = ["Win", "S"],
    // tiling-wm reservation
    ReserveTop = ["Win", "Shift", "I"],
    ReserveBottom = ["Win", "Shift", "K"],
    ReserveLeft = ["Win", "Shift", "J"],
    ReserveRight = ["Win", "Shift", "L"],
    ReserveFloat = ["Win", "Shift", "U"],
    ReserveStack = ["Win", "Shift", "O"],
    // tiling-wm focus change actions
    FocusTop = ["Alt", "I"],
    FocusBottom = ["Alt", "K"],
    FocusLeft = ["Alt", "J"],
    FocusRight = ["Alt", "L"],
    // wm focused window sizing
    IncreaseWidth = ["Win", "Alt", "="],
    DecreaseWidth = ["Win", "Alt", "-"],
    IncreaseHeight = ["Win", "Ctrl", "="],
    DecreaseHeight = ["Win", "Ctrl", "-"],
    RestoreSizes = ["Win", "Alt", "0"],
    // wm focused window positioning
    MoveWindowUp = ["Shift", "Alt", "I"],
    MoveWindowDown = ["Shift", "Alt", "K"],
    MoveWindowLeft = ["Shift", "Alt", "J"],
    MoveWindowRight = ["Shift", "Alt", "L"],
    // weg
    StartWegApp(usize),
    // virtual desktops
    SwitchWorkspace(usize),
    MoveToWorkspace(usize),
    SendToWorkspace(usize),
    SwitchToNextWorkspace = ["Ctrl", "Win", "Right"],
    SwitchToPreviousWorkspace = ["Ctrl", "Win", "Left"],
    DestroyCurrentWorkspace = ["Ctrl", "Win", "F4"],
    // misc
    MiscOpenSettings = ["Win", "Shift", "S"],
    MiscToggleLockTracing,
    MiscToggleWinEventTracing
}

/*TODO

!+q:: CycleFocus("previous")
!q:: CycleFocus("next")

; Move windows
#+a:: Move("left")
#+s:: Move("down")
#+w:: Move("up")
#+d:: Move("right")

#+Enter:: Promote()

#+x:: FlipLayout("horizontal")
#+z:: FlipLayout("vertical")

; Stack windows
#a:: Stack("left")
#d:: Stack("right")
#w:: Stack("up")
#s:: Stack("down")
#;:: Unstack()

#+q:: CycleStack("previous")
#q:: CycleStack("next")

; Manipulate windows
#f:: ToggleFloat()
#m:: ToggleMonocle() */

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
pub struct SluHotkey {
    pub action: SluHotkeyAction,
    pub keys: Vec<String>,
}

impl SluHotkey {
    pub fn new(action: SluHotkeyAction) -> Self {
        Self {
            action,
            keys: vec![],
        }
    }

    pub fn keys<'a, T, I>(mut self, keys: I) -> Self
    where
        T: AsRef<str> + 'a,
        I: IntoIterator<Item = T>,
    {
        self.keys = keys.into_iter().map(|k| k.as_ref().to_string()).collect();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct SluShortcutsSettings {
    pub enabled: bool,
    pub app_commands: Vec<SluHotkey>,
}

impl SluShortcutsSettings {
    pub fn contains_action(&self, action: SluHotkeyAction) -> bool {
        self.app_commands.iter().any(|h| h.action == action)
    }

    pub fn sanitize(&mut self) {
        let defaults = Self::default_shortcuts();
        for hotkey in defaults {
            // add missing hotkeys from defaults
            if !self.contains_action(hotkey.action) {
                self.app_commands.push(hotkey);
            }
        }
    }

    pub fn default_shortcuts() -> Vec<SluHotkey> {
        let mut shorcuts = Self::_default_shortcuts();

        for i in 0..10 {
            let digit_key = if i == 9 {
                String::from("0")
            } else {
                format!("{}", i + 1)
            };

            shorcuts.push(
                SluHotkey::new(SluHotkeyAction::StartWegApp(i)).keys(["Win", digit_key.as_str()]),
            );

            shorcuts.push(
                SluHotkey::new(SluHotkeyAction::SwitchWorkspace(i))
                    .keys(["Alt", digit_key.as_str()]),
            );

            shorcuts.push(SluHotkey::new(SluHotkeyAction::MoveToWorkspace(i)).keys([
                "Alt",
                "Shift",
                digit_key.as_str(),
            ]));

            shorcuts.push(SluHotkey::new(SluHotkeyAction::SendToWorkspace(i)).keys([
                "Win",
                "Shift",
                digit_key.as_str(),
            ]));
        }
        shorcuts
    }
}

impl Default for SluShortcutsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            app_commands: Self::default_shortcuts(),
        }
    }
}
