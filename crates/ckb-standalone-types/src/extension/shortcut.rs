use crate::{
    bytes,
    core::{self, BlockNumber},
    packed,
    prelude::*,
};

impl packed::Byte32 {
    /// Creates a new `Bytes32` whose bits are all zeros.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Creates a new `Byte32` whose bits are all ones.
    pub fn max_value() -> Self {
        [u8::max_value(); 32].pack()
    }

    /// Checks whether all bits in self are zeros.
    pub fn is_zero(&self) -> bool {
        self.as_slice().iter().all(|x| *x == 0)
    }

    /// Creates a new `Bytes32`.
    pub fn new(v: [u8; 32]) -> Self {
        v.pack()
    }
}

impl packed::ProposalShortId {
    /// Creates a new `ProposalShortId` from a transaction hash.
    pub fn from_tx_hash(h: &packed::Byte32) -> Self {
        let mut inner = [0u8; 10];
        inner.copy_from_slice(&h.as_slice()[..10]);
        inner.pack()
    }

    /// Creates a new `ProposalShortId` whose bits are all zeros.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Creates a new `ProposalShortId`.
    pub fn new(v: [u8; 10]) -> Self {
        v.pack()
    }
}

impl packed::OutPoint {
    /// Creates a new `OutPoint`.
    pub fn new(tx_hash: packed::Byte32, index: u32) -> Self {
        packed::OutPoint::new_builder()
            .tx_hash(tx_hash)
            .index(index.pack())
            .build()
    }

    /// Creates a new null `OutPoint`.
    pub fn null() -> Self {
        packed::OutPoint::new_builder()
            .index(u32::max_value().pack())
            .build()
    }

    /// Checks whether self is a null `OutPoint`.
    pub fn is_null(&self) -> bool {
        self.tx_hash().is_zero() && Unpack::<u32>::unpack(&self.index()) == u32::max_value()
    }
}

impl packed::CellInput {
    /// Creates a new `CellInput`.
    pub fn new(previous_output: packed::OutPoint, block_number: BlockNumber) -> Self {
        packed::CellInput::new_builder()
            .since(block_number.pack())
            .previous_output(previous_output)
            .build()
    }
    /// Creates a new `CellInput` with a null `OutPoint`.
    pub fn new_cellbase_input(block_number: BlockNumber) -> Self {
        Self::new(packed::OutPoint::null(), block_number)
    }
}

impl packed::Script {
    /// Converts self into bytes of [`CellbaseWitness`](struct.CellbaseWitness.html).
    pub fn into_witness(self) -> packed::Bytes {
        packed::CellbaseWitness::new_builder()
            .lock(self)
            .build()
            .as_bytes()
            .pack()
    }

    /// Converts from bytes of [`CellbaseWitness`](struct.CellbaseWitness.html).
    pub fn from_witness(witness: packed::Bytes) -> Option<Self> {
        packed::CellbaseWitness::from_slice(&witness.raw_data())
            .map(|cellbase_witness| cellbase_witness.lock())
            .ok()
    }

    /// Checks whether the own [`hash_type`](#method.hash_type) is
    /// [`Type`](../core/enum.ScriptHashType.html#variant.Type).
    pub fn is_hash_type_type(&self) -> bool {
        Into::<u8>::into(self.hash_type()) == Into::<u8>::into(core::ScriptHashType::Type)
    }
}

impl packed::Transaction {
    /// Checks whether self is a cellbase.
    pub fn is_cellbase(&self) -> bool {
        let raw_tx = self.raw();
        raw_tx.inputs().len() == 1
            && self.witnesses().len() == 1
            && raw_tx.inputs().get(0).unwrap().previous_output().is_null()
    }

    /// Generates a proposal short id after calculating the transaction hash.
    pub fn proposal_short_id(&self) -> packed::ProposalShortId {
        packed::ProposalShortId::from_tx_hash(&self.calc_tx_hash())
    }
}

impl packed::Block {
    /// Converts self to an uncle block.
    pub fn as_uncle(&self) -> packed::UncleBlock {
        packed::UncleBlock::new_builder()
            .header(self.header())
            .proposals(self.proposals())
            .build()
    }

    /// Gets the i-th extra field if it exists; i started from 0.
    pub fn extra_field(&self, index: usize) -> Option<bytes::Bytes> {
        let count = self.count_extra_fields();
        if count > index {
            let slice = self.as_slice();
            let i = (1 + Self::FIELD_COUNT + index) * molecule::NUMBER_SIZE;
            let start = molecule::unpack_number(&slice[i..]) as usize;
            if count == index + 1 {
                Some(self.as_bytes().slice(start..))
            } else {
                let j = i + molecule::NUMBER_SIZE;
                let end = molecule::unpack_number(&slice[j..]) as usize;
                Some(self.as_bytes().slice(start..end))
            }
        } else {
            None
        }
    }

    /// Gets the extension field if it existed.
    ///
    /// # Panics
    ///
    /// Panics if the first extra field exists but not a valid [`Bytes`](struct.Bytes.html).
    pub fn extension(&self) -> Option<packed::Bytes> {
        self.extra_field(0)
            .map(|data| packed::Bytes::from_slice(&data).unwrap())
    }
}

impl packed::BlockV1 {
    /// Converts to a compatible [`Block`](struct.Block.html) with an extra field.
    pub fn as_v0(&self) -> packed::Block {
        packed::Block::new_unchecked(self.as_bytes())
    }
}

impl<'r> packed::BlockReader<'r> {
    /// Gets the i-th extra field if it exists; i started from 0.
    pub fn extra_field(&self, index: usize) -> Option<&[u8]> {
        let count = self.count_extra_fields();
        if count > index {
            let slice = self.as_slice();
            let i = (1 + Self::FIELD_COUNT + index) * molecule::NUMBER_SIZE;
            let start = molecule::unpack_number(&slice[i..]) as usize;
            if count == index + 1 {
                Some(&self.as_slice()[start..])
            } else {
                let j = i + molecule::NUMBER_SIZE;
                let end = molecule::unpack_number(&slice[j..]) as usize;
                Some(&self.as_slice()[start..end])
            }
        } else {
            None
        }
    }

    /// Gets the extension field if it existed.
    ///
    /// # Panics
    ///
    /// Panics if the first extra field exists but not a valid [`BytesReader`](struct.BytesReader.html).
    pub fn extension(&self) -> Option<packed::BytesReader> {
        self.extra_field(0)
            .map(|data| packed::BytesReader::from_slice(data).unwrap())
    }
}

impl<'r> packed::BlockV1Reader<'r> {
    /// Converts to a compatible [`BlockReader`](struct.BlockReader.html) with an extra field.
    pub fn as_v0(&self) -> packed::BlockReader {
        packed::BlockReader::new_unchecked(self.as_slice())
    }
}
