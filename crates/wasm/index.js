import init, {
  render,
  set_height,
  set_width,
  add_svg,
  transform_color,
  transform_stroke,
  transform_move,
  transform_position,
} from "./pkg/rusvid_wasm.js";

function delay(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

init().then(() => {
  const WIDTH = 500;
  const HEIGHT = 500;

  set_width(WIDTH);
  set_height(HEIGHT);

  let app = document.getElementById("app");
  app.width = WIDTH;
  app.height = HEIGHT;
  let ctx = app.getContext("2d");
  ctx.imageSmoothingEnabled = false;

  function init_svg() {
    // 1   x y -> Move
    // 2   x y -> Line
    // 255     -> Close
    add_svg(
      new Uint32Array([0, 100, 100, 1, 150, 100, 1, 120, 150, 255]),
      new Uint8ClampedArray([255, 255, 255, 255])
    );

    transform_stroke(new Uint8ClampedArray([255, 0, 0, 255]), 2.0);
  }

  function re_render() {
    let pixels = render();

    const image = new ImageData(pixels, WIDTH, HEIGHT);
    ctx.putImageData(image, 0, 0);
  }

  function transform() {
    console.log("called");

    const c_r = document.getElementById("c_r").value;
    const c_g = document.getElementById("c_g").value;
    const c_b = document.getElementById("c_b").value;

    console.log({ red: c_r, green: c_g, blue: c_b });

    transform_color(new Uint8ClampedArray([c_r, c_g, c_b, 255]));

    re_render();
  }

  init_svg();
  transform();

  async function animation() {
    let x = 0;
    let op_x = 1;
    let y = 0;
    let op_y = 1;

    for (let r = 0; r < 255; r += 10) {
      for (let g = 0; g < 255; g += 10) {
        for (let b = 0; b < 255; b += 10) {
          document.getElementById("c_r").value = r;
          document.getElementById("c_g").value = g;
          document.getElementById("c_b").value = b;

          transform_position(x, y);

          y += op_y * 1.25;
          if (y > 450) {
            op_y = -1;
          } else if (y < 0) {
            op_y = 1;
          }

          x += op_x * 1.75;
          if (x > 450) {
            op_x = -1;
          } else if (x < 0) {
            op_x = 1;
          }

          transform();
          await delay(2);
        }
      }
    }
  }

  document.getElementById("b_render").addEventListener("click", re_render);
  document.getElementById("b_animation").addEventListener("click", animation);
  document.getElementById("c_r").addEventListener("input", transform);
  document.getElementById("c_g").addEventListener("input", transform);
  document.getElementById("c_b").addEventListener("input", transform);
});
