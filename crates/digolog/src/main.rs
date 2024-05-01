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
            position: [0, 0].into(),
            size: [1, 3].into(),
            color: [140, 100, 200, 0].into(),
        }
        .build(ui);
    }
}
