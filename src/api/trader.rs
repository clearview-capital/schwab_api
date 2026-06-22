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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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

        // // Deserialize into serde_json::Value
        let json_value: serde_json::Value = rsp.json().await?;
        // Print the JSON value
        println!("{:#?}", json_value);

        // Convert the JSON value back to model::Account
        let orders: Vec<model::Order> = serde_json::from_value(json_value)?;
        Ok(orders)

        //        rsp.json::<Vec<model::Order>>()
        //            .await
        //            .map_err(std::convert::Into::into)
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

        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
        let rsp = req.send().await?;

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
async fn cancel_order(client: &Client, access_token: &str, account_number: &str, order_id: i64) {
    match DeleteAccountOrderRequest::new(
        client,
        access_token.to_string(),
        account_number.to_string(),
        order_id,
    )
    .send()
    .await
    {
        Ok(()) => log::info!(
            "Cleanup: cancelled order {} for account {}",
            order_id,
            account_number
        ),
        Err(e) => log::warn!(
            "Cleanup: failed to cancel order {} for account {}: {}",
            order_id,
            account_number,
            e
        ),
    }
}

/// Fetches the current mid price for `symbol` by averaging the bid and ask prices.
/// The result is truncated to two decimal places.
// finddan with AI claude-sonnet-4-6
async fn fetch_mid_price(client: &Client, access_token: &str, symbol: &str) -> Result<f64, Error> {
    let quote = super::market_data::GetQuoteRequest::new(
        client,
        access_token.to_string(),
        symbol.to_string(),
    )
    .send()
    .await?;

    let bid = quote
        .bid_price()
        .ok_or_else(|| Error::AutoMid(format!("no bid price available for {symbol}")))?;
    let ask = quote
        .ask_price()
        .ok_or_else(|| Error::AutoMid(format!("no ask price available for {symbol}")))?;

    let mid = (bid + ask) / 2.0;
    Ok((mid * 100.0).round() / 100.0)
}

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
    symbol: String,
    quantity: f64,
    instruction: model::Instruction,
    asset_type: model::AutoMidAssetType,
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

impl AutoMidOrderRequest {
    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        symbol: String,
        quantity: f64,
        instruction: model::Instruction,
        asset_type: model::AutoMidAssetType,
        update_interval: f64,
        order_value_max_percent_change: f64,
        max_attempt_duration: Option<f64>,
        enable_market_order_conversion: bool,
    ) -> Self {
        Self {
            client: client.clone(),
            access_token,
            account_number,
            symbol,
            quantity,
            instruction,
            asset_type,
            update_interval,
            order_value_max_percent_change,
            max_attempt_duration,
            enable_market_order_conversion,
        }
    }

    pub async fn send(self) -> Result<model::AutoMidOrderResponse, Error> {
        use model::trader::order::Status as OrderStatus;

        let Self {
            client,
            access_token,
            account_number,
            symbol,
            quantity,
            instruction,
            asset_type,
            update_interval,
            order_value_max_percent_change,
            max_attempt_duration,
            enable_market_order_conversion,
        } = self;

        let attempt_duration = max_attempt_duration.unwrap_or(60.0);
        let attempt_limit = std::time::Duration::from_secs_f64(attempt_duration);

        // For buys we raise the limit over time; for sells we lower it.
        let is_buy = matches!(
            instruction,
            model::Instruction::Buy
                | model::Instruction::BuyToOpen
                | model::Instruction::BuyToClose
                | model::Instruction::BuyToCover
        );

        let make_instrument = |sym: &str| -> model::InstrumentRequest {
            match asset_type {
                model::AutoMidAssetType::Equity => model::InstrumentRequest::Equity {
                    symbol: sym.to_string(),
                },
                model::AutoMidAssetType::Option => model::InstrumentRequest::Option {
                    symbol: sym.to_string(),
                },
            }
        };

        // Fetch the initial mid price and place the first order there.
        let initial_mid = fetch_mid_price(&client, &access_token, &symbol)
            .await
            .map_err(|e| Error::AutoMid(format!("Failed to fetch initial mid price: {e}")))?;

        // Place the initial limit order at the current mid.
        let initial = model::OrderRequest::limit(
            make_instrument(&symbol),
            instruction,
            quantity,
            initial_mid,
        )?;

        log::debug!("Auto mid order will create the following initial order: {:?}", initial);

        let mut current_order_id = PostAccountOrderRequest::new(
            &client,
            access_token.clone(),
            account_number.clone(),
            initial,
        )
        .send()
        .await?
        .ok_or_else(|| Error::AutoMid("No order ID returned for limit order".into()))?;

        log::info!(
            "Auto-mid order {} placed for account {} at mid {:.4}",
            current_order_id,
            account_number,
            initial_mid
        );

        let start = std::time::Instant::now();
        let interval = std::time::Duration::from_secs_f64(update_interval);
        // Divide the total allowed percent change evenly across the expected number of
        // loops so the offset ramps from ~0% on loop 1 up to order_value_max_percent_change
        // by the final loop.
        let num_loops = (attempt_duration / update_interval).max(1.0);
        let step = order_value_max_percent_change / num_loops;
        let mut loop_count: u32 = 0;

        loop {
            tokio::time::sleep(interval).await;
            let elapsed = start.elapsed();
            loop_count += 1;

            // Attempt duration elapsed — convert to market or cancel depending on config.
            if elapsed >= attempt_limit {
                if enable_market_order_conversion {
                    log::warn!(
                        "Auto-mid order {} reached attempt duration for account {}, converting to market",
                        current_order_id,
                        account_number
                    );
                    let market = model::OrderRequest::market(
                        make_instrument(&symbol),
                        instruction,
                        quantity,
                    )?;
                    let new_id = PutAccountOrderRequest::new(
                        &client,
                        access_token.clone(),
                        account_number.clone(),
                        current_order_id,
                        market,
                    )
                    .send()
                    .await;
                    let new_id = match new_id {
                        Ok(id) => id,
                        Err(e) => {
                            cancel_order(&client, &access_token, &account_number, current_order_id)
                                .await;
                            return Err(Error::AutoMid(format!(
                                "Failed to convert order to market: {}",
                                e
                            )));
                        }
                    };
                    log::info!(
                        "Converted auto-mid {} to market order for account {} (new_id={:?})",
                        current_order_id,
                        account_number,
                        new_id
                    );
                    return Ok(model::AutoMidOrderResponse {
                        created: true,
                        order_id: Some(new_id.unwrap_or(current_order_id) as u64),
                        loops: loop_count,
                        fill_value: None,
                        market_order: true,
                        message: Some(format!(
                            "Converted to market order after {:.1}s",
                            attempt_duration
                        )),
                    });
                } else {
                    log::warn!(
                        "Auto-mid order {} reached attempt duration for account {}, cancelling",
                        current_order_id,
                        account_number
                    );
                    DeleteAccountOrderRequest::new(
                        &client,
                        access_token.clone(),
                        account_number.clone(),
                        current_order_id,
                    )
                    .send()
                    .await
                    .map_err(|e| Error::AutoMid(format!("Failed to cancel order: {}", e)))?;
                    return Err(Error::AutoMid(format!(
                        "Order {} cancelled after {:.1}s attempt duration",
                        current_order_id, attempt_duration
                    )));
                }
            }

            // Poll the current order status.
            let order = match GetAccountOrderRequest::new(
                &client,
                access_token.clone(),
                account_number.clone(),
                current_order_id,
            )
            .send()
            .await
            {
                Ok(o) => o,
                Err(e) => {
                    cancel_order(&client, &access_token, &account_number, current_order_id).await;
                    return Err(Error::AutoMid(format!(
                        "Failed to poll order status: {}",
                        e
                    )));
                }
            };

            match order.status {
                OrderStatus::Filled => {
                    log::info!(
                        "Auto-mid order {} filled for account {}",
                        current_order_id,
                        account_number
                    );
                    let multiplier = match asset_type {
                        model::AutoMidAssetType::Option => 100.0,
                        model::AutoMidAssetType::Equity => 1.0,
                    };
                    let fill_value = if let Some(activities) = &order.order_activity_collection {
                        let total: f64 = activities
                            .iter()
                            .flat_map(|a| a.execution_legs.iter())
                            .map(|leg| leg.price * leg.quantity * multiplier)
                            .sum();
                        if total > 0.0 { Some(total) } else { None }
                    } else {
                        order.price.map(|p| p * order.quantity * multiplier)
                    };
                    return Ok(model::AutoMidOrderResponse {
                        created: true,
                        order_id: Some(current_order_id as u64),
                        loops: loop_count,
                        fill_value,
                        market_order: false,
                        message: Some("Order filled".into()),
                    });
                }
                OrderStatus::Rejected | OrderStatus::Expired => {
                    return Err(Error::AutoMid(format!(
                        "Order {} ended with status {:?}",
                        current_order_id, order.status
                    )));
                }
                OrderStatus::Canceled => {
                    return Err(Error::AutoMid(format!(
                        "Order {} was unexpectedly canceled",
                        current_order_id
                    )));
                }
                _ => {} // Working, Queued, Accepted, etc.
            }

            // Re-fetch the live mid price to base this loop's limit price on.
            let current_mid = match fetch_mid_price(&client, &access_token, &symbol).await {
                Ok(m) => m,
                Err(e) => {
                    cancel_order(&client, &access_token, &account_number, current_order_id).await;
                    return Err(Error::AutoMid(format!("Failed to fetch mid price: {e}")));
                }
            };

            // Apply an incrementally increasing offset to the current mid:
            // loop 1 → step*1, loop 2 → step*2, … loop N → order_value_max_percent_change.
            let percent = step * f64::from(loop_count);
            let next_price = if is_buy {
                current_mid * (1.0 + percent)
            } else {
                current_mid * (1.0 - percent)
            };
            let next_price = (next_price * 100.0).round() / 100.0;

            log::info!(
                "Auto-mid {}: mid={:.4}, percent={:.4}%, next_price={:.2}, account={}",
                current_order_id,
                current_mid,
                percent * 100.0,
                next_price,
                account_number
            );

            let adjusted = match model::OrderRequest::limit(
                make_instrument(&symbol),
                instruction,
                quantity,
                next_price,
            ) {
                Ok(o) => o,
                Err(e) => {
                    cancel_order(&client, &access_token, &account_number, current_order_id).await;
                    return Err(e);
                }
            };

            let new_id = match PutAccountOrderRequest::new(
                &client,
                access_token.clone(),
                account_number.clone(),
                current_order_id,
                adjusted,
            )
            .send()
            .await
            {
                Ok(id) => id,
                Err(e) => {
                    cancel_order(&client, &access_token, &account_number, current_order_id).await;
                    return Err(Error::AutoMid(format!(
                        "Failed to update limit price: {}",
                        e
                    )));
                }
            };

            if let Some(new_id) = new_id {
                current_order_id = new_id;
            }

            log::info!(
                "Updated auto-mid {} to price {:.2} for account {}",
                current_order_id,
                next_price,
                account_number
            );
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
