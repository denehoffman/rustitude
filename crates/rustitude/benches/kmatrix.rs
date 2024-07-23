use criterion::{criterion_group, criterion_main, Criterion};
use rustitude::gluex::harmonics::Zlm;
use rustitude::gluex::resonances::{KMatrixA0, KMatrixA2, KMatrixF0, KMatrixF2};
use rustitude::gluex::utils::{Frame, Reflectivity, Wave};
use rustitude::prelude::*;

pub fn criterion_kmatrix_f64(c: &mut Criterion) {
    let dataset = Dataset::from_parquet("benches/test_data.parquet", ReadMethod::Standard).unwrap();
    let f0p = Amplitude::new("f0+", KMatrixF0::new(2));
    let f0n = Amplitude::new("f0-", KMatrixF0::new(2));
    let f2 = Amplitude::new("f2", KMatrixF2::new(2));
    let a0p = Amplitude::new("a0+", KMatrixA0::new(1));
    let a0n = Amplitude::new("a0-", KMatrixA0::new(1));
    let a2 = Amplitude::new("a2", KMatrixA2::new(1));
    let s0p = Amplitude::new(
        "s0+",
        Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity),
    );
    let s0n = Amplitude::new(
        "s0-",
        Zlm::new(Wave::S0, Reflectivity::Negative, Frame::Helicity),
    );
    let d2 = Amplitude::new(
        "d2",
        Zlm::new(Wave::D2, Reflectivity::Positive, Frame::Helicity),
    );
    let pos_real = (&f0p + &a0p) * s0p.real() + (&f2 + &a2) * d2.real();
    let pos_imag = (&f0p + &a0p) * s0p.imag() + (&f2 + &a2) * d2.imag();
    let neg_real = (&f0n + &a0n) * s0n.real();
    let neg_imag = (&f0n + &a0n) * s0n.imag();
    let mut model = model!(pos_real, pos_imag, neg_real, neg_imag);
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
            criterion::black_box(m.par_evaluate(&v))
        })
    });
    let dataset_mc =
        Dataset::from_parquet("benches/test_data.parquet", ReadMethod::Standard).unwrap();
    let nll = ExtendedLogLikelihood::new(m, Manager::new(&model, &dataset_mc).unwrap());
    c.bench_function("kmatrix_nll", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(nll.par_evaluate(&v))
        })
    });
    let indices_data = (0..dataset.len()).collect::<Vec<usize>>();
    let indices_mc = (0..dataset_mc.len()).collect::<Vec<usize>>();
    c.bench_function("kmatrix_nll_indexed", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f64>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(nll.par_evaluate_indexed(&v, &indices_data, &indices_mc))
        })
    });
}

pub fn criterion_kmatrix_f32(c: &mut Criterion) {
    let dataset = Dataset::from_parquet("benches/test_data.parquet", ReadMethod::Standard).unwrap();
    let f0p = Amplitude::new("f0+", KMatrixF0::new(2));
    let f0n = Amplitude::new("f0-", KMatrixF0::new(2));
    let f2 = Amplitude::new("f2", KMatrixF2::new(2));
    let a0p = Amplitude::new("a0+", KMatrixA0::new(1));
    let a0n = Amplitude::new("a0-", KMatrixA0::new(1));
    let a2 = Amplitude::new("a2", KMatrixA2::new(1));
    let s0p = Amplitude::new(
        "s0+",
        Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity),
    );
    let s0n = Amplitude::new(
        "s0-",
        Zlm::new(Wave::S0, Reflectivity::Negative, Frame::Helicity),
    );
    let d2 = Amplitude::new(
        "d2",
        Zlm::new(Wave::D2, Reflectivity::Positive, Frame::Helicity),
    );
    let pos_real = (&f0p + &a0p) * s0p.real() + (&f2 + &a2) * d2.real();
    let pos_imag = (&f0p + &a0p) * s0p.imag() + (&f2 + &a2) * d2.imag();
    let neg_real = (&f0n + &a0n) * s0n.real();
    let neg_imag = (&f0n + &a0n) * s0n.imag();
    let mut model = model!(pos_real, pos_imag, neg_real, neg_imag);
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
                .map(|_| rand::random::<f32>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(m.par_evaluate(&v))
        })
    });
    let dataset_mc =
        Dataset::from_parquet("benches/test_data.parquet", ReadMethod::Standard).unwrap();
    let nll = ExtendedLogLikelihood::new(m, Manager::new(&model, &dataset_mc).unwrap());
    c.bench_function("kmatrix_nll", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f32>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(nll.par_evaluate(&v))
        })
    });
    let indices_data = (0..dataset.len()).collect::<Vec<usize>>();
    let indices_mc = (0..dataset_mc.len()).collect::<Vec<usize>>();
    c.bench_function("kmatrix_nll_indexed", |b| {
        b.iter(|| {
            let v = (0..model.get_n_free())
                .map(|_| rand::random::<f32>() * 100.0)
                .collect::<Vec<_>>();
            criterion::black_box(nll.par_evaluate_indexed(&v, &indices_data, &indices_mc))
        })
    });
}
criterion_group!(benches, criterion_kmatrix_f64, criterion_kmatrix_f32);
criterion_main!(benches);
