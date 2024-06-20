use rustitude_core::assert_is_close;
use rustitude_core::prelude::*;
use rustitude_core::utils::*;
use rustitude_gluex::utils::{Frame, Reflectivity, Wave};
use rustitude_gluex::{
    harmonics::{Ylm, Zlm},
    resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2, KMatrixPi1, KMatrixRho},
};

#[test]
fn test_ylm() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let y00 = Ylm::new(Wave::S0, Frame::Helicity).into_amplitude("y00");
    let y11 = Ylm::new(Wave::P1, Frame::Helicity).into_amplitude("y11");
    let y22 = Ylm::new(Wave::D1, Frame::Helicity).into_amplitude("y22");
    let y33 = Ylm::new(Wave::F1, Frame::Helicity).into_amplitude("y33");
    let manager = Manager::new(&Model::new(vec![y00.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.07957747);
    let manager = Manager::new(&Model::new(vec![y00.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0);
    let manager = Manager::new(&Model::new(vec![y11.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.07321731);
    let manager = Manager::new(&Model::new(vec![y11.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.02033001);
    let manager = Manager::new(&Model::new(vec![y22.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.07918444);
    let manager = Manager::new(&Model::new(vec![y22.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.02198688);
    let manager = Manager::new(&Model::new(vec![y33.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0004255249);
    let manager = Manager::new(&Model::new(vec![y33.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0001181541);
    Ok(())
}

#[test]
fn test_zlm() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let z00p = Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity).into_amplitude("z00+");
    let z11p = Zlm::new(Wave::P1, Reflectivity::Positive, Frame::Helicity).into_amplitude("z11+");
    let z22p = Zlm::new(Wave::D1, Reflectivity::Positive, Frame::Helicity).into_amplitude("z22+");
    let z33p = Zlm::new(Wave::F1, Reflectivity::Positive, Frame::Helicity).into_amplitude("z33+");
    let z00n = Zlm::new(Wave::S0, Reflectivity::Negative, Frame::Helicity).into_amplitude("z00-");
    let z11n = Zlm::new(Wave::P1, Reflectivity::Negative, Frame::Helicity).into_amplitude("z11-");
    let z22n = Zlm::new(Wave::D1, Reflectivity::Negative, Frame::Helicity).into_amplitude("z22-");
    let z33n = Zlm::new(Wave::F1, Reflectivity::Negative, Frame::Helicity).into_amplitude("z33-");
    let manager = Manager::new(&Model::new(vec![z00p.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.014120844);
    let manager = Manager::new(&Model::new(vec![z00p.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.04262128);
    let manager = Manager::new(&Model::new(vec![z11p.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0018273925);
    let manager = Manager::new(&Model::new(vec![z11p.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.05665150);
    let manager = Manager::new(&Model::new(vec![z22p.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0019763229);
    let manager = Manager::new(&Model::new(vec![z22p.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.06126853);
    let manager = Manager::new(&Model::new(vec![z33p.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.000010620453);
    let manager = Manager::new(&Model::new(vec![z33p.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0003292476);
    let manager = Manager::new(&Model::new(vec![z00n.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.006259242);
    let manager = Manager::new(&Model::new(vec![z00n.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.09615357);
    let manager = Manager::new(&Model::new(vec![z11n.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0008100147);
    let manager = Manager::new(&Model::new(vec![z11n.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.12780573);
    let manager = Manager::new(&Model::new(vec![z22n.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0008760300);
    let manager = Manager::new(&Model::new(vec![z22n.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.13822176);
    let manager = Manager::new(&Model::new(vec![z33n.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.000004707649);
    let manager = Manager::new(&Model::new(vec![z33n.imag().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[])?[0], 0.0007427823);
    Ok(())
}

#[test]
fn test_f0() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let f0 = KMatrixF0::new(2).into_amplitude("F0(2)");
    let manager = Manager::new(&Model::new(vec![f0.real().as_cohsum()]), &dataset)?;
    assert_is_close!(
        manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.030680506
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.054357208
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.0027290857
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.0000011722429
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.122108042
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.153889632
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
        0.003648740
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
        0.0008152275
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
        0.000078969229
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
        0.017668038
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.0
    );
    Ok(())
}

#[test]
fn test_f2() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let f2 = KMatrixF2::new(2).into_amplitude("F2(2)");
    let manager = Manager::new(&Model::new(vec![f2.real().as_cohsum()]), &dataset)?;
    assert_is_close!(
        manager.evaluate(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.079874652
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.116874084
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.5885590974
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.0255428643
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])?[0],
        0.097405045
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])?[0],
        0.004549439
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])?[0],
        0.011321180
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0])?[0],
        0.007075583
    );
    assert_is_close!(
        manager.evaluate(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])?[0],
        0.0
    );
    Ok(())
}

#[test]
fn test_a0() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let a0 = KMatrixA0::new(1).into_amplitude("A0(1)");
    let manager = Manager::new(&Model::new(vec![a0.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0], 0.084485367);
    assert_is_close!(manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0], 0.0099839209);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0], 1.726739721);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0], 1.299322808);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0);
    Ok(())
}

#[test]
fn test_a2() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let a2 = KMatrixA2::new(1).into_amplitude("A2(1)");
    let manager = Manager::new(&Model::new(vec![a2.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0], 0.34870050);
    assert_is_close!(manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0], 0.91958706);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0], 0.005929765);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0], 0.026281367);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0);
    Ok(())
}

#[test]
fn test_rho() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let rho = KMatrixRho::new(1).into_amplitude("Rho(1)");
    let manager = Manager::new(&Model::new(vec![rho.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[1.0, 0.0, 0.0, 0.0])?[0], 0.0007601991);
    assert_is_close!(manager.evaluate(&[0.0, 1.0, 0.0, 0.0])?[0], 0.0007605480);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 1.0, 0.0])?[0], 0.266948124);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 1.0])?[0], 0.029465809);
    assert_is_close!(manager.evaluate(&[0.0, 0.0, 0.0, 0.0])?[0], 0.0);
    Ok(())
}

#[test]
fn test_pi1() -> Result<(), RustitudeError> {
    let dataset = Dataset::new(vec![generate_test_event()]);
    let pi1 = KMatrixPi1::new(1).into_amplitude("Rho(1)");
    let manager = Manager::new(&Model::new(vec![pi1.real().as_cohsum()]), &dataset)?;
    assert_is_close!(manager.evaluate(&[1.0, 0.0])?[0], 0.6947747815);
    assert_is_close!(manager.evaluate(&[0.0, 1.0])?[0], 0.9365046503);
    assert_is_close!(manager.evaluate(&[0.0, 0.0])?[0], 0.0);
    Ok(())
}
