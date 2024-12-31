#[cfg(target_os = "macos")]
mod imk;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() -> std::process::ExitCode {
    println!("Fig input method is only supported on macOS");
    std::process::ExitCode
}
