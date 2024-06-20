use rustitude_core::assert_is_close;
use rustitude_core::prelude::*;
use rustitude_core::utils::*;

#[test]
fn test_masses() {
    let event = generate_test_event();
    assert_is_close!(event.beam_p4.m(), 0.0);
    assert_is_close!(event.recoil_p4.m(), 0.9382720);
    assert_is_close!(event.daughter_p4s[0].m(), 0.497611, 1e-4);
    assert_is_close!(event.daughter_p4s[1].m(), 0.497611, 1e-4);
    let resonance: FourMomentum = event.daughter_p4s.iter().sum();
    assert_is_close!(resonance.m(), 1.3742740);
}

#[test]
fn test_activation() -> Result<(), RustitudeError> {
    let event = generate_test_event();
    let dataset = Dataset::new(vec![event]);
    let model = Model::new(vec![
        scalar("a") + scalar("b"),
        scalar("c") * scalar("d") + scalar("e"),
    ]);
    let mut manager = Manager::new(&model, &dataset)?;
    // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
    assert_is_close!(
        manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
        1440121.0
    );
    // |1|^2 = 1
    manager.deactivate_all();
    manager.activate("a");
    for amp in manager.model.amplitudes.iter() {
        println!("{}", amp);
    }
    assert_is_close!(manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0], 1.0);
    // |1 + 10|^2 = 121
    manager.activate("b");
    for amp in manager.model.amplitudes.iter() {
        println!("{}", amp);
    }
    assert_is_close!(
        manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
        121.0
    );
    // |1 + 10|^2 + |100|^2 = 121 + 10000 = 10121
    manager.activate("c");
    for amp in manager.model.amplitudes.iter() {
        println!("{}", amp);
    }
    assert_is_close!(
        manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
        10121.0
    );
    // |1 + 10|^2 + |2|^2 = 121 + 4 = 125
    manager.deactivate("c");
    manager.deactivate("c"); // this shouldn't cause problems
    manager.activate("d");
    for amp in manager.model.amplitudes.iter() {
        println!("{}", amp);
    }
    assert_is_close!(
        manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
        125.0
    );
    manager.activate_all();
    // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
    assert_is_close!(
        manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
        1440121.0
    );
    Ok(())
}
