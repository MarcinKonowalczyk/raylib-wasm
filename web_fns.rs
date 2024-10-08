use crate::{Rectangle, KeyboardKey, Vector2, Color};

use std::ptr::addr_of;

extern {
    pub fn DrawLine_(_: i32, _: i32, _: i32, _: i32, _: *const Color,);
    pub fn DrawRectangleV_(_: *const Vector2, _: *const Vector2, _: *const Color);
    pub fn DrawRectangleRec_(_: *const Rectangle, _: *const Color);
    pub fn DrawRectangle_(_: i32, _: i32, _: i32, _: i32, _: *const Color);
    pub fn SetTargetFPS(_: i32);
    pub fn InitWindow(_: i32, _: i32, _: *const i8);
    pub fn BeginDrawing();
    pub fn ClearBackground_(_: *const Color);
    pub fn GetFrameTime() -> f32;
    pub fn DrawFPS(_: i32, _: i32);
    pub fn DrawText_(_: *const i8, _: i32, _: i32, _: i32, _: *const Color);
    pub fn EndDrawing();
    pub fn SetExitKey(_: KeyboardKey);
    pub fn CloseWindow();
    pub fn IsKeyDown(_: KeyboardKey) -> bool;
    pub fn GetFPS() -> i32;
}

// This functions are mandotory because without them the whole thing won't work.
// You can't just pass a structure into a function and get it fields in JS via memory buffer,
// To this thing to work we've got create this layer of abstraction.

pub unsafe fn DrawLine(sx: i32, sy: i32, ex: i32, ey: i32, color: Color) {
    DrawLine_(sx, sy, ex, ey, addr_of!(color))
}

pub unsafe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color) {
    DrawRectangleV_(addr_of!(position), addr_of!(size), addr_of!(color));
}

pub unsafe fn WindowShouldClose() -> bool {
    false
}

pub unsafe fn ClearBackground(color: Color) {
    ClearBackground_(addr_of!(color));
}

pub unsafe fn DrawText(text: *const i8, x: i32, y: i32, size: i32, color: Color) {
    DrawText_(text, x, y, size, addr_of!(color));
}

pub unsafe fn DrawRectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    DrawRectangle_(x, y, w, h, addr_of!(color));
}

pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    DrawRectangleRec_(addr_of!(rec), addr_of!(color));
}
