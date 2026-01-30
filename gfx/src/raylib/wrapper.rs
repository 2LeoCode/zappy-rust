use {
  crate::raylib::bindings::{
    self, BeginDrawing, ClearBackground, CloseWindow, DisableCursor, DrawText, EnableCursor,
    EndDrawing, GetCurrentMonitor, GetMonitorRefreshRate, InitWindow, SetTargetFPS,
    WindowShouldClose,
  },
  std::ffi::{CString, NulError, c_int},
  thiserror::Error,
};

use crate::raylib::bindings::{
  BeginMode3D, CameraMode_CAMERA_CUSTOM, CameraMode_CAMERA_FIRST_PERSON, CameraMode_CAMERA_FREE,
  CameraMode_CAMERA_ORBITAL, CameraMode_CAMERA_THIRD_PERSON, CameraProjection_CAMERA_ORTHOGRAPHIC,
  CameraProjection_CAMERA_PERSPECTIVE, DrawCube, DrawCubeWires, DrawGrid, EndMode3D, PlaySound,
  UpdateCamera,
};
pub use crate::raylib::bindings::{Vector2, Vector3, Vector4};

#[derive(Error, Debug)]
pub enum InitWindowError {
  #[error("nul character in title")]
  TitleContainsNul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum DrawTextError {
  #[error("nul character in text")]
  TextContainsNul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum Camera3DError {
  #[error("invalid camera projection value: {0}")]
  InvalidProjection(u32),
  #[error("invalid camera mode value: {0}")]
  InvalidMode(u32),
}

#[derive(Clone)]
pub enum Color {
  LightGray,
  Gray,
  DarkGray,
  Yellow,
  Gold,
  Orange,
  Pink,
  Red,
  Maroon,
  Green,
  Lime,
  DarkGreen,
  SkyBlue,
  Blue,
  DarkBlue,
  Purple,
  Violet,
  DarkPurple,
  Beige,
  Brown,
  DarkBrown,
  White,
  Black,
  Blank,
  Magenta,
  RayWhite,
  Custom(u8, u8, u8, u8),
}

#[derive(Clone)]
pub enum Camera3DProjection {
  Perspective,
  Orthographic,
}

#[derive(Clone)]
pub enum Camera3DMode {
  Free,
  Custom,
  Orbital,
  FirstPerson,
  ThirdPerson,
}

pub struct Window {
  title: CString,
}

pub struct Monitor {
  id: i32,
}

pub struct Pen;
pub struct Pen3D;

#[derive(Clone)]
pub struct Camera3D {
  pub position: Vector3,
  pub target: Vector3,
  pub up: Vector3,
  pub fovy: f32,
  pub projection: Camera3DProjection,
}

impl From<Color> for bindings::Color {
  fn from(value: Color) -> bindings::Color {
    use Color::*;
    match value {
      LightGray => bindings::Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
      },
      Gray => bindings::Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
      },
      DarkGray => bindings::Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
      },
      Yellow => bindings::Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
      },
      Gold => bindings::Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
      },
      Orange => bindings::Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
      },
      Pink => bindings::Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
      },
      Red => bindings::Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
      },
      Maroon => bindings::Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
      },
      Green => bindings::Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
      },
      Lime => bindings::Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
      },
      DarkGreen => bindings::Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
      },
      SkyBlue => bindings::Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
      },
      Blue => bindings::Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
      },
      DarkBlue => bindings::Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
      },
      Purple => bindings::Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
      },
      Violet => bindings::Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
      },
      DarkPurple => bindings::Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
      },
      Beige => bindings::Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
      },
      Brown => bindings::Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
      },
      DarkBrown => bindings::Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
      },
      White => bindings::Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      },
      Black => bindings::Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      Blank => bindings::Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      },
      Magenta => bindings::Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
      },
      RayWhite => bindings::Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
      },
      Custom(r, g, b, a) => bindings::Color { r, g, b, a },
    }
  }
}

impl From<Camera3DProjection> for u32 {
  fn from(value: Camera3DProjection) -> Self {
    use Camera3DProjection::*;
    match value {
      Perspective => CameraProjection_CAMERA_PERSPECTIVE,
      Orthographic => CameraProjection_CAMERA_ORTHOGRAPHIC,
    }
  }
}

impl TryFrom<u32> for Camera3DProjection {
  type Error = Camera3DError;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DError::*;
    use Camera3DProjection::*;
    match value {
      CameraProjection_CAMERA_PERSPECTIVE => Ok(Perspective),
      CameraProjection_CAMERA_ORTHOGRAPHIC => Ok(Orthographic),
      value => Err(InvalidProjection(value)),
    }
  }
}

impl From<Camera3DMode> for u32 {
  fn from(value: Camera3DMode) -> Self {
    use Camera3DMode::*;
    match value {
      Free => CameraMode_CAMERA_FREE,
      Custom => CameraMode_CAMERA_CUSTOM,
      Orbital => CameraMode_CAMERA_ORBITAL,
      FirstPerson => CameraMode_CAMERA_FIRST_PERSON,
      ThirdPerson => CameraMode_CAMERA_THIRD_PERSON,
    }
  }
}

impl TryFrom<u32> for Camera3DMode {
  type Error = Camera3DError;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DError::*;
    use Camera3DMode::*;
    match value {
      CameraMode_CAMERA_FREE => Ok(Free),
      CameraMode_CAMERA_CUSTOM => Ok(Custom),
      CameraMode_CAMERA_ORBITAL => Ok(Orbital),
      CameraMode_CAMERA_FIRST_PERSON => Ok(FirstPerson),
      CameraMode_CAMERA_THIRD_PERSON => Ok(ThirdPerson),
      value => Err(InvalidMode(value)),
    }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      CloseWindow();
    }
  }
}

impl Window {
  pub fn title(&self) -> &str {
    unsafe { self.title.to_str().unwrap_unchecked() }
  }

  pub fn should_close(&self) -> bool {
    unsafe { WindowShouldClose() }
  }

  pub fn begin_drawing(&self) -> Pen {
    unsafe { BeginDrawing() }
    Pen {}
  }

  pub fn enable_cursor(&self) {
    unsafe { EnableCursor() }
  }

  pub fn disable_cursor(&self) {
    unsafe { DisableCursor() }
  }

  pub fn set_target_fps(&self, fps: i32) {
    unsafe { SetTargetFPS(fps as c_int) }
  }

  pub fn get_current_monitor(&self) -> Monitor {
    Monitor {
      id: unsafe { GetCurrentMonitor() } as i32,
    }
  }
}

impl Drop for Pen {
  fn drop(&mut self) {
    unsafe {
      EndDrawing();
    }
  }
}

impl Pen {
  pub fn clear_background(&self, color: Color) {
    unsafe { ClearBackground(color.into()) }
  }

  pub fn draw_text(
    &self,
    text: &str,
    pos_x: i32,
    pos_y: i32,
    font_size: i32,
    color: Color,
  ) -> Result<(), DrawTextError> {
    let text = CString::new(text)?;

    unsafe { DrawText(text.as_ptr(), pos_x, pos_y, font_size, color.into()) }
    Ok(())
  }

  pub fn begin_mode_3d(&self, camera: Camera3D) -> Pen3D {
    unsafe { BeginMode3D(camera.into()) }
    Pen3D {}
  }
}

impl TryFrom<bindings::Camera3D> for Camera3D {
  type Error = Camera3DError;
  fn try_from(value: bindings::Camera3D) -> Result<Self, Self::Error> {
    Ok(Camera3D {
      position: value.position,
      target: value.target,
      up: value.up,
      fovy: value.fovy,
      projection: Camera3DProjection::try_from(value.projection as u32)?,
    })
  }
}

impl From<Camera3D> for bindings::Camera3D {
  fn from(value: Camera3D) -> Self {
    bindings::Camera3D {
      position: value.position,
      target: value.target,
      up: value.up,
      fovy: value.fovy,
      projection: u32::from(value.projection) as i32,
    }
  }
}

impl Camera3D {
  pub fn update(&mut self, mode: Camera3DMode) {
    let mut cam = bindings::Camera3D::from(self.clone());
    unsafe {
      UpdateCamera(
        &mut cam as *mut bindings::Camera3D,
        u32::from(mode) as c_int,
      )
    }
    *self = unsafe { cam.try_into().unwrap_unchecked() }
  }
}

impl Monitor {
  pub fn get_refresh_rate(&self) -> i32 {
    (unsafe { GetMonitorRefreshRate(self.id as c_int) }) as i32
  }
}

impl Drop for Pen3D {
  fn drop(&mut self) {
    unsafe { EndMode3D() }
  }
}

impl Pen3D {
  pub fn draw_grid(&self, slices: i32, spacing: f32) {
    unsafe { DrawGrid(slices as c_int, spacing) }
  }

  pub fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe { DrawCube(position, width, height, length, color.into()) }
  }

  pub fn draw_cube_wires(
    &self,
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
  ) {
    unsafe { DrawCubeWires(position, width, height, length, color.into()) }
  }
}

pub fn init_window(width: i32, height: i32, title: &str) -> Result<Window, InitWindowError> {
  let title = CString::new(title)?;

  unsafe { InitWindow(width as c_int, height as c_int, title.as_ptr()) }
  Ok(Window { title })
}
