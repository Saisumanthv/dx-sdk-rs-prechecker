use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MoaxOrDctTokenIdentifier},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::DctTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct MoaxOrDctTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: MoaxOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> MoaxOrDctTokenPayment<M> {
    pub fn no_payment() -> Self {
        MoaxOrDctTokenPayment {
            token_identifier: MoaxOrDctTokenIdentifier::moax(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: MoaxOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        MoaxOrDctTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just DCT or terminate execution if the token is MOAX.
    pub fn unwrap_dct(self) -> DctTokenPayment<M> {
        DctTokenPayment::new(
            self.token_identifier.unwrap_dct(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (MoaxOrDctTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(MoaxOrDctTokenIdentifier<M>, u64, BigUint<M>)>
    for MoaxOrDctTokenPayment<M>
{
    #[inline]
    fn from(value: (MoaxOrDctTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for MoaxOrDctTokenPayment<M> {
    fn from(dct_payment: DctTokenPayment<M>) -> Self {
        MoaxOrDctTokenPayment {
            token_identifier: MoaxOrDctTokenIdentifier::dct(dct_payment.token_identifier),
            token_nonce: dct_payment.token_nonce,
            amount: dct_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for MoaxOrDctTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for MoaxOrDctTokenPayment<M> where M: ManagedTypeApi {}
