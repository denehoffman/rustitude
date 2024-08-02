mod f64_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    use rustitude_gluex::harmonics::Ylm;
    use rustitude_gluex::utils::Decay;
    use rustitude_gluex::utils::{Frame, Wave};
    #[test]
    fn test_ell() -> Result<(), RustitudeError> {
        let ds_tot = generate_test_dataset_f64();
        let ds_data = Dataset::new(ds_tot.events[0..3].to_vec());
        let mut ds_mc = Dataset::new(ds_tot.events[3..].to_vec());
        ds_mc.reindex();
        let y22 = Ylm::new(Wave::D2, Decay::default(), Frame::Helicity).named("y22");
        let model = model!(y22);
        let data_manager = Manager::new(&model, &ds_data)?;
        let mc_manager = Manager::new(&model, &ds_mc)?;
        let ell = ExtendedLogLikelihood::new(data_manager, mc_manager);
        let res = ell.evaluate(&ell.get_initial())?;
        assert_is_close!(res, 6.978_059, f64);
        Ok(())
    }
}

mod f32_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    use rustitude_gluex::harmonics::Ylm;
    use rustitude_gluex::utils::Decay;
    use rustitude_gluex::utils::{Frame, Wave};
    #[test]
    fn test_ell() -> Result<(), RustitudeError> {
        let ds_tot = generate_test_dataset_f32();
        let ds_data = Dataset::new(ds_tot.events[0..3].to_vec());
        let mut ds_mc = Dataset::new(ds_tot.events[3..].to_vec());
        ds_mc.reindex();
        let y22 = Ylm::new(Wave::D2, Decay::default(), Frame::Helicity).named("y22");
        let model = model!(y22);
        let data_manager = Manager::new(&model, &ds_data)?;
        let mc_manager = Manager::new(&model, &ds_mc)?;
        let ell = ExtendedLogLikelihood::new(data_manager, mc_manager);
        let res = ell.evaluate(&ell.get_initial())?;
        assert_is_close!(res, 6.978_059, f32);
        Ok(())
    }
}
