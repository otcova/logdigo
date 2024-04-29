use digolog_app::*;

fn main() {
    run_app(Digolog::setup());
}

struct Digolog;

impl Digolog {
    async fn setup() -> Self {
        Self
    }
}

impl AppBrain for Digolog {
    fn init(&mut self, ui: &mut UI) {
        ui::BlockBuilder {
            position: Vec2 { x: -0.5, y: -1. },
            color: Color::WHITE,
        }
        .build(ui);
    }
}
