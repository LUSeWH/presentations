use ninth_lesson;
use crate::constructor::start;

mod constructor;

#[test]
#[should_panic]
fn test_panic() {
    start();
    ninth_lesson::test_panic_lib(1);
}

#[test]
#[should_panic]
fn test_panic0() {
    start();
    ninth_lesson::test_panic_lib(1);
}

