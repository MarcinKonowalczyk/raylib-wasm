# raylib-wasm
- Library lets you run your raylib games in your browser and on your machine with NO CHANGES in your code.

- We don't use any emscripten's and shit, only pure Rust and pure JavaScript, no dependencies (you only need to have wasm and raylib installed).

- You just need to setup your project properly and start the game development!

- You can see a great example of using this library here: <https://github.com/rakivo/rust-raylib-hotreload-wasm-template>. This is a template, so you can start a new repo. with it.

# Cons
> Of course not all raylib functions are supported in browser atm, but if anyone is interested in this library, you can make a pull request, so I can see if I need to continue work on this peace of Software.

# A process of porting a function from native to wasm
> First, you need to check, if the function any structs, or it should be reimplemented manually in `JS`, because it can't work properly in browser as it does natively, do that:
- Go to `fns.rs`, find original function and add `_` to the end of its name if `feature="web"`, example:
```rs
extern "C" {
    #[cfg(not(feature = "web"))]
    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);

    #[cfg(feature = "web")]
    pub fn SetTargetFPS_(fps: ::std::os::raw::c_int);
}
```

- If the function accepts any structs, you need to pass these structs via their address in memory, example:
```rs
pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    DrawRectangleRec_(std::ptr::addr_of!(rec), std::ptr::addr_of!(color));
}
```
> Make this change in `web_fns.rs`

- Then, go to `raylib.js`, find the `WebAssembly.instantiateStreaming(fetch(WASM_PATH), {...` line, and implement your function in `JS` there, pretty self-explanatory from now on.
