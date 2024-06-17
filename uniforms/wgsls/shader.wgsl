struct OurStruct {
  color: vec4f,
  scale: vec2f,
  offset: vec2f,
}
// uniform 类似于shader全局变量，
@group(0) @binding(0) var<uniform> ourSturct: OurStruct;

@vertex
fn v_main(@builtin(vertex_index) vertexIndex: u32) -> @builtin(position) vec4f {
    var pos = array(
        vec2f(0.0, 0.5),
        vec2f(-0.5, -0.5),
        vec2f(0.5, -0.5)
    );
    return vec4f(pos[vertexIndex] * ourSturct.scale + ourSturct.offset, 0.0, 1.0);
}

@fragment
fn f_main() -> @location(0) vec4f {
  return ourSturct.color;
}