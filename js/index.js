const wasm = import("../pkg/index.js");

wasm.then((module) => {
  const button = document.getElementById("parse");

  button.addEventListener("click", () => {
    const input = document.getElementById("markdown");
    const output = document.getElementById("output");
    const result = module.parse_input(input.value);
    output.innerHTML = result;
  });
});
