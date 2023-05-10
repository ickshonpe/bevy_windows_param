use bevy::ecs::query::QueryIter;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Camera;
use bevy::prelude::Entity;
use bevy::prelude::GlobalTransform;
use bevy::prelude::Touches;
use bevy::prelude::UiCameraConfig;
use bevy::prelude::Vec2;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy::window::WindowRef;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct CursorPosition {
    window_ref: WindowRef,
    position: Vec2,
}

impl CursorPosition {
    /// The window belonging to this bevy app that the cursor is positioned above.
    pub fn window(&self) -> WindowRef {
        self.window_ref
    }

    /// The cursor's position
    pub fn position(&self) -> Vec2 {
        self.position
    }
}

impl From<CursorPosition> for Vec2 {
    fn from(value: CursorPosition) -> Self {
        value.position
    }
}

impl Deref for CursorPosition {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.position
    }
}

impl Display for CursorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.window_ref {
            WindowRef::Primary => write!(f, "cursor position: {} (Primary Window)", self.position),
            WindowRef::Entity(entity) => {
                write!(f, "cursor_position: {} (Window: {entity:?})", self.position)
            }
        }
    }
}

#[derive(SystemParam)]
pub struct Windows<'w, 's> {
    primary_window_query: Query<'w, 's, (Entity, &'static Window), With<PrimaryWindow>>,
    window_query: Query<'w, 's, (Entity, &'static Window)>,
    camera_query: Query<
        'w,
        's,
        (
            &'static Camera,
            Option<&'static UiCameraConfig>,
            &'static GlobalTransform,
        ),
    >,
    touches_input: Res<'w, Touches>,
}

impl Windows<'_, '_> {
    pub fn get_window(&self, window_ref: WindowRef) -> Option<&Window> {
        match window_ref {
            WindowRef::Primary => self.primary_window_query.get_single().ok(),
            WindowRef::Entity(entity) => self.window_query.get(entity).ok(),
        }
        .map(|(_, window)| window)
    }

    /// Returns the scale factor for the given window
    pub fn scale_factor(&self, window_ref: WindowRef) -> Option<f64> {
        self.get_window(window_ref)
            .map(|window| window.scale_factor())
    }

    /// Returns the physical resolution for the given window
    pub fn physical_resolution(&self, window_ref: WindowRef) -> Option<Vec2> {
        self.get_window(window_ref).map(|window| {
            Vec2::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
            )
        })
    }

    /// Returns the logical resolution for the given window
    pub fn resolution(&self, window_ref: WindowRef) -> Option<Vec2> {
        self.get_window(window_ref)
            .map(|window| Vec2::new(window.width(), window.height()))
    }

    /// Returns the cursor position in window coordinates
    pub fn raw_cursor_position(&self) -> Option<CursorPosition> {
        self.into_iter()
            .find_map(|(entity, window)| {
                window.cursor_position().map(|position| (entity, position))
            })
            .map(|(entity, position)| CursorPosition {
                position,
                window_ref: if self.primary_window_query.contains(entity) {
                    WindowRef::Primary
                } else {
                    WindowRef::Entity(entity)
                },
            })
            .or_else(|| {
                self.touches_input
                    .first_pressed_position()
                    .map(|position| CursorPosition {
                        window_ref: WindowRef::Primary,
                        position,
                    })
            })
    }

    /// Returns the cursor position in UI coordinates
    pub fn ui_cursor_position(&self) -> Option<CursorPosition> {
        self.camera_query
            .iter()
            .filter(|(_, camera_ui, _)| {
                !matches!(camera_ui, Some(&UiCameraConfig { show_ui: false, .. }))
            })
            .filter_map(|(camera, _, _)| {
                if let RenderTarget::Window(window_ref) = camera.target {
                    Some(window_ref)
                } else {
                    None
                }
            })
            .find_map(|window_ref| {
                self.get_window(window_ref)
                    .and_then(|window| window.cursor_position())
                    .map(|position| (window_ref, position))
            })
            .or_else(|| {
                self.touches_input
                    .first_pressed_position()
                    .map(|position| (WindowRef::Primary, position))
            })
            .and_then(|(window_ref, mut position)| {
                self.resolution(window_ref).map(|size| {
                    position.y = size.y - position.y;
                    CursorPosition {
                        window_ref,
                        position,
                    }
                })
            })
    }

    /// Returns the cursor position in world coordinates
    pub fn world_cursor_position(&self) -> Option<Vec2> {
        self.camera_query
            .into_iter()
            .filter_map(|(camera, _, transform)| {
                if let RenderTarget::Window(window_ref) = camera.target {
                    Some((camera, transform, window_ref))
                } else {
                    None
                }
            })
            .find_map(|(camera, transform, window_ref)| {
                self.get_window(window_ref).and_then(|window| {
                    window
                        .cursor_position()
                        .and_then(|cursor_position| {
                            camera.viewport_to_world(transform, cursor_position)
                        })
                        .map(|ray| ray.origin.truncate())
                })
            })
    }
}

impl<'w, 's> IntoIterator for &'w Windows<'_, 's> {
    type Item = (Entity, &'w Window);
    type IntoIter = QueryIter<'w, 's, (Entity, &'static Window), ()>;

    fn into_iter(self) -> Self::IntoIter {
        self.window_query.iter()
    }
}
