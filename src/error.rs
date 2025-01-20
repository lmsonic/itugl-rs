use std::ffi::CStr;

// #[cfg(debug_assertion)]
pub fn check_gl_error() {
    let error_code = unsafe { gl::GetError() };

    if error_code != gl::NO_ERROR {
        let error_string = match error_code {
            gl::INVALID_ENUM => "Invalid Enum",
            gl::INVALID_VALUE => "Invalid Value",
            gl::INVALID_OPERATION => "Invalid Operation",
            gl::STACK_OVERFLOW => "Stack Overflow",
            gl::STACK_UNDERFLOW => "Stack Underflow",
            gl::OUT_OF_MEMORY => "Out of Memory",
            _ => "Unknown Error",
        };

        log::error!(
            "OpenGL Error at FILE {} LINE {}: {}",
            file!(),
            line!(),
            error_string
        );
    }
}

// #[cfg(not(debug_assertion))]
// #[allow(clippy::missing_const_for_fn)]
// pub fn check_gl_error() {}

fn print_err(s: &str) {
    log::error!("{}", s); // might have been a println!
}
fn print_warn(s: &str) {
    log::warn!("{}", s); // might have been a println!
}
fn print_info(s: &str) {
    log::info!("{}", s); // might have been a println!
}
fn print_debug(s: &str) {
    log::info!("{}", s); // might have been a println!
}
pub extern "system" fn debug_callback(
    debug_source: u32,
    debug_type: u32,
    debug_id: u32,
    debug_severity: u32,
    _debug_length: i32,
    debug_message: *const i8,
    _user_param: *mut std::ffi::c_void,
) {
    let raw_string_ptr: *const i8 = debug_message;
    let c_str = unsafe { CStr::from_ptr(raw_string_ptr) };
    let debug_string = c_str.to_str().expect("Failed to convert CStr to str");
    let type_str = match debug_type {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
        gl::DEBUG_TYPE_OTHER => "Other",
        _ => "Unknown",
    };
    let print_fn = match debug_type {
        gl::DEBUG_TYPE_ERROR | gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => print_err,
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR | gl::DEBUG_TYPE_PORTABILITY => print_warn,
        gl::DEBUG_TYPE_MARKER | gl::DEBUG_TYPE_PUSH_GROUP | gl::DEBUG_TYPE_POP_GROUP => print_debug,
        _ => print_info,
    };
    let source_str = match debug_source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        _ => "Unknown",
    };

    let severity_str = match debug_severity {
        gl::DEBUG_SEVERITY_HIGH => "High",
        gl::DEBUG_SEVERITY_MEDIUM => "Medium",
        gl::DEBUG_SEVERITY_LOW => "Low",
        gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
        _ => "Unknown",
    };
    print_fn(format!(
        "DEBUG CALLBACK!\n\tsource = {source_str},\n\ttype = {type_str},\n\tid = {debug_id},\n\tseverity = {severity_str},\n\tmessage = {debug_string:?}\n").as_str());
}
