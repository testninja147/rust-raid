/// ! to run, execute `cargo run --bin assembly`
use std::arch::asm;

/// Inline Assembly
/// ---------------
///
/// Rust supports adding inline assembly when we need to add assembly inside our
/// rust code. There are `asm!` and `global_asm!` macros that can be used to add
/// assembly code. Assembly are supported on most popular architectures such as
/// x86 and X86-64, ARM, AARCH64, RISC-V, etc.
///
/// Using assembly needs rust code block  to be unsafe since we are directly
/// working with the registers.
///
/// IF we are doing very low level programs such as kernel development, we might
/// need to work with assembly programming where it is useful in this case.
///
/// to learn more about inline assembly, please checkout the following:
/// https://doc.rust-lang.org/reference/inline-assembly.html
fn main() {
    println!("⛔ Inline assembly ⛔");

    // a basic program to multiply and add  2 numbers in assembly
    #[allow(asm_sub_register)]
    unsafe {
        let x: u64 = 5;
        let y: u64 = 10;
        let mut sum = 0;
        let mut prod = 0;
        asm!(
        "mov {sum}, {x}",   // set sum = x
        "add {sum}, {y}",   // sum = sum + y

        "mov {prod}, {y}",  // set prod = y
        "shl {prod}, 2",    // left shift the value 2 times (i.e. multiply by 4) 1010 -> 1010[0][0] -> 101000
        "add {prod}, {y}",  // add y 1 more time [i.e.: (y + 4 + y) = y * 5]
        sum=inout(reg) sum,
        x=in(reg) x,
        y=in(reg) y,
        prod = inout(reg) prod,
        );
        println!("The sum is: {sum}"); // The value of x is: 15
        println!("The product is: {prod}"); // The value of x is: 15
    }
}
