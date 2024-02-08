use super::object::Object;
#[repr(u32)]
pub enum Target {
    // Vertex Buffer Object
    ArrayBuffer = gl::ARRAY_BUFFER,
    // Element Buffer Object
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    // TODO: There are more types, add them when they are supported
}
#[repr(u32)]
enum Usage {
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
}
pub trait BufferObject: Object {
    fn target() -> Target;
    fn allocate_data(&self);
    fn update_data(&self);
    fn unbind(&self);
}
