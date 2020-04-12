// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *reports* crate version *1.0.13+20191014*, where *20191014* is the exact revision of the *admin:reports_v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *reports* *v1_reports* API can be found at the
//! [official documentation site](/admin-sdk/reports/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/admin1_reports).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Reports.html) ... 
//! 
//! * [activities](struct.Activity.html)
//!  * [*list*](struct.ActivityListCall.html) and [*watch*](struct.ActivityWatchCall.html)
//! * [channels](struct.Channel.html)
//!  * [*stop*](struct.ChannelStopCall.html)
//! * customer usage reports
//!  * [*get*](struct.CustomerUsageReportGetCall.html)
//! * entity usage reports
//!  * [*get*](struct.EntityUsageReportGetCall.html)
//! * user usage report
//!  * [*get*](struct.UserUsageReportGetCall.html)
//! 
//! 
//! Subscription supported by ...
//! 
//! * [*list activities*](struct.ActivityListCall.html)
//! * [*watch activities*](struct.ActivityWatchCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Reports.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.user_usage_report().get(...).doit()
//! let r = hub.entity_usage_reports().get(...).doit()
//! let r = hub.customer_usage_reports().get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-admin1_reports = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_admin1_reports as admin1_reports;
//! use admin1_reports::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use admin1_reports::Reports;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.user_usage_report().get("userKey", "date")
//!              .parameters("amet.")
//!              .page_token("erat")
//!              .org_unit_id("labore")
//!              .max_results(92)
//!              .filters("nonumy")
//!              .customer_id("dolores")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View audit reports for your G Suite domain
    ReportAuditReadonly,

    /// View usage reports for your G Suite domain
    ReportUsageReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ReportAuditReadonly => "https://www.googleapis.com/auth/admin.reports.audit.readonly",
            Scope::ReportUsageReadonly => "https://www.googleapis.com/auth/admin.reports.usage.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ReportAuditReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Reports related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// use admin1_reports::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_usage_report().get("userKey", "date")
///              .parameters("aliquyam")
///              .page_token("ea")
///              .org_unit_id("no")
///              .max_results(80)
///              .filters("justo")
///              .customer_id("et")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Reports<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Reports<C, A> {}

impl<'a, C, A> Reports<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Reports<C, A> {
        Reports {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://www.googleapis.com/admin/reports/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C, A> {
        ActivityMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, C, A> {
        ChannelMethods { hub: &self }
    }
    pub fn customer_usage_reports(&'a self) -> CustomerUsageReportMethods<'a, C, A> {
        CustomerUsageReportMethods { hub: &self }
    }
    pub fn entity_usage_reports(&'a self) -> EntityUsageReportMethods<'a, C, A> {
        EntityUsageReportMethods { hub: &self }
    }
    pub fn user_usage_report(&'a self) -> UserUsageReportMethods<'a, C, A> {
        UserUsageReportMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/admin/reports/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as [{parameter: [{name: city, value: abc}]}]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityEventsParametersMessageValue {
    /// Parameter values
    pub parameter: Option<Vec<NestedParameter>>,
}

impl NestedType for ActivityEventsParametersMessageValue {}
impl Part for ActivityEventsParametersMessageValue {}


/// Key-value pairs to give detailed information on the warning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReportsWarningsData {
    /// Key associated with a key-value pair to give detailed information on the warning.
    pub key: Option<String>,
    /// Value associated with a key-value pair to give detailed information on the warning.
    pub value: Option<String>,
}

impl NestedType for UsageReportsWarningsData {}
impl Part for UsageReportsWarningsData {}


/// JSON template for a usage report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReport {
    /// The date of the report request.
    pub date: Option<String>,
    /// The type of API resource. For a usage report, the value is admin#reports#usageReport.
    pub kind: Option<String>,
    /// ETag of the resource.
    pub etag: Option<String>,
    /// Parameter value pairs for various applications. For the Customers usage report parameters and values, see the customer usage parameters reference.
    pub parameters: Option<Vec<UsageReportParameters>>,
    /// Information about the type of the item.
    pub entity: Option<UsageReportEntity>,
}

impl Part for UsageReport {}


/// Unique identifier for each activity record.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityId {
    /// Application name to which the event belongs. For possible values see the list of applications above in applicationName.
    #[serde(rename="applicationName")]
    pub application_name: Option<String>,
    /// Unique qualifier if multiple events have the same time.
    #[serde(rename="uniqueQualifier")]
    pub unique_qualifier: Option<String>,
    /// The unique identifier for a G suite account.
    #[serde(rename="customerId")]
    pub customer_id: Option<String>,
    /// Time of occurrence of the activity. This is in UNIX epoch time in seconds.
    pub time: Option<String>,
}

impl NestedType for ActivityId {}
impl Part for ActivityId {}


/// Warnings, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReportsWarnings {
    /// The human readable messages for a warning are:  
    /// - Data is not available warning - Sorry, data for date yyyy-mm-dd for application "application name" is not available.  
    /// - Partial data is available warning - Data for date yyyy-mm-dd for application "application name" is not available right now, please try again after a few hours.
    pub message: Option<String>,
    /// Machine readable code or warning type. The warning code value is 200.
    pub code: Option<String>,
    /// Key-value pairs to give detailed information on the warning.
    pub data: Option<Vec<UsageReportsWarningsData>>,
}

impl NestedType for UsageReportsWarnings {}
impl Part for UsageReportsWarnings {}


/// JSON template for the activity resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The type of API resource. For an activity report, the value is audit#activity.
    pub kind: Option<String>,
    /// ETag of the entry.
    pub etag: Option<String>,
    /// Activity events in the report.
    pub events: Option<Vec<ActivityEvents>>,
    /// IP address of the user doing the action. This is the Internet Protocol (IP) address of the user when logging into G Suite which may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. The API supports IPv4 and IPv6.
    #[serde(rename="ipAddress")]
    pub ip_address: Option<String>,
    /// This is the domain that is affected by the report's event. For example domain of Admin console or the Drive application's document owner.
    #[serde(rename="ownerDomain")]
    pub owner_domain: Option<String>,
    /// User doing the action.
    pub actor: Option<ActivityActor>,
    /// Unique identifier for each activity record.
    pub id: Option<ActivityId>,
}

impl Part for Activity {}


/// JSON template for a parameter used in various reports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NestedParameter {
    /// Multiple boolean values of the parameter.
    #[serde(rename="multiBoolValue")]
    pub multi_bool_value: Option<Vec<bool>>,
    /// Boolean value of the parameter.
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// Multiple string values of the parameter.
    #[serde(rename="multiValue")]
    pub multi_value: Option<Vec<String>>,
    /// The name of the parameter.
    pub name: Option<String>,
    /// Multiple integer values of the parameter.
    #[serde(rename="multiIntValue")]
    pub multi_int_value: Option<Vec<String>>,
    /// Integer value of the parameter.
    #[serde(rename="intValue")]
    pub int_value: Option<String>,
    /// String value of the parameter.
    pub value: Option<String>,
}

impl Part for NestedParameter {}


/// An notification channel used to watch for resource changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch activities](struct.ActivityWatchCall.html) (request|response)
/// * [stop channels](struct.ChannelStopCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// A version-specific identifier for the watched resource.
    #[serde(rename="resourceUri")]
    pub resource_uri: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    pub kind: Option<String>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename="resourceId")]
    pub resource_id: Option<String>,
    /// A UUID or similar unique string that identifies this channel.
    pub id: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    pub token: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    pub params: Option<HashMap<String, String>>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    pub expiration: Option<String>,
    /// The address where notifications are delivered for this channel.
    pub address: Option<String>,
    /// The type of delivery mechanism used for this channel.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    pub payload: Option<bool>,
}

impl RequestValue for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}


/// JSON template for a collection of activites.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](struct.ActivityListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activities {
    /// Token for retrieving the follow-on next page of the report. The nextPageToken value is used in the request's pageToken query string.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Each activity record in the response.
    pub items: Option<Vec<Activity>>,
    /// The type of API resource. For an activity report, the value is reports#activities.
    pub kind: Option<String>,
    /// ETag of the resource.
    pub etag: Option<String>,
}

impl ResponseResult for Activities {}


/// List of messageValue objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityEventsParametersMultiMessageValue {
    /// Parameter values
    pub parameter: Option<Vec<NestedParameter>>,
}

impl NestedType for ActivityEventsParametersMultiMessageValue {}
impl Part for ActivityEventsParametersMultiMessageValue {}


/// Parameter value pairs for various applications. For the Customers usage report parameters and values, see the customer usage parameters reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReportParameters {
    /// Nested message value of the parameter.
    #[serde(rename="msgValue")]
    pub msg_value: Option<Vec<HashMap<String, String>>>,
    /// The RFC 3339 formatted value of the parameter, for example 2010-10-28T10:26:35.000Z.
    #[serde(rename="datetimeValue")]
    pub datetime_value: Option<String>,
    /// no description provided
    pub name: Option<String>,
    /// String value of the parameter.
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
    /// Boolean value of the parameter.
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// Integer value of the parameter.
    #[serde(rename="intValue")]
    pub int_value: Option<String>,
}

impl NestedType for UsageReportParameters {}
impl Part for UsageReportParameters {}


/// User doing the action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActor {
    /// The unique G Suite profile ID of the actor. May be absent if the actor is not a G Suite user.
    #[serde(rename="profileId")]
    pub profile_id: Option<String>,
    /// The primary email address of the actor. May be absent if there is no email address associated with the actor.
    pub email: Option<String>,
    /// Only present when callerType is KEY. Can be the consumer_key of the requestor for OAuth 2LO API requests or an identifier for robot accounts.
    pub key: Option<String>,
    /// The type of actor.
    #[serde(rename="callerType")]
    pub caller_type: Option<String>,
}

impl NestedType for ActivityActor {}
impl Part for ActivityActor {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get user usage report](struct.UserUsageReportGetCall.html) (response)
/// * [get entity usage reports](struct.EntityUsageReportGetCall.html) (response)
/// * [get customer usage reports](struct.CustomerUsageReportGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReports {
    /// Token to specify next page. A report with multiple pages has a nextPageToken property in the response. For your follow-on requests getting all of the report's pages, enter the nextPageToken value in the pageToken query string.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The type of API resource. For a usage report, the value is admin#reports#usageReports.
    pub kind: Option<String>,
    /// ETag of the resource.
    pub etag: Option<String>,
    /// Various application parameter records.
    #[serde(rename="usageReports")]
    pub usage_reports: Option<Vec<UsageReport>>,
    /// Warnings, if any.
    pub warnings: Option<Vec<UsageReportsWarnings>>,
}

impl ResponseResult for UsageReports {}


/// Parameter value pairs for various applications. For more information about eventName parameters, see the list of event names for various applications above in applicationName.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityEventsParameters {
    /// The name of the parameter.
    pub name: Option<String>,
    /// Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as [{parameter: [{name: city, value: abc}]}]
    #[serde(rename="messageValue")]
    pub message_value: Option<ActivityEventsParametersMessageValue>,
    /// Boolean value of the parameter.
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// String value of the parameter.
    pub value: Option<String>,
    /// Integer value of the parameter.
    #[serde(rename="intValue")]
    pub int_value: Option<String>,
    /// String values of the parameter.
    #[serde(rename="multiValue")]
    pub multi_value: Option<Vec<String>>,
    /// Integer values of the parameter.
    #[serde(rename="multiIntValue")]
    pub multi_int_value: Option<Vec<String>>,
    /// List of messageValue objects.
    #[serde(rename="multiMessageValue")]
    pub multi_message_value: Option<Vec<ActivityEventsParametersMultiMessageValue>>,
}

impl NestedType for ActivityEventsParameters {}
impl Part for ActivityEventsParameters {}


/// Activity events in the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityEvents {
    /// Type of event. The G Suite service or feature that an administrator changes is identified in the type property which identifies an event using the eventName property. For a full list of the API's type categories, see the list of event names for various applications above in applicationName.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Name of the event. This is the specific name of the activity reported by the API. And each eventName is related to a specific G Suite service or feature which the API organizes into types of events.
    /// For eventName request parameters in general:  
    /// - If no eventName is given, the report returns all possible instances of an eventName.  
    /// - When you request an eventName, the API's response returns all activities which contain that eventName. It is possible that the returned activities will have other eventName properties in addition to the one requested.   
    /// For more information about eventName properties, see the list of event names for various applications above in applicationName.
    pub name: Option<String>,
    /// Parameter value pairs for various applications. For more information about eventName parameters, see the list of event names for various applications above in applicationName.
    pub parameters: Option<Vec<ActivityEventsParameters>>,
}

impl NestedType for ActivityEvents {}
impl Part for ActivityEvents {}


/// Information about the type of the item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsageReportEntity {
    /// The user's immutable G Suite profile identifier.
    #[serde(rename="profileId")]
    pub profile_id: Option<String>,
    /// The user's email address. Only relevant if entity.type = "USER"
    #[serde(rename="userEmail")]
    pub user_email: Option<String>,
    /// The type of item. The value is customer.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Object key. Only relevant if entity.type = "OBJECT" Note: external-facing name of report is "Entities" rather than "Objects".
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// The unique identifier of the customer's account.
    #[serde(rename="customerId")]
    pub customer_id: Option<String>,
}

impl NestedType for UsageReportEntity {}
impl Part for UsageReportEntity {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `Reports` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
}

impl<'a, C, A> MethodsBuilder for ChannelMethods<'a, C, A> {}

impl<'a, C, A> ChannelMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a, C, A> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *entityUsageReport* resources.
/// It is not used directly, but through the `Reports` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.entity_usage_reports();
/// # }
/// ```
pub struct EntityUsageReportMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
}

impl<'a, C, A> MethodsBuilder for EntityUsageReportMethods<'a, C, A> {}

impl<'a, C, A> EntityUsageReportMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides.
    /// 
    /// # Arguments
    ///
    /// * `entityType` - Represents the type of entity for the report.
    /// * `entityKey` - Represents the key of the object to filter the data with.
    /// * `date` - Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    pub fn get(&self, entity_type: &str, entity_key: &str, date: &str) -> EntityUsageReportGetCall<'a, C, A> {
        EntityUsageReportGetCall {
            hub: self.hub,
            _entity_type: entity_type.to_string(),
            _entity_key: entity_key.to_string(),
            _date: date.to_string(),
            _parameters: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `Reports` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActivityMethods<'a, C, A> {}

impl<'a, C, A> ActivityMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of activities for a specific customer's account and application such as the Admin console application or the Google Drive application. For more information, see the guides for administrator and Google Drive activity reports. For more information about the activity report's parameters, see the activity parameters reference guides.
    /// 
    /// # Arguments
    ///
    /// * `userKey` - Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    /// * `applicationName` - Application name for which the events are to be retrieved.
    pub fn list(&self, user_key: &str, application_name: &str) -> ActivityListCall<'a, C, A> {
        ActivityListCall {
            hub: self.hub,
            _user_key: user_key.to_string(),
            _application_name: application_name.to_string(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _org_unit_id: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _event_name: Default::default(),
            _end_time: Default::default(),
            _customer_id: Default::default(),
            _actor_ip_address: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Start receiving notifications for account activities. For more information, see Receiving Push Notifications.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userKey` - Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    /// * `applicationName` - Application name for which the events are to be retrieved.
    pub fn watch(&self, request: Channel, user_key: &str, application_name: &str) -> ActivityWatchCall<'a, C, A> {
        ActivityWatchCall {
            hub: self.hub,
            _request: request,
            _user_key: user_key.to_string(),
            _application_name: application_name.to_string(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _org_unit_id: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _event_name: Default::default(),
            _end_time: Default::default(),
            _customer_id: Default::default(),
            _actor_ip_address: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *customerUsageReport* resources.
/// It is not used directly, but through the `Reports` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.customer_usage_reports();
/// # }
/// ```
pub struct CustomerUsageReportMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
}

impl<'a, C, A> MethodsBuilder for CustomerUsageReportMethods<'a, C, A> {}

impl<'a, C, A> CustomerUsageReportMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report which is a collection of properties and statistics for a specific customer's account. For more information, see the Customers Usage Report guide. For more information about the customer report's parameters, see the Customers Usage parameters reference guides.
    /// 
    /// # Arguments
    ///
    /// * `date` - Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    pub fn get(&self, date: &str) -> CustomerUsageReportGetCall<'a, C, A> {
        CustomerUsageReportGetCall {
            hub: self.hub,
            _date: date.to_string(),
            _parameters: Default::default(),
            _page_token: Default::default(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userUsageReport* resources.
/// It is not used directly, but through the `Reports` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_admin1_reports as admin1_reports;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use admin1_reports::Reports;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.user_usage_report();
/// # }
/// ```
pub struct UserUsageReportMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserUsageReportMethods<'a, C, A> {}

impl<'a, C, A> UserUsageReportMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides.
    /// 
    /// # Arguments
    ///
    /// * `userKey` - Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    /// * `date` - Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    pub fn get(&self, user_key: &str, date: &str) -> UserUsageReportGetCall<'a, C, A> {
        UserUsageReportGetCall {
            hub: self.hub,
            _user_key: user_key.to_string(),
            _date: date.to_string(),
            _parameters: Default::default(),
            _page_token: Default::default(),
            _org_unit_id: Default::default(),
            _max_results: Default::default(),
            _filters: Default::default(),
            _customer_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Stop watching resources through this channel
///
/// A builder for the *stop* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// use admin1_reports::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().stop(req)
///              .doit();
/// # }
/// ```
pub struct ChannelStopCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _request: Channel,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ChannelStopCall<'a, C, A> {}

impl<'a, C, A> ChannelStopCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "admin.channels.stop",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "/admin/reports_v1/channels/stop";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportAuditReadonly.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ChannelStopCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ChannelStopCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportAuditReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ChannelStopCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides.
///
/// A builder for the *get* method supported by a *entityUsageReport* resource.
/// It is not used directly, but through a `EntityUsageReportMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.entity_usage_reports().get("entityType", "entityKey", "date")
///              .parameters("Lorem")
///              .page_token("et")
///              .max_results(31)
///              .filters("aliquyam")
///              .customer_id("sea")
///              .doit();
/// # }
/// ```
pub struct EntityUsageReportGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _entity_type: String,
    _entity_key: String,
    _date: String,
    _parameters: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filters: Option<String>,
    _customer_id: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for EntityUsageReportGetCall<'a, C, A> {}

impl<'a, C, A> EntityUsageReportGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UsageReports)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "reports.entityUsageReports.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("entityType", self._entity_type.to_string()));
        params.push(("entityKey", self._entity_key.to_string()));
        params.push(("date", self._date.to_string()));
        if let Some(value) = self._parameters {
            params.push(("parameters", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filters {
            params.push(("filters", value.to_string()));
        }
        if let Some(value) = self._customer_id {
            params.push(("customerId", value.to_string()));
        }
        for &field in ["alt", "entityType", "entityKey", "date", "parameters", "pageToken", "maxResults", "filters", "customerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "usage/{entityType}/{entityKey}/dates/{date}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportUsageReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{entityType}", "entityType"), ("{entityKey}", "entityKey"), ("{date}", "date")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["date", "entityKey", "entityType"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Represents the type of entity for the report.
    ///
    /// Sets the *entity type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity_type(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._entity_type = new_value.to_string();
        self
    }
    /// Represents the key of the object to filter the data with.
    ///
    /// Sets the *entity key* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity_key(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._entity_key = new_value.to_string();
        self
    }
    /// Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    ///
    /// Sets the *date* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn date(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._date = new_value.to_string();
        self
    }
    /// The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Entities usage report are only gplus.
    /// A parameter query string is in the CSV form of [app_name1:param_name1], [app_name2:param_name2]....
    /// Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.
    /// In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.
    /// 
    /// An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned.
    ///
    /// Sets the *parameters* query property to the given value.
    pub fn parameters(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._parameters = Some(new_value.to_string());
        self
    }
    /// Token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> EntityUsageReportGetCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The filters query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The filters query string includes the name of the application whose usage is returned in the report. The application values for the Entities usage report include accounts, docs, and gmail.
    /// Filters are in the form [application name]:[parameter name][relational operator][parameter value],....
    /// 
    /// In this example, the <> 'not equal to' operator is URL-encoded in the request's query string (%3C%3E):
    /// GET https://www.googleapis.com/admin/reports/v1/usage/gplus_communities/all/dates/2017-12-01 ?parameters=gplus:community_name,gplus:num_total_members &filters=gplus:num_total_members>0
    /// 
    /// 
    /// The relational operators include:  
    /// - == - 'equal to'. 
    /// - <> - 'not equal to'. It is URL-encoded (%3C%3E). 
    /// - < - 'less than'. It is URL-encoded (%3C). 
    /// - <= - 'less than or equal to'. It is URL-encoded (%3C=). 
    /// - > - 'greater than'. It is URL-encoded (%3E). 
    /// - >= - 'greater than or equal to'. It is URL-encoded (%3E=).  Filters can only be applied to numeric parameters.
    ///
    /// Sets the *filters* query property to the given value.
    pub fn filters(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._filters = Some(new_value.to_string());
        self
    }
    /// The unique ID of the customer to retrieve data for.
    ///
    /// Sets the *customer id* query property to the given value.
    pub fn customer_id(mut self, new_value: &str) -> EntityUsageReportGetCall<'a, C, A> {
        self._customer_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> EntityUsageReportGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> EntityUsageReportGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportUsageReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> EntityUsageReportGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of activities for a specific customer's account and application such as the Admin console application or the Google Drive application. For more information, see the guides for administrator and Google Drive activity reports. For more information about the activity report's parameters, see the activity parameters reference guides.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("userKey", "applicationName")
///              .start_time("erat")
///              .page_token("sadipscing")
///              .org_unit_id("dolor")
///              .max_results(-39)
///              .filters("elitr")
///              .event_name("amet")
///              .end_time("no")
///              .customer_id("labore")
///              .actor_ip_address("eirmod")
///              .doit();
/// # }
/// ```
pub struct ActivityListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _user_key: String,
    _application_name: String,
    _start_time: Option<String>,
    _page_token: Option<String>,
    _org_unit_id: Option<String>,
    _max_results: Option<i32>,
    _filters: Option<String>,
    _event_name: Option<String>,
    _end_time: Option<String>,
    _customer_id: Option<String>,
    _actor_ip_address: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityListCall<'a, C, A> {}

impl<'a, C, A> ActivityListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Activities)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "reports.activities.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("userKey", self._user_key.to_string()));
        params.push(("applicationName", self._application_name.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitID", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filters {
            params.push(("filters", value.to_string()));
        }
        if let Some(value) = self._event_name {
            params.push(("eventName", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if let Some(value) = self._customer_id {
            params.push(("customerId", value.to_string()));
        }
        if let Some(value) = self._actor_ip_address {
            params.push(("actorIpAddress", value.to_string()));
        }
        for &field in ["alt", "userKey", "applicationName", "startTime", "pageToken", "orgUnitID", "maxResults", "filters", "eventName", "endTime", "customerId", "actorIpAddress"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "activity/users/{userKey}/applications/{applicationName}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportAuditReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userKey}", "userKey"), ("{applicationName}", "applicationName")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["applicationName", "userKey"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    ///
    /// Sets the *user key* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_key(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._user_key = new_value.to_string();
        self
    }
    /// Application name for which the events are to be retrieved.
    ///
    /// Sets the *application name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn application_name(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._application_name = new_value.to_string();
        self
    }
    /// Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from startTime until endTime. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// The token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page. The maxResults query string is optional in the request. The default value is 1000.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: i32) -> ActivityListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The filters query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form [parameter1 name][relational operator][parameter1 value],[parameter2 name][relational operator][parameter2 value],... 
    /// These event parameters are associated with a specific eventName. An empty report is returned if the filtered request's parameter does not belong to the eventName. For more information about eventName parameters, see the list of event names for various applications above in applicationName.
    /// 
    /// In the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E):
    /// GET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS
    /// 
    /// In the following Drive example, the list can be a view or edit event's doc_id parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's doc_id. In the second example, the report returns each viewed document's doc_id that equals the value 12345 and does not return any viewed document's which have a doc_id value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E):
    /// 
    /// GET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765
    /// 
    /// The relational operators include:  
    /// - == - 'equal to'. 
    /// - <> - 'not equal to'. It is URL-encoded (%3C%3E). 
    /// - < - 'less than'. It is URL-encoded (%3C). 
    /// - <= - 'less than or equal to'. It is URL-encoded (%3C=). 
    /// - > - 'greater than'. It is URL-encoded (%3E). 
    /// - >= - 'greater than or equal to'. It is URL-encoded (%3E=).  
    /// Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.
    /// In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned.
    ///
    /// Sets the *filters* query property to the given value.
    pub fn filters(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._filters = Some(new_value.to_string());
        self
    }
    /// The name of the event being queried by the API. Each eventName is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings type structure has all of the Calendar eventName activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings type and eventName parameters. For more information about eventName query strings and parameters, see the list of event names for various applications above in applicationName.
    ///
    /// Sets the *event name* query property to the given value.
    pub fn event_name(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._event_name = Some(new_value.to_string());
        self
    }
    /// Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts:  
    /// - Date of the API's request for a report: When the API created and retrieved the report. 
    /// - Report's start time: The beginning of the timespan shown in the report. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error. 
    /// - Report's end time: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August.  If the endTime is not specified, the report returns all activities from the startTime until the current time or the most recent 180 days if the startTime is more than 180 days in the past.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// The unique ID of the customer to retrieve data for.
    ///
    /// Sets the *customer id* query property to the given value.
    pub fn customer_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._customer_id = Some(new_value.to_string());
        self
    }
    /// The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions.
    ///
    /// Sets the *actor ip address* query property to the given value.
    pub fn actor_ip_address(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._actor_ip_address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ActivityListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportAuditReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ActivityListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Start receiving notifications for account activities. For more information, see Receiving Push Notifications.
///
/// A builder for the *watch* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// use admin1_reports::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().watch(req, "userKey", "applicationName")
///              .start_time("aliquyam")
///              .page_token("accusam")
///              .org_unit_id("Lorem")
///              .max_results(-9)
///              .filters("et")
///              .event_name("duo")
///              .end_time("et")
///              .customer_id("eirmod")
///              .actor_ip_address("sanctus")
///              .doit();
/// # }
/// ```
pub struct ActivityWatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _request: Channel,
    _user_key: String,
    _application_name: String,
    _start_time: Option<String>,
    _page_token: Option<String>,
    _org_unit_id: Option<String>,
    _max_results: Option<i32>,
    _filters: Option<String>,
    _event_name: Option<String>,
    _end_time: Option<String>,
    _customer_id: Option<String>,
    _actor_ip_address: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityWatchCall<'a, C, A> {}

impl<'a, C, A> ActivityWatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "reports.activities.watch",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("userKey", self._user_key.to_string()));
        params.push(("applicationName", self._application_name.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitID", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filters {
            params.push(("filters", value.to_string()));
        }
        if let Some(value) = self._event_name {
            params.push(("eventName", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if let Some(value) = self._customer_id {
            params.push(("customerId", value.to_string()));
        }
        if let Some(value) = self._actor_ip_address {
            params.push(("actorIpAddress", value.to_string()));
        }
        for &field in ["alt", "userKey", "applicationName", "startTime", "pageToken", "orgUnitID", "maxResults", "filters", "eventName", "endTime", "customerId", "actorIpAddress"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "activity/users/{userKey}/applications/{applicationName}/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportAuditReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userKey}", "userKey"), ("{applicationName}", "applicationName")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["applicationName", "userKey"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ActivityWatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    ///
    /// Sets the *user key* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_key(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._user_key = new_value.to_string();
        self
    }
    /// Application name for which the events are to be retrieved.
    ///
    /// Sets the *application name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn application_name(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._application_name = new_value.to_string();
        self
    }
    /// Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from startTime until endTime. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// The token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page. The maxResults query string is optional in the request. The default value is 1000.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: i32) -> ActivityWatchCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The filters query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form [parameter1 name][relational operator][parameter1 value],[parameter2 name][relational operator][parameter2 value],... 
    /// These event parameters are associated with a specific eventName. An empty report is returned if the filtered request's parameter does not belong to the eventName. For more information about eventName parameters, see the list of event names for various applications above in applicationName.
    /// 
    /// In the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E):
    /// GET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS
    /// 
    /// In the following Drive example, the list can be a view or edit event's doc_id parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's doc_id. In the second example, the report returns each viewed document's doc_id that equals the value 12345 and does not return any viewed document's which have a doc_id value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E):
    /// 
    /// GET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765
    /// 
    /// The relational operators include:  
    /// - == - 'equal to'. 
    /// - <> - 'not equal to'. It is URL-encoded (%3C%3E). 
    /// - < - 'less than'. It is URL-encoded (%3C). 
    /// - <= - 'less than or equal to'. It is URL-encoded (%3C=). 
    /// - > - 'greater than'. It is URL-encoded (%3E). 
    /// - >= - 'greater than or equal to'. It is URL-encoded (%3E=).  
    /// Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.
    /// In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned.
    ///
    /// Sets the *filters* query property to the given value.
    pub fn filters(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._filters = Some(new_value.to_string());
        self
    }
    /// The name of the event being queried by the API. Each eventName is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings type structure has all of the Calendar eventName activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings type and eventName parameters. For more information about eventName query strings and parameters, see the list of event names for various applications above in applicationName.
    ///
    /// Sets the *event name* query property to the given value.
    pub fn event_name(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._event_name = Some(new_value.to_string());
        self
    }
    /// Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts:  
    /// - Date of the API's request for a report: When the API created and retrieved the report. 
    /// - Report's start time: The beginning of the timespan shown in the report. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error. 
    /// - Report's end time: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August.  If the endTime is not specified, the report returns all activities from the startTime until the current time or the most recent 180 days if the startTime is more than 180 days in the past.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// The unique ID of the customer to retrieve data for.
    ///
    /// Sets the *customer id* query property to the given value.
    pub fn customer_id(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._customer_id = Some(new_value.to_string());
        self
    }
    /// The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions.
    ///
    /// Sets the *actor ip address* query property to the given value.
    pub fn actor_ip_address(mut self, new_value: &str) -> ActivityWatchCall<'a, C, A> {
        self._actor_ip_address = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> ActivityWatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityWatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportAuditReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ActivityWatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a report which is a collection of properties and statistics for a specific customer's account. For more information, see the Customers Usage Report guide. For more information about the customer report's parameters, see the Customers Usage parameters reference guides.
///
/// A builder for the *get* method supported by a *customerUsageReport* resource.
/// It is not used directly, but through a `CustomerUsageReportMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customer_usage_reports().get("date")
///              .parameters("amet")
///              .page_token("et")
///              .customer_id("consetetur")
///              .doit();
/// # }
/// ```
pub struct CustomerUsageReportGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _date: String,
    _parameters: Option<String>,
    _page_token: Option<String>,
    _customer_id: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CustomerUsageReportGetCall<'a, C, A> {}

impl<'a, C, A> CustomerUsageReportGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UsageReports)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "reports.customerUsageReports.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("date", self._date.to_string()));
        if let Some(value) = self._parameters {
            params.push(("parameters", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._customer_id {
            params.push(("customerId", value.to_string()));
        }
        for &field in ["alt", "date", "parameters", "pageToken", "customerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "usage/dates/{date}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportUsageReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{date}", "date")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["date"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    ///
    /// Sets the *date* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn date(mut self, new_value: &str) -> CustomerUsageReportGetCall<'a, C, A> {
        self._date = new_value.to_string();
        self
    }
    /// The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers usage report include accounts, app_maker, apps_scripts, calendar, classroom, cros, docs, gmail, gplus, device_management, meet, and sites.
    /// A parameters query string is in the CSV form of app_name1:param_name1, app_name2:param_name2.
    /// Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.
    /// In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.
    /// 
    /// An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned.
    ///
    /// Sets the *parameters* query property to the given value.
    pub fn parameters(mut self, new_value: &str) -> CustomerUsageReportGetCall<'a, C, A> {
        self._parameters = Some(new_value.to_string());
        self
    }
    /// Token to specify next page. A report with multiple pages has a nextPageToken property in the response. For your follow-on requests getting all of the report's pages, enter the nextPageToken value in the pageToken query string.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerUsageReportGetCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The unique ID of the customer to retrieve data for.
    ///
    /// Sets the *customer id* query property to the given value.
    pub fn customer_id(mut self, new_value: &str) -> CustomerUsageReportGetCall<'a, C, A> {
        self._customer_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> CustomerUsageReportGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CustomerUsageReportGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportUsageReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerUsageReportGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides.
///
/// A builder for the *get* method supported by a *userUsageReport* resource.
/// It is not used directly, but through a `UserUsageReportMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_admin1_reports as admin1_reports;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use admin1_reports::Reports;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Reports::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_usage_report().get("userKey", "date")
///              .parameters("sed")
///              .page_token("dolor")
///              .org_unit_id("dolor")
///              .max_results(53)
///              .filters("et")
///              .customer_id("consetetur")
///              .doit();
/// # }
/// ```
pub struct UserUsageReportGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Reports<C, A>,
    _user_key: String,
    _date: String,
    _parameters: Option<String>,
    _page_token: Option<String>,
    _org_unit_id: Option<String>,
    _max_results: Option<u32>,
    _filters: Option<String>,
    _customer_id: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserUsageReportGetCall<'a, C, A> {}

impl<'a, C, A> UserUsageReportGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UsageReports)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "reports.userUsageReport.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("userKey", self._user_key.to_string()));
        params.push(("date", self._date.to_string()));
        if let Some(value) = self._parameters {
            params.push(("parameters", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitID", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filters {
            params.push(("filters", value.to_string()));
        }
        if let Some(value) = self._customer_id {
            params.push(("customerId", value.to_string()));
        }
        for &field in ["alt", "userKey", "date", "parameters", "pageToken", "orgUnitID", "maxResults", "filters", "customerId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "usage/users/{userKey}/dates/{date}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ReportUsageReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userKey}", "userKey"), ("{date}", "date")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["date", "userKey"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Represents the profile ID or the user email for which the data should be filtered. Can be all for all information, or userKey for a user's unique G Suite profile ID or their primary email address.
    ///
    /// Sets the *user key* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_key(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._user_key = new_value.to_string();
        self
    }
    /// Represents the date the usage occurred. The timestamp is in the ISO 8601 format, yyyy-mm-dd. We recommend you use your account's time zone for this.
    ///
    /// Sets the *date* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn date(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._date = new_value.to_string();
        self
    }
    /// The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers usage report include accounts, app_maker, apps_scripts, calendar, classroom, cros, docs, gmail, gplus, device_management, meet, and sites.
    /// A parameters query string is in the CSV form of app_name1:param_name1, app_name2:param_name2.
    /// Note: The API doesn't accept multiple values of a parameter.
    /// If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.
    /// 
    /// An example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned.
    ///
    /// Sets the *parameters* query property to the given value.
    pub fn parameters(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._parameters = Some(new_value.to_string());
        self
    }
    /// Token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// ID of the organizational unit to report on. User activity will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page.
    /// The maxResults query string is optional.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> UserUsageReportGetCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The filters query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The filters query string includes the name of the application whose usage is returned in the report. The application values for the Users Usage Report include accounts, docs, and gmail.
    /// Filters are in the form [application name]:[parameter name][relational operator][parameter value],....
    /// 
    /// In this example, the <> 'not equal to' operator is URL-encoded in the request's query string (%3C%3E):
    /// GET https://www.googleapis.com/admin/reports/v1/usage/users/all/dates/2013-03-03 ?parameters=accounts:last_login_time &filters=accounts:last_login_time>2010-10-28T10:26:35.000Z
    /// 
    /// 
    /// The relational operators include:  
    /// - == - 'equal to'. 
    /// - <> - 'not equal to'. It is URL-encoded (%3C%3E). 
    /// - < - 'less than'. It is URL-encoded (%3C). 
    /// - <= - 'less than or equal to'. It is URL-encoded (%3C=). 
    /// - > - 'greater than'. It is URL-encoded (%3E). 
    /// - >= - 'greater than or equal to'. It is URL-encoded (%3E=).
    ///
    /// Sets the *filters* query property to the given value.
    pub fn filters(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._filters = Some(new_value.to_string());
        self
    }
    /// The unique ID of the customer to retrieve data for.
    ///
    /// Sets the *customer id* query property to the given value.
    pub fn customer_id(mut self, new_value: &str) -> UserUsageReportGetCall<'a, C, A> {
        self._customer_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserUsageReportGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> UserUsageReportGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ReportUsageReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserUsageReportGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


