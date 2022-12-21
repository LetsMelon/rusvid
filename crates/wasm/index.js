import init, {
  render,
  set_height,
  set_width,
  add_svg,
} from "./pkg/rusvid_wasm.js";

init().then(() => {
  const WIDTH = 300;
  const HEIGHT = 300;

  set_width(WIDTH);
  set_height(HEIGHT);

  let app = document.getElementById("app");
  app.width = WIDTH;
  app.height = HEIGHT;
  let ctx = app.getContext("2d");

  function call_render() {
    console.log("called");

    const c_r = document.getElementById("c_r").value;
    const c_g = document.getElementById("c_g").value;
    const c_b = document.getElementById("c_b").value;

    console.log({c_r, c_g, c_b});

    // 1   x y -> Move
    // 2   x y -> Line
    // 255     -> Close
    add_svg(
      new Uint32Array([0, 100, 100, 1, 150, 100, 1, 120, 150, 255]),
      new Uint8ClampedArray([c_r, c_g, c_b, 255])
    );

    let pixels = render();

    const image = new ImageData(pixels, WIDTH, HEIGHT);
    ctx.putImageData(image, 0, 0);
  }

  // init_triangle();
  call_render();

  document.getElementById("b_render").addEventListener("click", call_render);
  document.getElementById("c_r").addEventListener('change', call_render);
  document.getElementById("c_g").addEventListener('change', call_render);
  document.getElementById("c_b").addEventListener('change', call_render);
});
