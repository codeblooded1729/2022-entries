use our_ec::AffineCurve;
use our_ff::PrimeField;
use ark_std::rand;
use our_bls12_377::{ G1Affine, G1Projective} ;
use zprize_fpga_msm::{multi_scalar_mult, multi_scalar_mult_init};
type BigInt = <<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt;
fn main() {
    // num points generated will be 1 << size
    let size = 3;
    let (points, scalars) = random_points(size);
    let mut context = multi_scalar_mult_init(points.as_slice());
    let _out = multi_scalar_mult(&mut context, points.as_slice(), scalars.as_slice());
    println!("This worked!");
}

pub fn random_points(size: u8) -> (Vec<G1Affine>, Vec<BigInt>) {
    use rand_core::SeedableRng;
    let mut rng = rand::prelude::StdRng::from_entropy();

    use ark_std::UniformRand;
    let points: Vec<_> = (0..(1 << size))
        .map(|_| G1Projective::rand(&mut rng))
        .collect();
    let scalars: Vec<BigInt> = (0..(1 << size)).map(|_| BigInt::rand(&mut rng)).collect();
    use our_ec::ProjectiveCurve;
    let points = G1Projective::batch_normalization_into_affine(&points);
    (points, scalars)
}
