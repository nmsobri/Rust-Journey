use ggez::*;
use ggez::graphics::DrawParam;
use ggez::event::EventHandler;

enum Shape {
    Rectangle(graphics::Rect),
    Circle(mint::Point2<f32>, f32),
}

struct State {
    shapes: Vec<Shape>
}

impl State {
    fn new(shapes: Vec<Shape>) -> Self {
        Self {
            shapes
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for shape in &self.shapes {
            let mesh = match shape {
                &Shape::Rectangle(rectangle) => graphics::Mesh::new_rectangle(
                    ctx, graphics::DrawMode::fill(),
                    rectangle,
                    graphics::WHITE,
                )?,

                &Shape::Circle(origin, radius) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    origin,
                    radius,
                    0.1,
                    graphics::WHITE,
                )?
            };

            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let mut shapes = Vec::new();

    shapes.push(Shape::Rectangle(graphics::Rect::new(500.0, 100.0, 100.0, 100.0)));
    shapes.push(Shape::Circle(mint::Point2 { x: 200.0, y: 250.0 }, 50.0));

    let mut state = &mut State::new(shapes);
    let cb = ContextBuilder::new("generative_art", "Mohd Sobri");
    let (ref mut context, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(context, event_loop, state).unwrap();
}
