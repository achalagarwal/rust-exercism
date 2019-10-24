// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.
// static lifetime would imply that its there always?
// whats a static variable?

pub fn hello() -> &'static str {
    "Hello, World!"
}
