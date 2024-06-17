struct VertexOutput {
  // 表示 clip_position 是内建变量，数据是顶点的位置
  @builtin(position) clip_position: vec4f,
  @location(0) position: vec2f,
}

@vertex
fn v_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
  var out: VertexOutput;
  let x = f32(1-i32(vertex_index)) * 0.5;
  let y = f32(i32(vertex_index & u32(1)) * 2 - 1) * 0.5;
  out.clip_position = vec4f(x, y, 0.0, 1.0);
  out.position = vec2f(x, y);
  return out;
}

// @location 通常用来指定顶点缓冲区相关的顶点数据
// @location 0 表示你GPU显存中标记为0的顶点缓冲区中顶点数据。
@fragment
fn f_main(in:VertexOutput) -> @location(0) vec4f {
  return vec4f(in.position, 0.0, 1.0);
}