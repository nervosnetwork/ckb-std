use core::cmp::Ordering;

/// Transaction input's since field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Since(u64);

impl Since {
    const LOCK_TYPE_FLAG: u64 = 1 << 63;
    const METRIC_TYPE_FLAG_MASK: u64 = 0x6000_0000_0000_0000;
    const FLAGS_MASK: u64 = 0xff00_0000_0000_0000;
    const VALUE_MASK: u64 = 0x00ff_ffff_ffff_ffff;
    const REMAIN_FLAGS_BITS: u64 = 0x1f00_0000_0000_0000;
    const LOCK_BY_BLOCK_NUMBER_MASK: u64 = 0x0000_0000_0000_0000;
    const LOCK_BY_EPOCH_MASK: u64 = 0x2000_0000_0000_0000;
    const LOCK_BY_TIMESTAMP_MASK: u64 = 0x4000_0000_0000_0000;

    pub fn from_block_number(number: u64, absolute: bool) -> Option<Self> {
        if number & Self::FLAGS_MASK != 0 {
            return None;
        }
        Some(Self::new(
            number
                | Self::LOCK_BY_BLOCK_NUMBER_MASK
                | (if absolute { 0 } else { Self::LOCK_TYPE_FLAG }),
        ))
    }

    pub fn from_timestamp(timestamp: u64, absolute: bool) -> Option<Self> {
        if timestamp & Self::FLAGS_MASK != 0 {
            return None;
        }
        Some(Self::new(
            timestamp
                | Self::LOCK_BY_TIMESTAMP_MASK
                | (if absolute { 0 } else { Self::LOCK_TYPE_FLAG }),
        ))
    }

    pub fn from_epoch(epoch: EpochNumberWithFraction, absolute: bool) -> Self {
        debug_assert!(epoch.full_value() & Self::FLAGS_MASK == 0);
        Self::new(
            epoch.full_value()
                | Self::LOCK_BY_EPOCH_MASK
                | (if absolute { 0 } else { Self::LOCK_TYPE_FLAG }),
        )
    }

    pub fn new(v: u64) -> Self {
        Since(v)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn is_absolute(self) -> bool {
        self.0 & Self::LOCK_TYPE_FLAG == 0
    }

    #[inline]
    pub fn is_relative(self) -> bool {
        !self.is_absolute()
    }

    pub fn flags_is_valid(self) -> bool {
        (self.0 & Self::REMAIN_FLAGS_BITS == 0)
            && ((self.0 & Self::METRIC_TYPE_FLAG_MASK) != Self::METRIC_TYPE_FLAG_MASK)
    }

    pub fn flags(self) -> u64 {
        self.0 & Self::FLAGS_MASK
    }

    pub fn extract_lock_value(self) -> Option<LockValue> {
        let value = self.0 & Self::VALUE_MASK;
        match self.0 & Self::METRIC_TYPE_FLAG_MASK {
            //0b0000_0000
            Self::LOCK_BY_BLOCK_NUMBER_MASK => Some(LockValue::BlockNumber(value)),
            //0b0010_0000
            Self::LOCK_BY_EPOCH_MASK => Some(LockValue::EpochNumberWithFraction(
                EpochNumberWithFraction::from_full_value(value),
            )),
            //0b0100_0000
            Self::LOCK_BY_TIMESTAMP_MASK => Some(LockValue::Timestamp(value * 1000)),
            _ => None,
        }
    }

    /// Given the base commitment block, this method converts a relative since
    /// value to an absolute since value for later comparison.
    #[cfg(feature = "ckb-types")]
    pub fn to_absolute_value(self, base_block: ckb_types::packed::Header) -> Option<Self> {
        debug_assert!(self.is_relative());

        let to_le_u64 = |v: &ckb_types::packed::Uint64| {
            let mut tmp = [0u8; 8];
            tmp.copy_from_slice(&v.raw_data());
            u64::from_le_bytes(tmp)
        };

        match self.extract_lock_value() {
            Some(LockValue::BlockNumber(number)) => to_le_u64(&base_block.raw().number())
                .checked_add(number)
                .and_then(|block_number| Self::from_block_number(block_number, true)),
            Some(LockValue::Timestamp(timestamp)) => to_le_u64(&base_block.raw().timestamp())
                .checked_add(timestamp)
                .and_then(|timestamp| Self::from_timestamp(timestamp, true)),
            Some(LockValue::EpochNumberWithFraction(epoch)) => {
                let base_epoch = EpochNumberWithFraction::from_full_value(
                    to_le_u64(&base_block.raw().epoch()) & Self::VALUE_MASK,
                );
                Some(Self::from_epoch((epoch + base_epoch)?, true))
            }
            _ => None,
        }
    }
}

impl PartialOrd for Since {
    fn partial_cmp(&self, other: &Since) -> Option<Ordering> {
        // Given 2 since values alone, there is no way to compare an absolute value
        // to a relative value. However, a higher level method can convert a relative
        // value to an absolute value.
        if self.is_absolute() != other.is_absolute() {
            return None;
        }

        match (self.extract_lock_value(), other.extract_lock_value()) {
            (Some(LockValue::BlockNumber(a)), Some(LockValue::BlockNumber(b))) => a.partial_cmp(&b),
            (
                Some(LockValue::EpochNumberWithFraction(a)),
                Some(LockValue::EpochNumberWithFraction(b)),
            ) => a.partial_cmp(&b),
            (Some(LockValue::Timestamp(a)), Some(LockValue::Timestamp(b))) => a.partial_cmp(&b),
            _ => None,
        }
    }
}

pub enum LockValue {
    BlockNumber(u64),
    EpochNumberWithFraction(EpochNumberWithFraction),
    Timestamp(u64),
}

impl LockValue {
    pub fn block_number(&self) -> Option<u64> {
        if let Self::BlockNumber(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn epoch(&self) -> Option<EpochNumberWithFraction> {
        if let Self::EpochNumberWithFraction(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn timestamp(&self) -> Option<u64> {
        if let Self::Timestamp(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct EpochNumberWithFraction(u64);

impl EpochNumberWithFraction {
    pub const NUMBER_OFFSET: usize = 0;
    pub const NUMBER_BITS: usize = 24;
    pub const NUMBER_MAXIMUM_VALUE: u64 = (1u64 << Self::NUMBER_BITS);
    pub const NUMBER_MASK: u64 = (Self::NUMBER_MAXIMUM_VALUE - 1);
    pub const INDEX_OFFSET: usize = Self::NUMBER_BITS;
    pub const INDEX_BITS: usize = 16;
    pub const INDEX_MAXIMUM_VALUE: u64 = (1u64 << Self::INDEX_BITS);
    pub const INDEX_MASK: u64 = (Self::INDEX_MAXIMUM_VALUE - 1);
    pub const LENGTH_OFFSET: usize = Self::NUMBER_BITS + Self::INDEX_BITS;
    pub const LENGTH_BITS: usize = 16;
    pub const LENGTH_MAXIMUM_VALUE: u64 = (1u64 << Self::LENGTH_BITS);
    pub const LENGTH_MASK: u64 = (Self::LENGTH_MAXIMUM_VALUE - 1);

    pub fn create(number: u64, index: u64, length: u64) -> Option<EpochNumberWithFraction> {
        if number < Self::NUMBER_MAXIMUM_VALUE
            && index < Self::INDEX_MAXIMUM_VALUE
            && length < Self::LENGTH_MAXIMUM_VALUE
            && length > 0
            && index < length
        {
            Some(Self::new_unchecked(number, index, length))
        } else {
            None
        }
    }

    pub fn new(number: u64, index: u64, length: u64) -> EpochNumberWithFraction {
        debug_assert!(number < Self::NUMBER_MAXIMUM_VALUE);
        debug_assert!(index < Self::INDEX_MAXIMUM_VALUE);
        debug_assert!(length < Self::LENGTH_MAXIMUM_VALUE);
        debug_assert!(length > 0);
        debug_assert!(index < length);
        Self::new_unchecked(number, index, length)
    }

    pub const fn new_unchecked(number: u64, index: u64, length: u64) -> Self {
        EpochNumberWithFraction(
            (length << Self::LENGTH_OFFSET)
                | (index << Self::INDEX_OFFSET)
                | (number << Self::NUMBER_OFFSET),
        )
    }

    pub fn number(self) -> u64 {
        (self.0 >> Self::NUMBER_OFFSET) & Self::NUMBER_MASK
    }

    pub fn index(self) -> u64 {
        (self.0 >> Self::INDEX_OFFSET) & Self::INDEX_MASK
    }

    pub fn length(self) -> u64 {
        (self.0 >> Self::LENGTH_OFFSET) & Self::LENGTH_MASK
    }

    pub fn full_value(self) -> u64 {
        self.0
    }

    // One caveat here, is that if the user specifies a zero epoch length either
    // delibrately, or by accident, calling to_rational() after that might
    // result in a division by zero panic. To prevent that, this method would
    // automatically rewrite the value to epoch index 0 with epoch length to
    // prevent panics
    pub fn from_full_value(value: u64) -> Self {
        let epoch = Self(value);
        if epoch.length() == 0 {
            Self::new(epoch.number(), 0, 1)
        } else {
            epoch
        }
    }
}

impl PartialOrd for EpochNumberWithFraction {
    fn partial_cmp(&self, other: &EpochNumberWithFraction) -> Option<Ordering> {
        if self.number() < other.number() {
            Some(Ordering::Less)
        } else if self.number() > other.number() {
            Some(Ordering::Greater)
        } else {
            let block_a = (self.index() as u128) * (other.length() as u128);
            let block_b = (other.index() as u128) * (self.length() as u128);
            block_a.partial_cmp(&block_b)
        }
    }
}

impl core::ops::Add for EpochNumberWithFraction {
    type Output = Option<EpochNumberWithFraction>;

    fn add(self, rhs: EpochNumberWithFraction) -> Self::Output {
        let mut number = self.number().checked_add(rhs.number())?;

        let mut numerator = ((self.index() as u128) * (rhs.length() as u128))
            .checked_add((rhs.index() as u128) * (self.length() as u128))?;
        let mut denominator = (self.length() as u128) * (rhs.length() as u128);
        let divisor = gcd::binary_u128(numerator, denominator);
        debug_assert!(numerator % divisor == 0);
        debug_assert!(denominator % divisor == 0);
        numerator /= divisor;
        denominator /= divisor;

        let full_epoches = u64::try_from(numerator / denominator).ok()?;
        number = number.checked_add(full_epoches)?;
        numerator %= denominator;

        let numerator = u64::try_from(numerator).ok()?;
        let denominator = u64::try_from(denominator).ok()?;

        EpochNumberWithFraction::create(number, numerator, denominator)
    }
}
