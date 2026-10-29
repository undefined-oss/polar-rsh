use http_body_util::BodyExt;
use presence_rs::Presence;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    errors::CannotCreateOrganizationError,
    types::{
        Organization, OrganizationCustomerEmailSettings, OrganizationCustomerPortalSettings,
        OrganizationDetails, OrganizationFeatureSettingsUpdate, OrganizationLegalEntity,
        OrganizationSocialLink, OrganizationSubscriptionSettings,
    },
};
use crate::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
    models::common::{
        OrganizationName, OrganizationSlug,
        enums::{CountryAlpha2Input, PresentmentCurrency, TaxBehaviorOption},
        errors::HttpValidationError,
        serde_helpers::deserialize_optional_non_null,
    },
};

#[derive(Debug, Clone)]
pub struct OrganizationCreateRequired {
    pub name: OrganizationName,
    pub slug: OrganizationSlug,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationCreate {
    pub name: OrganizationName,
    pub slug: OrganizationSlug,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub avatar_url: Presence<Url>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub email: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub website: Presence<Url>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub subscription_settings: Presence<OrganizationSubscriptionSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_email_settings: Presence<OrganizationCustomerEmailSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_portal_settings: Presence<OrganizationCustomerPortalSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub socials: Presence<Vec<OrganizationSocialLink>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub legal_entity: Presence<OrganizationLegalEntity>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub details: Presence<OrganizationDetails>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub country: Presence<CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub feature_settings: Presence<OrganizationFeatureSettingsUpdate>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub default_presentment_currency: Option<PresentmentCurrency>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub default_tax_behavior: Option<TaxBehaviorOption>,
}

impl OrganizationCreate {
    pub fn new(required: OrganizationCreateRequired) -> Self {
        Self {
            name: required.name,
            slug: required.slug,
            avatar_url: Presence::Absent,
            email: Presence::Absent,
            website: Presence::Absent,
            subscription_settings: Presence::Absent,
            customer_email_settings: Presence::Absent,
            customer_portal_settings: Presence::Absent,
            socials: Presence::Absent,
            legal_entity: Presence::Absent,
            details: Presence::Absent,
            country: Presence::Absent,
            feature_settings: Presence::Absent,
            default_presentment_currency: None,
            default_tax_behavior: None,
        }
    }

    pub fn avatar_url(mut self, value: Url) -> Self {
        self.avatar_url = Presence::Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Presence::Some(value.into());
        self
    }

    pub fn website(mut self, value: Url) -> Self {
        self.website = Presence::Some(value);
        self
    }

    pub fn subscription_settings(mut self, value: OrganizationSubscriptionSettings) -> Self {
        self.subscription_settings = Presence::Some(value);
        self
    }

    pub fn customer_email_settings(mut self, value: OrganizationCustomerEmailSettings) -> Self {
        self.customer_email_settings = Presence::Some(value);
        self
    }

    pub fn customer_portal_settings(mut self, value: OrganizationCustomerPortalSettings) -> Self {
        self.customer_portal_settings = Presence::Some(value);
        self
    }

    pub fn socials(mut self, value: Vec<OrganizationSocialLink>) -> Self {
        self.socials = Presence::Some(value);
        self
    }

    pub fn legal_entity(mut self, value: OrganizationLegalEntity) -> Self {
        self.legal_entity = Presence::Some(value);
        self
    }

    pub fn details(mut self, value: OrganizationDetails) -> Self {
        self.details = Presence::Some(value);
        self
    }

    pub fn country(mut self, value: CountryAlpha2Input) -> Self {
        self.country = Presence::Some(value);
        self
    }

    pub fn feature_settings(mut self, value: OrganizationFeatureSettingsUpdate) -> Self {
        self.feature_settings = Presence::Some(value);
        self
    }

    pub fn default_presentment_currency(mut self, value: PresentmentCurrency) -> Self {
        self.default_presentment_currency = Some(value);
        self
    }

    pub fn default_tax_behavior(mut self, value: TaxBehaviorOption) -> Self {
        self.default_tax_behavior = Some(value);
        self
    }
}

/// Errors returned by the create organization operation.
#[derive(Debug, thiserror::Error)]
pub enum CreateOrganizationError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("cannot create organization: {0}")]
    CannotCreateOrganization(#[source] CannotCreateOrganizationError),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),
}

pub(super) type CreateOrganizationErrors =
    (RequestFailure, CannotCreateOrganizationError, HttpValidationError);

impl<T> From<eros::ErrorUnion<CreateOrganizationErrors>> for CreateOrganizationError<T> {
    fn from(error: eros::ErrorUnion<CreateOrganizationErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E3::B(error) => Self::CannotCreateOrganization(error),
            eros::E3::C(error) => Self::Validation(error),
        }
    }
}
impl<T> CreateOrganizationError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<CreateOrganizationErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}

pub struct CreateOrganization<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) payload: OrganizationCreate,
    pub(super) options: RequestOptions,
}

impl<'a, ClientService> CreateOrganization<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/organizations/");

        let payload_bytes = serde_json::to_vec(&self.payload)?;

        self.client.build_request(
            http::Method::POST,
            &path,
            Some(payload_bytes.into()),
            &self.options,
        )
    }

    pub fn options(mut self, new_options: RequestOptions) -> Self {
        self.options = new_options;
        self
    }
}

impl<'a, ClientService, Body> CreateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<Organization, CreateOrganizationError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            CreateOrganizationError::from_failure(RequestFailure::BuildRequest(error))
        })?;

        let response =
            self.client.send(request).await.map_err(CreateOrganizationError::Transport)?;
        let result: eros::Result<_, CreateOrganizationErrors> = async {
            let response_code = response.status();

            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();

            match response_code {
                http::StatusCode::CREATED => serde_json::from_slice(&body_slice)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error =
                        serde_json::from_slice::<CannotCreateOrganizationError>(&body_slice)
                            .map_err(|error| {
                                eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                            })?;

                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::HttpValidationError,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;

                    Err(eros::ErrorUnion::new(error))
                }
                _ => Err(eros::ErrorUnion::new(RequestFailure::UnexpectedStatus {
                    status: response_code,
                    body: body_slice,
                })),
            }
        }
        .await;
        result.map_err(Into::into)
    }

    pub async fn send(self) -> Result<Organization, CreateOrganizationError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());

        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CreateOrganizationError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for CreateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<Organization, CreateOrganizationError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[cfg(test)]
mod create_payload_tests {
    use serde_json::json;
    use url::Url;

    use super::{
        super::OrganizationSocialPlatforms, OrganizationCreate, OrganizationCreateRequired,
        OrganizationLegalEntity, OrganizationName, OrganizationSlug, OrganizationSocialLink,
        Presence,
    };

    fn payload() -> OrganizationCreate {
        OrganizationCreate::new(OrganizationCreateRequired {
            name: OrganizationName::try_from("Acme").expect("valid test name"),
            slug: OrganizationSlug::try_from("acme").expect("valid test slug"),
        })
    }

    #[test]
    fn constructor_omits_optional_fields_and_deserialization_restores_absence() {
        let expected = json!({"name": "Acme", "slug": "acme"});
        assert_eq!(serde_json::to_value(payload()).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert_eq!(decoded.name.as_ref(), "Acme");
        assert_eq!(decoded.slug.as_ref(), "acme");
        assert!(matches!(decoded.avatar_url, Presence::Absent));
        assert!(matches!(decoded.email, Presence::Absent));
        assert!(matches!(decoded.website, Presence::Absent));
        assert!(matches!(decoded.subscription_settings, Presence::Absent));
        assert!(matches!(decoded.customer_email_settings, Presence::Absent));
        assert!(matches!(decoded.customer_portal_settings, Presence::Absent));
        assert!(matches!(decoded.socials, Presence::Absent));
        assert!(matches!(decoded.legal_entity, Presence::Absent));
    }

    #[test]
    fn preserves_explicit_null_for_each_optional_field() {
        let mut payload = payload();
        payload.avatar_url = Presence::Null;
        payload.email = Presence::Null;
        payload.website = Presence::Null;
        let expected = json!({
            "name": "Acme", "slug": "acme",
            "avatar_url": null, "email": null, "website": null
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert!(matches!(decoded.avatar_url, Presence::Null));
        assert!(matches!(decoded.email, Presence::Null));
        assert!(matches!(decoded.website, Presence::Null));
    }

    #[test]
    fn preserves_values_for_each_optional_field() {
        let payload = payload()
            .avatar_url(Url::parse("https://example.com/avatar.png").expect("valid test URL"))
            .email("old@example.com")
            .email("support@example.com".to_owned())
            .website(Url::parse("https://example.com/").expect("valid test URL"));
        let expected = json!({
            "name": "Acme", "slug": "acme",
            "avatar_url": "https://example.com/avatar.png",
            "email": "support@example.com",
            "website": "https://example.com/"
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert!(
            matches!(decoded.avatar_url, Presence::Some(value) if value.as_str() == "https://example.com/avatar.png")
        );
        assert!(matches!(decoded.email, Presence::Some(value) if value == "support@example.com"));
        assert!(
            matches!(decoded.website, Presence::Some(value) if value.as_str() == "https://example.com/")
        );
    }

    #[test]
    fn url_fields_reject_invalid_values_in_payloads_and_responses() {
        for field in ["avatar_url", "website", "socials"] {
            for invalid in ["not a URL", "https://[invalid"] {
                let value = if field == "socials" {
                    json!([{"platform": "github", "url": invalid}])
                } else {
                    json!(invalid)
                };
                let mut create = json!({"name": "Acme", "slug": "acme"});
                create[field] = value.clone();
                assert!(serde_json::from_value::<OrganizationCreate>(create).is_err());

                let mut update = json!({});
                update[field] = value.clone();
                assert!(
                    serde_json::from_value::<super::super::OrganizationUpdate>(update).is_err()
                );

                let mut response = super::super::item_stream_tests::organization(1);
                response[field] = value;
                assert!(serde_json::from_value::<super::Organization>(response).is_err());
            }
        }
    }

    #[test]
    fn rejects_missing_or_null_required_fields() {
        for value in [
            json!({"slug": "acme"}),
            json!({"name": "Acme"}),
            json!({"name": null, "slug": "acme"}),
            json!({"name": "Acme", "slug": null}),
        ] {
            assert!(
                serde_json::from_value::<OrganizationCreate>(value.clone()).is_err(),
                "expected invalid required fields to be rejected: {value}"
            );
        }
    }

    #[test]
    fn empty_socials_are_present_instead_of_omitted() {
        let omitted = serde_json::to_value(payload()).unwrap();
        assert!(omitted.get("socials").is_none());
        let empty = serde_json::to_value(payload().socials(vec![])).unwrap();
        assert_eq!(empty, json!({"name":"Acme", "slug":"acme", "socials":[]}));
        let decoded: OrganizationCreate = serde_json::from_value(empty).unwrap();
        assert!(matches!(decoded.socials, Presence::Some(links) if links.is_empty()));
    }

    #[test]
    fn socials_preserve_order_and_replace_the_previous_list() {
        let payload = payload()
            .socials(vec![OrganizationSocialLink {
                platform: OrganizationSocialPlatforms::Other,
                url: Url::parse("https://old.example.com").expect("valid test URL"),
            }])
            .socials(vec![
                OrganizationSocialLink {
                    platform: OrganizationSocialPlatforms::Github,
                    url: Url::parse("https://github.com/acme").expect("valid test URL"),
                },
                OrganizationSocialLink {
                    platform: OrganizationSocialPlatforms::X,
                    url: Url::parse("https://x.com/acme").expect("valid test URL"),
                },
            ]);
        let expected = json!({
            "name":"Acme", "slug":"acme",
            "socials":[
                {"platform":"github","url":"https://github.com/acme"},
                {"platform":"x","url":"https://x.com/acme"}
            ]
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);
        let decoded: OrganizationCreate = serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    }

    #[test]
    fn rejects_incomplete_social_links_and_unknown_platforms() {
        for link in [
            json!({"url":"https://github.com/acme"}),
            json!({"platform":"github"}),
            json!({"platform":"unknown","url":"https://example.com/"}),
        ] {
            let result = serde_json::from_value::<OrganizationCreate>(json!({
                "name":"Acme", "slug":"acme", "socials":[link]
            }));
            assert!(result.is_err(), "invalid social link must fail deserialization");
        }
    }

    #[test]
    fn legal_entities_use_the_expected_tagged_shapes() {
        for (entity, expected) in [
            (OrganizationLegalEntity::Individual, json!({"type":"individual"})),
            (
                OrganizationLegalEntity::Company { registered_name: "Acme Ltd".into() },
                json!({"type":"company", "registered_name":"Acme Ltd"}),
            ),
        ] {
            let encoded = serde_json::to_value(payload().legal_entity(entity)).unwrap();
            assert_eq!(encoded, json!({"name":"Acme", "slug":"acme", "legal_entity":expected}));
            let decoded: OrganizationCreate = serde_json::from_value(encoded).unwrap();
            match decoded.legal_entity {
                Presence::Some(OrganizationLegalEntity::Individual) => {
                    assert_eq!(expected["type"], "individual");
                }
                Presence::Some(OrganizationLegalEntity::Company { registered_name }) => {
                    assert_eq!(expected["type"], "company");
                    assert_eq!(registered_name, "Acme Ltd");
                }
                other => panic!("expected a populated legal entity, got {other:?}"),
            }
        }
    }

    #[test]
    fn rejects_missing_or_invalid_legal_entity_discriminators_and_company_names() {
        for entity in [
            json!({}),
            json!({"registered_name":"Acme Ltd"}),
            json!({"type":"unknown"}),
            json!({"type":null}),
            json!({"type":"company"}),
            json!({"type":"company", "registered_name":null}),
        ] {
            let result = serde_json::from_value::<OrganizationCreate>(json!({
                "name":"Acme", "slug":"acme", "legal_entity":entity
            }));
            assert!(result.is_err(), "invalid legal entity must fail deserialization");
        }
    }

    #[test]
    fn legal_entity_setter_replaces_the_previous_variant() {
        let company = payload()
            .legal_entity(OrganizationLegalEntity::Individual)
            .legal_entity(OrganizationLegalEntity::Company { registered_name: "Acme Ltd".into() });
        assert_eq!(
            serde_json::to_value(&company).unwrap()["legal_entity"],
            json!({"type":"company", "registered_name":"Acme Ltd"})
        );
        let individual = company.legal_entity(OrganizationLegalEntity::Individual);
        assert_eq!(
            serde_json::to_value(individual).unwrap()["legal_entity"],
            json!({"type":"individual"})
        );
    }
}
