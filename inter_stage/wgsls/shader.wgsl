struct OurVertexShaderOutput {
  // 在顶点着色器中，@builtin(position) 是 GPU 绘制三角形/线/点所需的输出。
  // 在片段着色器中，@builtin(position) 是一个输入。它是片段着色器当前被要求计算颜色的像素坐标。
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>,
}

@vertex
fn v_main(@builtin(vertex_index) index: u32) -> OurVertexShaderOutput {
    var output: OurVertexShaderOutput;
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(0.5, 0.5),
        vec2<f32>(-0.7, -0.5),
        vec2<f32>(0.5, -0.5),
        vec2<f32>(0.5, 0.5),
        vec2<f32>(-0.7, 0.5),
        vec2<f32>(-0.7, -0.5)
    );
    var color = array<vec4<f32>, 6>(
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
        vec4<f32>(0.7216, 0.9451, 0.9294, 1.0),
    );
    output.position = vec4<f32>(pos[index], 0.0, 1.0);
    output.color = color[index];
    return output;
}

// @fragment
// fn f_main(input: OurVertexShaderOutput) -> @location(0) vec4<f32> {
//     return input.color;
// }

// 与上面的代码等价
@fragment
fn f_main(@location(0) color: vec4<f32>) -> @location(0) vec4f {
    return color;
}