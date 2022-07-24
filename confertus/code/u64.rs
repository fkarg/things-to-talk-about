use core::arch::x86_64::{_pdep_u64, _popcnt64, _tzcnt_u64};

#[inline]
#[cfg(all(target_arch = "x86_64", target_feature = "bmi1", target_feature = "bmi2"))]
unsafe fn select_internal(&self, bit: bool, n: usize) -> usize {
    _tzcnt_u64(_pdep_u64(1 << n, if bit { *self } else { !self })) as usize
}

#[inline]
#[cfg(target_arch = "x86_64")]
unsafe fn rank_internal(&self, bit: bool, index: usize) -> usize {
    _popcnt64({if bit {*self} else {!self}}.overflowing_shl(u64::BITS - index as u32).0 as i64)
        as usize
}
