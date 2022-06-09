use super::*;
use sp_runtime::DispatchError;

pub struct BadOrigin;

impl From<BadOrigin> for &'static str {
	fn from(_: BadOrigin) -> &'static str {
		"Bad origin"
	}
}

pub trait BaseDaoCallFilter<Call> {
	fn contains(&self, call: Call) -> bool;
}
pub trait GetCollectiveMembersChecked<
	AccountId: Clone + Ord,
	DaoId: Clone + Default + Copy,
	DispathErr,
>
{
	fn get_members_sorted(
		dao_id: DaoId,
		members: &[AccountId],
	) -> result::Result<Vec<AccountId>, DispathErr>;
}

pub trait GetCollectiveMembers<AccountId: Clone + Ord, DaoId: Clone + Default + Copy> {
	fn get_members(dao_id: DaoId) -> Vec<AccountId>;
	fn get_prime(dao_id: DaoId) -> Option<AccountId>;
}

impl<AccountId: Clone + Ord, DaoId: Clone + Default + Copy> GetCollectiveMembers<AccountId, DaoId>
	for ()
{
	fn get_members(dao_id: DaoId) -> Vec<AccountId> {
		vec![]
	}

	fn get_prime(dao_id: DaoId) -> Option<AccountId> {
		None
	}
}

pub trait Checked<AccountId: Clone + Ord, DaoId: Clone, DispatchError> {
	fn is_can_create(&self, who: AccountId, dao_id: DaoId) -> result::Result<(), DispatchError>;
}

impl<AccountId: Clone + Ord, DaoId: Clone + Default + Copy>
	GetCollectiveMembersChecked<AccountId, DaoId, DispatchError> for ()
{
	fn get_members_sorted(
		dao_id: DaoId,
		members: &[AccountId],
	) -> result::Result<Vec<AccountId>, DispatchError> {
		Ok(Vec::new())
	}
}

/// Trait for type that can handle incremental changes to a set of account IDs.
pub trait ChangeMembers<AccountId: Clone + Ord, DaoId: Clone + Default + Copy> {
	/// A number of members `incoming` just joined the set and replaced some `outgoing` ones. The
	/// new set is given by `new`, and need not be sorted.
	///
	/// This resets any previous value of prime.
	fn change_members(
		dao_id: DaoId,
		incoming: &[AccountId],
		outgoing: &[AccountId],
		mut new: Vec<AccountId>,
	) {
		new.sort();
		Self::change_members_sorted(dao_id, incoming, outgoing, &new[..]);
	}

	/// A number of members `_incoming` just joined the set and replaced some `_outgoing` ones. The
	/// new set is thus given by `sorted_new` and **must be sorted**.
	///
	/// NOTE: This is the only function that needs to be implemented in `ChangeMembers`.
	///
	/// This resets any previous value of prime.
	fn change_members_sorted(
		dao_id: DaoId,
		incoming: &[AccountId],
		outgoing: &[AccountId],
		sorted_new: &[AccountId],
	);

	/// Set the new members; they **must already be sorted**. This will compute the diff and use it
	/// to call `change_members_sorted`.
	///
	/// This resets any previous value of prime.
	fn set_members_sorted(dao_id: DaoId, new_members: &[AccountId], old_members: &[AccountId]) {
		let (incoming, outgoing) = Self::compute_members_diff_sorted(new_members, old_members);
		Self::change_members_sorted(dao_id, &incoming[..], &outgoing[..], &new_members);
	}

	/// Compute diff between new and old members; they **must already be sorted**.
	///
	/// Returns incoming and outgoing members.
	fn compute_members_diff_sorted(
		new_members: &[AccountId],
		old_members: &[AccountId],
	) -> (Vec<AccountId>, Vec<AccountId>) {
		let mut old_iter = old_members.iter();
		let mut new_iter = new_members.iter();
		let mut incoming = Vec::new();
		let mut outgoing = Vec::new();
		let mut old_i = old_iter.next();
		let mut new_i = new_iter.next();
		loop {
			match (old_i, new_i) {
				(None, None) => break,
				(Some(old), Some(new)) if old == new => {
					old_i = old_iter.next();
					new_i = new_iter.next();
				},
				(Some(old), Some(new)) if old < new => {
					outgoing.push(old.clone());
					old_i = old_iter.next();
				},
				(Some(old), None) => {
					outgoing.push(old.clone());
					old_i = old_iter.next();
				},
				(_, Some(new)) => {
					incoming.push(new.clone());
					new_i = new_iter.next();
				},
			}
		}
		(incoming, outgoing)
	}

	/// Set the prime member.
	fn set_prime(dao_id: DaoId, _prime: Option<AccountId>) {}

	/// Get the current prime.
	fn get_prime(dao_id: DaoId) -> Option<AccountId> {
		None
	}
}

/// Some sort of check on the origin is performed by this object.
pub trait EnsureOriginWithArg<OuterOrigin, Argument> {
	/// A return type.
	type Success;

	/// Perform the origin check.
	fn ensure_origin(o: OuterOrigin, a: &Argument) -> Result<Self::Success, BadOrigin> {
		Self::try_origin(o, a).map_err(|_| BadOrigin)
	}

	/// Perform the origin check, returning the origin value if unsuccessful. This allows chaining.
	fn try_origin(o: OuterOrigin, a: &Argument) -> Result<Self::Success, OuterOrigin>;

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin(a: &Argument) -> OuterOrigin;
}
