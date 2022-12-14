//! Graphics!
//!
//! ## Goals
//!
//! - Feature set
//!  - Integrations with the platform windowing system for image presentation.
//!   - Note (2022-12-19): Should this instead belong to the shell?
//!  - Support for 2D rendering (and only 2D rendering).
//!   - Triangle Meshes
//!     - Vertex colors
//!     - Textured meshes
//!   - Vector graphics
//!     - SVG-compatible paths
//!   - Text
//!   - Images
//!   - Effects
//!    - Drop shadows
//!    - Blurs
//!    - Transparency & Color Filters
//!  - Render to texture
//!  - Render to window
//!
//! ## Thoughts & Rationale
//!
//! - Why not use a library like 'wgpu' instead of rolling your own graphics
//!   HAL?
//!  - 'wgpu' is currently in flux and is not yet stable. Furthermore, the
//!    anticipated feature set (see above) is simple enough that porting it to
//!    other platforms shouldn't be too difficult (I hope...).
//!
//! ## Development Timeline
//!
//!  A timeline of significant events in the development of this crate.
//!
//! - 2022-12-19: Work begins after a few false starts.

use std::cell::RefCell;

use geometry::Point;
use raw_window_handle::HasRawWindowHandle;

mod render_graph;

#[cfg(target_os = "windows")]
mod dx12;

#[cfg(target_os = "windows")]
use dx12 as platform;

pub use render_graph::{RenderGraph, RenderGraphNodeId};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    #[must_use]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Point<f32>,
    pub color: Color,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PowerPreference {
    #[default]
    DontCare,
    LowPower,
    HiPower,
}

/// Options for configuring the graphics context on initialization. Once set,
/// these options cannot be changed without recreating the graphics context.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsConfig {
    pub debug_mode: bool,
    pub power_preference: PowerPreference,
}

pub struct GraphicsContext {
    inner: RefCell<platform::GraphicsContext>,
}

impl GraphicsContext {
    #[must_use]
    pub fn new(config: &GraphicsConfig) -> Self {
        Self {
            inner: RefCell::new(platform::GraphicsContext::new(config)),
        }
    }

    #[must_use]
    pub fn create_surface(&self, window: impl HasRawWindowHandle) -> Surface {
        Surface {
            inner: self
                .inner
                .borrow()
                .create_surface(window.raw_window_handle()),
        }
    }

    pub fn draw(&self, target: &Image, content: &RenderGraph) {
        self.inner.borrow_mut().draw(&target.inner, content);
    }
}

pub struct Surface {
    inner: platform::Surface,
}

impl Surface {
    /// Retrieves the next image from the surface's swapchain.
    ///
    /// This fucntion will block until the next image is available.
    pub fn get_next_image(&mut self) -> SurfaceImage {
        SurfaceImage {
            inner: self.inner.get_next_image(),
        }
    }

    pub fn resize(&mut self) {
        self.inner.resize();
    }
}

pub struct SurfaceImage<'a> {
    inner: platform::SurfaceImage<'a>,
}

impl<'a> SurfaceImage<'a> {
    /// Presents the swapchain image to the surface.
    pub fn present(self) {
        self.inner.present();
    }

    #[must_use]
    pub fn image(&self) -> &Image {
        // This is safe as long as Image remains repr(transparent).
        unsafe { &*((self.inner.get_image() as *const dx12::Image).cast()) }
    }
}

#[repr(transparent)]
pub struct Image {
    inner: platform::Image,
}
