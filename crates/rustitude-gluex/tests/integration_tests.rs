mod f64_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    use rustitude_gluex::utils::Decay;
    use rustitude_gluex::utils::{Frame, Reflectivity, Wave};
    use rustitude_gluex::{
        harmonics::{Ylm, Zlm},
        resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2, KMatrixPi1, KMatrixRho},
    };
    #[test]
    fn test_ylm() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let y00 = Ylm::new(Wave::S0, Decay::default(), Frame::Helicity).named("y00");
        let y11 = Ylm::new(Wave::P1, Decay::default(), Frame::Helicity).named("y11");
        let y22 = Ylm::new(Wave::D1, Decay::default(), Frame::Helicity).named("y22");
        let y33 = Ylm::new(Wave::F1, Decay::default(), Frame::Helicity).named("y33");
        let manager = Manager::new(&model!(y00.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.07957747, f64);
        let manager = Manager::new(&model!(y00.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0, f64);
        let manager = Manager::new(&model!(y11.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.07321731, f64);
        let manager = Manager::new(&model!(y11.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.02033001, f64);
        let manager = Manager::new(&model!(y22.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.07918444, f64);
        let manager = Manager::new(&model!(y22.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.02198688, f64);
        let manager = Manager::new(&model!(y33.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_425_627_3, f64);
        let manager = Manager::new(&model!(y33.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_118_182_5, f64);
        Ok(())
    }

    #[test]
    fn test_zlm() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let z00p = Zlm::new(
            Wave::S0,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z00+");
        let z11p = Zlm::new(
            Wave::P1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z11+");
        let z22p = Zlm::new(
            Wave::D1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z22+");
        let z33p = Zlm::new(
            Wave::F1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z33+");
        let z00n = Zlm::new(
            Wave::S0,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z00-");
        let z11n = Zlm::new(
            Wave::P1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z11-");
        let z22n = Zlm::new(
            Wave::D1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z22-");
        let z33n = Zlm::new(
            Wave::F1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z33-");
        let manager = Manager::new(&model!(z00p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.014120844, f64);
        let manager = Manager::new(&model!(z00p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.04262128, f64);
        let manager = Manager::new(&model!(z11p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0018273925, f64);
        let manager = Manager::new(&model!(z11p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.05665150, f64);
        let manager = Manager::new(&model!(z22p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0019763229, f64);
        let manager = Manager::new(&model!(z22p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.06126853, f64);
        let manager = Manager::new(&model!(z33p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_010_623_025, f64);
        let manager = Manager::new(&model!(z33p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_329_326_8, f64);
        let manager = Manager::new(&model!(z00n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.006259242, f64);
        let manager = Manager::new(&model!(z00n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.09615357, f64);
        let manager = Manager::new(&model!(z11n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0008100147, f64);
        let manager = Manager::new(&model!(z11n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.12780573, f64);
        let manager = Manager::new(&model!(z22n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0008760300, f64);
        let manager = Manager::new(&model!(z22n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.13822176, f64);
        let manager = Manager::new(&model!(z33n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_004_708_789, f64);
        let manager = Manager::new(&model!(z33n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_742_961, f64);
        Ok(())
    }

    #[test]
    fn test_f0() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let f0 = KMatrixF0::new(2, Decay::default()).named("F0(2)");
        let manager = Manager::new(&model!(f0.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.030680506,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.054_358_566,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.002_729_284_8,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.000_001_175_634_7,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.122108042,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.153889632,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
            0.003648740,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
            0.000_815_257_8,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
            0.000_078_950_513,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
            0.017668038,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.0,
            f64
        );
        Ok(())
    }

    #[test]
    fn test_f2() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let f2 = KMatrixF2::new(2, Decay::default()).named("F2(2)");
        let manager = Manager::new(&model!(f2.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.079874652,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.116_876_834,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.5885590974,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.025_541_601_7,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
            0.097405045,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
            0.004549439,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
            0.011321180,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
            0.007075583,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.0,
            f64
        );
        Ok(())
    }

    #[test]
    fn test_a0() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let a0 = KMatrixA0::new(1, Decay::default()).named("A0(1)");
        let manager = Manager::new(&model!(a0.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0],
            0.084485367,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.0099839209,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            1.726739721,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            1.299_382_378,
            f64
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f64);
        Ok(())
    }

    #[test]
    fn test_a2() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let a2 = KMatrixA2::new(1, Decay::default()).named("A2(1)");
        let manager = Manager::new(&model!(a2.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0], 0.34870050, f64);
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.919_619_85,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            0.005_930_287,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            0.026281367,
            f64
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f64);
        Ok(())
    }

    #[test]
    fn test_rho() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let rho = KMatrixRho::new(1, Decay::default()).named("Rho(1)");
        let manager = Manager::new(&model!(rho.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0],
            0.0007601991,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.0007605480,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            0.266948124,
            f64
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            0.029465809,
            f64
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f64);
        Ok(())
    }

    #[test]
    fn test_pi1() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f64()]);
        let pi1 = KMatrixPi1::new(1, Decay::default()).named("Rho(1)");
        let manager = Manager::new(&model!(pi1.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[1.0, 0.0])?[0], 0.6947747815, f64);
        assert_is_close!(manager.evaluate(&[0.0, 1.0])?[0], 0.9365046503, f64);
        assert_is_close!(manager.evaluate(&[0.0, 0.0])?[0], 0.0, f64);
        Ok(())
    }
}
mod f32_tests {
    use rustitude_core::assert_is_close;
    use rustitude_core::prelude::*;
    use rustitude_core::utils::*;
    use rustitude_gluex::utils::Decay;
    use rustitude_gluex::utils::{Frame, Reflectivity, Wave};
    use rustitude_gluex::{
        harmonics::{Ylm, Zlm},
        resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2, KMatrixPi1, KMatrixRho},
    };

    #[test]
    fn test_ylm() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let y00 = Ylm::new(Wave::S0, Decay::default(), Frame::Helicity).named("y00");
        let y11 = Ylm::new(Wave::P1, Decay::default(), Frame::Helicity).named("y11");
        let y22 = Ylm::new(Wave::D1, Decay::default(), Frame::Helicity).named("y22");
        let y33 = Ylm::new(Wave::F1, Decay::default(), Frame::Helicity).named("y33");
        let manager = Manager::new(&model!(y00.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.07957747, f32);
        let manager = Manager::new(&model!(y00.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0, f32);
        let manager = Manager::new(&model!(y11.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.07321755, f32);
        let manager = Manager::new(&model!(y11.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.020_330_08, f32);
        let manager = Manager::new(&model!(y22.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.079_183_795, f32);
        let manager = Manager::new(&model!(y22.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.021_986_704, f32);
        let manager = Manager::new(&model!(y33.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_425_394_8, f32);
        let manager = Manager::new(&model!(y33.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_118_118_02, f32);
        Ok(())
    }

    #[test]
    fn test_zlm() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let z00p = Zlm::new(
            Wave::S0,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z00+");
        let z11p = Zlm::new(
            Wave::P1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z11+");
        let z22p = Zlm::new(
            Wave::D1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z22+");
        let z33p = Zlm::new(
            Wave::F1,
            Reflectivity::Positive,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z33+");
        let z00n = Zlm::new(
            Wave::S0,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z00-");
        let z11n = Zlm::new(
            Wave::P1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z11-");
        let z22n = Zlm::new(
            Wave::D1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z22-");
        let z33n = Zlm::new(
            Wave::F1,
            Reflectivity::Negative,
            Decay::default(),
            Frame::Helicity,
        )
        .named("z33-");
        let manager = Manager::new(&model!(z00p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.014120844, f32);
        let manager = Manager::new(&model!(z00p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.04262128, f32);
        let manager = Manager::new(&model!(z11p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.0018273992, f32);
        let manager = Manager::new(&model!(z11p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.056_651_685, f32);
        let manager = Manager::new(&model!(z22p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.001_976_306_6, f32);
        let manager = Manager::new(&model!(z22p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.061_268_03, f32);
        let manager = Manager::new(&model!(z33p.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_010_617_216_5, f32);
        let manager = Manager::new(&model!(z33p.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_329_146_95, f32);
        let manager = Manager::new(&model!(z00n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.006259242, f32);
        let manager = Manager::new(&model!(z00n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.09615357, f32);
        let manager = Manager::new(&model!(z11n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_810_017_7, f32);
        let manager = Manager::new(&model!(z11n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.127_806_13, f32);
        let manager = Manager::new(&model!(z22n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_876_022_74, f32);
        let manager = Manager::new(&model!(z22n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.138_220_62, f32);
        let manager = Manager::new(&model!(z33n.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_004_706_215_3, f32);
        let manager = Manager::new(&model!(z33n.imag()), &dataset)?;
        assert_is_close!(manager.evaluate(&[])?[0], 0.000_742_555_25, f32);
        Ok(())
    }

    #[test]
    fn test_f0() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let f0 = KMatrixF0::new(2, Decay::default()).named("F0(2)");
        let manager = Manager::new(&model!(f0.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.030_680_573,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.054_358_285,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.002_729_235,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.000_001_174_773,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.122_109,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.153_888_26,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
            0.003_648_736_3,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
            0.000_815_246,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
            0.000_078_956_4,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
            0.017_668_229,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.0,
            f32
        );
        Ok(())
    }

    #[test]
    fn test_f2() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let f2 = KMatrixF2::new(2, Decay::default()).named("F2(2)");
        let manager = Manager::new(&model!(f2.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.079_874_024,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.116_876_364,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.588_550_03,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.025_541_838,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
            0.097_404_32,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
            0.004_549_420_4,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
            0.011_321_053,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
            0.007_075_636,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
            0.0,
            f32
        );
        Ok(())
    }

    #[test]
    fn test_a0() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let a0 = KMatrixA0::new(1, Decay::default()).named("A0(1)");
        let manager = Manager::new(&model!(a0.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0],
            0.084_485_64,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.009_983_987,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            1.726_750_9,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            1.299_370_9,
            f32
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f32);
        Ok(())
    }

    #[test]
    fn test_a2() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let a2 = KMatrixA2::new(1, Decay::default()).named("A2(1)");
        let manager = Manager::new(&model!(a2.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0],
            0.348_697_72,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.919_610_8,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            0.005_930_149_5,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            0.026_281_15,
            f32
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f32);
        Ok(())
    }

    #[test]
    fn test_rho() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let rho = KMatrixRho::new(1, Decay::default()).named("Rho(1)");
        let manager = Manager::new(&model!(rho.real()), &dataset)?;
        assert_is_close!(
            manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0],
            0.000_760_193_04,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0],
            0.000_760_549,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0],
            0.266_946_82,
            f32
        );
        assert_is_close!(
            manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0],
            0.029_465_48,
            f32
        );
        assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0, f32);
        Ok(())
    }

    #[test]
    fn test_pi1() -> Result<(), RustitudeError> {
        let dataset = Dataset::new(vec![generate_test_event_f32()]);
        let pi1 = KMatrixPi1::new(1, Decay::default()).named("Rho(1)");
        let manager = Manager::new(&model!(pi1.real()), &dataset)?;
        assert_is_close!(manager.evaluate(&[1.0, 0.0])?[0], 0.694_777_7, f32);
        assert_is_close!(manager.evaluate(&[0.0, 1.0])?[0], 0.936_495_9, f32);
        assert_is_close!(manager.evaluate(&[0.0, 0.0])?[0], 0.0, f32);
        Ok(())
    }
}
