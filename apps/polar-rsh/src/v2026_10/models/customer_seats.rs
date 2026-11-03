pub mod assign_seat;
pub mod claim_seat;
pub mod get_claim_info;
pub mod list_seats;
pub mod resend_invitation;
pub mod revoke_seat;
pub mod types;
pub use self::{
    assign_seat::*, claim_seat::*, get_claim_info::*, list_seats::*, resend_invitation::*,
    revoke_seat::*, types::*,
};
pub struct CustomerSeats<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerSeats<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Assign a seat for a subscription or order.
    ///
    /// **Scopes**: `customer_seats:write`
    pub fn assign_seat(
        &self,
        payload: crate::v2026_10::models::customer_seats::types::SeatAssign,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsAssignSeat<'a, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeatsAssignSeat {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Claim a customer seat with an invitation token.
    pub fn claim_seat(
        &self,
        payload: crate::v2026_10::models::customer_seats::types::SeatClaim,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsClaimSeat<'a, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeatsClaimSeat {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get claim information for a customer seat invitation.
    pub fn get_claim_info(
        &self,
        invitation_token: String,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsGetClaimInfo<'a, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeatsGetClaimInfo {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            invitation_token,
        }
    }

    /// List seats for a subscription or order.
    ///
    /// **Scopes**: `customer_seats:read`
    pub fn list_seats(
        &self,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsListSeats<'a, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeatsListSeats {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Resend an invitation for a pending customer seat.
    ///
    /// **Scopes**: `customer_seats:write`
    pub fn resend_invitation(
        &self,
        seat_id: crate::v2026_10::models::common::ids::SeatId,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsResendInvitation<'a, ClientService>
    {
        crate::v2026_10::models::customer_seats::CustomerSeatsResendInvitation {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            seat_id,
        }
    }

    /// Revoke a customer seat.
    ///
    /// **Scopes**: `customer_seats:write`
    pub fn revoke_seat(
        &self,
        seat_id: crate::v2026_10::models::common::ids::SeatId,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeatsRevokeSeat<'a, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeatsRevokeSeat {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            seat_id,
        }
    }
}
