use std::i32;

use rand::Rng;

use crate::ans::Slot;

#[test]
fn slot_check() {
    let slot = Slot::new(2);
    for _ in 0..5 {
        assert!(slot.allocate_slot(i32::MIN).is_none());
        assert!(slot.allocate_slot(-1).is_none());
        assert!(slot.allocate_slot(0).is_none());
        assert!(slot.allocate_slot(1).is_some());
        assert!(slot.allocate_slot(2).is_some());
        assert!(slot.allocate_slot(3).is_none());
        assert!(slot.allocate_slot(i32::MAX).is_none());
    }
}

#[test]
fn slot_recycle() {
    let slot = Slot::new(2);

    let res1 = slot.allocate_slot(1).unwrap();
    assert!(slot.allocate_slot(2).is_none());
    assert!(slot.allocate_slot(1).is_some());

    drop(res1);
    assert!(slot.allocate_slot(2).is_some());
}

#[test]
fn random_test() {
    let mut rng = rand::thread_rng();

    const SLOTS: i32 = 100;
    let slot = Slot::new(SLOTS);
    let mut guards = Vec::with_capacity(10);
    let mut current = 0;

    for _ in 0..1000000 {
        let random_slots = rng.gen_range(1..20);
        let should_some = slot.number_slot() >= random_slots;
        let res = slot.allocate_slot(random_slots);
        match res {
            Some(res) => {
                assert!(should_some);
                assert!(random_slots + current <= SLOTS);
                assert_eq!(res.number_slot(), random_slots);

                current += random_slots;
                guards.push(res);
            }
            None => {
                assert!(!should_some);
                assert!(random_slots + current > SLOTS);
                assert!(slot.number_slot() < random_slots);
                assert!(!guards.is_empty());

                let res = guards.pop().unwrap();
                current -= res.number_slot();
                assert!(current >= 0);
            }
        }
    }
}
