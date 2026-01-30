mod raylib;

use raylib::wrapper::{Color, init_window};

use crate::raylib::wrapper::{Camera3D, Camera3DMode, Camera3DProjection, Vector3};

fn main() {
  const WIDTH: i32 = 800;
  const HEIGHT: i32 = 600;
  let window = init_window(WIDTH, HEIGHT, "zappy").expect("Invalid arguments to init_window");
  let current_monitor = window.get_current_monitor();
  let cube_position = Vector3 {
    x: 0.,
    y: 0.,
    z: 0.,
  };

  let mut camera = Camera3D {
    position: Vector3 {
      x: 10.,
      y: 10.,
      z: 10.,
    },
    target: Vector3 {
      x: 0.,
      y: 0.,
      z: 0.,
    },
    up: Vector3 {
      x: 0.,
      y: 1.,
      z: 0.,
    },
    fovy: 45.,
    projection: Camera3DProjection::Perspective,
  };

  window.disable_cursor();
  window.set_target_fps(current_monitor.get_refresh_rate());
  while !window.should_close() {
    camera.update(Camera3DMode::Free);

    let pen = window.begin_drawing();
    pen.clear_background(Color::RayWhite);

    {
      let pen_3d = pen.begin_mode_3d(camera.clone());
      pen_3d.draw_grid(10, 1.);
      pen_3d.draw_cube(cube_position, 2., 2., 2., Color::Red);
      pen_3d.draw_cube_wires(cube_position, 2., 2., 2., Color::Blue);
    }

    pen
      .draw_text("Urmom", WIDTH / 64, 0, HEIGHT / 8, Color::Blue)
      .expect("Invalid arguments to draw_text");
  }
}
