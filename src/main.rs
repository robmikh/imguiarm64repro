use imgui_sys::{
    igSetCurrentContext, igSetNextWindowPos, ImGuiCond_FirstUseEver, ImGuiContext, ImVec2,
};

fn main() {
    // Dummy ImGuiContext
    let mut bytes = vec![0u8; 1025];
    let context = bytes.as_mut_ptr() as *mut ImGuiContext;
    unsafe { igSetCurrentContext(context) };

    // When you step through igSetNextWindowPos and follow it to the C++ side, you can
    // see that pos, cond, and pivot have incorrect values. It may or not trigger the
    // assert in ImGui::SetNextWindowPos depending on what cond becomes. On my machine,
    // cond becomes 1025 (seemingly from vec size above) and triggers the assert.
    let pos = ImVec2 { x: 200.0, y: 300.0 };
    let cond = ImGuiCond_FirstUseEver; // 4
    let pivot = ImVec2::zero();
    unsafe { igSetNextWindowPos(pos, cond as i32, pivot) };
}
