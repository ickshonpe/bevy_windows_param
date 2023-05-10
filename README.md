# bevy_windows_param

[![crates.io](https://img.shields.io/crates/v/bevy_windows_param)](https://crates.io/crates/bevy_windows_param)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_windows_param)
[![crates.io](https://img.shields.io/crates/d/bevy_windows_param)](https://crates.io/crates/bevy_windows_param)

`bevy_windows_param::Windows` is a Bevy `SystemParam` providing a more ergonomic interface for accessing window-specific information, like resolutions and cursor positions."

The implemented methods include:

* `get_window`: Retrieves a `Window`
* `resolution`: Returns the logical resolution of a window
* `physical_resolution`: Returns the physical resolution of a window
* `scale_factor`: Returns the scale factor of a window 
* `cursor_position`:  Determines the window over which the cursor is positioned, as well as the cursor's location within that window.
* `ui_cursor_position`: Similar to `cursor_position`, but returns the cursor's position in UI coordinates.
* `world_cursor_position`: Similar to `cursor_position`, but returns the cursor's position in world coordinates.

Supports Bevy version 0.10

## Examples        

* ```cargo run --example cursor_position```
* ```cargo run --example multiple_windows```




