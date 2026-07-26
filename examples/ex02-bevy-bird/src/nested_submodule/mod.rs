// Register the files inside the auth/ directory
pub mod nested_bye;
pub mod nested_hello;

pub fn handle_auth() {
    // Calling code internally within the auth module
    nested_hello::hello_nested_submodule();
}
