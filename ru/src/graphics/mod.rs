pub mod binding;
pub mod gpu;
pub mod pipeline;
pub mod shader;
pub mod texture;
// fn get_frustum_vertices(camera: &Camera, perspective: &CameraPerspective) -> [Vec3; 8] {
//     let cam_position = camera.position;
//     let cam_forward = normalize_vec3(camera.forward);
//     let cam_right = normalize_vec3(camera.right);
//     let cam_up = normalize_vec3(camera.up);

//     // Near and far clip distances
//     let near_clip = perspective.near_clip;
//     let far_clip = perspective.far_clip;

//     // Calculate the height and width of the near and far planes
//     let tan_half_fov = (perspective.fov.to_radians() / 2.0).tan();
//     let near_height = 2.0 * tan_half_fov * near_clip;
//     let near_width = near_height * perspective.aspect_ratio;
//     let far_height = 2.0 * tan_half_fov * far_clip;
//     let far_width = far_height * perspective.aspect_ratio;

//     // Calculate the centers of the near and far planes
//     let near_center = vec3_add(cam_position, scale_vec3(cam_forward, near_clip));
//     let far_center = vec3_add(cam_position, scale_vec3(cam_forward, far_clip));

//     // Near plane corners
//     let near_top_left = vec3_sub(
//         vec3_add(near_center, scale_vec3(cam_up, near_height / 2.0)),
//         scale_vec3(cam_right, near_width / 2.0),
//     );
//     let near_top_right = vec3_add(
//         vec3_add(near_center, scale_vec3(cam_up, near_height / 2.0)),
//         scale_vec3(cam_right, near_width / 2.0),
//     );
//     let near_bottom_left = vec3_sub(
//         vec3_sub(near_center, scale_vec3(cam_up, near_height / 2.0)),
//         scale_vec3(cam_right, near_width / 2.0),
//     );
//     let near_bottom_right = vec3_add(
//         vec3_sub(near_center, scale_vec3(cam_up, near_height / 2.0)),
//         scale_vec3(cam_right, near_width / 2.0),
//     );

//     // Far plane corners
//     let far_top_left = vec3_sub(
//         vec3_add(far_center, scale_vec3(cam_up, far_height / 2.0)),
//         scale_vec3(cam_right, far_width / 2.0),
//     );
//     let far_top_right = vec3_add(
//         vec3_add(far_center, scale_vec3(cam_up, far_height / 2.0)),
//         scale_vec3(cam_right, far_width / 2.0),
//     );
//     let far_bottom_left = vec3_sub(
//         vec3_sub(far_center, scale_vec3(cam_up, far_height / 2.0)),
//         scale_vec3(cam_right, far_width / 2.0),
//     );
//     let far_bottom_right = vec3_add(
//         vec3_sub(far_center, scale_vec3(cam_up, far_height / 2.0)),
//         scale_vec3(cam_right, far_width / 2.0),
//     );

//     [
//         near_top_left,     // Vertex 0
//         near_top_right,    // Vertex 1
//         near_bottom_right, // Vertex 2
//         near_bottom_left,  // Vertex 3
//         far_top_left,      // Vertex 4
//         far_top_right,     // Vertex 5
//         far_bottom_right,  // Vertex 6
//         far_bottom_left,   // Vertex 7
//     ]
// }
