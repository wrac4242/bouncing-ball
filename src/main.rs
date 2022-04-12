extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::context::Context;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const SCREEN_SIZE : [u32; 2] = [640, 480];

pub struct Ball {
    x: f64,
    y: f64,
    radius: f64,
    x_speed: f64,
    y_speed: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, radius: f64) -> Ball {
        Ball {
            x: x,
            y: y,
            radius: radius,
            x_speed: 40.0,
            y_speed: 40.0,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.x = self.x + self.x_speed * args.dt;
        self.y = self.y + self.y_speed * args.dt;

        if self.x + self.radius >= SCREEN_SIZE[0] as f64 || self.x <= 0.0 {
            self.x_speed = -self.x_speed;
        }

        if self.y + self.radius >= SCREEN_SIZE[1] as f64 || self.y <= 0.0 {
            self.y_speed = -self.y_speed;
        }
    }

    pub fn render(&mut self, _args: &RenderArgs, gl: &mut GlGraphics, c: Context) {

        let circle_boarder = graphics::rectangle::square(self.x, self.y, self.radius);
        let transform = c.transform;
        graphics::ellipse(RED, circle_boarder, transform, gl);
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ball: Ball,  
}

impl App {
    fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
            ball: Ball::new(SCREEN_SIZE[0] as f64 / 2.0, SCREEN_SIZE[1] as f64 / 2.0, 30.0),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            self.ball.render(args, gl, c);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.ball.update(args);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", SCREEN_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
