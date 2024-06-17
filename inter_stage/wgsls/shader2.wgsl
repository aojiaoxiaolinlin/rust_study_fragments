struct OurVertexShaderOutput {
  // 在顶点着色器中，@builtin(position) 是 GPU 绘制三角形/线/点所需的输出。
  // 在片段着色器中，@builtin(position) 是一个输入。它是片源着色器当前被要求计算颜色的像素坐标。
  @builtin(position) position: vec4<f32>,
}

@vertex
fn v_main(@builtin(vertex_index) index: u32) -> OurVertexShaderOutput {
    var output: OurVertexShaderOutput;
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    output.position = vec4<f32>(pos[index], 0.0, 1.0);
    return output;
}

@fragment
fn f_main(input: OurVertexShaderOutput) -> @location(0) vec4<f32> {
    let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let cyan = vec4<f32>(0.0, 1.0, 1.0, 1.0);
    let grid = vec2<f32>(input.position.xy) / 8.0;
    let checker = i32(grid.x + grid.y) % 2 == 1;
    return select(red, cyan, checker);
}