use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{PayloadAuthData, StakeDelegationAuthData};

#[no_mangle]
pub unsafe extern "C" fn payload_auth_data_for_no_payload(
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| PayloadAuthData::for_no_payload())
    .map(|payload_auth_data| payload_auth_data.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn payload_auth_data_for_stake_delegation(
  stake_delegation_auth_data: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    stake_delegation_auth_data.typed_ref::<StakeDelegationAuthData>()
        .map(|stake_delegation_auth_data| PayloadAuthData::for_stake_delegation(stake_delegation_auth_data))
  })
  .map(|payload_auth_data| payload_auth_data.rptr())
  .response(result, error)
}
