use num_traits::int::PrimInt;
use std::ops;

use bit::BitIndex;

#[derive(Debug, PartialEq)]
pub struct BitVector<T = u8> {
    data: Vec<T>,
    length: usize
}

impl<T: BitIndex + PrimInt> BitVector<T> {
    pub fn new(length: usize, data: Vec<T>) -> BitVector<T> {
        debug_assert!(length <= data.len() * T::bit_length());
        if length > 0 {
            debug_assert!(data.last().unwrap().leading_zeros() as usize >= data.len() * T::bit_length() - length);
        }

        BitVector{
            data,
            length
        }
    }

    pub fn from_elem(nbits: usize, initial: bool) -> BitVector<T> {
        let zero = T::zero();
        let nblocks = nbits / (T::bit_length());
        let data = vec![if initial { zero } else { !zero }; nblocks]; 

        BitVector {
            data,
            length: nbits
        }
    }
}

impl<T> BitVector<T> {

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn count_ones(self) -> u32
    where
        T: PrimInt,
    {
        self.data.iter().fold(0, |a, &b| a + b.count_ones())
    }

    pub fn count_zeros(self) -> u32
    where
        T: PrimInt,
    {
        self.data.iter().fold(0, |a, &b| a + b.count_zeros())
    }


}

macro_rules! implement_index {
    ($ity: ty, $t: ty, $size: expr) => {
        impl ops::Index<$ity> for BitVector<$t> {
            type Output = u8;
            
            fn index(&self, index: $ity) -> &Self::Output {
                let block_index = (index / $size) as usize;
                let remainder = index % $size;

                if (self.data[block_index] >> remainder) & 1 == 1 {
                    &1
                } else {
                    &0
                }
            }
        }
    }
}

macro_rules! implement_index_types {
    ($t: ty) =>  {
        implement_index!($t, u8, 8);
        implement_index!($t, u16, 16);
        implement_index!($t, u32, 32);
        implement_index!($t, u64, 64);
        #[cfg(unstable)]
        implement_index!($t, u128, 128);
    }
}

implement_index_types!(u8);
implement_index_types!(i8);
implement_index_types!(u16);
implement_index_types!(i16);
implement_index_types!(i32);
implement_index_types!(u32);
implement_index_types!(i64);
implement_index_types!(u64);



macro_rules! length_check {
        ($a: expr, $b: expr) => {
            debug_assert_eq!($a.len(), $b.len());
        }
    }

macro_rules! binary_operation {
    ($traitname: tt, $functionname: ident, $op: tt, $traitassign: tt, $assignfunctionname: ident) =>
        {
            impl<T> ops::$traitname for BitVector<T>
            where
                T: ops::$traitname<Output=T>
            {
                type Output = BitVector<T>;

                fn $functionname(self, other: BitVector<T>) -> Self::Output{
                    length_check!(self, other);
                    let data = self.data.into_iter()
                        .zip(other.data)
                        .map(|(a, b)| a $op b)
                        .collect::<Vec<T>>();

                    BitVector {
                        data, length: self.length
                    }
                }
            }

            impl<'a, T> ops::$traitname for &'a BitVector<T>
            where
                &'a T: ops::$traitname<Output=T>
            {
                type Output = BitVector<T>;

                fn $functionname(self, other: &'a BitVector<T>) -> Self::Output{
                    length_check!(self, other);
                    let data = self.data.iter()
                        .zip(other.data.iter())
                        .map(|(a, b)| a $op b)
                        .collect::<Vec<T>>();

                    BitVector { data, length: self.length }
                }
            }

            impl<T> ops::$traitassign for BitVector<T>
                where T: ops::$traitassign
            {
                fn $assignfunctionname(&mut self, other: Self) {
                    length_check!(self, other);

                    for (i, e) in other.data.into_iter().enumerate() {
                        self.data[i].$assignfunctionname(e);
                    }
                }
            }
        }
    }

binary_operation!(BitXor, bitxor, ^, BitXorAssign, bitxor_assign);
binary_operation!(BitAnd, bitand, &, BitAndAssign, bitand_assign);


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! binary_test {
        ($name: ident, $op: tt, $opassign: tt) => {
            mod $name {
                use std::cmp;
                use super::*;

                quickcheck! {
                    fn qc_binary_operator(a: Vec<u8>, b: Vec<u8>) -> bool {
                        let min_length = cmp::min(a.len(), b.len());
                        let bits = min_length * 8;
                        let v1 = BitVector::new(bits, a[..min_length].to_vec());
                        let v2 = BitVector::new(bits, b[..min_length].to_vec());

                        let result = v1 $op v2;
                        let expected_result_vec = a.into_iter().zip(b).map(|(a, b)| a $op b).collect::<Vec<u8>>();
                        result.data == expected_result_vec
                    }
                }

                quickcheck! {
                    fn qc_with_and_with_reference(a: Vec<u8>, b: Vec<u8>) -> bool {
                        let min_length = cmp::min(a.len(), b.len());
                        let bits = min_length * 8;
                        let mut v1 = BitVector::new(bits, a[..min_length].to_vec());
                        let v2 = BitVector::new(bits, b[..min_length].to_vec());

                        let result_ref = &v1 $op &v2;
                        let result_noref = v1 $op v2;
                        result_ref == result_noref
                    }
                }

                quickcheck! {
                    fn qc_assign_and_binary(a: Vec<u8>, b: Vec<u8>) -> bool {
                        let min_length = cmp::min(a.len(), b.len());
                        let bits = min_length * 8;
                        let mut v1 = BitVector::new(bits, a[..min_length].to_vec());
                        let v2 = BitVector::new(bits, b[..min_length].to_vec());

                        let result = &v1 $op &v2;
                        v1 $opassign v2;

                        v1 == result
                    }
                }

                #[test]
                fn from_elem() {
                    let bv = BitVector::from_elem(10, false);
                    for i in 0..10 {
                        assert_eq!(bv[i], &1);
                    }
                }
            }
        }
    }
    binary_test!(xor, ^, ^=);
    binary_test!(and, &, &=);
}
