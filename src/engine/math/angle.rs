use glam::Quat;

pub fn quat_to_xy(q: Quat) -> f32 {
    // Z-up convention: yaw = rotation in XY plane
    let yaw = f32::atan2(
        2.0 * (q.w * q.z + q.x * q.y),
        1.0 - 2.0 * (q.y * q.y + q.z * q.z),
    );

    let deg = yaw.to_degrees();
    return (deg + 360.0) % 360.0;
}
