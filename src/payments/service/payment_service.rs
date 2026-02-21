use crate::payments::models::yoco_checkout_request::YocoCheckoutRequest;
use reqwest::header::HeaderMap;

pub struct PaymentService {}

impl PaymentService {
    // OUTBOUND
    pub async fn make_payment(
        checkout_request: YocoCheckoutRequest,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = "https://payments.yoco.com/api/checkouts";
        let token = std::env::var("PAYMENT_TOKEN_TEST").expect("PAYMENT_TOKEN not set");
        let token = format!("Bearer {}", token);
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", token.parse().unwrap());

        match reqwest::Client::new()
            .post(url)
            .headers(headers)
            .json(&checkout_request)
            .send()
            .await
        {
            Ok(response) => {
                log::info!("Response Status received: {}", response.status());
                Ok(response)
            },
            Err(error) => {
                log::error!("make_payment error: {}", error);
                Err(error)
            },
        }
    }
}