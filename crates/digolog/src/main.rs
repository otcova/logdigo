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
        for x in -10..10 {
            for y in -10..10 {
                ui::BlockBuilder {
                    position: [4 * x, 4 * y].into(),
                    size: [3, 3].into(),
                    color: [140, 100, 200, 0].into(),
                }
                .build(ui);
            }
        }

        ui::WireBuilder {
            position_a: [0, 0].into(),
            position_b: [10, 20].into(),
            color: [140, 200, 100, 0].into(),
        }
        .build(ui);
    }
}
