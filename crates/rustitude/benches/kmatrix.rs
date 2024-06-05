use criterion::{criterion_group, criterion_main, Criterion};
use rustitude::gluex::harmonics::Zlm;
use rustitude::gluex::resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2};
use rustitude::gluex::utils::{Frame, Reflectivity, Wave};
use rustitude::prelude::*;

pub fn criterion_kmatrix(c: &mut Criterion) {
    let dataset = Dataset::from_parquet("benches/test_data.parquet").unwrap();
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
    let pos_real = (&f0p + &a0p) * s0p.real() + (&f2 + &a2) * d2.real();
    let pos_imag = (&f0p + &a0p) * s0p.imag() + (&f2 + &a2) * d2.imag();
    let neg_real = (&f0n + &a0n) * s0n.real();
    let neg_imag = (&f0n + &a0n) * s0n.imag();
    let mut model = Model::new(vec![pos_real + pos_imag + neg_real + neg_imag]);
    model.fix("f0+", "f0_500 re", 0.0).unwrap();
    model.fix("f0+", "f0_500 im", 0.0).unwrap();
    model.fix("f0+", "f0_980 im", 0.0).unwrap();
    model.fix("f0-", "f0_500 re", 0.0).unwrap();
    model.fix("f0-", "f0_500 im", 0.0).unwrap();
    model.fix("f0-", "f0_980 im", 0.0).unwrap();
    let m = Manager::new(&model, &dataset).unwrap();
    c.bench_function("kmatrix", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(m.evaluate(&v))
        })
    });
    c.bench_function("kmatrix_norm_int", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(m.norm_int(&v))
        })
    });
    let dataset_mc = Dataset::from_parquet("benches/test_data.parquet").unwrap();
    let nll = ExtendedLogLikelihood::new(m, Manager::new(&model, &dataset_mc).unwrap());
    c.bench_function("kmatrix_nll", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(nll.evaluate(&v, num_cpus::get()))
        })
    });
}

criterion_group!(benches, criterion_kmatrix);
criterion_main!(benches);
