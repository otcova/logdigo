use digolog_app::ui::ObjectBuilder;
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
        let block = ui::BlockBuilder {
            color: ui::Color::RED,
        }
        .build(ui);
    }
}
