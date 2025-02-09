// This file was generated via rust macros. Don't modify manually.
export enum SeelenCommand {
  Run = 'run',
  IsDevMode = 'is_dev_mode',
  IsAppxPackage = 'is_appx_package',
  OpenFile = 'open_file',
  RunAsAdmin = 'run_as_admin',
  SelectFileOnExplorer = 'select_file_on_explorer',
  IsVirtualDesktopSupported = 'is_virtual_desktop_supported',
  GetUserEnvs = 'get_user_envs',
  ShowAppSettings = 'show_app_settings',
  SwitchWorkspace = 'switch_workspace',
  SendKeys = 'send_keys',
  GetIcon = 'get_icon',
  SimulateFullscreen = 'simulate_fullscreen',
  CheckForUpdates = 'check_for_updates',
  InstallLastAvailableUpdate = 'install_last_available_update',
  SystemGetMonitors = 'get_connected_monitors',
  SystemGetColors = 'get_system_colors',
  SystemGetLanguages = 'get_system_languages',
  SetAutoStart = 'set_auto_start',
  GetAutoStartStatus = 'get_auto_start_status',
  StateGetThemes = 'state_get_themes',
  StateGetWegItems = 'state_get_weg_items',
  StateWriteWegItems = 'state_write_weg_items',
  StateGetToolbarItems = 'state_get_toolbar_items',
  StateGetSettings = 'state_get_settings',
  StateWriteSettings = 'state_write_settings',
  StateGetDefaultSettings = 'state_get_default_settings',
  StateGetDefaultMonitorSettings = 'state_get_default_monitor_settings',
  StateGetSpecificAppsConfigurations = 'state_get_specific_apps_configurations',
  StateGetWallpaper = 'state_get_wallpaper',
  StateSetWallpaper = 'state_set_wallpaper',
  StateGetHistory = 'state_get_history',
  StateGetPlugins = 'state_get_plugins',
  StateGetWidgets = 'state_get_widgets',
  StateGetIconPacks = 'state_get_icon_packs',
  StateGetProfiles = 'state_get_profiles',
  StateDeleteCachedIcons = 'state_delete_cached_icons',
  GetUser = 'get_user',
  GetUserFolderContent = 'get_user_folder_content',
  SetUserFolderLimit = 'set_user_folder_limit',
  MediaPrev = 'media_prev',
  MediaTogglePlayPause = 'media_toggle_play_pause',
  MediaNext = 'media_next',
  SetVolumeLevel = 'set_volume_level',
  MediaToggleMute = 'media_toggle_mute',
  MediaSetDefaultDevice = 'media_set_default_device',
  GetMainMonitorBrightness = 'get_main_monitor_brightness',
  SetMainMonitorBrightness = 'set_main_monitor_brightness',
  LogOut = 'log_out',
  Suspend = 'suspend',
  Restart = 'restart',
  Shutdown = 'shutdown',
  Lock = 'lock',
  WegGetItemsForWidget = 'weg_get_items_for_widget',
  WegCloseApp = 'weg_close_app',
  WegKillApp = 'weg_kill_app',
  WegToggleWindowState = 'weg_toggle_window_state',
  WegRequestUpdatePreviews = 'weg_request_update_previews',
  WegPinItem = 'weg_pin_item',
  SetWindowPosition = 'set_window_position',
  RequestFocus = 'request_focus',
  LauncherGetApps = 'launcher_get_apps',
  TempGetByEventTrayInfo = 'temp_get_by_event_tray_info',
  OnClickTrayIcon = 'on_click_tray_icon',
  OnContextMenuTrayIcon = 'on_context_menu_tray_icon',
  WlanGetProfiles = 'wlan_get_profiles',
  WlanStartScanning = 'wlan_start_scanning',
  WlanStopScanning = 'wlan_stop_scanning',
  WlanConnect = 'wlan_connect',
  WlanDisconnect = 'wlan_disconnect',
  NotificationsClose = 'notifications_close',
  NotificationsCloseAll = 'notifications_close_all',
}
