use nannou::{prelude::{Vec2, rgb8}, Draw};

pub struct Pendulum {
  pub  radius: f32,
  pub angle: f32,
  pub origin: Vec2,
  pub position: Vec2,
  pub angular_velocity: f32,
  pub angular_acceleration: f32,
  pub gravity: f32,
  pub friction: f32,
  pub mass: f32,
}

impl Pendulum {
  pub fn update(&mut self) {
      self.angular_acceleration = self.gravity * self.angle.sin();
      self.angular_velocity += self.angular_acceleration / self.radius;
      self.angular_velocity *= self.friction;
      self.angle += self.angular_velocity;
      self.position += self.origin;
      self.position.x = self.radius * self.angle.sin();
      self.position.y = self.radius * self.angle.cos();
  }

  pub fn draw(&self, draw: &Draw) {
      draw.line()
          .stroke_weight(2.0)
          .color(rgb8(214, 213, 168)) //rgb(214, 213, 168)
          .points(self.origin, self.position);

      draw.ellipse()
          .color(rgb8(129, 103, 151)) // rgb(129, 103, 151)
          .x(self.position.x)
          .y(self.position.y)
          .w_h(self.mass, self.mass);
  }
}