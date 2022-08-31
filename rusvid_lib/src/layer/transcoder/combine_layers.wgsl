fn combine_layers(c1: vec4<u32>, c2: vec4<u32>) -> vec4<u32> {
  var new_c = c1;

  if (c1.a == c2.a && c1.a == 0u) || (c2.a == 0u) { // current color and/or new color are/is fully transparent
    // do nothing
  } else if (c1.a == 0u) || (c1.a == c2.a && c1.a == 255u) { // old color is fully transparent or both colors are fully visible
    new_c = c2;
  } // else {
    // let const_number = f64(255);
    
    // convert colors to 0..1
    // var bg_r = f64(new_c.r) / const_number;
    // var bg_g = f64(new_c.g) / const_number;
    // var bg_b = f64(new_c.b) / const_number;
    // var bg_a = f64(new_c.a) / const_number;

    // convert colors to 0..1
    // var fg_r = f64(c2.r) / const_number;
    // var fg_g = f64(c2.g) / const_number;
    // var fg_b = f64(c2.b) / const_number;
    // var fg_a = f64(c2.a) / const_number;
  
    // var mix_a = f64(0);
    // var mix_r = f64(0);
    // var mix_g = f64(0);
    // var mix_b = f64(0);

    // let mix_a = f64(1) - (f64(1) - fg_a) * (f64(1) - bg_a);
    // let mix_r = fg_r * fg_a / mix_a + bg_r * bg_a * (f64(1) - fg_a) / mix_a;
    // let mix_g = fg_g * fg_a / mix_a + bg_g * bg_a * (f64(1) - fg_a) / mix_a;
    // let mix_b = fg_b * fg_a / mix_a + bg_b * bg_a * (f64(1) - fg_a) / mix_a;

    // new_c.r = u32(mix_r * const_number);
    // new_c.g = u32(mix_g * const_number);
    // new_c.b = u32(mix_b * const_number);
    // new_c.a = u32(mix_a * const_number);
  // }

  return new_c;
}

@group(0) @binding(0) var layerOne: texture_2d<u32>;
@group(0) @binding(1) var layerTwo: texture_2d<u32>;
@group(0) @binding(2) var layerDst: texture_storage_2d<rgba8uint, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_invocation_id : vec3<u32>) {
  let dimensions = textureDimensions(layerOne);
  let coords = vec2<i32>(global_invocation_id.xy);

  if(coords.x >= dimensions.x || coords.y >= dimensions.y) {
    return;
  }

  let c1 = textureLoad(layerOne, coords.xy, 0);
  let c2 = textureLoad(layerTwo, coords.xy, 0);

  textureStore(layerDst, coords.xy, combine_layers(c1, c2));
}
