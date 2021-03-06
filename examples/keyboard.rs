use tetra::graphics::{self, Color, DrawParams, Texture, Vec2};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

struct GameState {
    texture: Texture,
    position: Vec2,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./examples/resources/player.png")?,
            position: Vec2::new(160.0 / 2.0, 144.0 / 2.0),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::A) {
            self.position.x -= 2.0;
        }

        if input::is_key_down(ctx, Key::D) {
            self.position.x += 2.0;
        }

        if input::is_key_down(ctx, Key::W) {
            self.position.y -= 2.0;
        }

        if input::is_key_down(ctx, Key::S) {
            self.position.y += 2.0;
        }

        let mut pressed = input::get_keys_pressed(ctx).peekable();
        if pressed.peek().is_some() {
            println!("Keys pressed this tick: {:?}", pressed.collect::<Vec<_>>());
        }

        let mut released = input::get_keys_released(ctx).peekable();
        if released.peek().is_some() {
            println!(
                "Keys released this tick: {:?}",
                released.collect::<Vec<_>>()
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(8.0, 8.0)),
        );

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Keyboard Input", 160, 144)
        .maximized(true)
        .resizable(true)
        .quit_on_escape(true)
        .build()?
        .run_with(GameState::new)
}
