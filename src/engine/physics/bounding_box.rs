use glam::Vec2;

/*

Bounding boxes come in a pair:
- First Layer stores simple rectangle for easy mathematical check on intersection
- Second Layer stores complex, animation fit bounds for indepth OBB check

First layer is stored as a close fit rect
Second layer is rotated to fit inside that polygon

All translations and rotations are stored in the collider and used when 
checking intersections

*/

pub struct MTV {
    pub magnitude: f32,
    pub direction: Vec2,
}

/// oriented bounding box
/// - only encapsulates convex shapes
/// - convex decomposisition check before loading this struct
pub struct OBB {
    pub corners: Vec<Vec2>, // all faces of a convex shape
    axis: Vec<Vec2>, // all axis for intersection testing
}

impl OBB {
    pub fn new(
        corners: Vec<Vec2>,
    ) -> Self {
        return Self {
            corners,
            axis: Vec::new(),
        };
    }

    /// vertices or rotation change/recalculate
    pub fn update_axis(&mut self) {
        self.axis.clear();
        for i in 0..self.corners.len() {
            if i == self.corners.len() - 1 {
                self.axis.push(OBB::face_normal(
                    self.corners[i], 
                    self.corners[0], 
                ));
                break;
            }
            self.axis.push(OBB::face_normal(
                self.corners[i], 
                self.corners[i + 1], 
            ));
        }
    }

    /// ...
    pub fn intersects(
        &self, 
        _other: &OBB, 
        _translation: Vec2, // from 0,0
        _angle: f32, // angular 0-360
    ) -> MTV {
        // for axis in self.axis.iter().chain(other.axis.iter()) {
        //     let (min0, max0) = OBB::project(&self.corners, axis);
        //     let (min1, max1) = OBB::project(&other.corners, axis);
        //
        //     if max0 < min1 || max1 < min0 {
        //         return false;
        //     }
        // }

        return MTV {
            magnitude: 0.0,
            direction: Vec2::new(0.0, 0.0),
        };
    }

    fn face_normal(p1: Vec2, p2: Vec2) -> Vec2 {
        let edge = p2 - p1;
        return Vec2::new(-edge.y, edge.x).normalize();
    }

    /// projects normal onto axis and returns "shadow" of polygon
    fn project(poly: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
        let mut min = poly[0].dot(*axis);
        let mut max = min;

        for point in &poly[1..] {
            let pos = point.dot(*axis);
            min = min.min(pos);
            max = max.max(pos);
        }

        return (min, max);
    }
}

/// oriented aabb rectangular bb
/// - simple collision checks
/// - helps reduce higher cost polygon checks
pub struct AABB {
    corners: [Vec2; 4], 
}

impl AABB {
    pub fn new(
        corners: [Vec2; 4],
    ) -> Self {
        return Self {
            corners,
        }
    }

    pub fn intersects(
        &mut self,
        _other: &AABB,
        _translation: Vec2,
        _angle: f32,
    ) -> bool {
        return true;
    }
}









