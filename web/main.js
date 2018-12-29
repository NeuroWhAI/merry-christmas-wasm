// Fit canvas to page.
let canvas = document.getElementById("screen");
canvas.width = window.innerWidth
canvas.height = window.innerHeight

let wasmImports = {
    env: {
        unsafe_random: function() {
            return Math.random();
        },
        unsafe_log_num: function(num) {
            console.log(num);
        }
    },
};

fetch('main.wasm')
.then(response => response.arrayBuffer())
.then(bytes => WebAssembly.instantiate(bytes, wasmImports))
.then(results => {
    let mod = {};
    let inst = results.instance;
    mod.alloc = inst.exports.alloc;
    mod.initialize = inst.exports.initialize;
    mod.step = inst.exports.step;

    let width = canvas.width;
    let height = canvas.height;

    if (canvas.getContext) {
        var ctx = canvas.getContext('2d');

        let byteSize = width * height * 4;
        var pointer = mod.alloc(byteSize);

        var usub = new Uint8ClampedArray(inst.exports.memory.buffer, pointer, byteSize);
        var img = new ImageData(usub, width, height);

        mod.initialize(width, height);

        function step() {
            mod.step(pointer, width, height);
        
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.putImageData(img, 0, 0);

            window.requestAnimationFrame(step);
        }
        
        window.requestAnimationFrame(step);
    }
})
.catch(console.error);
