pub mod aslice;
pub mod iter;
pub mod rand;

// fn sized_seq<const N: usize>() -> [usize; N] {
//     // SAFETY: MaybeUninit does not have to be initialized
//     let mut ret: [MaybeUninit<usize>; N] = unsafe { MaybeUninit::uninit().assume_init() };
//
//     ret.iter_mut().enumerate().for_each(|(i, e)| {
//         e.write(i);
//     });
//
//     unsafe { ret.transpose().assume_init() }
// }

pub fn sigmoid(i: f32) -> f32 {
    1. / (1. + f32::exp(-i))
}
