use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{PerCertificateFee, Value};

#[no_mangle]
pub extern "C" fn per_certificate_fee_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| PerCertificateFee::new())
    .map(|per_certificate_fee| per_certificate_fee.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn per_certificate_fee_set_pool_registration(
  per_certificate_fee: RPtr, val: RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    per_certificate_fee.typed_ref::<PerCertificateFee>().zip(val.typed_ref::<Value>()).map(
      |(per_certificate_fee, val)| {
        PerCertificateFee::set_pool_registration(per_certificate_fee, val)
      }
    )
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn per_certificate_fee_set_stake_delegation(
  per_certificate_fee: RPtr, val: RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    per_certificate_fee.typed_ref::<PerCertificateFee>().zip(val.typed_ref::<Value>()).map(
      |(per_certificate_fee, val)| {
        PerCertificateFee::set_stake_delegation(per_certificate_fee, val)
      }
    )
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn per_certificate_fee_set_owner_stake_delegation(
  per_certificate_fee: RPtr, val: RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    per_certificate_fee.typed_ref::<PerCertificateFee>().zip(val.typed_ref::<Value>()).map(
      |(per_certificate_fee, val)| {
        PerCertificateFee::set_owner_stake_delegation(per_certificate_fee, val)
      }
    )
  })
  .response(&mut (), error)
}
