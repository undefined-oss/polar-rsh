pub mod assign_seat;
pub mod list_claimed_subscriptions;
pub mod list_seats;
pub mod resend_invitation;
pub mod revoke_seat;
pub use self::{
    assign_seat::*, list_claimed_subscriptions::*, list_seats::*, resend_invitation::*,
    revoke_seat::*,
};
pub struct CustomerPortalSeats<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalSeats<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Assign Seat
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn assign_seat(
        &self,
        payload: crate::v2027_01::models::seats::types::CustomerSeatAssign,
    ) -> crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsAssignSeat<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsAssignSeat {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// List all subscriptions where the authenticated customer has claimed a
    /// seat.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list_claimed_subscriptions(
        &self,
    ) -> crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsListClaimedSubscriptions<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsListClaimedSubscriptions {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list_seats(
        &self,
    ) -> crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsListSeats<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsListSeats {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Resend Invitation
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn resend_invitation(
        &self,
        seat_id: crate::v2027_01::models::common::ids::SeatId,
    ) -> crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsResendInvitation<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsResendInvitation {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            seat_id,
        }
    }

    /// Revoke Seat
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn revoke_seat(
        &self,
        seat_id: crate::v2027_01::models::common::ids::SeatId,
    ) -> crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsRevokeSeat<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::seats::CustomerPortalSeatsRevokeSeat {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            seat_id,
        }
    }
}
