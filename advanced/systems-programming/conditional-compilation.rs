/// ! to execute, run `cargo run --bin cc`
/// * Conditional compilation
/// -------------------------
///
/// Rust's conditional compilation helps us enable or disable code blocks based
/// on different conditions such as platform, compiler version, custom configs,
/// etc.
///
/// Some of the common use-case of it are as follows:
/// * Writing platform-specific codes (eg: OS, hardware, etc)
/// * Adding or using optional features
/// * Separating Debug and Release modes
///

fn main() {
    println!("Conditional Compilation");

    // platform-specific code
    {
        /// # Display Platform
        /// This function prints out the platform-specific message when called.
        /// the example uses  a `#[cfg()]` configuration that takes the target
        /// Operating system.
        ///
        /// Instead of targeting specific  line, we can target different
        /// functions, code blocks, etc. so that our code gets compiled based on
        /// the platform we are using.
        fn display_platform() {
            // The following code runs only on windows
            #[cfg(target_os = "windows")]
            println!("⛔ The Platform is Windows ⛔");

            // The following code runs only on linux
            #[cfg(target_os = "linux")]
            println!("⛔ The Platform is Linux ⛔");

            // The following code runs only on Mac OS
            #[cfg(target_os = "macos")]
            println!("⛔ The Platform is Mac OS ⛔");
        }

        display_platform();
    }
}
