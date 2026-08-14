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

/// convex oriented bounding box
/// - only encapsulates convex shapes
/// - convex decomposisition check before loading this struct
pub struct COBB {
    // model space info
    local_corners: Vec<Vec2>,
    position: Vec2,
    angle: f32,

    pub corners: Vec<Vec2>, // all faces of a convex shape
    axis: Vec<Vec2>, // all axis for intersection testing
}

impl COBB {
    pub fn new(
        corners: Vec<Vec2>,
    ) -> Self {
        let mut obb = Self { 
            local_corners: corners, 
            position: Vec2::ZERO,
            angle: 0.0,
            corners: Vec::new(),
            axis: Vec::new() 
        };

        obb.transform(Vec2::ZERO, 0.0);
        return obb;
    }

    pub fn transform(&mut self, position: Vec2, angle: f32) {
        if position == self.position && angle == self.angle {
            return;
        }

        self.position = position;
        self.angle = angle;

        let (sin, cos) = angle.sin_cos();
        self.corners.clear();
        self.corners.extend(self.local_corners.iter().map(|p| {
            Vec2::new(p.x * cos - p.y * sin, p.x * sin + p.y * cos) + position
        }));

        self.axis.clear();
        let n = self.corners.len();
        for i in 0..n {
            let next = (i + 1) % n;
            self.axis.push(Self::face_normal(self.corners[i], self.corners[next]));
        }
    }

    /// Minimal Translation Vector returned if collision
    pub fn collision(&self, other: &COBB) -> MTV {
        let mut min_overlap = f32::MAX;
        let mut min_axis = Vec2::ZERO;

        for axis in self.axis.iter().chain(other.axis.iter()) {
            let (min0, max0) = Self::project(&self.corners, axis);
            let (min1, max1) = Self::project(&other.corners, axis);

            if max0 < min1 || max1 < min0 {
                return MTV {
                    magnitude: 0.0,
                    direction: Vec2::ZERO,
                };
            }

            let overlap = max0.min(max1) - min0.max(min1);
            if overlap < min_overlap {
                min_overlap = overlap;
                min_axis = *axis;
            }
        }

        // finds mtv
        let self_center = Self::centroid(&self.corners);
        let other_center = Self::centroid(&other.corners);
        if (other_center - self_center).dot(min_axis) < 0.0 {
            min_axis = -min_axis;
        }

        return MTV { 
            magnitude: min_overlap, 
            direction: min_axis 
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

    fn centroid(poly: &[Vec2]) -> Vec2 {
        let sum: Vec2 = poly.iter().fold(Vec2::ZERO, |acc, &v| acc + v);
        return sum / poly.len() as f32;
    }
}

/// oriented aabb rectangular bb
/// - simple collision checks
/// - helps reduce higher cost polygon checks
pub struct AABB {
    // recomputes model transform from these values, stay "const"
    half_extents: Vec2, // x/y from 0,0,0
    local_center: Vec2, // centroid of local model, might not be 0,0
    local_angle: f32,

    // world transforms
    position: Vec2,
    angle: f32, 

    // world space
    axis: [Vec2; 2], 
    world_corners: [Vec2; 4],
}

impl AABB {
    /// REQUIRE: model origin
    pub fn new(corners: [Vec2; 4]) -> Self {
        let edge = corners[1] - corners[0];

        let mut aabb =  Self {
            half_extents: Vec2::new(
                (corners[1] - corners[0]).length() * 0.5,
                (corners[2] - corners[1]).length() * 0.5,
            ),
            local_center: (corners[0] + corners[1] + corners[2] + corners[3]) * 0.25,
            local_angle: edge.y.atan2(edge.x),
            position: Vec2::ZERO,
            angle: 0.0,
            axis: [Vec2::X, Vec2::Y],
            world_corners: [Vec2::ZERO; 4],
        };

        aabb.transform(Vec2::ZERO, 0.0);
        return aabb;
    }

    /// update if object changes
    pub fn transform(&mut self, position: Vec2, angle: f32) {
        if angle != self.angle || position != self.position {
            self.position = position;
            self.angle = angle;

            let world_angle: f32 = self.local_angle + angle;
            let (sin, cos) = world_angle.sin_cos();
            let ax = Vec2::new(cos, sin);
            let ay = Vec2::new(-sin, cos);

            // rotate the local object into world space positioning
            let (msin, mcos) = angle.sin_cos();
            let rotated_center = Vec2::new(
                self.local_center.x * mcos - self.local_center.y * msin,
                self.local_center.x * msin + self.local_center.y * mcos,
            );
            let center = position + rotated_center;

            let ex = ax * self.half_extents.x;
            let ey = ay * self.half_extents.y;

            self.axis = [ax, ay];
            self.world_corners = [
                center - ex - ey,
                center + ex - ey,
                center + ex + ey,
                center - ex + ey,
            ];
        } 
    }

    pub fn collision(&mut self, other: &AABB) -> bool {
        for axis in self.axis.iter().chain(other.axis.iter()) {
            let (min0, max0) = AABB::project(&self.world_corners, axis);
            let (min1, max1) = AABB::project(&other.world_corners, axis);

            if max0 < min1 || max1 < min0 {
                return false;
            }
        }

        return true;
    }

    /// projects normal onto axis and returns "shadow" of polygon
    fn project(corners: &[Vec2; 4], axis: &Vec2) -> (f32, f32) {
        let mut min = corners[0].dot(*axis);
        let mut max = min;

        for point in &corners[1..4] {
            let pos = point.dot(*axis);
            min = min.min(pos);
            max = max.max(pos);
        }

        return (min, max);
    }
}
