use crate::{mock::*, Error, Event, Pallet as Governance, Proposals, ProposalStatus};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn propose_and_vote_works() {
	new_test_ext().execute_with(|| {
		let call = Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![1] }));
		assert_ok!(Governance::propose(RuntimeOrigin::signed(1), call, 200));

		assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true));
		assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, false));

		let proposal = Proposals::<Test>::get(0).unwrap();
		assert_eq!(proposal.yes_votes, 500);
		assert_eq!(proposal.no_votes, 500);
	});
}

#[test]
fn double_vote_fails() {
	new_test_ext().execute_with(|| {
		let call = Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![] }));
		assert_ok!(Governance::propose(RuntimeOrigin::signed(1), call, 200));
		assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true));
		assert_noop!(
			Governance::vote(RuntimeOrigin::signed(2), 0, false),
			Error::<Test>::AlreadyVoted
		);
	});
}

#[test]
fn cancel_returns_deposit() {
	new_test_ext().execute_with(|| {
		let call = Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![] }));
		assert_ok!(Governance::propose(RuntimeOrigin::signed(1), call, 200));
		let reserved_before = Balances::reserved_balance(1);
		assert_eq!(reserved_before, 200);

		assert_ok!(Governance::cancel(RuntimeOrigin::signed(1), 0));
		assert_eq!(Balances::reserved_balance(1), 0);

		let proposal = Proposals::<Test>::get(0).unwrap();
		assert_eq!(proposal.status, ProposalStatus::Cancelled);
	});
}

#[test]
fn close_before_period_fails() {
	new_test_ext().execute_with(|| {
		let call = Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![] }));
		assert_ok!(Governance::propose(RuntimeOrigin::signed(1), call, 200));
		assert_noop!(
			Governance::close(RuntimeOrigin::signed(2), 0),
			Error::<Test>::VotingPeriodNotEnded
		);
	});
}
