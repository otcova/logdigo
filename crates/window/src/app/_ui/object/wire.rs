use crate::*;

pub struct WireBuilder {
    pub position_a: Vec2<i32>,
    pub position_b: Vec2<i32>,
    pub color: Color,
}

pub struct WireHandle {
    id: ObjectId,
}

impl WireBuilder {
    pub fn build(self, ui: &mut UI) -> WireHandle {
        let id = ui.painters.new_object_id();
        ui.painters.wire.insert(id, self);
        WireHandle { id }
    }
}

impl WireHandle {
    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn delete(self, _ui: &mut UI) {}
}
