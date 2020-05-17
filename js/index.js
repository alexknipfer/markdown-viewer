const wasm = import("../pkg/index.js");

wasm.then((module) => {
  const button = document.getElementById("parse");

  button.addEventListener("click", () => {
    const input = document.getElementById("markdown");
    module.parse_input(input.value);
  });
});
