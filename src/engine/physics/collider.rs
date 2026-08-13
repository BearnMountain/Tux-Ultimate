use glam::Vec2;

use super::bounding_box::{OBB, AABB, MTV};

/// Each collider has a rectangular bb with a fitted polygonal bb
/// - First layer provides basic check for rectangular aabb intersection
/// - Second layer is a fit polygon for accurate intersections
pub struct Collider {
    first_layer: AABB, 
    second_layer: OBB, 

    // polygon itself
    angle: f32,
    translation: Vec2,
    local_corners: Vec<Vec2>,
}

impl Collider {
    /// all corners of the polygon, 0 and len-1 connect back to each other
    pub fn new(
        _polygon: Vec<Vec2>, 
    ) -> Self {
        let aabb_corners = [
            Vec2::ZERO,
            Vec2::ZERO,
            Vec2::ZERO,
            Vec2::ZERO,
        ];
        let obb_corners = Vec::new();

        return Self {
            first_layer: AABB::new(aabb_corners),
            second_layer: OBB::new(obb_corners),
            angle: 0.0,
            translation: Vec2::new(0.0, 0.0),
        };
    }

    pub fn intersects(
        &mut self,
        other: &Collider,
    ) -> MTV {
        if self.first_layer.intersects(
            &other.first_layer,
            self.translation,
            self.angle,
        ) {
            return self.second_layer.intersects(
                &other.second_layer, 
                self.translation, 
                self.angle,
            );
        }

        return MTV {
            magnitude: 0.0,
            direction: Vec2::new(0.0, 0.0),
        };
    }

    /// ---- Change Collider Data ----
    /// set new polygon bb
    pub fn set_bounding_box(&mut self, bb: Vec<Vec2>) {
        self.second_layer.corners = bb;
        self.second_layer.update_axis(); // update faces with new normal axis
        self.fit_aabb(); // updates first_layer
    }
    pub fn set_rotation(&mut self, angle: f32) {
        self.angle = (angle % 360.0 + 360.0) % 360.0;
    }
    pub fn set_translation(&mut self, translation: Vec2) {
        self.translation = translation;
    }

    fn fit_aabb(&mut self) {

    }
}
