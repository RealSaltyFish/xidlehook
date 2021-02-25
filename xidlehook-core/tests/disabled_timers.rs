use std::{cell::Cell, time::Duration};
use xidlehook_core::{timers::CallbackTimer, Action::*, Xidlehook};

const TEST_UNIT: Duration = Duration::from_millis(50);

#[test]
fn disabled_timers_test() {
    let triggered = Cell::new(0);

    let mut timer = Xidlehook::new(vec![
        CallbackTimer::new(TEST_UNIT * 08, || triggered.set(triggered.get() | 1)),
        CallbackTimer::new(TEST_UNIT * 16, || triggered.set(triggered.get() | 1 << 1)),
        CallbackTimer::new(TEST_UNIT * 04, || triggered.set(triggered.get() | 1 << 2)),
        CallbackTimer::new(TEST_UNIT * 06, || triggered.set(triggered.get() | 1 << 3)),
    ]);

    // Just one good old test round first
    assert_eq!(timer.poll(TEST_UNIT * 00).unwrap(), Sleep(TEST_UNIT * 08));
    assert_eq!(timer.poll(TEST_UNIT * 08).unwrap(), Sleep(TEST_UNIT * 08)); // timer 0
    assert_eq!(timer.poll(TEST_UNIT * 24).unwrap(), Sleep(TEST_UNIT * 04)); // timer 1
    assert_eq!(timer.poll(TEST_UNIT * 28).unwrap(), Sleep(TEST_UNIT * 06)); // timer 2
    assert_eq!(timer.poll(TEST_UNIT * 34).unwrap(), Sleep(TEST_UNIT * 08)); // timer 3
    assert_eq!(triggered.get(), 0b1111);

    // Now disable the first timer and reset
    timer.timers_mut().unwrap()[0].disabled = true;
    triggered.set(0);

    // Make sure first timer is ignored
    assert_eq!(timer.poll(TEST_UNIT * 00).unwrap(), Sleep(TEST_UNIT * 08));
    assert_eq!(timer.poll(TEST_UNIT * 08).unwrap(), Sleep(TEST_UNIT * 08)); // ~timer 0~
    assert_eq!(triggered.get(), 0b0000);
    assert_eq!(timer.poll(TEST_UNIT * 16).unwrap(), Sleep(TEST_UNIT * 04)); // timer 1
    assert_eq!(timer.poll(TEST_UNIT * 20).unwrap(), Sleep(TEST_UNIT * 06)); // timer 2
    assert_eq!(timer.poll(TEST_UNIT * 26).unwrap(), Sleep(TEST_UNIT * 08)); // timer 3
    assert_eq!(triggered.get(), 0b1110);

    // Now disable a timer in the middle and reset
    timer.timers_mut().unwrap()[2].disabled = true;
    triggered.set(0);

    // Make sure first timer is ignored
    assert_eq!(timer.poll(TEST_UNIT * 00).unwrap(), Sleep(TEST_UNIT * 08));
    assert_eq!(timer.poll(TEST_UNIT * 08).unwrap(), Sleep(TEST_UNIT * 08)); // ~timer 0~
    assert_eq!(triggered.get(), 0b0000);
    assert_eq!(timer.poll(TEST_UNIT * 16).unwrap(), Sleep(TEST_UNIT * 04)); // timer 1
    assert_eq!(timer.poll(TEST_UNIT * 20).unwrap(), Sleep(TEST_UNIT * 02)); // ~timer 2~
    assert_eq!(timer.poll(TEST_UNIT * 22).unwrap(), Sleep(TEST_UNIT * 08)); // timer 3
    assert_eq!(triggered.get(), 0b1010);

    // Now disable all remaining timers and reset
    timer.timers_mut().unwrap()[1].disabled = true;
    timer.timers_mut().unwrap()[3].disabled = true;
    triggered.set(0);

    // Make sure xidlehook doesn't panic
    assert_eq!(timer.poll(TEST_UNIT * 00).unwrap(), Forever);
    assert_eq!(timer.poll(TEST_UNIT * 100_000).unwrap(), Forever);
    assert_eq!(triggered.get(), 0b0000);

    // ... and make sure re-enabling is fine
    timer.timers_mut().unwrap()[2].disabled = false;

    assert_eq!(timer.poll(TEST_UNIT * 00).unwrap(), Sleep(TEST_UNIT * 04));
    assert_eq!(triggered.get(), 0b0000);
    assert_eq!(timer.poll(TEST_UNIT * 04).unwrap(), Sleep(TEST_UNIT * 04));
    assert_eq!(triggered.get(), 0b0100);
}
