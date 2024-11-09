use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use thiserror::Error;

use std::env;

#[derive(Error, Debug, PartialEq)]
pub enum VerifyHeadersError {
    #[error("X-Shopify-Topic header is missing")]
    MissingTopic,

    #[error("X-Shopify-Webhook-Id header is missing")]
    MissingWebhookId,

    #[error("X-Shopify-Event-Id header is missing")]
    MissingEventId,

    #[error("X-Shopify-Shop-Domain header is missing")]
    MissingShopDomain,

    #[error("X-Shopify-Shop-Domain header is incorrect")]
    IncorrectShopDomain,

    #[error("X-Shopify-Hmac-Sha256 header is missing")]
    MissingHmacSha256,

    #[error("X-Shopify-Hmac-Sha256 header is incorrect")]
    IncorrectHmacSha256,

    #[error("X-Shopify-Api-Version header is missing")]
    MissingApiVersion,

    #[error("X-Shopify-Api-Version header is incorrect")]
    IncorrectApiVersion,
}

#[derive(Error, Debug, PartialEq)]
pub enum VerifyHmacSha256Error {
    #[error("HMAC-SHA256 length is incorrect")]
    IncorrectLength,

    #[error("HMAC-SHA256 is invalid")]
    InvalidHmacSha256,
}

pub async fn verify_shopify_origin(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    if let Err(e) = verify_headers(&req.headers()) {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    // TODO: Verify it is not duplicate event

    Ok(next.run(req).await)
}

fn verify_headers(headers: &HeaderMap) -> Result<(), VerifyHeadersError> {
    headers.get("X-Shopify-Topic").ok_or(VerifyHeadersError::MissingTopic)?;
    headers.get("X-Shopify-Webhook-Id").ok_or(VerifyHeadersError::MissingWebhookId)?;
    headers.get("X-Shopify-Event-Id").ok_or(VerifyHeadersError::MissingEventId)?;

    let shop_domain = headers.get("X-Shopify-Shop-Domain").ok_or(VerifyHeadersError::MissingShopDomain)?;
    if shop_domain.to_str().unwrap() != env::var("shopify_shop_url").unwrap() {
        return Err(VerifyHeadersError::IncorrectShopDomain);
    }

    let hmac_sha256 = headers.get("X-Shopify-Hmac-Sha256").ok_or(VerifyHeadersError::MissingHmacSha256)?;
    if verify_hmac_sha256(hmac_sha256.as_bytes()).is_err() {
        return Err(VerifyHeadersError::IncorrectHmacSha256);
    }

    let api_version = headers.get("X-Shopify-Api-Version").ok_or(VerifyHeadersError::MissingApiVersion)?;
    if api_version.to_str().unwrap() != env::var("shopify_api_version").unwrap() {
        return Err(VerifyHeadersError::IncorrectApiVersion);
    }

    Ok(())
}

fn verify_hmac_sha256(hmac_sha256: &[u8]) -> Result<(), VerifyHmacSha256Error> {
    if hmac_sha256.len() != 64 {
        return Err(VerifyHmacSha256Error::IncorrectLength);
    }

    let webhook_secret = env::var("shopify_webhook_secret").unwrap();

    let mut diff = 0;
    for (a, b) in webhook_secret.as_bytes().iter().zip(hmac_sha256.iter()) {
        diff |= a ^ b;
    }

    if diff == 0 {
        Ok(())
    } else {
        Err(VerifyHmacSha256Error::InvalidHmacSha256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test for verify_headers
    const SHOP_DOMAIN: &str = "test.myshopify.com";
    const API_VERSION: &str = "2024-10";

    fn setup_verify_headers() {
        env::set_var("shopify_shop_url", SHOP_DOMAIN);
        env::set_var("shopify_webhook_secret", HMAC_VALUE);
        env::set_var("shopify_api_version", API_VERSION);
    }

    #[test]
    fn test_verify_headers_correct_headers() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers).unwrap();
        let expected = ();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify_headers_missing_topic() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::MissingTopic);
    }

    #[test]
    fn test_verify_headers_missing_webhook_id() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::MissingWebhookId);
    }

    #[test]
    fn test_verify_headers_missing_event_id() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::MissingEventId);
    }

    #[test]
    fn test_verify_headers_wrong_shop_domain() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", "wrong-shop.myshopify.com".parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::IncorrectShopDomain);
    }

    #[test]
    fn test_verify_headers_wrong_hmac() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", "invalid_hmac_value".parse().unwrap());
        headers.insert("X-Shopify-Api-Version", API_VERSION.parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::IncorrectHmacSha256);
    }

    #[test]
    fn test_verify_headers_wrong_api_version() {
        setup_verify_headers();
        let mut headers = HeaderMap::new();
        headers.insert("X-Shopify-Topic", "Create Order".parse().unwrap());
        headers.insert("X-Shopify-Webhook-Id", "81292983".parse().unwrap());
        headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
        headers.insert("X-Shopify-Shop-Domain", SHOP_DOMAIN.parse().unwrap());
        headers.insert("X-Shopify-Hmac-Sha256", HMAC_VALUE.parse().unwrap());
        headers.insert("X-Shopify-Api-Version", "2023-01".parse().unwrap());
        let result = verify_headers(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHeadersError::IncorrectApiVersion);
    }

    // Test for verify_hmac_sha256
    const HMAC_VALUE: &str = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2";

    fn setup_verify_hmac_sha256() {
        env::set_var("shopify_webhook_secret", HMAC_VALUE);
    }

    #[test]
    fn test_verify_hmac_sha256_correct_hmac_sha256() {
        setup_verify_hmac_sha256();
        let result = verify_hmac_sha256(HMAC_VALUE.as_bytes()).unwrap();
        let expected = ();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_verify_hmac_sha256_incorrect_length_hmac_sha256() {
        setup_verify_hmac_sha256();
        let result = verify_hmac_sha256(b"000000000000000000000000000000000000000000000000000000000000001");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHmacSha256Error::IncorrectLength);
    }

    #[test]
    fn test_verify_hmac_sha256_invalid_hmac_sha256() {
        setup_verify_hmac_sha256();
        let result = verify_hmac_sha256(b"0000000000000000000000000000000000000000000000000000000000000002");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VerifyHmacSha256Error::InvalidHmacSha256);
    }
}
