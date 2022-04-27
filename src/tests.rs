#[cfg(test)]
use crate::{print, println};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu::*;

    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
pub fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("OK!");
}