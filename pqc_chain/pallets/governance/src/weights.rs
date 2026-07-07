use frame_support::weights::Weight;

pub trait WeightInfo {
	fn propose() -> Weight;
	fn vote() -> Weight;
	fn close(_proposal_id: u32) -> Weight;
	fn cancel() -> Weight;
}

pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
impl<T> WeightInfo for SubstrateWeight<T> {
	fn propose() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}
	fn vote() -> Weight {
		Weight::from_parts(30_000_000, 0)
	}
	fn close(_proposal_id: u32) -> Weight {
		Weight::from_parts(100_000_000, 0)
	}
	fn cancel() -> Weight {
		Weight::from_parts(25_000_000, 0)
	}
}

impl WeightInfo for () {
	fn propose() -> Weight {
		Weight::from_parts(50_000_000, 0)
	}
	fn vote() -> Weight {
		Weight::from_parts(30_000_000, 0)
	}
	fn close(_proposal_id: u32) -> Weight {
		Weight::from_parts(100_000_000, 0)
	}
	fn cancel() -> Weight {
		Weight::from_parts(25_000_000, 0)
	}
}
