# raylib-wasm
- Library lets you run your raylib games in your browser and on your machine with NO CHANGES in your code.

- We don't use any emscripten's and shit, only pure Rust and pure JavaScript, no dependencies (you only need to have wasm and raylib installed).

- You just need to setup your project properly and start your game development!

- You can see a great example of using this library here: <https://github.com/rakivo/rust-raylib-hotreload-wasm-template>. This is a template, so you can start a new repo. with it.

# Cons
> Of course not all raylib functions are supported in browser atm, but if anyone is interested in this library, you can make a pull request, so I can see if I need to continue work on this peace of Software.

# A process of porting a function from native to JS
> First, you need to check, if the function any structs, or it should be reimplemented manually in `JS`, because it can't work properly in browser as it does natively, do that:
- Go to `fns.rs`, find original function and add `_` to the end of its name if `feature="web"`, example:
```rs
extern "C" {
    #[cfg(not(feature = "web"))]
    pub fn ClearBackground(color: Color);

    #[cfg(feature = "web")]
    pub fn ClearBackground_(color: Color);
}
```

- If the function accepts any structs, you need to pass these structs via their address in memory, do that in `web_fns.rs` example:
```rs
pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    DrawRectangleRec_(std::ptr::addr_of!(rec), std::ptr::addr_of!(color));
}
```

- Then, go to `raylib.js`, find the `WebAssembly.instantiateStreaming(fetch(WASM_PATH), {...` line, and implement your function in `JS` there. But keep in mind, if you added an `_` to the end of your function in Rust, you need to add it to the end in `JS` as well, example:
```js
WebAssembly.instantiateStreaming(fetch(WASM_PATH), {
    "env": make_environment({
        ...

        DrawRectangleRec_: (rec_ptr, color_ptr) => {
            const buffer = wf.memory.buffer;
            const [x, y, w, h] = new Float32Array(buffer, rec_ptr, 4);
            const color = getColorFromMemory(buffer, color_ptr);
            ctx.fillStyle = color;
            ctx.fillRect(x, y, w, h);
        },

        ...
    })
}
```
