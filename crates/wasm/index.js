import init, {
  render,
  set_height,
  set_width,
  add_svg,
  transform_color,
  transform_stroke,
  transform_move,
  transform_rotate,
  transform_scale,
  calculate_bounding_box,
} from "./pkg/rusvid_wasm.js";

function delay(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

let should_animate = true;

const MOVEMENT_X = 1.25;
const MOVEMENT_Y = 1.75;

class Queue {
  constructor(length) {
    this.data = [];
    this.length = length;
  }

  push(item) {
    this.data.push(item);

    if (this.data.length == this.length + 1) {
      this.data.shift();
    }
  }

  median() {
    const items = this.data.length;
    const sum = this.data.reduce((acc, item) => acc + item, 0);

    return sum / items;
  }
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

  let id = undefined;

  function init_svg() {
    // 1   x y -> Move
    // 2   x y -> Line
    // 255     -> Close
    id = add_svg(
      // new Uint32Array([0, 100, 100, 1, 150, 100, 1, 120, 150, 255]),
      new Int32Array([
        0, 100, 100, 1, 150, 50, 2, 100, 25, 169, 11, 119, -13, 2, 50, 50, 80,
        -13, 30, 11, 255,
      ]),
      new Uint8ClampedArray([255, 255, 255, 255])
    );

    console.log({ id });

    transform_stroke(new Uint8ClampedArray([255, 0, 0, 255]), 2.0);
  }

  function re_render() {
    let pixels = render();

    const image = new ImageData(pixels, WIDTH, HEIGHT);
    ctx.putImageData(image, 0, 0);
  }

  function transform(change_slider = false) {
    console.log("called");

    const c_r = document.getElementById("c_r").value;
    const c_g = document.getElementById("c_g").value;
    const c_b = document.getElementById("c_b").value;

    console.log({ red: c_r, green: c_g, blue: c_b });

    transform_color(new Uint8ClampedArray([c_r, c_g, c_b, 255]));

    if (change_slider) {
      document.getElementById("s_rotate").value = 0;
      document.getElementById("s_scale").value = 1;
    }

    re_render();
  }

  init_svg();
  transform(true);

  async function animation() {
    let op_x = 1;
    let op_y = 1;

    let frames = new Queue(50);

    let fps_p = document.createElement("p");
    document.getElementById("fps_p_show").appendChild(fps_p);
    const node = document.createTextNode("0.00 fps");
    fps_p.appendChild(node);

    for (let r = 0; r < 255; r += 10) {
      for (let g = 0; g < 255; g += 10) {
        for (let b = 0; b < 255; b += 10) {
          if (should_animate) {
            let startTime = new Date();

            document.getElementById("c_r").value = r;
            document.getElementById("c_g").value = g;
            document.getElementById("c_b").value = b;

            transform_move(MOVEMENT_X * op_x, MOVEMENT_Y * op_y);
            let bounding_box = calculate_bounding_box(id);

            let x1 = bounding_box[0];
            let y1 = bounding_box[1];
            let x2 = bounding_box[2];
            let y2 = bounding_box[3];

            if (Math.max(x1, x2) > WIDTH || Math.min(x1, x2) < 0) {
              op_x *= -1;
            }
            if (Math.max(y1, y2) > HEIGHT || Math.min(y1, y2) < 0) {
              op_y *= -1;
            }

            transform();

            let endTime = new Date();
            const timeDiff = endTime - startTime;
            const fps = 1000 / timeDiff;
            frames.push(fps);
            let median = frames.median();
            fps_p.innerText = `${median.toFixed(2)} fps`;

            await delay(2);
          } else {
            fps_p.remove();
            return;
          }
        }
      }
    }

    fps_p.remove();
  }

  const rotate = () => {
    let current_angle = document.getElementById("s_rotate").value;

    transform_rotate(current_angle);
    re_render();
  };

  const scale = () => {
    let current_scale = document.getElementById("s_scale").value;

    transform_scale(current_scale, current_scale);
    re_render();
  };

  document.getElementById("b_render").addEventListener("click", re_render);
  document.getElementById("b_animation").addEventListener("click", animation);
  document
    .getElementById("b_stop")
    .addEventListener("click", () => (should_animate = false));
  document.getElementById("c_r").addEventListener("input", transform);
  document.getElementById("c_g").addEventListener("input", transform);
  document.getElementById("c_b").addEventListener("input", transform);

  document.getElementById("s_rotate").addEventListener("input", rotate);
  document.getElementById("s_scale").addEventListener("input", scale);
});
