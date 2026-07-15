use bevy::prelude::*;

pub(crate) trait AssignVec {
    fn assign_from(&mut self, v: Vec2);
}

impl AssignVec for Vec3 {
    fn assign_from(&mut self, v: Vec2) {
        self.x = v.x;
        self.y = v.y;
    }
}
