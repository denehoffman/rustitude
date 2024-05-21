use divan::black_box;
use rustitude::gluex::harmonics::Zlm;
use rustitude::gluex::resonances::KMatrixF0;
use rustitude::gluex::utils::{Frame, Reflectivity, Wave};
use rustitude::prelude::*;

fn main() {
    divan::main();
}

#[divan::bench(threads = [0, 1, 4, 8, 16])]
fn kmatrix(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let dataset = Dataset::from_parquet("benches/data_pol.parquet");
            let f0: AmpOp = amplitude!("f0", KMatrixF0::new(2));
            let s0: AmpOp = amplitude!(
                "s0",
                Zlm::new(Wave::S0, Reflectivity::Positive, Frame::Helicity)
            );
            let amp = (f0 * s0).norm_sqr();
            let mut model = Model::new(amp);
            model.fix("f0", "f0_500 re", 0.0);
            model.fix("f0", "f0_500 im", 0.0);
            model.fix("f0", "f0_980 im", 0.0);
            Manager::new(model, dataset)
        })
        .bench_values(|m| black_box(m.evaluate(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])));
}
