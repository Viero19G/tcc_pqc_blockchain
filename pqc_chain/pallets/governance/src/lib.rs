//! # Entangle Governance
//!
//! Governança on-chain simplificada para a Fase 2.
//!
//! - Propostas com depósito em **Strand (STR)**
//! - Votação ponderada pelo saldo livre de STR
//! - Execução automática de calls aprovados
//!
//! Evolui para OpenGov (`pallet-referenda`) na Fase 3+.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use alloc::boxed::Box;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{traits::{Dispatchable, Saturating}, Permill};

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Estado de uma proposta.
	#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ProposalStatus {
		Active,
		Approved,
		Rejected,
		Cancelled,
	}

	/// Proposta de governança.
	#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal<T: Config> {
		pub proposer: T::AccountId,
		pub call: Box<<T as Config>::RuntimeCall>,
		pub deposit: BalanceOf<T>,
		pub end_block: BlockNumberFor<T>,
		pub yes_votes: BalanceOf<T>,
		pub no_votes: BalanceOf<T>,
		pub status: ProposalStatus,
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ From<frame_system::Call<Self>>
			+ IsType<<Self as frame_system::Config>::RuntimeCall>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type WeightInfo: WeightInfo;

		/// Depósito mínimo para submeter proposta (STR).
		#[pallet::constant]
		type MinProposalDeposit: Get<BalanceOf<Self>>;

		/// Período de votação em blocos.
		#[pallet::constant]
		type VotingPeriod: Get<BlockNumberFor<Self>>;

		/// Quorum mínimo (fração do total issuance votante).
		#[pallet::constant]
		type MinTurnout: Get<Permill>;

		/// Aprovação mínima (fração dos votos a favor).
		#[pallet::constant]
		type MinApproval: Get<Permill>;
	}

	/// Propostas ativas e históricas.
	#[pallet::storage]
	pub type Proposals<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Proposal<T>, OptionQuery>;

	/// Votos registrados: (proposal_id, voter) => aye.
	#[pallet::storage]
	pub type Votes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		T::AccountId,
		bool,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type NextProposalId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Proposed { proposal_id: u32, proposer: T::AccountId, deposit: BalanceOf<T> },
		Voted { proposal_id: u32, voter: T::AccountId, aye: bool, weight: BalanceOf<T> },
		Executed { proposal_id: u32 },
		Cancelled { proposal_id: u32 },
		Rejected { proposal_id: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		ProposalNotFound,
		ProposalNotActive,
		AlreadyVoted,
		InsufficientDeposit,
		VotingPeriodNotEnded,
		VotingPeriodEnded,
		QuorumNotReached,
		NotApproved,
		NotProposer,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submete uma proposta de governança com depósito em STR.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::propose())]
		pub fn propose(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
			deposit: BalanceOf<T>,
		) -> DispatchResult {
			let proposer = ensure_signed(origin)?;
			ensure!(deposit >= T::MinProposalDeposit::get(), Error::<T>::InsufficientDeposit);

			T::Currency::reserve(&proposer, deposit)?;

			let proposal_id = NextProposalId::<T>::mutate(|id| {
				let current = *id;
				*id = id.saturating_add(1);
				current
			});

			let end_block = frame_system::Pallet::<T>::block_number()
				.saturating_add(T::VotingPeriod::get());

			Proposals::<T>::insert(
				proposal_id,
				Proposal {
					proposer: proposer.clone(),
					call,
					deposit,
					end_block,
					yes_votes: BalanceOf::<T>::zero(),
					no_votes: BalanceOf::<T>::zero(),
					status: ProposalStatus::Active,
				},
			);

			Self::deposit_event(Event::Proposed { proposal_id, proposer, deposit });
			Ok(())
		}

		/// Vota em uma proposta (peso = saldo livre de STR).
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::vote())]
		pub fn vote(origin: OriginFor<T>, proposal_id: u32, aye: bool) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			let mut proposal =
				Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
			ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
			ensure!(
				frame_system::Pallet::<T>::block_number() < proposal.end_block,
				Error::<T>::VotingPeriodEnded
			);
			ensure!(!Votes::<T>::contains_key(proposal_id, &voter), Error::<T>::AlreadyVoted);

			let weight = T::Currency::free_balance(&voter);
			ensure!(!weight.is_zero(), Error::<T>::InsufficientDeposit);

			if aye {
				proposal.yes_votes = proposal.yes_votes.saturating_add(weight);
			} else {
				proposal.no_votes = proposal.no_votes.saturating_add(weight);
			}

			Votes::<T>::insert(proposal_id, &voter, aye);
			Proposals::<T>::insert(proposal_id, proposal);

			Self::deposit_event(Event::Voted { proposal_id, voter, aye, weight });
			Ok(())
		}

		/// Fecha e executa (ou rejeita) uma proposta após o período de votação.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::close(*proposal_id))]
		pub fn close(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
			ensure_signed(origin)?;
			let mut proposal =
				Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
			ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
			ensure!(
				frame_system::Pallet::<T>::block_number() >= proposal.end_block,
				Error::<T>::VotingPeriodNotEnded
			);

			let total_votes = proposal.yes_votes.saturating_add(proposal.no_votes);
			let turnout = Self::turnout_ratio(total_votes);
			let approval = Self::approval_ratio(proposal.yes_votes, total_votes);

			let approved = turnout >= T::MinTurnout::get() && approval >= T::MinApproval::get();

			if approved {
				proposal.status = ProposalStatus::Approved;
				Proposals::<T>::insert(proposal_id, &proposal);

				let call = proposal.call.clone();
				let _ = call.dispatch(frame_system::RawOrigin::Root.into()).map_err(|err| err.error)?;

				T::Currency::unreserve(&proposal.proposer, proposal.deposit);
				Self::deposit_event(Event::Executed { proposal_id });
			} else {
				proposal.status = ProposalStatus::Rejected;
				Proposals::<T>::insert(proposal_id, &proposal);
				// Depósito perdido (slash para treasury futuro — queimado por ora).
				let _ = T::Currency::slash_reserved(&proposal.proposer, proposal.deposit);
				Self::deposit_event(Event::Rejected { proposal_id });
			}

			Ok(())
		}

		/// Cancela proposta ativa (somente o proposer, antes do fim da votação).
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::cancel())]
		pub fn cancel(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut proposal =
				Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
			ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
			ensure!(proposal.proposer == who, Error::<T>::NotProposer);

			proposal.status = ProposalStatus::Cancelled;
			Proposals::<T>::insert(proposal_id, &proposal);
			T::Currency::unreserve(&proposal.proposer, proposal.deposit);

			Self::deposit_event(Event::Cancelled { proposal_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn turnout_ratio(voted: BalanceOf<T>) -> Permill {
			let total = T::Currency::total_issuance();
			if total.is_zero() {
				return Permill::zero();
			}
			Permill::from_rational(voted, total)
		}

		fn approval_ratio(yes: BalanceOf<T>, total: BalanceOf<T>) -> Permill {
			if total.is_zero() {
				return Permill::zero();
			}
			Permill::from_rational(yes, total)
		}
	}
}
