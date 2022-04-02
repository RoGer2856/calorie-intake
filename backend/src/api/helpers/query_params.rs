pub struct MissingAccessTokenInQueryParams;

impl From<MissingAccessTokenInQueryParams> for crate::hyper_helpers::ErrorResponse {
    fn from(_: MissingAccessTokenInQueryParams) -> Self {
        let msg = crate::api::helpers::ErrorMessage {
            reason: "access_token should be provided in query params".to_string(),
        };

        msg.into()
    }
}

pub fn get_access_token_from_query_params(
    query_params: &str,
) -> Result<String, MissingAccessTokenInQueryParams> {
    let mut access_token = None;

    for param in querystring::querify(query_params) {
        let name = param.0;
        let value = param.1;

        if name.to_lowercase() == "access_token" {
            access_token = Some(value.into());
            break;
        }
    }

    access_token.ok_or(MissingAccessTokenInQueryParams)
}
