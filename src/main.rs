use imgui_sys::{
    igSetCurrentContext, igSetNextWindowPos, ImGuiCond_FirstUseEver, ImGuiContext, ImVec2,
};

fn main() {
    // Dummy ImGuiContext
    let mut bytes = vec![0u8; std::mem::size_of::<ImGuiContext>()];
    let context = bytes.as_mut_ptr() as *mut ImGuiContext;
    unsafe { igSetCurrentContext(context) };

    // This will AV because our dummy context isn't set up properly, but you 
    // can see that pos, cond, and pivot have incorrect values on the C++ side.
    let pos = ImVec2::zero();
    let cond = ImGuiCond_FirstUseEver; // 4
    let pivot = ImVec2::zero();
    unsafe { igSetNextWindowPos(pos, cond as i32, pivot) };
}
