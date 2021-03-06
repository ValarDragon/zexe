use crate::groups::curves::twisted_edwards::AffineGadget;
use algebra::ed_on_mnt4_298::*;

use crate::instantiated::ed_on_mnt4_298::fields::FqGadget;

pub type EdwardsGadget = AffineGadget<EdwardsParameters, Fq, FqGadget>;

#[test]
fn test() {
    crate::groups::curves::twisted_edwards::test::<_, EdwardsParameters, EdwardsGadget>();
}
