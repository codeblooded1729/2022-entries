use ark_bls12_377::{G1Affine as G1Affine_ark, G1Projective as G1Projective_ark};
use ark_ec::AffineCurve;
use ark_ff::PrimeField;
use ark_std::rand;
use zprize_fpga_msm::{multi_scalar_mult, multi_scalar_mult_init};
type BigInt = <<G1Affine_ark as AffineCurve>::ScalarField as PrimeField>::BigInt;
fn main() {
    // num points generated will be 1 << size
    let size = 3;
    let (points, scalars) = random_points(size);
    let mut context = multi_scalar_mult_init(points.as_slice());
    let _out = multi_scalar_mult(&mut context, points.as_slice(), scalars.as_slice());
    println!()
}

pub fn random_points(size: u8) -> (Vec<G1Affine_ark>, Vec<BigInt>) {
    use rand_core::SeedableRng;
    let mut rng = rand::prelude::StdRng::from_entropy();

    use ark_std::UniformRand;
    let points: Vec<_> = (0..(1 << size))
        .map(|_| G1Projective_ark::rand(&mut rng))
        .collect();
    let scalars: Vec<BigInt> = (0..(1 << size)).map(|_| BigInt::rand(&mut rng)).collect();
    use ark_ec::ProjectiveCurve;
    let points = G1Projective_ark::batch_normalization_into_affine(&points);
    (points, scalars)
}
