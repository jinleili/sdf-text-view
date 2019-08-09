use nalgebra_glm as glm;

#[allow(dead_code)]
pub fn default_mvp(sc_desc: &wgpu::SwapChainDescriptor) -> [[f32; 4]; 4] {
    //
    let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(75.0));
    let p_matrix: glm::TMat4<f32> =
        glm::perspective_fov(radian[0], sc_desc.width as f32, sc_desc.height as f32, 0.01, 100.0);
    //        let mut  p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -2.12));
    // vm_matrix = glm::scale(&vm_matrix, &glm::vec3(1.0, 2.0, 2.0));
    // vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(0.0, 1.0, 0.0));
    (p_matrix * vm_matrix).into()
}

#[allow(dead_code)]
pub fn ortho_default_mvp() -> [[f32; 4]; 4] {
    let p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
    let vm_matrix = glm::TMat4::identity();
    (p_matrix * vm_matrix).into()
}

#[allow(dead_code)]
pub fn ortho_pixel_mvp(width: f32, height: f32) -> [[f32; 4]; 4] {
    let p_matrix: glm::TMat4<f32> = ortho_pixel(width, height);
    let vm_matrix = glm::TMat4::identity();
    (p_matrix * vm_matrix).into()
}

#[allow(dead_code)]
pub fn ortho_pixel(width: f32, height: f32) -> glm::TMat4<f32> {
    // 屏幕中心为原点，z轴指向屏幕内为正，向上旋转为正
    // https://nalgebra.org/projections/
    // 在计算机中通常使用的是左手坐标系，而数学中则通常使用右手坐标系。
    // 左手坐标系，z 轴方向指向屏幕内
    glm::ortho(-width / 2.0, width / 2.0, -height / 2.0, height / 2.0, -1000.0, 1000.0)
}

#[allow(dead_code)]
pub fn dot(v1: [f32; 2], v2: [f32; 2]) -> f32 {
    let vec1: glm::TVec2<f32> = v1.into();
    let vec2: glm::TVec2<f32> = v2.into();
    glm::dot(&vec1, &vec2)
}
