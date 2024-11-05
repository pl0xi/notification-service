use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use thiserror::Error;

use std::env;

#[derive(Error, Debug)]
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

#[derive(Error, Debug)]
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
