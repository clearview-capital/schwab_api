//! APIs to access Account Balances & Positions, to perform trading activities
//! [API Documentation](https://developer.schwab.com/products/trader-api--individual/details/specifications/Retail%20Trader%20API%20Production)

use reqwest::{Client, RequestBuilder, StatusCode, header::HeaderMap};

use super::endpoints;
use super::parameter::{Status, TransactionType};
use crate::api::Error;
use crate::model;

/// Get list of account numbers and their encrypted values
#[derive(Debug)]
pub struct GetAccountNumbersRequest {
    req: RequestBuilder,
}

impl GetAccountNumbersRequest {
    fn endpoint() -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::AccountNumbers
    }

    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::AccountNumbers, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<model::AccountNumbers>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Get linked account(s) balances and positions for the logged in user.
#[derive(Debug)]
pub struct GetAccountsRequest {
    req: RequestBuilder,

    /// This allows one to determine which fields they want returned.
    ///
    /// Possible value in this String can be: `positions`
    ///
    /// Example:
    ///
    /// fields=`positions`
    fields: Option<String>,
}

impl GetAccountsRequest {
    fn endpoint() -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::Accounts
    }

    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req, fields: None }
    }

    /// This allows one to determine which fields they want returned.
    ///
    /// Possible value in this String can be: `positions`
    ///
    /// Example:
    ///
    /// fields=`positions`
    pub fn fields(&mut self, val: String) -> &mut Self {
        self.fields = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req;
        if let Some(x) = self.fields {
            req = req.query(&[("fields", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Accounts, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<model::Accounts>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Get a specific account balance and positions for the logged in user.
#[derive(Debug)]
pub struct GetAccountRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    /// This allows one to determine which fields they want returned.
    ///
    /// Possible value in this String can be: `positions`
    ///
    /// Example:
    ///
    /// fields=`positions`
    fields: Option<String>,
}

impl GetAccountRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::Account { account_number }
    }

    pub(crate) fn new(client: &Client, access_token: String, account_number: String) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number)
    }

    fn new_with(req: RequestBuilder, account_number: String) -> Self {
        Self {
            req,
            account_number,
            fields: None,
        }
    }

    /// This allows one to determine which fields they want returned.
    ///
    /// Possible value in this String can be: `positions`
    ///
    /// Example:
    ///
    /// fields=`positions`
    pub fn fields(&mut self, val: String) -> &mut Self {
        self.fields = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req;
        if let Some(x) = self.fields {
            req = req.query(&[("fields", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Account, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<model::Account>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Get all orders for a specific account.
#[derive(Debug)]
pub struct GetAccountOrdersRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    /// The max number of orders to retrieve.
    /// Default is `3000`.
    max_results: Option<i64>,

    /// Specifies that no orders entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// `to_entered_time` must also be set.
    // Valid ISO-8601 formats are :  yyyy-MM-dd'T'HH:mm:ss.SSSZ
    from_entered_time: chrono::DateTime<chrono::Utc>,

    /// Specifies that no orders entered after this time should be returned.
    ///
    /// `from_entered_time` must also be set.
    // Valid ISO-8601 formats are :  yyyy-MM-dd'T'HH:mm:ss.SSSZ.
    to_entered_time: chrono::DateTime<chrono::Utc>,

    /// Specifies that only orders of this status should be returned.
    ///
    /// Available values : `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_STOP_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, `EXPIRED`, `NEW`, `AWAITING_RELEASE_TIME`, `PENDING_ACKNOWLEDGEMENT`, `PENDING_RECALL`, `UNKNOWN`
    status: Option<Status>,
}

impl GetAccountOrdersRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::OrdersAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, from_entered_time, to_entered_time)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            req,
            account_number,
            max_results: None,
            from_entered_time,
            to_entered_time,
            status: None,
        }
    }

    /// The max number of orders to retrieve.
    /// Default is `3000`.
    pub fn max_results(&mut self, val: i64) -> &mut Self {
        self.max_results = Some(val);
        self
    }

    /// Specifies that only orders of this status should be returned.
    ///
    /// Available values : `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_STOP_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, `EXPIRED`, `NEW`, `AWAITING_RELEASE_TIME`, `PENDING_ACKNOWLEDGEMENT`, `PENDING_RECALL`, `UNKNOWN`
    pub fn status(&mut self, val: Status) -> &mut Self {
        self.status = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            (
                "fromEnteredTime",
                self.from_entered_time.format("%+").to_string(),
            ),
            (
                "toEnteredTime",
                self.to_entered_time.format("%+").to_string(),
            ),
        ]);
        if let Some(x) = self.max_results {
            req = req.query(&[("maxResults", x)]);
        }
        if let Some(x) = self.status {
            req = req.query(&[("status", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Order>, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // std::fs::write("Orders_real.json", &json).expect("Unable to write file");
        // let item: Vec<model::Order> = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", item);
        // panic!();

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        let json_value: serde_json::Value = rsp.json().await?;
        // println!("{:#?}", json_value);
        let orders: Vec<model::Order> = serde_json::from_value(json_value)?;
        Ok(orders)
    }
}

/// Place order for a specific account.
#[derive(Debug)]
pub struct PostAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    body: model::OrderRequest,
}

impl PostAccountOrderRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::OrdersAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        body: model::OrderRequest,
    ) -> Self {
        let req = client
            .post(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, body)
    }

    fn new_with(req: RequestBuilder, account_number: String, body: model::OrderRequest) -> Self {
        Self {
            req,
            account_number,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<Option<i64>, Error> {
        let req = self.build();

        let rsp = super::send_timed(req).await?;

        let status = rsp.status();

        if status != StatusCode::CREATED {
            let raw = rsp.text().await?;
            log::error!("PostAccountOrder failed (status {}): {}", status, raw);
            let error_response: model::ServiceError = serde_json::from_str(&raw)?;
            return Err(Error::Service(error_response));
        }

        Ok(parse_order_id_from_headers(rsp.headers()))
    }
}

fn parse_order_id_from_headers(headers: &HeaderMap) -> Option<i64> {
    let url = headers.get("location")?.to_str().ok()?;

    url::Url::parse(url)
        .ok()?
        .path_segments()?
        .next_back()?
        .parse::<i64>()
        .ok()
}

/// Get a specific order by its ID, for a specific account
#[derive(Debug)]
pub struct GetAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    /// The ID of the order being retrieved.
    order_id: i64,
}

impl GetAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }
    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, order_id: i64) -> Self {
        Self {
            req,
            account_number,
            order_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::Order, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // std::fs::write("Order_real.json", &json).expect("Unable to write file");
        // let item: Vec<model::Order> = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", item);
        // panic!();

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<model::Order>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Cancel an order for a specific account
#[derive(Debug)]
pub struct DeleteAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    /// The ID of the order being retrieved.
    order_id: i64,
}

impl DeleteAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
    ) -> Self {
        let req = client
            .delete(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, order_id: i64) -> Self {
        Self {
            req,
            account_number,
            order_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<(), Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();

        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        Ok(())
    }
}

/// Replace order for a specific account
#[derive(Debug)]
pub struct PutAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    /// The ID of the order being retrieved.
    order_id: i64,

    body: model::OrderRequest,
}

impl PutAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Self {
        let req = client
            .put(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id, body)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Self {
        Self {
            req,
            account_number,
            order_id,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<Option<i64>, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::CREATED {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        Ok(parse_order_id_from_headers(rsp.headers()))
    }
}

/// Get all orders for all accounts
#[derive(Debug)]
pub struct GetAccountsOrdersRequest {
    req: RequestBuilder,

    /// The max number of orders to retrieve.
    ///
    /// Default is `3000`.
    max_results: Option<i64>,

    /// Specifies that no orders entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// `to_entered_time` must also be set.
    // Valid ISO-8601 formats are- yyyy-MM-dd'T'HH:mm:ss.SSSZ
    from_entered_time: chrono::DateTime<chrono::Utc>,

    /// Specifies that no orders entered after this time should be returned.
    ///
    /// `from_entered_time` must also be set.
    // Valid ISO-8601 formats are - yyyy-MM-dd'T'HH:mm:ss.SSSZ.
    to_entered_time: chrono::DateTime<chrono::Utc>,

    /// Specifies that only orders of this status should be returned.
    ///
    /// Available values : `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_STOP_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, `EXPIRED`, `NEW`, `AWAITING_RELEASE_TIME`, `PENDING_ACKNOWLEDGEMENT`, `PENDING_RECALL`, `UNKNOWN`
    status: Option<Status>,
}

impl GetAccountsOrdersRequest {
    fn endpoint() -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Orders
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, from_entered_time, to_entered_time)
    }

    fn new_with(
        req: RequestBuilder,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            req,
            max_results: None,
            from_entered_time,
            to_entered_time,
            status: None,
        }
    }

    /// The max number of orders to retrieve.
    ///
    /// Default is `3000`.
    pub fn max_results(&mut self, val: i64) -> &mut Self {
        self.max_results = Some(val);
        self
    }

    /// Specifies that only orders of this status should be returned.
    ///
    /// Available values : `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_STOP_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, `EXPIRED`, `NEW`, `AWAITING_RELEASE_TIME`, `PENDING_ACKNOWLEDGEMENT`, `PENDING_RECALL`, `UNKNOWN`
    pub fn status(&mut self, val: Status) -> &mut Self {
        self.status = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            (
                "fromEnteredTime",
                self.from_entered_time.format("%+").to_string(),
            ),
            (
                "toEnteredTime",
                self.to_entered_time.format("%+").to_string(),
            ),
        ]);
        if let Some(x) = self.max_results {
            req = req.query(&[("maxResults", x)]);
        }
        if let Some(x) = self.status {
            req = req.query(&[("status", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Order>, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<Vec<model::Order>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Preview order for a specific account.
#[derive(Debug)]
pub struct PostAccountPreviewOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    body: model::OrderRequest,
}

impl PostAccountPreviewOrderRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::PreviewOrderAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        body: model::OrderRequest,
    ) -> Self {
        let req = client
            .post(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, body)
    }

    fn new_with(req: RequestBuilder, account_number: String, body: model::OrderRequest) -> Self {
        Self {
            req,
            account_number,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<model::PreviewOrder, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // let v: model::PreviewOrder = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", v);
        // panic!();

        rsp.json::<model::PreviewOrder>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Get all transactions information for a specific account.
#[derive(Debug)]
pub struct GetAccountTransactions {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    /// Specifies that no transactions entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// [`Self::end_date`] must also be set.
    // Valid ISO-8601 formats are : yyyy-MM-dd'T'HH:mm:ss.SSSZ
    start_date: chrono::DateTime<chrono::Utc>,

    /// Specifies that no transactions entered after this time should be returned.
    ///
    /// [`Self::start_date`] must also be set.
    // Valid ISO-8601 formats are : yyyy-MM-dd'T'HH:mm:ss.SSSZ.
    end_date: chrono::DateTime<chrono::Utc>,

    /// It filters all the transaction activities based on the symbol specified.
    // NOTE: If there is any special character in the symbol, please send th encoded value.
    symbol: Option<String>,

    /// Specifies that only transactions of this status should be returned.
    ///
    /// Available values : `TRADE`, `RECEIVE_AND_DELIVER`, `DIVIDEND_OR_INTEREST`, `ACH_RECEIPT`, `ACH_DISBURSEMENT`, `CASH_RECEIPT`, `CASH_DISBURSEMENT`, `ELECTRONIC_FUND`, `WIRE_OUT`, `WIRE_IN`, `JOURNAL`, `MEMORANDUM`, `MARGIN_CALL`, `MONEY_MARKET`, `SMA_ADJUSTMENT`
    types: TransactionType,
}

impl GetAccountTransactions {
    fn endpoint(account_number: String) -> endpoints::EndpointTransaction {
        endpoints::EndpointTransaction::TransactionsAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: TransactionType,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, start_date, end_date, types)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: TransactionType,
    ) -> Self {
        Self {
            req,
            account_number,
            start_date,
            end_date,
            symbol: None,
            types,
        }
    }

    /// It filters all the transaction activities based on the symbol specified.
    pub fn symbol(&mut self, val: String) -> &mut Self {
        self.symbol = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("startDate", self.start_date.format("%+").to_string()),
            ("endDate", self.end_date.format("%+").to_string()),
        ]);
        req = req.query(&[("types", self.types)]);
        if let Some(x) = self.symbol {
            req = req.query(&[("symbol", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Transaction>, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // let v: Vec<model::Transaction> = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", v);
        // panic!();

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json().await.map_err(std::convert::Into::into)
    }
}

/// Get specific transaction information for a specific account
#[derive(Debug)]
pub struct GetAccountTransaction {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    /// The ID of the transaction being retrieved.
    transaction_id: i64,
}

impl GetAccountTransaction {
    fn endpoint(account_number: String, transaction_id: i64) -> endpoints::EndpointTransaction {
        endpoints::EndpointTransaction::Transaction {
            account_number,
            transaction_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        transaction_id: i64,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone(), transaction_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, transaction_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, transaction_id: i64) -> Self {
        Self {
            req,
            account_number,
            transaction_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    /// # Panics
    ///
    /// Will panic if no transaction found
    pub async fn send(self) -> Result<model::Transaction, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // std::fs::write("Transaction_real.json", &json).expect("Unable to write file");
        // let item: model::Transaction = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", item);
        // panic!();

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json().await.map_err(std::convert::Into::into)
    }
}

/// Get user preference information for the logged in user.
#[derive(Debug)]
pub struct GetUserPreferenceRequest {
    req: RequestBuilder,
}

impl GetUserPreferenceRequest {
    fn endpoint() -> endpoints::EndpointUserPreference {
        endpoints::EndpointUserPreference::UserPreference
    }
    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::UserPreferences, Error> {
        let req = self.build();
        let rsp = super::send_timed(req).await?;

        // let json = rsp.text().await.unwrap();
        // dbg!(&json);
        // std::fs::write("UserPreferences_real.json", &json).expect("Unable to write file");
        // let item: model::UserPreferences = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", item);
        // panic!();

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::Service(error_response));
        }

        rsp.json::<model::UserPreferences>()
            .await
            .map_err(std::convert::Into::into)
    }
}

/// Best-effort order cancellation used for cleanup on unexpected errors.
/// Logs a warning if the API call itself fails — there is nothing more that can
/// be done at that point, but the caller should surface an error to the user.

/// Computes the per-loop fractional price step so that the limit ramps from ~0 % on loop 1
/// up to `order_value_max_percent_change` by the final loop.
///
/// `attempt_duration / update_interval` gives the expected number of loops; clamped to at
/// least 1 to avoid division by zero when the interval equals or exceeds the duration.
// finddan with AI claude-sonnet-4-6
fn price_step(
    attempt_duration: f64,
    update_interval: f64,
    order_value_max_percent_change: f64,
) -> f64 {
    let num_loops = (attempt_duration / update_interval).max(1.0);
    order_value_max_percent_change / num_loops
}

/// Calculates the next limit price by applying a directional percentage offset to `mid`,
/// then clamping the result so it never crosses the live spread:
/// - buys are capped at `ask` (we never need to pay more than the market offers)
/// - sells are floored at `bid` (we never need to accept less than the market bids)
/// The result is rounded to two decimal places.
// finddan with AI claude-sonnet-4-6
fn next_limit_price(mid: f64, percent: f64, is_buy: bool, bid: f64, ask: f64) -> f64 {
    let price = if is_buy {
        (mid * (1.0 + percent)).min(ask)
    } else {
        (mid * (1.0 - percent)).max(bid)
    };
    (price * 100.0).round() / 100.0
}

/// Computes the total fill value for an order from its execution legs, or falls back to
/// `price * quantity` if no activity data is present. Returns `None` if the value is zero.
// finddan with AI claude-sonnet-4-6
fn compute_fill_value(order: &model::Order, instrument: &model::InstrumentRequest) -> Option<f64> {
    let multiplier = match instrument {
        model::InstrumentRequest::Option { .. } => 100.0,
        model::InstrumentRequest::Equity { .. } => 1.0,
    };
    if let Some(activities) = &order.order_activity_collection {
        let total: f64 = activities
            .iter()
            .flat_map(|a| a.execution_legs.iter())
            .map(|leg| leg.price * leg.quantity * multiplier)
            .sum();
        if total > 0.0 { Some(total) } else { None }
    } else {
        order.price.map(|p| p * order.quantity * multiplier)
    }
}

/// How an order should be submitted to the broker in [`AutoMidOrderRequest::submit_order`].
enum Submission {
    /// A brand-new order placed via `POST`. It has no predecessor in the replacement chain.
    New,
    /// A replacement of an existing order placed via `PUT`. Retains the id of the order being
    /// replaced so a race-window fill on it can be detected if the replacement is rejected.
    Replace {
        /// The id of the order this submission replaces.
        previous_order_id: i64,
    },
}

/// Durable outcome of submitting (creating or replacing) an order, after any rejection has been
/// validated against the replacement chain by [`AutoMidOrderRequest::resolve_order`].
enum OrderOutcome {
    /// The order is live and working in the market. Holds the active order id.
    Live(i64),
    /// An order in the chain filled — possibly the predecessor, during a replace race. Holds the
    /// filled order so its real id and fill value can be reported.
    Filled(model::Order),
    /// The chain reached a genuine, non-fill terminal state (Rejected, Canceled, Expired).
    Terminal {
        /// The id of the order that reached the terminal state.
        order_id: i64,
        /// The terminal status the order settled into.
        status: model::trader::order::Status,
    },
}

/// Maximum number of times [`AutoMidOrderRequest::resolve_order`] re-fetches an order while it
/// sits in a transitional state (e.g. `PendingReplace`) before treating it as terminal.
const MAX_RESOLVE_ATTEMPTS: u32 = 3;

/// Delay between transitional-state re-fetches in [`AutoMidOrderRequest::resolve_order`].
const RESOLVE_RETRY_DELAY: std::time::Duration = std::time::Duration::from_millis(250);

/// Runs an auto-escalating limit order that hunts the current mid price.
///
/// On each loop the live bid/ask mid is re-fetched and the limit price is set
/// to `mid * (1 +/- order_value_max_percent_change)`.
/// Once `max_attempt_duration` elapses the order is either converted to a
/// market order (when `enable_market_order_conversion` is `true`) or cancelled
/// and an error is returned.
#[derive(Debug)]
pub struct AutoMidOrderRequest {
    client: Client,
    access_token: String,
    account_number: String,
    instrument: model::InstrumentRequest,
    quantity: f64,
    instruction: model::Instruction,
    /// Seconds between each price-adjustment poll.
    update_interval: f64,
    /// Fractional step applied to the current mid each loop (e.g. `0.001` = 0.1 %).
    order_value_max_percent_change: f64,
    /// How long (seconds) to run before giving up. Defaults to 60 seconds.
    max_attempt_duration: Option<f64>,
    /// When `true` and `max_attempt_duration` elapses, the order is replaced
    /// with a market order. When `false` the order is cancelled and an error
    /// is returned instead.
    enable_market_order_conversion: bool,
}

/// Bid, ask, and rounded mid price for a symbol.
struct MidPrice {
    bid: f64,
    ask: f64,
    mid: f64,
}

impl AutoMidOrderRequest {
    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        instrument: model::InstrumentRequest,
        quantity: f64,
        instruction: model::Instruction,
        update_interval: f64,
        order_value_max_percent_change: f64,
        max_attempt_duration: Option<f64>,
        enable_market_order_conversion: bool,
    ) -> Self {
        Self {
            client: client.clone(),
            access_token,
            account_number,
            instrument,
            quantity,
            instruction,
            update_interval,
            order_value_max_percent_change,
            max_attempt_duration,
            enable_market_order_conversion,
        }
    }

    pub async fn send(self) -> Result<model::AutoMidOrderResponse, Error> {
        let attempt_duration = self.max_attempt_duration.unwrap_or(60.0);
        let attempt_limit = std::time::Duration::from_secs_f64(attempt_duration);

        // Absolute deadline for the entire run, measured from the moment auto-mid starts (before
        // the initial placement). Once it passes we stop replacing and either convert to market or
        // cancel — we never initiate a new limit replace past the budget, even when individual
        // network calls (mid fetch, PUT replace) run long.
        let start = std::time::Instant::now();
        let deadline = start + attempt_limit;

        log::info!(
            "Auto-mid starting: instrument={:?}, quantity={}, instruction={:?}, \
             update_interval={:.1}s, max_percent_change={:.1}%, attempt_duration={:.1}s, market_conversion={}",
            self.instrument.symbol(),
            self.quantity,
            self.instruction,
            self.update_interval,
            self.order_value_max_percent_change * 100.0,
            attempt_duration,
            self.enable_market_order_conversion
        );

        // For buys we raise the limit over time; for sells we lower it.
        let is_buy = matches!(
            self.instruction,
            model::Instruction::Buy
                | model::Instruction::BuyToOpen
                | model::Instruction::BuyToClose
                | model::Instruction::BuyToCover
        );

        // Fetch the initial mid price and place the first order there.
        let initial_quote = self
            .fetch_mid_price()
            .await
            .map_err(|e| Error::AutoMid(format!("Failed to fetch initial mid price: {e}")))?;

        // Place the initial limit order at the current mid.
        let initial = self.limit_order(initial_quote.mid)?;

        log::debug!(
            "Auto mid order will create the following initial order: {:?}",
            initial
        );

        // Place the initial limit order and validate it landed in a live state.
        let place_start = std::time::Instant::now();
        let mut current_order_id = match self.submit_order(initial, Submission::New).await? {
            OrderOutcome::Live(id) => id,
            OrderOutcome::Filled(order) => {
                log::info!("Auto-mid initial order {} filled immediately", order.order_id);
                return Ok(self.fill_response(&order, 0, "Order filled"));
            }
            OrderOutcome::Terminal { order_id, status } => {
                return Err(Error::AutoMid(format!(
                    "Initial order {} ended with terminal status {:?}",
                    order_id, status
                )));
            }
        };
        log::info!(
            "Auto-mid for {} initial order {} placed at mid {:.4} in {:.0}ms",
            self.instrument.symbol(),
            current_order_id,
            initial_quote.mid,
            place_start.elapsed().as_secs_f64() * 1000.0
        );

        let interval = std::time::Duration::from_secs_f64(self.update_interval);
        let step = price_step(
            attempt_duration,
            self.update_interval,
            self.order_value_max_percent_change,
        );
        let mut loop_count: u32 = 0;

        loop {
            // Sleep until the next poll, but never past the deadline so the run can't overshoot
            // the attempt budget by a whole interval.
            let now = std::time::Instant::now();
            if now >= deadline {
                return self
                    .handle_attempt_timeout(current_order_id, attempt_duration, loop_count)
                    .await;
            }
            tokio::time::sleep(interval.min(deadline - now)).await;
            loop_count += 1;

            // Attempt duration elapsed during the sleep — convert to market or cancel.
            if std::time::Instant::now() >= deadline {
                return self
                    .handle_attempt_timeout(current_order_id, attempt_duration, loop_count)
                    .await;
            }

            // Re-fetch the live mid price to base this loop's limit price on.
            let current_quote = match self.fetch_mid_price().await {
                Ok(q) => q,
                Err(e) => {
                    log::warn!(
                        "Failed to fetch mid price for order {}, skipping loop: {e}",
                        current_order_id
                    );
                    continue;
                }
            };

            // Apply an incrementally increasing offset to the current mid,
            // clamped so the price never crosses the live bid/ask spread.
            let percent = step * f64::from(loop_count);
            let next_price = next_limit_price(
                current_quote.mid,
                percent,
                is_buy,
                current_quote.bid,
                current_quote.ask,
            );

            log::info!(
                "Next auto-mid request for {} order {}: mid={:.4}, percent={:.4}%, order_price={:.2}",
                self.instrument.symbol(),
                current_order_id,
                current_quote.mid,
                percent * 100.0,
                next_price
            );

            // The mid fetch above can take several seconds. Re-check the deadline before issuing a
            // replace so we never start a new limit order past the attempt budget — convert or
            // cancel instead.
            if std::time::Instant::now() >= deadline {
                return self
                    .handle_attempt_timeout(current_order_id, attempt_duration, loop_count)
                    .await;
            }

            let adjusted = self.limit_order(next_price)?;

            // Replace the working order. `submit_order` validates any rejection against the
            // order being replaced, so a race-window fill is reported as a success rather
            // than surfaced as a spurious error.
            match self
                .submit_order(
                    adjusted,
                    Submission::Replace {
                        previous_order_id: current_order_id,
                    },
                )
                .await
            {
                Ok(OrderOutcome::Live(new_id)) => {
                    current_order_id = new_id;
                    log::info!(
                        "Updated auto-mid {} to price {:.2}",
                        current_order_id,
                        next_price
                    );
                }
                Ok(OrderOutcome::Filled(order)) => {
                    log::info!("Auto-mid order {} filled", order.order_id);
                    return Ok(self.fill_response(&order, loop_count, "Order filled"));
                }
                Ok(OrderOutcome::Terminal { order_id, status }) => {
                    return Err(Error::AutoMid(format!(
                        "Order {} ended with terminal status {:?} before replace",
                        order_id, status
                    )));
                }
                Err(e) => {
                    self.cancel_order(current_order_id).await;
                    return Err(e);
                }
            }
        }
    }

    /// Cancels an existing order, logging any errors without propagating them.
    /// If the DELETE request fails, the order is re-fetched to check whether it was
    /// already filled — a filled order is logged at `info` rather than `warn` since
    /// the position has been satisfied. Any other terminal status is logged at `warn`.
    /// Emits an `info` log with the elapsed time for the cancel call.
    // finddan with AI claude-sonnet-4-6
    async fn cancel_order(&self, order_id: i64) {
        use model::trader::order::Status as OrderStatus;

        let cancel_start = std::time::Instant::now();
        match DeleteAccountOrderRequest::new(
            &self.client,
            self.access_token.to_string(),
            self.account_number.to_string(),
            order_id,
        )
        .send()
        .await
        {
            Ok(()) => log::info!(
                "Cleanup: cancelled order {} in {:.0}ms",
                order_id,
                cancel_start.elapsed().as_secs_f64() * 1000.0
            ),
            Err(delete_err) => {
                log::warn!(
                    "Cleanup: DELETE for order {} failed ({:.0}ms): {} — re-fetching to check status",
                    order_id,
                    cancel_start.elapsed().as_secs_f64() * 1000.0,
                    delete_err
                );

                // Re-fetch the order to determine whether the cancel failure was because
                // the order had already filled (benign) or some other reason.
                match self.fetch_order_status(order_id).await {
                    Ok(order) if order.status == OrderStatus::Filled => {
                        log::info!(
                            "Cleanup: order {} re-fetched after cancel failure — status=Filled, position satisfied",
                            order_id
                        );
                    }
                    Ok(order) => {
                        log::warn!(
                            "Cleanup: order {} re-fetched after cancel failure — status={:?}",
                            order_id,
                            order.status
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "Cleanup: could not re-fetch order {} after cancel failure: {}",
                            order_id,
                            e
                        );
                    }
                }
            }
        }
    }

    /// Fetches the current state of an order by ID, timing the network call and emitting
    /// an `info` log with the elapsed duration and resolved status.
    // finddan with AI claude-sonnet-4-6
    async fn fetch_order_status(
        &self,
        order_id: i64,
    ) -> Result<model::trader::order::Order, Error> {
        let fetch_start = std::time::Instant::now();
        let order = GetAccountOrderRequest::new(
            &self.client,
            self.access_token.to_string(),
            self.account_number.to_string(),
            order_id,
        )
        .send()
        .await
        .map_err(|e| Error::AutoMid(format!("Failed to fetch order {order_id} status: {e}")))?;
        log::info!(
            "fetch_order_status: order {} status={:?} in {:.0}ms",
            order_id,
            order.status,
            fetch_start.elapsed().as_secs_f64() * 1000.0
        );
        Ok(order)
    }

    /// Fetches the current mid price for the instrument by averaging bid and ask.
    /// The result is rounded to two decimal places.
    // finddan with AI claude-sonnet-4-6
    async fn fetch_mid_price(&self) -> Result<MidPrice, Error> {
        let symbol = self.instrument.symbol();
        let fetch_start = std::time::Instant::now();
        let quote = super::market_data::GetQuoteRequest::new(
            &self.client,
            self.access_token.to_string(),
            symbol.to_string(),
        )
        .send()
        .await?;
        log::info!(
            "fetch_mid_price: fetched quote for {} in {:.0}ms",
            symbol,
            fetch_start.elapsed().as_secs_f64() * 1000.0
        );

        let bid = quote
            .bid_price()
            .ok_or_else(|| Error::AutoMid(format!("no bid price available for {symbol}")))?;
        let ask = quote
            .ask_price()
            .ok_or_else(|| Error::AutoMid(format!("no ask price available for {symbol}")))?;

        let mid = (bid + ask) / 2.0;
        let mid_rounded = (mid * 100.0).round() / 100.0;
        log::info!(
            "fetch_mid_price: {} bid={:.4} ask={:.4} mid={:.4}",
            symbol,
            bid,
            ask,
            mid_rounded
        );
        Ok(MidPrice {
            bid,
            ask,
            mid: mid_rounded,
        })
    }

    /// Builds a limit [`model::OrderRequest`] for this auto-mid run at `price`, marked
    /// `AllOrNone` so the order can only execute in full. This prevents partial fills, which
    /// would otherwise leave a residual position stranded whenever the order is replaced.
    // finddan with AI claude-opus-4-6
    fn limit_order(&self, price: f64) -> Result<model::OrderRequest, Error> {
        let mut order = model::OrderRequest::limit(
            self.instrument.clone(),
            self.instruction,
            self.quantity,
            price,
        )?;
        order.special_instruction = Some(model::trader::order::SpecialInstruction::AllOrNone);
        Ok(order)
    }

    /// Builds a successful [`model::AutoMidOrderResponse`] for a filled order, using the order's
    /// own id and computed fill value. `loops` is the number of polling loops that ran.
    // finddan with AI claude-opus-4-6
    fn fill_response(
        &self,
        order: &model::Order,
        loops: u32,
        message: &str,
    ) -> model::AutoMidOrderResponse {
        model::AutoMidOrderResponse {
            created: true,
            order_id: Some(order.order_id as u64),
            loops,
            fill_value: compute_fill_value(order, &self.instrument),
            market_order: false,
            message: Some(message.into()),
        }
    }

    /// Submits an order to the broker — a fresh `POST` for [`Submission::New`] or a `PUT`
    /// replacement for [`Submission::Replace`] — and resolves the result into a durable
    /// [`OrderOutcome`].
    ///
    /// This is the single entry point for every order create/replace in an auto-mid run. Its
    /// durability guarantee is that a `Rejected` result is never taken at face value: a
    /// replacement is frequently rejected precisely because the order it replaced filled in the
    /// race window between the pre-replace state and the `PUT` reaching the broker. In that case
    /// the replaced order is re-checked (see [`AutoMidOrderRequest::resolve_order`]) and, if
    /// filled, the outcome is reported as [`OrderOutcome::Filled`] rather than surfaced as an error.
    // finddan with AI claude-opus-4-6
    async fn submit_order(
        &self,
        order: model::OrderRequest,
        submission: Submission,
    ) -> Result<OrderOutcome, Error> {
        let submit_start = std::time::Instant::now();
        let (order_id, predecessor_id) = match submission {
            Submission::New => {
                let id = PostAccountOrderRequest::new(
                    &self.client,
                    self.access_token.clone(),
                    self.account_number.clone(),
                    order,
                )
                .send()
                .await?
                .ok_or_else(|| Error::AutoMid("No order ID returned for limit order".into()))?;
                log::info!(
                    "submit_order: POST created order {} in {:.0}ms",
                    id,
                    submit_start.elapsed().as_secs_f64() * 1000.0
                );
                (id, None)
            }
            Submission::Replace { previous_order_id } => {
                match PutAccountOrderRequest::new(
                    &self.client,
                    self.access_token.clone(),
                    self.account_number.clone(),
                    previous_order_id,
                    order,
                )
                .send()
                .await
                {
                    Ok(new_id) => {
                        let id = new_id.unwrap_or(previous_order_id);
                        log::info!(
                            "submit_order: PUT replaced order {} with {} in {:.0}ms",
                            previous_order_id,
                            id,
                            submit_start.elapsed().as_secs_f64() * 1000.0
                        );
                        (id, Some(previous_order_id))
                    }
                    Err(put_err) => {
                        // The PUT itself was rejected. The most common cause is that the order
                        // filled in the race window, so validate the replaced order for a fill
                        // before surfacing the error.
                        log::warn!(
                            "submit_order: PUT for order {} failed: {} — validating replaced order for a fill",
                            previous_order_id,
                            put_err
                        );
                        return match self.resolve_order(previous_order_id, None, 0).await? {
                            OrderOutcome::Filled(filled) => {
                                log::info!(
                                    "submit_order: replaced order {} is Filled after PUT failure — treating as fill",
                                    previous_order_id
                                );
                                Ok(OrderOutcome::Filled(filled))
                            }
                            _ => Err(put_err),
                        };
                    }
                }
            }
        };

        self.resolve_order(order_id, predecessor_id, 0).await
    }

    /// Re-fetches `order_id` and classifies it into a durable [`OrderOutcome`], validating any
    /// non-fill terminal status against the replacement chain.
    ///
    /// - `Filled` → [`OrderOutcome::Filled`].
    /// - Any live/working state → [`OrderOutcome::Live`].
    /// - Transitional states (`PendingReplace`, `PendingCancel`, `PendingRecall`, `Unknown`) are
    ///   settled by recursing after [`RESOLVE_RETRY_DELAY`], bounded by [`MAX_RESOLVE_ATTEMPTS`].
    /// - A non-fill terminal status (`Rejected`, `Canceled`, `Expired`, or an exhausted
    ///   transitional) triggers a recursive check of `predecessor_id` — the order this one
    ///   replaced. If that predecessor filled (the classic replace-after-fill race), the fill is
    ///   reported instead of a terminal outcome.
    ///
    /// `attempt` tracks the transitional-retry depth and must be `0` on the first call. Recurses
    /// via `Box::pin` because it is an async self-recursive method.
    // finddan with AI claude-opus-4-6
    async fn resolve_order(
        &self,
        order_id: i64,
        predecessor_id: Option<i64>,
        attempt: u32,
    ) -> Result<OrderOutcome, Error> {
        use model::trader::order::Status as S;

        let order = self.fetch_order_status(order_id).await?;

        match order.status {
            S::Filled => Ok(OrderOutcome::Filled(order)),

            // Live, working states — the order is still active in the market.
            S::Working
            | S::Accepted
            | S::Queued
            | S::New
            | S::PendingActivation
            | S::AwaitingParentOrder
            | S::AwaitingCondition
            | S::AwaitingStopCondition
            | S::AwaitingManualReview
            | S::AwaitingUrOut
            | S::AwaitingReleaseTime
            | S::PendingAcknowledgement => Ok(OrderOutcome::Live(order_id)),

            // Transitional states — settle with a bounded, delayed retry.
            S::PendingReplace | S::PendingCancel | S::PendingRecall | S::Unknown
                if attempt < MAX_RESOLVE_ATTEMPTS =>
            {
                log::info!(
                    "resolve_order: order {} in transitional status {:?}, retrying ({}/{})",
                    order_id,
                    order.status,
                    attempt + 1,
                    MAX_RESOLVE_ATTEMPTS
                );
                tokio::time::sleep(RESOLVE_RETRY_DELAY).await;
                Box::pin(self.resolve_order(order_id, predecessor_id, attempt + 1)).await
            }

            // Non-fill terminal (or an exhausted transitional) status. Before treating this as a
            // failure, check whether the order this one replaced filled in the race window.
            status => {
                if let Some(prev) = predecessor_id {
                    log::info!(
                        "resolve_order: order {} is {:?}; checking replaced order {} for a race-window fill",
                        order_id,
                        status,
                        prev
                    );
                    if let OrderOutcome::Filled(filled) =
                        Box::pin(self.resolve_order(prev, None, 0)).await?
                    {
                        log::info!(
                            "resolve_order: order {} is {:?} because replaced order {} filled — treating as fill",
                            order_id,
                            status,
                            prev
                        );
                        return Ok(OrderOutcome::Filled(filled));
                    }
                }
                log::warn!(
                    "resolve_order: order {} reached terminal status {:?}",
                    order_id,
                    status
                );
                Ok(OrderOutcome::Terminal { order_id, status })
            }
        }
    }

    /// Handles the case where `max_attempt_duration` has elapsed.
    /// - When `enable_market_order_conversion` is `true`, converts the open limit order to a
    ///   market order via [`AutoMidOrderRequest::submit_order`], reporting fills (including
    ///   race-window fills detected during the replace) and genuine terminal states correctly.
    /// - When `false`, cancels the order and returns an error.
    // finddan with AI claude-opus-4-6
    async fn handle_attempt_timeout(
        &self,
        current_order_id: i64,
        attempt_duration: f64,
        loop_count: u32,
    ) -> Result<model::AutoMidOrderResponse, Error> {
        if self.enable_market_order_conversion {
            log::warn!(
                "Auto-mid order {} reached attempt duration, converting to market",
                current_order_id
            );
            let market = model::OrderRequest::market(
                self.instrument.clone(),
                self.instruction,
                self.quantity,
            )?;

            let outcome = self
                .submit_order(
                    market,
                    Submission::Replace {
                        previous_order_id: current_order_id,
                    },
                )
                .await;

            match outcome {
                Ok(OrderOutcome::Live(new_id)) => {
                    log::info!(
                        "Converted auto-mid {} to market order (new_id={})",
                        current_order_id,
                        new_id
                    );
                    Ok(model::AutoMidOrderResponse {
                        created: true,
                        order_id: Some(new_id as u64),
                        loops: loop_count,
                        fill_value: None,
                        market_order: true,
                        message: Some(format!(
                            "Converted to market order after {:.1}s",
                            attempt_duration
                        )),
                    })
                }
                Ok(OrderOutcome::Filled(order)) => {
                    log::info!(
                        "Auto-mid order {} filled (detected during market conversion)",
                        order.order_id
                    );
                    Ok(self.fill_response(&order, loop_count, "Order filled"))
                }
                Ok(OrderOutcome::Terminal { order_id, status }) => {
                    // The market conversion did not take. Cancel to ensure the limit order is
                    // closed rather than left resting past the attempt budget.
                    log::warn!(
                        "Market conversion for order {} ended terminal {:?}; cancelling to close",
                        order_id,
                        status
                    );
                    self.cancel_order(current_order_id).await;
                    Err(Error::AutoMid(format!(
                        "Order {} ended with terminal status {:?} during market conversion; cancelled",
                        order_id, status
                    )))
                }
                Err(e) => {
                    // The conversion request itself failed. Cancel to ensure the limit order is
                    // closed rather than left resting past the attempt budget.
                    log::warn!(
                        "Market conversion for order {} failed: {}; cancelling to close",
                        current_order_id,
                        e
                    );
                    self.cancel_order(current_order_id).await;
                    Err(e)
                }
            }
        } else {
            log::warn!(
                "Auto-mid order {} reached attempt duration, cancelling",
                current_order_id
            );
            let timeout_cancel_start = std::time::Instant::now();
            DeleteAccountOrderRequest::new(
                &self.client,
                self.access_token.clone(),
                self.account_number.clone(),
                current_order_id,
            )
            .send()
            .await
            .map_err(|e| Error::AutoMid(format!("Failed to cancel order: {}", e)))?;
            log::info!(
                "handle_attempt_timeout: cancelled order {} in {:.0}ms",
                current_order_id,
                timeout_cancel_start.elapsed().as_secs_f64() * 1000.0
            );
            Err(Error::AutoMid(format!(
                "Order {} cancelled after {:.1}s attempt duration",
                current_order_id, attempt_duration
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::trader::accounts::SecuritiesAccount;

    use mockito::Matcher;
    use pretty_assertions::assert_eq;
    use reqwest::Client;
    use reqwest::header::HeaderValue;

    #[tokio::test]
    async fn test_parse_order_id_from_headers() {
        let mut header_map = HeaderMap::new();
        let url = endpoints::EndpointOrder::Order {
            account_number: "account_number".to_string(),
            order_id: 123_456,
        }
        .url();
        let value = HeaderValue::from_str(&url).unwrap();
        header_map.insert("location", value);

        let result = parse_order_id_from_headers(&header_map);

        // Check happy path
        assert_eq!(result.unwrap(), 123_456);

        // Check for failure when location missing
        header_map.remove("location");
        let result = parse_order_id_from_headers(&header_map);
        assert_eq!(result, None,);

        // Check for failure when not parsable to i64
        let url = "https://api.schwabapi.com/trader/v1/accounts/accountNumber/orders/not_an_i64";
        let value = HeaderValue::from_str(url).unwrap();
        header_map.insert("location", value);
        let result = parse_order_id_from_headers(&header_map);
        assert_eq!(result, None);

        // We don't currently test the "not a String" or next_back() failures as it does not appear
        // to be possible to construct a HeaderValue without a String.
    }

    #[tokio::test]
    async fn test_get_account_numbers_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        // none

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/accountNumbers")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/AccountNumbers.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountNumbersRequest::endpoint().url_endpoint()
        ));

        let req = GetAccountNumbersRequest::new_with(req);

        // check initial value
        // none

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result[0].account_number, "string");
    }

    #[tokio::test]
    async fn test_get_accounts_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let fields = "positions".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "fields".into(),
                fields.clone(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Accounts_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountsRequest::endpoint().url_endpoint()
        ));

        let mut req = GetAccountsRequest::new_with(req);

        // check initial value
        assert_eq!(req.fields, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert!(matches!(
            result[0].securities_account,
            SecuritiesAccount::Cash(_)
        ));
    }

    #[tokio::test]
    async fn test_get_account_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let fields = "positions".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "fields".into(),
                fields.clone(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Account_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountRequest::new_with(req, account_number.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.fields, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert!(matches!(
            result.securities_account,
            SecuritiesAccount::Cash(_)
        ));
    }

    #[tokio::test]
    async fn test_get_account_orders_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let max_results = 10;
        let from_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let to_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let status = Status::AwaitingParentOrder;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/orders")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("maxResults".into(), max_results.to_string()),
                Matcher::UrlEncoded(
                    "fromEnteredTime".into(),
                    from_entered_time.format("%+").to_string(),
                ),
                Matcher::UrlEncoded(
                    "toEnteredTime".into(),
                    to_entered_time.format("%+").to_string(),
                ),
                Matcher::UrlEncoded("status".into(), "AWAITING_PARENT_ORDER".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Orders_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountOrdersRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountOrdersRequest::new_with(
            req,
            account_number.clone(),
            from_entered_time,
            to_entered_time,
        );

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.max_results, None);
        assert_eq!(req.from_entered_time, from_entered_time);
        assert_eq!(req.to_entered_time, to_entered_time);
        assert_eq!(req.status, None);

        // check setter
        req.max_results(max_results);
        assert_eq!(req.max_results, Some(max_results));
        req.status(status);
        assert_eq!(req.status, Some(status));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 15);
    }

    #[tokio::test]
    async fn test_post_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let body = model::OrderRequest::default();

        // Create a mock
        let mock = server
            .mock("POST", "/accounts/account_number/orders")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_header(
                "location",
                &endpoints::EndpointOrder::Order {
                    account_number: "account_number".to_string(),
                    order_id: 123_456,
                }
                .url(),
            )
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(body.clone()).unwrap(),
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.post(format!(
            "{url}{}",
            PostAccountOrderRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let req = PostAccountOrderRequest::new_with(req, account_number.clone(), body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(123_456));
    }

    #[tokio::test]
    async fn test_get_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/orders/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Order_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req = GetAccountOrderRequest::new_with(req, account_number.clone(), order_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.session, model::trader::order::Session::Normal);
    }

    #[tokio::test]
    async fn test_delete_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;

        // Create a mock
        let mock = server
            .mock("DELETE", "/accounts/account_number/orders/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = Client::new();
        let req = client.delete(format!(
            "{url}{}",
            DeleteAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req = DeleteAccountOrderRequest::new_with(req, account_number.clone(), order_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;
        let body = model::OrderRequest::default();

        // Create a mock
        let mock = server
            .mock("PUT", "/accounts/account_number/orders/123")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_header(
                "location",
                &endpoints::EndpointOrder::Order {
                    account_number: "account_number".to_string(),
                    order_id: 123_456,
                }
                .url(),
            )
            .match_body(Matcher::Json(serde_json::to_value(body.clone()).unwrap()))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.put(format!(
            "{url}{}",
            PutAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req =
            PutAccountOrderRequest::new_with(req, account_number.clone(), order_id, body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(123_456));
    }

    #[tokio::test]
    async fn test_get_accounts_orders_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let max_results = 10;
        let from_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let to_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let status = Status::AwaitingParentOrder;

        // Create a mock
        let mock = server
            .mock("GET", "/orders")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("maxResults".into(), max_results.to_string()),
                Matcher::UrlEncoded(
                    "fromEnteredTime".into(),
                    from_entered_time.format("%+").to_string(),
                ),
                Matcher::UrlEncoded(
                    "toEnteredTime".into(),
                    to_entered_time.format("%+").to_string(),
                ),
                Matcher::UrlEncoded("status".into(), "AWAITING_PARENT_ORDER".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Orders_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountsOrdersRequest::endpoint().url_endpoint()
        ));

        let mut req = GetAccountsOrdersRequest::new_with(req, from_entered_time, to_entered_time);

        // check initial value
        assert_eq!(req.max_results, None);
        assert_eq!(req.from_entered_time, from_entered_time);
        assert_eq!(req.to_entered_time, to_entered_time);
        assert_eq!(req.status, None);

        // check setter
        req.max_results(max_results);
        assert_eq!(req.max_results, Some(max_results));
        req.status(status);
        assert_eq!(req.status, Some(status));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 15);
    }

    #[tokio::test]
    async fn test_post_account_preview_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let body = model::OrderRequest::default();

        // Create a mock
        let mock = server
            .mock("POST", "/accounts/account_number/previewOrder")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/PreviewOrder.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.post(format!(
            "{url}{}",
            PostAccountPreviewOrderRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let req =
            PostAccountPreviewOrderRequest::new_with(req, account_number.clone(), body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.order_id, 0);
    }

    #[tokio::test]
    async fn test_post_account_preview_order_request_real() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();

        let body = model::OrderRequest::limit(
            model::InstrumentRequest::Equity {
                symbol: "VEA".to_string(),
            },
            model::Instruction::Buy,
            1.0,
            10.0,
        )
        .unwrap();

        // Create a mock
        let mock = server
            .mock("POST", "/accounts/account_number/previewOrder")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/PreviewOrder_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.post(format!(
            "{url}{}",
            PostAccountPreviewOrderRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let req =
            PostAccountPreviewOrderRequest::new_with(req, account_number.clone(), body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.order_id, 0);
    }

    #[tokio::test]
    async fn test_get_account_transactions_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let start_date = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let end_date = chrono::NaiveDate::from_ymd_opt(2016, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let symbol = "VTI".to_string();
        let types = TransactionType::ReceiveAndDeliver;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/transactions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("startDate".into(), start_date.format("%+").to_string()),
                Matcher::UrlEncoded("endDate".into(), end_date.format("%+").to_string()),
                Matcher::UrlEncoded("symbol".into(), symbol.clone()),
                Matcher::UrlEncoded("types".into(), "RECEIVE_AND_DELIVER".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Transactions_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountTransactions::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountTransactions::new_with(
            req,
            account_number.clone(),
            start_date,
            end_date,
            types,
        );

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.start_date, start_date);
        assert_eq!(req.end_date, end_date);
        assert_eq!(req.symbol, None);
        assert_eq!(req.types, types);

        // check setter
        req.symbol(symbol.clone());
        assert_eq!(req.symbol, Some(symbol));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 122);
    }

    #[tokio::test]
    async fn test_get_account_transaction_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let transaction_id = 123;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/transactions/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Transaction_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountTransaction::endpoint(account_number.clone(), transaction_id).url_endpoint()
        ));

        let req = GetAccountTransaction::new_with(req, account_number.clone(), transaction_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.transaction_id, transaction_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.activity_id, 12_345_678_910);
    }

    #[tokio::test]
    async fn test_get_user_preference_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        // none

        // Create a mock
        let mock = server
            .mock("GET", "/userPreference")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/UserPreferences.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetUserPreferenceRequest::endpoint().url_endpoint()
        ));

        let req = GetUserPreferenceRequest::new_with(req);

        // check initial value
        // none

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert!(matches!(result, model::UserPreferences::Mutiple(_)));
    }
}
