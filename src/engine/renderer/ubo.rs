// use super::bind_group;
// use super::math::any_as_u8_slice;
//
// pub struct UBO {
//     pub buffer: wgpu::Buffer,
//     pub bind_groups: Vec<wgpu::BindGroup>,
//     alignment: u64,
// }
//
// impl UBO {
//     pub fn new (
//         device: &wgpu::Device, 
//         object_count: usize, 
//         layout: bind_group::LayoutInfo,
//     ) -> Self {
//         // aligns data into chunks in the gpu
//         let alignment = glm::max(
//             device.limits().min_uniform_buffer_offset_alignment as u32,
//             std::mem::size_of::<glm::Mat4>() as u32,
//         ) as u64;
//
//         // creates uniform buffer
//         let buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("ubo"),
//             size: object_count as u64 * alignment,
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });
//
//         let mut bind_groups: Vec<wgpu::BindGroup> = Vec::new();
//         for i in 0..object_count {
//             // adds bind groups 
//             let mut bind_group = bind_group::ResourceBuilder::new(device, &layout)
//                 .buffer(&buffer).unwrap()
//                 .build(&format!("ubo-{i}").to_string()).unwrap();
//             bind_groups.push(bind_group);
//         }
//
//         return Self {
//             buffer,
//             bind_groups,
//             alignment,
//         };
//     }
//
//     pub fn upload (self, i: u64, matrix: &glm::Mat4, queue: &wgpu::Queue) {
//         let offset = i * self.alignment;
//         let data = unsafe { any_as_u8_slice(matrix) };
//         queue.write_buffer(&self.buffer, offset, data);
//     }
// }
