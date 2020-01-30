use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::PoolId;

#[no_mangle]
pub extern "C" fn pool_id_from_hex(
  hex_string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| PoolId::from_hex(hex_string.into_str()).into_result())
    .map(|pool_id| pool_id.rptr())
    .response(result, error)
}
