mod f64_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    #[test]
    fn test_masses() {
        let event = generate_test_event_f64();
        assert_is_close!(event.beam_p4.m(), 0.0, f64);
        assert_is_close!(event.recoil_p4.m(), 0.938_272_0, f64);
        assert_is_close!(event.daughter_p4s[0].m(), 0.497_611, 1e-4, f64);
        assert_is_close!(event.daughter_p4s[1].m(), 0.497_611, 1e-4, f64);
        let resonance: FourMomentum<f64> = event.daughter_p4s.into_iter().sum();
        assert_is_close!(resonance.m(), 1.374_272_5, f64);
    }

    #[test]
    fn test_activation() -> Result<(), RustitudeError> {
        let event = generate_test_event_f64();
        let dataset = Dataset::new(vec![event]);
        let model = model!(
            scalar("a") + scalar("b"),
            scalar("c") * scalar("d") + scalar("e"),
        );
        let mut manager = Manager::new(&model, &dataset)?;
        // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1440121.0,
            f64
        );
        // |1|^2 = 1
        manager.deactivate_all();
        manager.activate("a")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1.0,
            f64
        );
        // |1 + 10|^2 = 121
        manager.activate("b")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            121.0,
            f64
        );
        // |1 + 10|^2 + |100|^2 = 121 + 10000 = 10121
        manager.activate("c")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            10121.0,
            f64
        );
        // |1 + 10|^2 + |2|^2 = 121 + 4 = 125
        manager.deactivate("c")?;
        manager.deactivate("c")?; // this shouldn't cause problems
        manager.activate("d")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            125.0,
            f64
        );
        manager.activate_all();
        // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1440121.0,
            f64
        );
        Ok(())
    }
    #[test]
    fn test_distribution() -> Result<(), RustitudeError> {
        let event = generate_test_event_f64();
        let dataset = Dataset::new(vec![event]);
        let model = model!((scalar("a") + scalar("b")) * scalar("c") + scalar("d"));
        let manager = Manager::new(&model, &dataset)?;
        // |(2 + 3) * 4 + 10|^2 = |(2 * 4) + (3 * 4) + 10|^2 = |8 + 12 + 10|^2 = |30|^2 = 900
        // Note the arguments are "out of order due" to amplitude distribution
        assert_is_close!(manager.evaluate(&[2.0, 4.0, 3.0, 10.0])?[0], 900.0, f64);
        Ok(())
    }
}

mod f32_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    #[test]
    fn test_masses() {
        let event = generate_test_event_f32();
        assert_is_close!(event.beam_p4.m(), 0.0, f32);
        assert_is_close!(event.recoil_p4.m(), 0.938_272, f32);
        assert_is_close!(event.daughter_p4s[0].m(), 0.497611, 1e-4, f32);
        assert_is_close!(event.daughter_p4s[1].m(), 0.497611, 1e-4, f32);
        let resonance: FourMomentum<f32> = event.daughter_p4s.into_iter().sum();
        assert_is_close!(resonance.m(), 1.374_273, f32);
    }

    #[test]
    fn test_activation() -> Result<(), RustitudeError> {
        let event = generate_test_event_f32();
        let dataset = Dataset::new(vec![event]);
        let model = model!(
            scalar("a") + scalar("b"),
            scalar("c") * scalar("d") + scalar("e"),
        );
        let mut manager = Manager::new(&model, &dataset)?;
        // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1440121.0,
            f32
        );
        // |1|^2 = 1
        manager.deactivate_all();
        manager.activate("a")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1.0,
            f32
        );
        // |1 + 10|^2 = 121
        manager.activate("b")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            121.0,
            f32
        );
        // |1 + 10|^2 + |100|^2 = 121 + 10000 = 10121
        manager.activate("c")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            10121.0,
            f32
        );
        // |1 + 10|^2 + |2|^2 = 121 + 4 = 125
        manager.deactivate("c")?;
        manager.deactivate("c")?; // this shouldn't cause problems
        manager.activate("d")?;
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            125.0,
            f32
        );
        manager.activate_all();
        // |(1 + 10)|^2 + |(100 * 2 + 1000)|^2 = 1440121
        assert_is_close!(
            manager.evaluate(&[1.0, 10.0, 100.0, 2.0, 1000.0])?[0],
            1440121.0,
            f32
        );
        Ok(())
    }
    #[test]
    fn test_distribution() -> Result<(), RustitudeError> {
        let event = generate_test_event_f32();
        let dataset = Dataset::new(vec![event]);
        let model = model!((scalar("a") + scalar("b")) * scalar("c") + scalar("d"));
        let manager = Manager::new(&model, &dataset)?;
        // |(2 + 3) * 4 + 10|^2 = |(2 * 4) + (3 * 4) + 10|^2 = |8 + 12 + 10|^2 = |30|^2 = 900
        // Note the arguments are "out of order due" to amplitude distribution
        assert_is_close!(manager.evaluate(&[2.0, 4.0, 3.0, 10.0])?[0], 900.0, f32);
        Ok(())
    }
}
