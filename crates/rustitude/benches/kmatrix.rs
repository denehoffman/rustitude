use divan::black_box;
use rustitude::gluex::harmonics::Zlm;
use rustitude::gluex::resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2};
use rustitude::gluex::utils::{Frame, Reflectivity, Wave};
use rustitude::prelude::*;

fn main() {
    divan::main();
}

#[divan::bench(threads = [0, 1, 4, 8, 16])]
fn kmatrix(bencher: divan::Bencher) {
    let dataset = Dataset::from_parquet("benches/data_pol.parquet");
    let f0p: AmpOp = amplitude!("f0+", KMatrixF0::new(2));
    let f0n: AmpOp = amplitude!("f0-", KMatrixF0::new(2));
    let f2: AmpOp = amplitude!("f2", KMatrixF2::new(2));
    let a0p: AmpOp = amplitude!("a0+", KMatrixA0::new(1));
    let a0n: AmpOp = amplitude!("a0-", KMatrixA0::new(1));
    let a2: AmpOp = amplitude!("a2", KMatrixA2::new(1));
    let s0p: AmpOp = amplitude!(
        "s0+",
        Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity)
    );
    let s0n: AmpOp = amplitude!(
        "s0-",
        Zlm::new(Wave::S0, Reflectivity::Negative, Frame::Helicity)
    );
    let d2: AmpOp = amplitude!(
        "d2",
        Zlm::new(Wave::D2, Reflectivity::Positive, Frame::Helicity)
    );
    let pos_real = ((&f0p + &a0p) * s0p.real() + (&f2 + &a2) * d2.real()).norm_sqr();
    let pos_imag = ((&f0p + &a0p) * s0p.imag() + (&f2 + &a2) * d2.imag()).norm_sqr();
    let neg_real = ((&f0n + &a0n) * s0n.real()).norm_sqr();
    let neg_imag = ((&f0n + &a0n) * s0n.imag()).norm_sqr();
    let mut model = Model::new(pos_real + pos_imag + neg_real + neg_imag);
    model.fix("f0+", "f0_500 re", 0.0);
    model.fix("f0+", "f0_500 im", 0.0);
    model.fix("f0+", "f0_980 im", 0.0);
    model.fix("f0-", "f0_500 re", 0.0);
    model.fix("f0-", "f0_500 im", 0.0);
    model.fix("f0-", "f0_980 im", 0.0);
    let m = Manager::new(&model, &dataset);
    bencher
        .with_inputs(|| {
            (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>()
        })
        .bench_values(|v| black_box(m.evaluate(&v)));
}
