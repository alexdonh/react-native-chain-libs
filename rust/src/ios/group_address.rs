use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::panic::{handle_exception, handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::GroupAddress;

#[no_mangle]
pub unsafe extern "C" fn group_address_get_account_key(
  group_address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    group_address
      .typed_ref::<GroupAddress>()
      .map(|group_address| group_address.get_account_key())
  })
  .map(|public_key| public_key.rptr())
  .response(result, error)
}
