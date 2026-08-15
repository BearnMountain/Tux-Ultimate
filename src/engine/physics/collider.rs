use glam::Vec2;

use super::bounding_box::{COBB, AABB, MTV};

/// Each collider has a rectangular bb with a fitted polygonal bb
/// - First layer provides basic check for rectangular aabb intersection
/// - Second layer is a fit polygon for accurate intersections
pub struct Collider {
    first_layer: AABB, 
    second_layer: COBB, 

    position: Vec2,
    angle: f32,
}

impl Collider {
    /// all corners of the polygon, 0 and len-1 connect back to each other
    pub fn new(
        polygon: Vec<Vec2>, 
    ) -> Self {
        let aabb_oriented_rect = Collider::fit_aabb(&polygon);

        return Self {
            first_layer: AABB::new(aabb_oriented_rect),
            second_layer: COBB::new(polygon),
            position: Vec2::new(0.0, 0.0),
            angle: 0.0,
        };
    }

    pub fn collision(
        &mut self,
        other: &Collider,
    ) -> MTV {
        self.first_layer.transform(self.position, self.angle);
        if self.first_layer.collision(
            &other.first_layer,
        ) {
            self.second_layer.transform(self.position, self.angle);
            return self.second_layer.collision(
                &other.second_layer, 
            );
        }

        return MTV {
            magnitude: 0.0,
            direction: Vec2::ZERO,
        };
    }

    /// ---- Change Collider Data ----
    /// set new polygon bb, recalculate everything
    pub fn set_bounding_box(&mut self, bb: Vec<Vec2>) {
        self.first_layer = AABB::new(Collider::fit_aabb(&bb));
        self.second_layer = COBB::new(bb);
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = (angle % 360.0 + 360.0) % 360.0;
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    /// todo: this going to fuck things up, dam caves
    fn convex_polygon_decomposition(
        _polygon: Vec<Vec2>,
    ) {
        log::debug!("convex_polygon_decomposition not implemented yet");
    }

    /// Rotating Calipers Method
    fn fit_aabb(polygon: &Vec<Vec2>) -> [Vec2; 4] {
        // Gift Wrapping (Jarvis March) -> hull for calipers
        let mut hull: Vec<Vec2>;
        let n = polygon.len();
        
        // There must be at least 3 polygon to form a hull.
        if n < 3 {
            hull = polygon.to_vec();
        } else {
            // - based off https://github.com/WillKirkmanM/gift-wrapping/tree/master
            // - hopefully works
            hull = Vec::new();

            // Step 1: Find the leftmost point (min x).
            // If x is same, pick the one with min y.
            let mut l = 0;
            for i in 1..n {
                if polygon[i].x < polygon[l].x || (polygon[i].x == polygon[l].x && polygon[i].y < polygon[l].y) {
                    l = i;
                }
            }

            // Start from the leftmost point
            let mut p = l;
            let mut q; 

            loop {
                hull.push(polygon[p]);

                // Step 2: Search for a point 'q' such that orientation(p, q, x) 
                // is counter-clockwise for all other polygon 'x'.
                
                // Initialize q as the next point in the list to start comparisons
                q = (p + 1) % n;

                for i in 0..n {
                    // If i is more counter-clockwise than current q, then i is a better candidate.
                    let (a,b,c) = (polygon[p], polygon[i], polygon[q]);
                    if (b-a).perp_dot(c-a) > 0.0 {
                        q = i;
                    }
                }

                // Set p as q for the next iteration
                p = q;

                // Step 3: Stop if we have returned to the starting point
                if p == l {
                    break;
                }
            }
        }

        // Calipers
        let h = hull.len();
        if h < 3 {
            return [
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            ];
        }

        let get_edge = |i: usize| -> Vec2 {
            if i == n - 1 {
                return polygon[0] - polygon[i];
            } else {
                return polygon[i+1] - polygon[i];
            }
        };

        let mut best_area = f32::MAX;
        let mut best_corners = [Vec2::ZERO; 4];

        for i in 0..h {
            let edge = get_edge(i);
            if edge.length_squared() < 1e-12 {
                continue;
            }
            let u = edge.normalize();
            let v = Vec2::new(-u.y, u.x); // perpendicular

            let mut min_u = f32::MAX;
            let mut max_u = f32::MIN;
            let mut min_v = f32::MAX;
            let mut max_v = f32::MIN;

            for &pt in &hull {
                let pu = pt.dot(u);
                let pv = pt.dot(v);
                min_u = min_u.min(pu);
                max_u = max_u.max(pu);
                min_v = min_v.min(pv);
                max_v = max_v.max(pv);
            }

            let area = (max_u - min_u) * (max_v - min_v);
            if area < best_area {
                best_area = area;
                // Reconstruct the 4 corners in world space
                best_corners = [
                    u * min_u + v * min_v,
                    u * max_u + v * min_v,
                    u * max_u + v * max_v,
                    u * min_u + v * max_v,
                ];
            }
        }

        // final tight fit
        return best_corners;
    }
}
