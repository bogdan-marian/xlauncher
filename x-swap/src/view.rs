elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ Offer };

#[elrond_wasm::module]
pub trait ViewModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{    
    #[view(getOffers)]
    fn get_offers(
        &self,
    ) -> ManagedVec<Offer<Self::Api>> {
        let offer_ids = self.offer_ids();
        let mut offers = ManagedVec::new();
        for offer_id in offer_ids.iter() {
            offers.push(self.offers(offer_id).get());
        }
        offers
    }

    #[view(getOffersByFromToken)]
    fn get_offers_by_from_token(
        &self,
        from_token: TokenIdentifier,
    ) -> ManagedVec<Offer<Self::Api>> {
        let offer_ids_by_from_token = self.offer_ids_by_from_token(&from_token);
        let mut offers = ManagedVec::new();
        for offer_id in offer_ids_by_from_token.iter() {
            offers.push(self.offers(offer_id).get());
        }
        offers
    }

    #[view(getOffersByToToken)]
    fn get_offers_by_to_token(
        &self,
        to_token: TokenIdentifier,
    ) -> ManagedVec<Offer<Self::Api>> {
        let offer_ids_by_to_token = self.offer_ids_by_to_token(&to_token);
        let mut offers = ManagedVec::new();
        for offer_id in offer_ids_by_to_token.iter() {
            offers.push(self.offers(offer_id).get());
        }
        offers
    }
}