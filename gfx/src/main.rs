mod raylib;

use raylib::wrapper::{Color, init_window};

fn main() {
  const WIDTH: i32 = 800;
  const HEIGHT: i32 = 600;
  let window = init_window(WIDTH, HEIGHT, "zappy").expect("Invalid arguments to init_window");
  let current_monitor = window.get_current_monitor();

  window.set_target_fps(current_monitor.get_refresh_rate());
  while !window.should_close() {
    let pen = window.begin_drawing();
    pen.clear_background(Color::Red);
    pen
      .draw_text(
        "urmom",
        WIDTH / 2 - (2.5 * HEIGHT as f32 / 16.) as i32,
        HEIGHT / 2 - HEIGHT / 16,
        HEIGHT / 8,
        Color::Blue,
      )
      .expect("Invalid arguments to draw_text");
  }
}
