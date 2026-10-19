#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "Optional annotations for the client. The client can use annotations to inform how objects are used or displayed"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct Annotations {
    #[doc = "Describes who the intended audience of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[doc = "The moment the resource was last modified, as an ISO 8601 formatted string.\n\nShould be an ISO 8601 formatted string (e.g., \"2025-01-12T15:00:58Z\").\n\nExamples: last activity timestamp in an open file, timestamp when the resource\nwas attached, etc."]
    #[serde(
        rename = "lastModified",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified: ::std::option::Option<::std::string::String>,
    #[doc = "Describes how important this data is for operating the server.\n\nA value of 1 means \"most important,\" and indicates that the data is\neffectively required, while 0 means \"least important,\" and indicates that\nthe data is entirely optional."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl Annotations {
    pub fn builder() -> builder::Annotations {
        Default::default()
    }
}
#[doc = "Audio provided to or from an LLM."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AudioContent {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "The base64-encoded audio data."]
    pub data: ::std::string::String,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of the audio. Different providers may support different audio types."]
    #[serde(rename = "mimeType")]
    pub mime_type: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl AudioContent {
    pub fn builder() -> builder::AudioContent {
        Default::default()
    }
}
#[doc = "Base interface for metadata with name (identifier) and title (display name) properties."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseMetadata {
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl BaseMetadata {
    pub fn builder() -> builder::BaseMetadata {
        Default::default()
    }
}
#[doc = "`BlobResourceContents`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BlobResourceContents {
    #[doc = "A base64-encoded string representing the binary data of the item."]
    pub blob: ::std::string::String,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl BlobResourceContents {
    pub fn builder() -> builder::BlobResourceContents {
        Default::default()
    }
}
#[doc = "`BooleanSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BooleanSchema {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl BooleanSchema {
    pub fn builder() -> builder::BooleanSchema {
        Default::default()
    }
}
#[doc = "A result that supports a time-to-live (TTL) hint for client-side caching."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CacheableResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: CacheableResultCacheScope,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl CacheableResult {
    pub fn builder() -> builder::CacheableResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CacheableResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for CacheableResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for CacheableResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CacheableResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CacheableResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Used by the client to invoke a tool provided by the server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: CallToolRequestParams,
}
impl CallToolRequest {
    pub fn builder() -> builder::CallToolRequest {
        Default::default()
    }
}
#[doc = "Parameters for a `tools/call` request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequestParams {
    #[doc = "Arguments to use for the tool call."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub arguments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "inputResponses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub input_responses: ::std::option::Option<InputResponses>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[doc = "The name of the tool."]
    pub name: ::std::string::String,
    #[serde(
        rename = "requestState",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_state: ::std::option::Option<::std::string::String>,
}
impl CallToolRequestParams {
    pub fn builder() -> builder::CallToolRequestParams {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link CallToolRequesttools/call} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolResult {
    #[doc = "A list of content objects that represent the unstructured result of the tool call."]
    pub content: ::std::vec::Vec<ContentBlock>,
    #[doc = "Whether the tool call ended in an error.\n\nIf not set, this is assumed to be false (the call was successful).\n\nAny errors that originate from the tool SHOULD be reported inside the result\nobject, with `isError` set to true, _not_ as an MCP protocol-level error\nresponse. Otherwise, the LLM would not be able to see that an error occurred\nand self-correct.\n\nHowever, any errors in _finding_ the tool, an error indicating that the\nserver does not support tool calls, or any other exceptional conditions,\nshould be reported as an MCP error response."]
    #[serde(
        rename = "isError",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_error: ::std::option::Option<bool>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "An optional JSON value that represents the structured result of the tool call.\n\nThis can be any JSON value (object, array, string, number, boolean, or null)\nthat conforms to the tool's outputSchema if one is defined."]
    #[serde(
        rename = "structuredContent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub structured_content: ::std::option::Option<::serde_json::Value>,
}
impl CallToolResult {
    pub fn builder() -> builder::CallToolResult {
        Default::default()
    }
}
#[doc = "A successful response from the server for a {@link CallToolRequesttools/call} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: CallToolResultResponseResult,
}
impl CallToolResultResponse {
    pub fn builder() -> builder::CallToolResultResponse {
        Default::default()
    }
}
#[doc = "`CallToolResultResponseResult`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CallToolResultResponseResult {
    InputRequiredResult(InputRequiredResult),
    CallToolResult(CallToolResult),
}
impl ::std::convert::From<InputRequiredResult> for CallToolResultResponseResult {
    fn from(value: InputRequiredResult) -> Self {
        Self::InputRequiredResult(value)
    }
}
impl ::std::convert::From<CallToolResult> for CallToolResultResponseResult {
    fn from(value: CallToolResult) -> Self {
        Self::CallToolResult(value)
    }
}
#[doc = "This notification is sent by the client to indicate that it is cancelling a request it previously issued.\n\nOn stdio, the server also sends this notification, solely to terminate a {@link SubscriptionsListenRequestsubscriptions/listen} stream: it references the ID of the `subscriptions/listen` request that opened the stream. Servers MUST NOT use this notification to cancel any other request.\n\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\n\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: CancelledNotificationParams,
}
impl CancelledNotification {
    pub fn builder() -> builder::CancelledNotification {
        Default::default()
    }
}
#[doc = "Parameters for a `notifications/cancelled` notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotificationParams {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
    #[doc = "An optional string describing the reason for the cancellation. This MAY be logged or presented to the user."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the request to cancel.\n\nThis MUST correspond to the ID of a request the client previously issued."]
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
impl CancelledNotificationParams {
    pub fn builder() -> builder::CancelledNotificationParams {
        Default::default()
    }
}
#[doc = "Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub elicitation: ::std::option::Option<ClientCapabilitiesElicitation>,
    #[doc = "Experimental, non-standard capabilities that the client supports."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub experimental: ::std::collections::HashMap<::std::string::String, JsonObject>,
    #[doc = "Optional MCP extensions that the client supports. Keys are extension identifiers\n(e.g., \"io.modelcontextprotocol/oauth-client-credentials\"), and values are\nper-extension settings objects. An empty object indicates support with no settings.\n\nKeys MUST follow the {@link MetaObject`_meta` key naming rules}, with a\nmandatory prefix."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extensions: ::std::collections::HashMap<::std::string::String, JsonObject>,
    #[doc = "Present if the client supports listing roots."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub roots: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sampling: ::std::option::Option<ClientCapabilitiesSampling>,
}
impl ClientCapabilities {
    pub fn builder() -> builder::ClientCapabilities {
        Default::default()
    }
}
#[doc = "Present if the client supports elicitation from the server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClientCapabilitiesElicitation {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub form: ::std::option::Option<JsonObject>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<JsonObject>,
}
impl ClientCapabilitiesElicitation {
    pub fn builder() -> builder::ClientCapabilitiesElicitation {
        Default::default()
    }
}
#[doc = "Present if the client supports sampling from an LLM."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ClientCapabilitiesSampling {
    #[doc = "Whether the client supports context inclusion via `includeContext` parameter.\nIf not declared, servers SHOULD only use `includeContext: \"none\"` (or omit it)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<JsonObject>,
    #[doc = "Whether the client supports tool use via `tools` and `toolChoice` parameters."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<JsonObject>,
}
impl ClientCapabilitiesSampling {
    pub fn builder() -> builder::ClientCapabilitiesSampling {
        Default::default()
    }
}
#[doc = "This notification is sent by the client to indicate that it is cancelling a request it previously issued.\n\nOn stdio, the server also sends this notification, solely to terminate a {@link SubscriptionsListenRequestsubscriptions/listen} stream: it references the ID of the `subscriptions/listen` request that opened the stream. Servers MUST NOT use this notification to cancel any other request.\n\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\n\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: CancelledNotificationParams,
}
impl ClientNotification {
    pub fn builder() -> builder::ClientNotification {
        Default::default()
    }
}
#[doc = "`ClientRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientRequest {
    DiscoverRequest(DiscoverRequest),
    ListResourcesRequest(ListResourcesRequest),
    ListResourceTemplatesRequest(ListResourceTemplatesRequest),
    ReadResourceRequest(ReadResourceRequest),
    SubscriptionsListenRequest(SubscriptionsListenRequest),
    ListPromptsRequest(ListPromptsRequest),
    GetPromptRequest(GetPromptRequest),
    ListToolsRequest(ListToolsRequest),
    CallToolRequest(CallToolRequest),
    CompleteRequest(CompleteRequest),
}
impl ::std::convert::From<DiscoverRequest> for ClientRequest {
    fn from(value: DiscoverRequest) -> Self {
        Self::DiscoverRequest(value)
    }
}
impl ::std::convert::From<ListResourcesRequest> for ClientRequest {
    fn from(value: ListResourcesRequest) -> Self {
        Self::ListResourcesRequest(value)
    }
}
impl ::std::convert::From<ListResourceTemplatesRequest> for ClientRequest {
    fn from(value: ListResourceTemplatesRequest) -> Self {
        Self::ListResourceTemplatesRequest(value)
    }
}
impl ::std::convert::From<ReadResourceRequest> for ClientRequest {
    fn from(value: ReadResourceRequest) -> Self {
        Self::ReadResourceRequest(value)
    }
}
impl ::std::convert::From<SubscriptionsListenRequest> for ClientRequest {
    fn from(value: SubscriptionsListenRequest) -> Self {
        Self::SubscriptionsListenRequest(value)
    }
}
impl ::std::convert::From<ListPromptsRequest> for ClientRequest {
    fn from(value: ListPromptsRequest) -> Self {
        Self::ListPromptsRequest(value)
    }
}
impl ::std::convert::From<GetPromptRequest> for ClientRequest {
    fn from(value: GetPromptRequest) -> Self {
        Self::GetPromptRequest(value)
    }
}
impl ::std::convert::From<ListToolsRequest> for ClientRequest {
    fn from(value: ListToolsRequest) -> Self {
        Self::ListToolsRequest(value)
    }
}
impl ::std::convert::From<CallToolRequest> for ClientRequest {
    fn from(value: CallToolRequest) -> Self {
        Self::CallToolRequest(value)
    }
}
impl ::std::convert::From<CompleteRequest> for ClientRequest {
    fn from(value: CompleteRequest) -> Self {
        Self::CompleteRequest(value)
    }
}
#[doc = "Common result fields."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ClientResult(pub Result);
impl ::std::ops::Deref for ClientResult {
    type Target = Result;
    fn deref(&self) -> &Result {
        &self.0
    }
}
impl ::std::convert::From<ClientResult> for Result {
    fn from(value: ClientResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<Result> for ClientResult {
    fn from(value: Result) -> Self {
        Self(value)
    }
}
#[doc = "A request from the client to the server, to ask for completion options."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: CompleteRequestParams,
}
impl CompleteRequest {
    pub fn builder() -> builder::CompleteRequest {
        Default::default()
    }
}
#[doc = "Parameters for a `completion/complete` request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParams {
    pub argument: CompleteRequestParamsArgument,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<CompleteRequestParamsContext>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[serde(rename = "ref")]
    pub ref_: CompleteRequestParamsRef,
}
impl CompleteRequestParams {
    pub fn builder() -> builder::CompleteRequestParams {
        Default::default()
    }
}
#[doc = "The argument's information"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParamsArgument {
    #[doc = "The name of the argument"]
    pub name: ::std::string::String,
    #[doc = "The value of the argument to use for completion matching."]
    pub value: ::std::string::String,
}
impl CompleteRequestParamsArgument {
    pub fn builder() -> builder::CompleteRequestParamsArgument {
        Default::default()
    }
}
#[doc = "Additional, optional context for completions"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct CompleteRequestParamsContext {
    #[doc = "Previously-resolved variables in a URI template or prompt."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub arguments: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl CompleteRequestParamsContext {
    pub fn builder() -> builder::CompleteRequestParamsContext {
        Default::default()
    }
}
#[doc = "`CompleteRequestParamsRef`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CompleteRequestParamsRef {
    PromptReference(PromptReference),
    ResourceTemplateReference(ResourceTemplateReference),
}
impl ::std::convert::From<PromptReference> for CompleteRequestParamsRef {
    fn from(value: PromptReference) -> Self {
        Self::PromptReference(value)
    }
}
impl ::std::convert::From<ResourceTemplateReference> for CompleteRequestParamsRef {
    fn from(value: ResourceTemplateReference) -> Self {
        Self::ResourceTemplateReference(value)
    }
}
#[doc = "The result returned by the server for a {@link CompleteRequestcompletion/complete} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResult {
    pub completion: CompleteResultCompletion,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
}
impl CompleteResult {
    pub fn builder() -> builder::CompleteResult {
        Default::default()
    }
}
#[doc = "`CompleteResultCompletion`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResultCompletion {
    #[doc = "Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown."]
    #[serde(
        rename = "hasMore",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_more: ::std::option::Option<bool>,
    #[doc = "The total number of completion options available. This can exceed the number of values actually sent in the response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<i64>,
    #[doc = "An array of completion values. Must not exceed 100 items."]
    pub values: ::std::vec::Vec<::std::string::String>,
}
impl CompleteResultCompletion {
    pub fn builder() -> builder::CompleteResultCompletion {
        Default::default()
    }
}
#[doc = "A successful response from the server for a {@link CompleteRequestcompletion/complete} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: CompleteResult,
}
impl CompleteResultResponse {
    pub fn builder() -> builder::CompleteResultResponse {
        Default::default()
    }
}
#[doc = "`ContentBlock`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ContentBlock {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ResourceLink(ResourceLink),
    EmbeddedResource(EmbeddedResource),
}
impl ::std::convert::From<TextContent> for ContentBlock {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for ContentBlock {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for ContentBlock {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
impl ::std::convert::From<ResourceLink> for ContentBlock {
    fn from(value: ResourceLink) -> Self {
        Self::ResourceLink(value)
    }
}
impl ::std::convert::From<EmbeddedResource> for ContentBlock {
    fn from(value: EmbeddedResource) -> Self {
        Self::EmbeddedResource(value)
    }
}
#[doc = "A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequest {
    pub method: ::std::string::String,
    pub params: CreateMessageRequestParams,
}
impl CreateMessageRequest {
    pub fn builder() -> builder::CreateMessageRequest {
        Default::default()
    }
}
#[doc = "Parameters for a `sampling/createMessage` request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequestParams {
    #[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt.\nThe client MAY ignore this request.\n\nDefault is `\"none\"`. The values `\"thisServer\"` and `\"allServers\"` are deprecated (SEP-2596): servers SHOULD\nomit this field or use `\"none\"`, and SHOULD only use the deprecated values if the client declares\n{@link ClientCapabilities.sampling.context}."]
    #[serde(
        rename = "includeContext",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_context: ::std::option::Option<CreateMessageRequestParamsIncludeContext>,
    #[doc = "The requested maximum number of tokens to sample (to prevent runaway completions).\n\nThe client MAY choose to sample fewer tokens than the requested maximum."]
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,
    pub messages: ::std::vec::Vec<SamplingMessage>,
    #[doc = "Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<JsonObject>,
    #[doc = "The server's preferences for which model to select. The client MAY ignore these preferences."]
    #[serde(
        rename = "modelPreferences",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_preferences: ::std::option::Option<ModelPreferences>,
    #[serde(
        rename = "stopSequences",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stop_sequences: ::std::vec::Vec<::std::string::String>,
    #[doc = "An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt."]
    #[serde(
        rename = "systemPrompt",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_prompt: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "Controls how the model uses tools.\nThe client MUST return an error if this field is provided but {@link ClientCapabilities.sampling.tools} is not declared.\nDefault is `{ mode: \"auto\" }`."]
    #[serde(
        rename = "toolChoice",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tool_choice: ::std::option::Option<ToolChoice>,
    #[doc = "Tools that the model may use during generation.\nThe client MUST return an error if this field is provided but {@link ClientCapabilities.sampling.tools} is not declared."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<Tool>,
}
impl CreateMessageRequestParams {
    pub fn builder() -> builder::CreateMessageRequestParams {
        Default::default()
    }
}
#[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt.\nThe client MAY ignore this request.\n\nDefault is `\"none\"`. The values `\"thisServer\"` and `\"allServers\"` are deprecated (SEP-2596): servers SHOULD\nomit this field or use `\"none\"`, and SHOULD only use the deprecated values if the client declares\n{@link ClientCapabilities.sampling.context}."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateMessageRequestParamsIncludeContext {
    #[serde(rename = "allServers")]
    AllServers,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thisServer")]
    ThisServer,
}
impl ::std::fmt::Display for CreateMessageRequestParamsIncludeContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllServers => f.write_str("allServers"),
            Self::None => f.write_str("none"),
            Self::ThisServer => f.write_str("thisServer"),
        }
    }
}
impl ::std::str::FromStr for CreateMessageRequestParamsIncludeContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allServers" => Ok(Self::AllServers),
            "none" => Ok(Self::None),
            "thisServer" => Ok(Self::ThisServer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The result returned by the client for a {@link CreateMessageRequestsampling/createMessage} request.\nThe client should inform the user before returning the sampled message, to allow them\nto inspect the response (human in the loop) and decide whether to allow the server to see it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageResult {
    pub content: CreateMessageResultContent,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The name of the model that generated the message."]
    pub model: ::std::string::String,
    pub role: Role,
    #[doc = "The reason why sampling stopped, if known.\n\nStandard values:\n- `\"endTurn\"`: Natural end of the assistant's turn\n- `\"stopSequence\"`: A stop sequence was encountered\n- `\"maxTokens\"`: Maximum token limit was reached\n- `\"toolUse\"`: The model wants to use one or more tools\n\nThis field is an open string to allow for provider-specific stop reasons."]
    #[serde(
        rename = "stopReason",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_reason: ::std::option::Option<::std::string::String>,
}
impl CreateMessageResult {
    pub fn builder() -> builder::CreateMessageResult {
        Default::default()
    }
}
#[doc = "`CreateMessageResultContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CreateMessageResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ToolUseContent(ToolUseContent),
    ToolResultContent(ToolResultContent),
    Array(::std::vec::Vec<SamplingMessageContentBlock>),
}
impl ::std::convert::From<TextContent> for CreateMessageResultContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for CreateMessageResultContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for CreateMessageResultContent {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
impl ::std::convert::From<ToolUseContent> for CreateMessageResultContent {
    fn from(value: ToolUseContent) -> Self {
        Self::ToolUseContent(value)
    }
}
impl ::std::convert::From<ToolResultContent> for CreateMessageResultContent {
    fn from(value: ToolResultContent) -> Self {
        Self::ToolResultContent(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<SamplingMessageContentBlock>>
    for CreateMessageResultContent
{
    fn from(value: ::std::vec::Vec<SamplingMessageContentBlock>) -> Self {
        Self::Array(value)
    }
}
#[doc = "An opaque token used to represent a cursor for pagination."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Cursor(pub ::std::string::String);
impl ::std::ops::Deref for Cursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Cursor> for ::std::string::String {
    fn from(value: Cursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Cursor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for Cursor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "A request from the client asking the server to advertise its supported\nprotocol versions, capabilities, and other metadata. Servers **MUST**\nimplement `server/discover`. Clients **MAY** call it but are not required\nto — version negotiation can also happen inline via per-request `_meta`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscoverRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: RequestParams,
}
impl DiscoverRequest {
    pub fn builder() -> builder::DiscoverRequest {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link DiscoverRequestserver/discover} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscoverResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: DiscoverResultCacheScope,
    #[doc = "The capabilities of the server."]
    pub capabilities: ServerCapabilities,
    #[doc = "Natural-language guidance describing the server and its features.\n\nThis can be used by clients to improve an LLM's understanding of\navailable tools (e.g., by including it in a system prompt). It should\nfocus on information that helps the model use the server effectively\nand should not duplicate information already in tool descriptions."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "MCP Protocol Versions this server supports. The client should choose a\nversion from this list for use in subsequent requests."]
    #[serde(rename = "supportedVersions")]
    pub supported_versions: ::std::vec::Vec<::std::string::String>,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl DiscoverResult {
    pub fn builder() -> builder::DiscoverResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DiscoverResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for DiscoverResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for DiscoverResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DiscoverResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DiscoverResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A successful response from the server for a {@link DiscoverRequestserver/discover} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscoverResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: DiscoverResult,
}
impl DiscoverResultResponse {
    pub fn builder() -> builder::DiscoverResultResponse {
        Default::default()
    }
}
#[doc = "A request from the server to elicit additional information from the user via the client."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequest {
    pub method: ::std::string::String,
    pub params: ElicitRequestParams,
}
impl ElicitRequest {
    pub fn builder() -> builder::ElicitRequest {
        Default::default()
    }
}
#[doc = "The parameters for a request to elicit non-sensitive information from the user via a form in the client."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequestFormParams {
    #[doc = "The message to present to the user describing what information is being requested."]
    pub message: ::std::string::String,
    #[doc = "The elicitation mode."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedSchema")]
    pub requested_schema: ElicitRequestFormParamsRequestedSchema,
}
impl ElicitRequestFormParams {
    pub fn builder() -> builder::ElicitRequestFormParams {
        Default::default()
    }
}
#[doc = "A restricted subset of JSON Schema.\nOnly top-level properties are allowed, without nesting."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequestFormParamsRequestedSchema {
    pub properties: ::std::collections::HashMap<::std::string::String, PrimitiveSchemaDefinition>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub required: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "$schema",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub schema: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ElicitRequestFormParamsRequestedSchema {
    pub fn builder() -> builder::ElicitRequestFormParamsRequestedSchema {
        Default::default()
    }
}
#[doc = "The parameters for a request to elicit additional information from the user via the client."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ElicitRequestParams {
    FormParams(ElicitRequestFormParams),
    UrlParams(ElicitRequestUrlParams),
}
impl ::std::convert::From<ElicitRequestFormParams> for ElicitRequestParams {
    fn from(value: ElicitRequestFormParams) -> Self {
        Self::FormParams(value)
    }
}
impl ::std::convert::From<ElicitRequestUrlParams> for ElicitRequestParams {
    fn from(value: ElicitRequestUrlParams) -> Self {
        Self::UrlParams(value)
    }
}
#[doc = "The parameters for a request to elicit information from the user via a URL in the client."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitRequestUrlParams {
    #[doc = "The message to present to the user explaining why the interaction is needed."]
    pub message: ::std::string::String,
    #[doc = "The elicitation mode."]
    pub mode: ::std::string::String,
    #[doc = "The URL that the user should navigate to."]
    pub url: ::std::string::String,
}
impl ElicitRequestUrlParams {
    pub fn builder() -> builder::ElicitRequestUrlParams {
        Default::default()
    }
}
#[doc = "The result returned by the client for an {@link ElicitRequestelicitation/create} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ElicitResult {
    #[doc = "The user action in response to the elicitation.\n- `\"accept\"`: User submitted the form/confirmed the action\n- `\"decline\"`: User explicitly declined the action\n- `\"cancel\"`: User dismissed without making an explicit choice"]
    pub action: ElicitResultAction,
    #[doc = "The submitted form data, only present when action is `\"accept\"` and mode was `\"form\"`.\nContains values matching the requested schema.\nOmitted for out-of-band mode responses."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub content: ::std::collections::HashMap<::std::string::String, ElicitResultContentValue>,
}
impl ElicitResult {
    pub fn builder() -> builder::ElicitResult {
        Default::default()
    }
}
#[doc = "The user action in response to the elicitation.\n- `\"accept\"`: User submitted the form/confirmed the action\n- `\"decline\"`: User explicitly declined the action\n- `\"cancel\"`: User dismissed without making an explicit choice"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ElicitResultAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "decline")]
    Decline,
}
impl ::std::fmt::Display for ElicitResultAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accept => f.write_str("accept"),
            Self::Cancel => f.write_str("cancel"),
            Self::Decline => f.write_str("decline"),
        }
    }
}
impl ::std::str::FromStr for ElicitResultAction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accept" => Ok(Self::Accept),
            "cancel" => Ok(Self::Cancel),
            "decline" => Ok(Self::Decline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ElicitResultAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElicitResultAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ElicitResultContentValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ElicitResultContentValue {
    Variant0(::std::vec::Vec<::std::string::String>),
    Variant1(ElicitResultContentValueVariant1),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for ElicitResultContentValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<ElicitResultContentValueVariant1> for ElicitResultContentValue {
    fn from(value: ElicitResultContentValueVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`ElicitResultContentValueVariant1`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ElicitResultContentValueVariant1 {
    Boolean(bool),
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for ElicitResultContentValueVariant1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ElicitResultContentValueVariant1 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<i64> for ElicitResultContentValueVariant1 {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "The contents of a resource, embedded into a prompt or tool call result.\n\nIt is up to the client how best to render embedded resources for the benefit\nof the LLM and/or the user."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmbeddedResource {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    pub resource: EmbeddedResourceResource,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl EmbeddedResource {
    pub fn builder() -> builder::EmbeddedResource {
        Default::default()
    }
}
#[doc = "`EmbeddedResourceResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl ::std::convert::From<TextResourceContents> for EmbeddedResourceResource {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl ::std::convert::From<BlobResourceContents> for EmbeddedResourceResource {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "Common result fields."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EmptyResult(pub Result);
impl ::std::ops::Deref for EmptyResult {
    type Target = Result;
    fn deref(&self) -> &Result {
        &self.0
    }
}
impl ::std::convert::From<EmptyResult> for Result {
    fn from(value: EmptyResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<Result> for EmptyResult {
    fn from(value: Result) -> Self {
        Self(value)
    }
}
#[doc = "`EnumSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct EnumSchema {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<UntitledSingleSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<TitledSingleSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<UntitledMultiSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<TitledMultiSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_4: ::std::option::Option<LegacyTitledEnumSchema>,
}
impl EnumSchema {
    pub fn builder() -> builder::EnumSchema {
        Default::default()
    }
}
#[doc = "`Error`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Error {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl Error {
    pub fn builder() -> builder::Error {
        Default::default()
    }
}
#[doc = "Used by the client to get a prompt provided by the server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: GetPromptRequestParams,
}
impl GetPromptRequest {
    pub fn builder() -> builder::GetPromptRequest {
        Default::default()
    }
}
#[doc = "Parameters for a `prompts/get` request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequestParams {
    #[doc = "Arguments to use for templating the prompt."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub arguments: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(
        rename = "inputResponses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub input_responses: ::std::option::Option<InputResponses>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[doc = "The name of the prompt or prompt template."]
    pub name: ::std::string::String,
    #[serde(
        rename = "requestState",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_state: ::std::option::Option<::std::string::String>,
}
impl GetPromptRequestParams {
    pub fn builder() -> builder::GetPromptRequestParams {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link GetPromptRequestprompts/get} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptResult {
    #[doc = "An optional description for the prompt."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub messages: ::std::vec::Vec<PromptMessage>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
}
impl GetPromptResult {
    pub fn builder() -> builder::GetPromptResult {
        Default::default()
    }
}
#[doc = "A successful response from the server for a {@link GetPromptRequestprompts/get} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: GetPromptResultResponseResult,
}
impl GetPromptResultResponse {
    pub fn builder() -> builder::GetPromptResultResponse {
        Default::default()
    }
}
#[doc = "`GetPromptResultResponseResult`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetPromptResultResponseResult {
    InputRequiredResult(InputRequiredResult),
    GetPromptResult(GetPromptResult),
}
impl ::std::convert::From<InputRequiredResult> for GetPromptResultResponseResult {
    fn from(value: InputRequiredResult) -> Self {
        Self::InputRequiredResult(value)
    }
}
impl ::std::convert::From<GetPromptResult> for GetPromptResultResponseResult {
    fn from(value: GetPromptResult) -> Self {
        Self::GetPromptResult(value)
    }
}
#[doc = "Returned when a server rejects a request because the values in the HTTP\nheaders do not match the corresponding values in the request body, or\nbecause required headers are missing or malformed. For HTTP, the response\nstatus code MUST be `400 Bad Request`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HeaderMismatchError {
    pub error: HeaderMismatchErrorError,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RequestId>,
    pub jsonrpc: ::std::string::String,
}
impl HeaderMismatchError {
    pub fn builder() -> builder::HeaderMismatchError {
        Default::default()
    }
}
#[doc = "`HeaderMismatchErrorError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HeaderMismatchErrorError {
    pub code: HeaderMismatchErrorErrorCode,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl HeaderMismatchErrorError {
    pub fn builder() -> builder::HeaderMismatchErrorError {
        Default::default()
    }
}
#[doc = "`HeaderMismatchErrorErrorCode`"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct HeaderMismatchErrorErrorCode(i64);
impl ::std::ops::Deref for HeaderMismatchErrorErrorCode {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<HeaderMismatchErrorErrorCode> for i64 {
    fn from(value: HeaderMismatchErrorErrorCode) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for HeaderMismatchErrorErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![-32020_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for HeaderMismatchErrorErrorCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "An optionally-sized icon that can be displayed in a user interface."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Icon {
    #[doc = "Optional MIME type override if the source MIME type is missing or generic.\nFor example: `\"image/png\"`, `\"image/jpeg\"`, or `\"image/svg+xml\"`."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Optional array of strings that specify sizes at which the icon can be used.\nEach string should be in WxH format (e.g., `\"48x48\"`, `\"96x96\"`) or `\"any\"` for scalable formats like SVG.\n\nIf not provided, the client should assume that the icon can be used at any size."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sizes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A standard URI pointing to an icon resource. May be an HTTP/HTTPS URL or a\n`data:` URI with Base64-encoded image data.\n\nConsumers SHOULD take steps to ensure URLs serving icons are from the\nsame domain as the client/server or a trusted domain.\n\nConsumers SHOULD take appropriate precautions when consuming SVGs as they can contain\nexecutable JavaScript."]
    pub src: ::std::string::String,
    #[doc = "Optional specifier for the theme this icon is designed for. `\"light\"` indicates\nthe icon is designed to be used with a light background, and `\"dark\"` indicates\nthe icon is designed to be used with a dark background.\n\nIf not provided, the client should assume the icon can be used with any theme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub theme: ::std::option::Option<IconTheme>,
}
impl Icon {
    pub fn builder() -> builder::Icon {
        Default::default()
    }
}
#[doc = "Optional specifier for the theme this icon is designed for. `\"light\"` indicates\nthe icon is designed to be used with a light background, and `\"dark\"` indicates\nthe icon is designed to be used with a dark background.\n\nIf not provided, the client should assume the icon can be used with any theme."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum IconTheme {
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}
impl ::std::fmt::Display for IconTheme {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dark => f.write_str("dark"),
            Self::Light => f.write_str("light"),
        }
    }
}
impl ::std::str::FromStr for IconTheme {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IconTheme {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IconTheme {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Base interface to add `icons` property."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct Icons {
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
}
impl Icons {
    pub fn builder() -> builder::Icons {
        Default::default()
    }
}
#[doc = "An image provided to or from an LLM."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImageContent {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "The base64-encoded image data."]
    pub data: ::std::string::String,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of the image. Different providers may support different image types."]
    #[serde(rename = "mimeType")]
    pub mime_type: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ImageContent {
    pub fn builder() -> builder::ImageContent {
        Default::default()
    }
}
#[doc = "Describes the MCP implementation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Implementation {
    #[doc = "An optional human-readable description of what this implementation does.\n\nThis can be used by clients or servers to provide context about their purpose\nand capabilities. For example, a server might describe the types of resources\nor tools it provides, while a client might describe its intended use case."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "The version of this implementation."]
    pub version: ::std::string::String,
    #[doc = "An optional URL of the website for this implementation."]
    #[serde(
        rename = "websiteUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub website_url: ::std::option::Option<::std::string::String>,
}
impl Implementation {
    pub fn builder() -> builder::Implementation {
        Default::default()
    }
}
#[doc = "`InputRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InputRequest {
    CreateMessageRequest(CreateMessageRequest),
    ListRootsRequest(ListRootsRequest),
    ElicitRequest(ElicitRequest),
}
impl ::std::convert::From<CreateMessageRequest> for InputRequest {
    fn from(value: CreateMessageRequest) -> Self {
        Self::CreateMessageRequest(value)
    }
}
impl ::std::convert::From<ListRootsRequest> for InputRequest {
    fn from(value: ListRootsRequest) -> Self {
        Self::ListRootsRequest(value)
    }
}
impl ::std::convert::From<ElicitRequest> for InputRequest {
    fn from(value: ElicitRequest) -> Self {
        Self::ElicitRequest(value)
    }
}
#[doc = "A map of server-initiated requests that the client must fulfill.\nKeys are server-assigned identifiers; values are the request objects."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct InputRequests(pub ::std::collections::HashMap<::std::string::String, InputRequest>);
impl ::std::ops::Deref for InputRequests {
    type Target = ::std::collections::HashMap<::std::string::String, InputRequest>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, InputRequest> {
        &self.0
    }
}
impl ::std::convert::From<InputRequests>
    for ::std::collections::HashMap<::std::string::String, InputRequest>
{
    fn from(value: InputRequests) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, InputRequest>>
    for InputRequests
{
    fn from(value: ::std::collections::HashMap<::std::string::String, InputRequest>) -> Self {
        Self(value)
    }
}
#[doc = "An InputRequiredResult sent by the server to indicate that additional input is needed\nbefore the request can be completed.\n\nAt least one of `inputRequests` or `requestState` MUST be present."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InputRequiredResult {
    #[serde(
        rename = "inputRequests",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub input_requests: ::std::option::Option<InputRequests>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[serde(
        rename = "requestState",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_state: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
}
impl InputRequiredResult {
    pub fn builder() -> builder::InputRequiredResult {
        Default::default()
    }
}
#[doc = "`InputResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InputResponse {
    CreateMessageResult(CreateMessageResult),
    ListRootsResult(ListRootsResult),
    ElicitResult(ElicitResult),
}
impl ::std::convert::From<CreateMessageResult> for InputResponse {
    fn from(value: CreateMessageResult) -> Self {
        Self::CreateMessageResult(value)
    }
}
impl ::std::convert::From<ListRootsResult> for InputResponse {
    fn from(value: ListRootsResult) -> Self {
        Self::ListRootsResult(value)
    }
}
impl ::std::convert::From<ElicitResult> for InputResponse {
    fn from(value: ElicitResult) -> Self {
        Self::ElicitResult(value)
    }
}
#[doc = "`InputResponseRequestParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InputResponseRequestParams {
    #[serde(
        rename = "inputResponses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub input_responses: ::std::option::Option<InputResponses>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[serde(
        rename = "requestState",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_state: ::std::option::Option<::std::string::String>,
}
impl InputResponseRequestParams {
    pub fn builder() -> builder::InputResponseRequestParams {
        Default::default()
    }
}
#[doc = "A map of client responses to server-initiated requests.\nKeys correspond to the keys in the {@link InputRequests} map;\nvalues are the client's result for each request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct InputResponses(pub ::std::collections::HashMap<::std::string::String, InputResponse>);
impl ::std::ops::Deref for InputResponses {
    type Target = ::std::collections::HashMap<::std::string::String, InputResponse>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, InputResponse> {
        &self.0
    }
}
impl ::std::convert::From<InputResponses>
    for ::std::collections::HashMap<::std::string::String, InputResponse>
{
    fn from(value: InputResponses) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, InputResponse>>
    for InputResponses
{
    fn from(value: ::std::collections::HashMap<::std::string::String, InputResponse>) -> Self {
        Self(value)
    }
}
#[doc = "A JSON-RPC error indicating that an internal error occurred on the receiver. This error is returned when the receiver encounters an unexpected condition that prevents it from fulfilling the request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InternalError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl InternalError {
    pub fn builder() -> builder::InternalError {
        Default::default()
    }
}
#[doc = "A JSON-RPC error indicating that the method parameters are invalid or malformed.\n\nIn MCP, this error is returned in various contexts when request parameters fail validation:\n\n- **Tools**: Unknown tool name or invalid tool arguments\n- **Prompts**: Unknown prompt name or missing required arguments\n- **Pagination**: Invalid or expired cursor values\n- **Logging**: Invalid log level\n- **Elicitation**: Server requests an elicitation mode not declared in client capabilities\n- **Sampling**: Missing tool result or tool results mixed with other content"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InvalidParamsError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl InvalidParamsError {
    pub fn builder() -> builder::InvalidParamsError {
        Default::default()
    }
}
#[doc = "A JSON-RPC error indicating that the request is not a valid request object. This error is returned when the message structure does not conform to the JSON-RPC 2.0 specification requirements for a request (e.g., missing required fields like `jsonrpc` or `method`, or using invalid types for these fields)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InvalidRequestError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl InvalidRequestError {
    pub fn builder() -> builder::InvalidRequestError {
        Default::default()
    }
}
#[doc = "`JsonArray`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct JsonArray(pub ::std::vec::Vec<JsonValue>);
impl ::std::ops::Deref for JsonArray {
    type Target = ::std::vec::Vec<JsonValue>;
    fn deref(&self) -> &::std::vec::Vec<JsonValue> {
        &self.0
    }
}
impl ::std::convert::From<JsonArray> for ::std::vec::Vec<JsonValue> {
    fn from(value: JsonArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<JsonValue>> for JsonArray {
    fn from(value: ::std::vec::Vec<JsonValue>) -> Self {
        Self(value)
    }
}
#[doc = "`JsonObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct JsonObject(pub ::std::collections::HashMap<::std::string::String, JsonValue>);
impl ::std::ops::Deref for JsonObject {
    type Target = ::std::collections::HashMap<::std::string::String, JsonValue>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, JsonValue> {
        &self.0
    }
}
impl ::std::convert::From<JsonObject>
    for ::std::collections::HashMap<::std::string::String, JsonValue>
{
    fn from(value: JsonObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, JsonValue>>
    for JsonObject
{
    fn from(value: ::std::collections::HashMap<::std::string::String, JsonValue>) -> Self {
        Self(value)
    }
}
#[doc = "`JsonValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonValue {
    Variant0(JsonObject),
    Variant1(::std::vec::Vec<JsonValue>),
    Variant2(JsonValueVariant2),
}
impl ::std::convert::From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<JsonValue>> for JsonValue {
    fn from(value: ::std::vec::Vec<JsonValue>) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<JsonValueVariant2> for JsonValue {
    fn from(value: JsonValueVariant2) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "`JsonValueVariant2`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonValueVariant2 {
    Boolean(bool),
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for JsonValueVariant2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for JsonValueVariant2 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<i64> for JsonValueVariant2 {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "A response to a request that indicates an error occurred."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcErrorResponse {
    pub error: Error,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RequestId>,
    pub jsonrpc: ::std::string::String,
}
impl JsonrpcErrorResponse {
    pub fn builder() -> builder::JsonrpcErrorResponse {
        Default::default()
    }
}
#[doc = "Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcMessage {
    Request(JsonrpcRequest),
    Notification(JsonrpcNotification),
    ResultResponse(JsonrpcResultResponse),
    ErrorResponse(JsonrpcErrorResponse),
}
impl ::std::convert::From<JsonrpcRequest> for JsonrpcMessage {
    fn from(value: JsonrpcRequest) -> Self {
        Self::Request(value)
    }
}
impl ::std::convert::From<JsonrpcNotification> for JsonrpcMessage {
    fn from(value: JsonrpcNotification) -> Self {
        Self::Notification(value)
    }
}
impl ::std::convert::From<JsonrpcResultResponse> for JsonrpcMessage {
    fn from(value: JsonrpcResultResponse) -> Self {
        Self::ResultResponse(value)
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for JsonrpcMessage {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::ErrorResponse(value)
    }
}
#[doc = "A notification which does not expect a response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl JsonrpcNotification {
    pub fn builder() -> builder::JsonrpcNotification {
        Default::default()
    }
}
#[doc = "A request that expects a response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl JsonrpcRequest {
    pub fn builder() -> builder::JsonrpcRequest {
        Default::default()
    }
}
#[doc = "A response to a request, containing either the result or error."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcResponse {
    ResultResponse(JsonrpcResultResponse),
    ErrorResponse(JsonrpcErrorResponse),
}
impl ::std::convert::From<JsonrpcResultResponse> for JsonrpcResponse {
    fn from(value: JsonrpcResultResponse) -> Self {
        Self::ResultResponse(value)
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for JsonrpcResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::ErrorResponse(value)
    }
}
#[doc = "A successful (non-error) response to a request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: Result,
}
impl JsonrpcResultResponse {
    pub fn builder() -> builder::JsonrpcResultResponse {
        Default::default()
    }
}
#[doc = "Use {@link TitledSingleSelectEnumSchema} instead.\nThis interface will be removed in a future version."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LegacyTitledEnumSchema {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enum")]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[doc = "(Legacy) Display names for enum values.\nNon-standard according to JSON schema 2020-12."]
    #[serde(
        rename = "enumNames",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_names: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl LegacyTitledEnumSchema {
    pub fn builder() -> builder::LegacyTitledEnumSchema {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of prompts and prompt templates the server has."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: PaginatedRequestParams,
}
impl ListPromptsRequest {
    pub fn builder() -> builder::ListPromptsRequest {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link ListPromptsRequestprompts/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: ListPromptsResultCacheScope,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub prompts: ::std::vec::Vec<Prompt>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl ListPromptsResult {
    pub fn builder() -> builder::ListPromptsResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListPromptsResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for ListPromptsResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for ListPromptsResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListPromptsResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListPromptsResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A successful response from the server for a {@link ListPromptsRequestprompts/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: ListPromptsResult,
}
impl ListPromptsResultResponse {
    pub fn builder() -> builder::ListPromptsResultResponse {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of resource templates the server has."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: PaginatedRequestParams,
}
impl ListResourceTemplatesRequest {
    pub fn builder() -> builder::ListResourceTemplatesRequest {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link ListResourceTemplatesRequestresources/templates/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: ListResourceTemplatesResultCacheScope,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: ::std::vec::Vec<ResourceTemplate>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl ListResourceTemplatesResult {
    pub fn builder() -> builder::ListResourceTemplatesResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListResourceTemplatesResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for ListResourceTemplatesResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for ListResourceTemplatesResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListResourceTemplatesResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListResourceTemplatesResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A successful response from the server for a {@link ListResourceTemplatesRequestresources/templates/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: ListResourceTemplatesResult,
}
impl ListResourceTemplatesResultResponse {
    pub fn builder() -> builder::ListResourceTemplatesResultResponse {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of resources the server has."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: PaginatedRequestParams,
}
impl ListResourcesRequest {
    pub fn builder() -> builder::ListResourcesRequest {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link ListResourcesRequestresources/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: ListResourcesResultCacheScope,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub resources: ::std::vec::Vec<Resource>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl ListResourcesResult {
    pub fn builder() -> builder::ListResourcesResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListResourcesResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for ListResourcesResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for ListResourcesResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListResourcesResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListResourcesResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A successful response from the server for a {@link ListResourcesRequestresources/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: ListResourcesResult,
}
impl ListResourcesResultResponse {
    pub fn builder() -> builder::ListResourcesResultResponse {
        Default::default()
    }
}
#[doc = "Sent from the server to request a list of root URIs from the client. Roots allow\nservers to ask for specific directories or files to operate on. A common example\nfor roots is providing a set of repositories or directories a server should operate\non.\n\nThis request is typically used when the server needs to understand the file system\nstructure or access specific locations that the client has permission to read from."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequest {
    pub method: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListRootsRequestParams>,
}
impl ListRootsRequest {
    pub fn builder() -> builder::ListRootsRequest {
        Default::default()
    }
}
#[doc = "`ListRootsRequestParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ListRootsRequestParams {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
}
impl ListRootsRequestParams {
    pub fn builder() -> builder::ListRootsRequestParams {
        Default::default()
    }
}
#[doc = "The result returned by the client for a {@link ListRootsRequestroots/list} request.\nThis result contains an array of {@link Root} objects, each representing a root directory\nor file that the server can operate on."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsResult {
    pub roots: ::std::vec::Vec<Root>,
}
impl ListRootsResult {
    pub fn builder() -> builder::ListRootsResult {
        Default::default()
    }
}
#[doc = "Sent from the client to request a list of tools the server has."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: PaginatedRequestParams,
}
impl ListToolsRequest {
    pub fn builder() -> builder::ListToolsRequest {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link ListToolsRequesttools/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: ListToolsResultCacheScope,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    pub tools: ::std::vec::Vec<Tool>,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl ListToolsResult {
    pub fn builder() -> builder::ListToolsResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListToolsResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for ListToolsResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for ListToolsResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListToolsResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListToolsResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A successful response from the server for a {@link ListToolsRequesttools/list} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: ListToolsResult,
}
impl ListToolsResultResponse {
    pub fn builder() -> builder::ListToolsResultResponse {
        Default::default()
    }
}
#[doc = "The severity of a log message.\n\nThese map to syslog message severities, as specified in RFC-5424:\nhttps://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LoggingLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl ::std::fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alert => f.write_str("alert"),
            Self::Critical => f.write_str("critical"),
            Self::Debug => f.write_str("debug"),
            Self::Emergency => f.write_str("emergency"),
            Self::Error => f.write_str("error"),
            Self::Info => f.write_str("info"),
            Self::Notice => f.write_str("notice"),
            Self::Warning => f.write_str("warning"),
        }
    }
}
impl ::std::str::FromStr for LoggingLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alert" => Ok(Self::Alert),
            "critical" => Ok(Self::Critical),
            "debug" => Ok(Self::Debug),
            "emergency" => Ok(Self::Emergency),
            "error" => Ok(Self::Error),
            "info" => Ok(Self::Info),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JSONRPCNotification of a log message passed from server to client. The client opts in by setting `\"io.modelcontextprotocol/logLevel\"` in a request's `_meta`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: LoggingMessageNotificationParams,
}
impl LoggingMessageNotification {
    pub fn builder() -> builder::LoggingMessageNotification {
        Default::default()
    }
}
#[doc = "Parameters for a `notifications/message` notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotificationParams {
    #[doc = "The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here."]
    pub data: ::serde_json::Value,
    #[doc = "The severity of this log message."]
    pub level: LoggingLevel,
    #[doc = "An optional name of the logger issuing this message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logger: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
}
impl LoggingMessageNotificationParams {
    pub fn builder() -> builder::LoggingMessageNotificationParams {
        Default::default()
    }
}
#[doc = "Represents the contents of a `_meta` field, which clients and servers use to attach additional metadata to their interactions.\n\nCertain key names are reserved by MCP for protocol-level metadata; implementations MUST NOT make assumptions about values at these keys. Additionally, specific schema definitions may reserve particular names for purpose-specific metadata, as declared in those definitions.\n\nValid keys have two segments:\n\n**Prefix:**\n- Optional — if specified, MUST be a series of _labels_ separated by dots (`.`), followed by a slash (`/`).\n- Labels MUST start with a letter and end with a letter or digit. Interior characters may be letters, digits, or hyphens (`-`).\n- Implementations SHOULD use reverse DNS notation (e.g., `com.example/` rather than `example.com/`).\n- Any prefix where the second label is `modelcontextprotocol` or `mcp` is **reserved** for MCP use. For example: `io.modelcontextprotocol/`, `dev.mcp/`, `org.modelcontextprotocol.api/`, and `com.mcp.tools/` are all reserved. However, `com.example.mcp/` is NOT reserved, as the second label is `example`.\n\n**Name:**\n- Unless empty, MUST start and end with an alphanumeric character (`[a-z0-9A-Z]`).\n- Interior characters may be alphanumeric, hyphens (`-`), underscores (`_`), or dots (`.`)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MetaObject(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for MetaObject {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<MetaObject>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: MetaObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for MetaObject
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "A JSON-RPC error indicating that the requested method does not exist or is not available.\n\nIn MCP, a server returns this error when a client invokes a method the server does not implement — either a genuinely unknown method, or one gated behind a server capability the server did not advertise (e.g., calling `prompts/list` when the `prompts` capability was not advertised).\n\nA request that requires a client capability the client did not declare is signalled instead by {@link MissingRequiredClientCapabilityError} (`-32021`)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MethodNotFoundError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl MethodNotFoundError {
    pub fn builder() -> builder::MethodNotFoundError {
        Default::default()
    }
}
#[doc = "Returned when processing a request requires a capability the client did not\ndeclare in `clientCapabilities`. For HTTP, the response status code MUST be\n`400 Bad Request`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MissingRequiredClientCapabilityError {
    pub error: MissingRequiredClientCapabilityErrorError,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RequestId>,
    pub jsonrpc: ::std::string::String,
}
impl MissingRequiredClientCapabilityError {
    pub fn builder() -> builder::MissingRequiredClientCapabilityError {
        Default::default()
    }
}
#[doc = "`MissingRequiredClientCapabilityErrorError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MissingRequiredClientCapabilityErrorError {
    pub code: MissingRequiredClientCapabilityErrorErrorCode,
    pub data: MissingRequiredClientCapabilityErrorErrorData,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl MissingRequiredClientCapabilityErrorError {
    pub fn builder() -> builder::MissingRequiredClientCapabilityErrorError {
        Default::default()
    }
}
#[doc = "`MissingRequiredClientCapabilityErrorErrorCode`"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MissingRequiredClientCapabilityErrorErrorCode(i64);
impl ::std::ops::Deref for MissingRequiredClientCapabilityErrorErrorCode {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MissingRequiredClientCapabilityErrorErrorCode> for i64 {
    fn from(value: MissingRequiredClientCapabilityErrorErrorCode) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for MissingRequiredClientCapabilityErrorErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![-32021_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for MissingRequiredClientCapabilityErrorErrorCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`MissingRequiredClientCapabilityErrorErrorData`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MissingRequiredClientCapabilityErrorErrorData {
    #[doc = "The capabilities the server requires from the client to process this request."]
    #[serde(rename = "requiredCapabilities")]
    pub required_capabilities: ClientCapabilities,
}
impl MissingRequiredClientCapabilityErrorErrorData {
    pub fn builder() -> builder::MissingRequiredClientCapabilityErrorErrorData {
        Default::default()
    }
}
#[doc = "Hints to use for model selection.\n\nKeys not declared here are currently left unspecified by the spec and are up\nto the client to interpret."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ModelHint {
    #[doc = "A hint for a model name.\n\nThe client SHOULD treat this as a substring of a model name; for example:\n - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`\n - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.\n - `claude` should match any Claude model\n\nThe client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:\n - `gemini-1.5-flash` could match `claude-3-haiku-20240307`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ModelHint {
    pub fn builder() -> builder::ModelHint {
        Default::default()
    }
}
#[doc = "The server's preferences for model selection, requested of the client during sampling.\n\nBecause LLMs can vary along multiple dimensions, choosing the \"best\" model is\nrarely straightforward.  Different models excel in different areas—some are\nfaster but less capable, others are more capable but more expensive, and so\non. This interface allows servers to express their priorities across multiple\ndimensions to help clients make an appropriate selection for their use case.\n\nThese preferences are always advisory. The client MAY ignore them. It is also\nup to the client to decide how to interpret these preferences and how to\nbalance them against other considerations."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ModelPreferences {
    #[doc = "How much to prioritize cost when selecting a model. A value of 0 means cost\nis not important, while a value of 1 means cost is the most important\nfactor."]
    #[serde(
        rename = "costPriority",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cost_priority: ::std::option::Option<f64>,
    #[doc = "Optional hints to use for model selection.\n\nIf multiple hints are specified, the client MUST evaluate them in order\n(such that the first match is taken).\n\nThe client SHOULD prioritize these hints over the numeric priorities, but\nMAY still use the priorities to select from ambiguous matches."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hints: ::std::vec::Vec<ModelHint>,
    #[doc = "How much to prioritize intelligence and capabilities when selecting a\nmodel. A value of 0 means intelligence is not important, while a value of 1\nmeans intelligence is the most important factor."]
    #[serde(
        rename = "intelligencePriority",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intelligence_priority: ::std::option::Option<f64>,
    #[doc = "How much to prioritize sampling speed (latency) when selecting a model. A\nvalue of 0 means speed is not important, while a value of 1 means speed is\nthe most important factor."]
    #[serde(
        rename = "speedPriority",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_priority: ::std::option::Option<f64>,
}
impl ModelPreferences {
    pub fn builder() -> builder::ModelPreferences {
        Default::default()
    }
}
#[doc = "`MultiSelectEnumSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct MultiSelectEnumSchema {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<UntitledMultiSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<TitledMultiSelectEnumSchema>,
}
impl MultiSelectEnumSchema {
    pub fn builder() -> builder::MultiSelectEnumSchema {
        Default::default()
    }
}
#[doc = "`Notification`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Notification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl Notification {
    pub fn builder() -> builder::Notification {
        Default::default()
    }
}
#[doc = "Extends {@link MetaObject} with additional notification-specific fields. All key naming rules from `MetaObject` apply."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct NotificationMetaObject {
    #[doc = "Identifies the subscription stream a notification was delivered on. The\nserver MUST include this key on every notification delivered via a\n{@link SubscriptionsListenRequestsubscriptions/listen} stream, so the\nclient can correlate the notification with the originating subscription.\nThe key is absent on notifications not delivered via a subscription\nstream (e.g. progress notifications for an in-flight request), which is\nwhy it is optional here.\n\nThe value is the JSON-RPC ID of the `subscriptions/listen` request that\nopened the stream."]
    #[serde(
        rename = "io.modelcontextprotocol/subscriptionId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub io_modelcontextprotocol_subscription_id: ::std::option::Option<RequestId>,
}
impl NotificationMetaObject {
    pub fn builder() -> builder::NotificationMetaObject {
        Default::default()
    }
}
#[doc = "Common params for any notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct NotificationParams {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
}
impl NotificationParams {
    pub fn builder() -> builder::NotificationParams {
        Default::default()
    }
}
#[doc = "`NumberSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NumberSchema {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: NumberSchemaType,
}
impl NumberSchema {
    pub fn builder() -> builder::NumberSchema {
        Default::default()
    }
}
#[doc = "`NumberSchemaType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NumberSchemaType {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
}
impl ::std::fmt::Display for NumberSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Integer => f.write_str("integer"),
            Self::Number => f.write_str("number"),
        }
    }
}
impl ::std::str::FromStr for NumberSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NumberSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NumberSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaginatedRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: PaginatedRequestParams,
}
impl PaginatedRequest {
    pub fn builder() -> builder::PaginatedRequest {
        Default::default()
    }
}
#[doc = "Common params for paginated requests."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
}
impl PaginatedRequestParams {
    pub fn builder() -> builder::PaginatedRequestParams {
        Default::default()
    }
}
#[doc = "`PaginatedResult`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedResult {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
}
impl PaginatedResult {
    pub fn builder() -> builder::PaginatedResult {
        Default::default()
    }
}
#[doc = "A JSON-RPC error indicating that invalid JSON was received by the server. This error is returned when the server cannot parse the JSON text of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ParseError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl ParseError {
    pub fn builder() -> builder::ParseError {
        Default::default()
    }
}
#[doc = "Restricted schema definitions that only allow primitive types\nwithout nested objects or arrays."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct PrimitiveSchemaDefinition {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<StringSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<NumberSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<BooleanSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<UntitledSingleSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_4: ::std::option::Option<TitledSingleSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_5: ::std::option::Option<UntitledMultiSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_6: ::std::option::Option<TitledMultiSelectEnumSchema>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_7: ::std::option::Option<LegacyTitledEnumSchema>,
}
impl PrimitiveSchemaDefinition {
    pub fn builder() -> builder::PrimitiveSchemaDefinition {
        Default::default()
    }
}
#[doc = "An out-of-band notification used to inform the receiver of a progress update for a long-running request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: ProgressNotificationParams,
}
impl ProgressNotification {
    pub fn builder() -> builder::ProgressNotification {
        Default::default()
    }
}
#[doc = "Parameters for a {@link ProgressNotificationnotifications/progress} notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotificationParams {
    #[doc = "An optional message describing the current progress."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
    #[doc = "The progress thus far. This should increase every time progress is made, even if the total is unknown."]
    pub progress: f64,
    #[doc = "The progress token which was given in the initial request, used to associate this notification with the request that is proceeding."]
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    #[doc = "Total number of items to process (or total progress required), if known."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<f64>,
}
impl ProgressNotificationParams {
    pub fn builder() -> builder::ProgressNotificationParams {
        Default::default()
    }
}
#[doc = "A progress token, used to associate progress notifications with the original request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ProgressToken {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for ProgressToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for ProgressToken {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "A prompt or prompt template that the server offers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Prompt {
    #[doc = "A list of arguments to use for templating the prompt."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub arguments: ::std::vec::Vec<PromptArgument>,
    #[doc = "An optional description of what this prompt provides"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl Prompt {
    pub fn builder() -> builder::Prompt {
        Default::default()
    }
}
#[doc = "Describes an argument that a prompt can accept."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptArgument {
    #[doc = "A human-readable description of the argument."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Whether this argument must be provided."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl PromptArgument {
    pub fn builder() -> builder::PromptArgument {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `promptsListChanged` filter field."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptListChangedNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<NotificationParams>,
}
impl PromptListChangedNotification {
    pub fn builder() -> builder::PromptListChangedNotification {
        Default::default()
    }
}
#[doc = "Describes a message returned as part of a prompt.\n\nThis is similar to {@link SamplingMessage}, but also supports the embedding of\nresources from the MCP server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptMessage {
    pub content: ContentBlock,
    pub role: Role,
}
impl PromptMessage {
    pub fn builder() -> builder::PromptMessage {
        Default::default()
    }
}
#[doc = "Identifies a prompt."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptReference {
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl PromptReference {
    pub fn builder() -> builder::PromptReference {
        Default::default()
    }
}
#[doc = "Sent from the client to the server, to read a specific resource URI."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: ReadResourceRequestParams,
}
impl ReadResourceRequest {
    pub fn builder() -> builder::ReadResourceRequest {
        Default::default()
    }
}
#[doc = "Parameters for a `resources/read` request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequestParams {
    #[serde(
        rename = "inputResponses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub input_responses: ::std::option::Option<InputResponses>,
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[serde(
        rename = "requestState",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_state: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl ReadResourceRequestParams {
    pub fn builder() -> builder::ReadResourceRequestParams {
        Default::default()
    }
}
#[doc = "The result returned by the server for a {@link ReadResourceRequestresources/read} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceResult {
    #[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
    #[serde(rename = "cacheScope")]
    pub cache_scope: ReadResourceResultCacheScope,
    pub contents: ::std::vec::Vec<ReadResourceResultContentsItem>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[doc = "A hint from the server indicating how long (in milliseconds) the\nclient MAY cache this response before re-fetching. Semantics are\nanalogous to HTTP Cache-Control max-age.\n\n- If 0, The response SHOULD be considered immediately stale,\n  The client MAY re-fetch every time the result is needed.\n- If positive, the client SHOULD consider the result fresh for this many\n  milliseconds after receiving the response."]
    #[serde(rename = "ttlMs")]
    pub ttl_ms: u64,
}
impl ReadResourceResult {
    pub fn builder() -> builder::ReadResourceResult {
        Default::default()
    }
}
#[doc = "Indicates the intended scope of the cached response, analogous to HTTP\n`Cache-Control: public` vs `Cache-Control: private`.\n\n- `\"public\"`: The response does not contain user-specific data. Any\n  client or intermediary (e.g., shared gateway, caching proxy) MAY cache\n  the response and serve it across authorization contexts.\n- `\"private\"`: The response MAY be cached and reused only within the\n  same authorization context. Caches MUST NOT be shared across\n  authorization contexts (e.g., a different access token requires a\n  different cache)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReadResourceResultCacheScope {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ::std::fmt::Display for ReadResourceResultCacheScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Private => f.write_str("private"),
            Self::Public => f.write_str("public"),
        }
    }
}
impl ::std::str::FromStr for ReadResourceResultCacheScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReadResourceResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReadResourceResultCacheScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReadResourceResultContentsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReadResourceResultContentsItem {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl ::std::convert::From<TextResourceContents> for ReadResourceResultContentsItem {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl ::std::convert::From<BlobResourceContents> for ReadResourceResultContentsItem {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "A successful response from the server for a {@link ReadResourceRequestresources/read} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: ReadResourceResultResponseResult,
}
impl ReadResourceResultResponse {
    pub fn builder() -> builder::ReadResourceResultResponse {
        Default::default()
    }
}
#[doc = "`ReadResourceResultResponseResult`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReadResourceResultResponseResult {
    InputRequiredResult(InputRequiredResult),
    ReadResourceResult(ReadResourceResult),
}
impl ::std::convert::From<InputRequiredResult> for ReadResourceResultResponseResult {
    fn from(value: InputRequiredResult) -> Self {
        Self::InputRequiredResult(value)
    }
}
impl ::std::convert::From<ReadResourceResult> for ReadResourceResultResponseResult {
    fn from(value: ReadResourceResult) -> Self {
        Self::ReadResourceResult(value)
    }
}
#[doc = "`Request`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Request {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl Request {
    pub fn builder() -> builder::Request {
        Default::default()
    }
}
#[doc = "A uniquely identifying ID for a request in JSON-RPC."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestId {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for RequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Extends {@link MetaObject} with additional request-specific fields. All key naming rules from `MetaObject` apply."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestMetaObject {
    #[doc = "The client's capabilities for this specific request. Required.\n\nCapabilities are declared per-request rather than once at initialization;\nan empty object means the client supports no optional capabilities.\nServers MUST NOT infer capabilities from prior requests."]
    #[serde(rename = "io.modelcontextprotocol/clientCapabilities")]
    pub io_modelcontextprotocol_client_capabilities: ClientCapabilities,
    #[doc = "Identifies the client software making the request. Clients SHOULD\ninclude this field on every request unless specifically configured not\nto do so.\n\nThe {@link Implementation} schema requires `name` and `version`; other\nfields are optional.\n\nThe value is self-reported by the client and is not verified by the\nprotocol. It is intended for display, logging, and debugging. Servers\nSHOULD NOT use it to change their behavior, and SHOULD NOT rely on it for\nsecurity decisions."]
    #[serde(
        rename = "io.modelcontextprotocol/clientInfo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub io_modelcontextprotocol_client_info: ::std::option::Option<Implementation>,
    #[doc = "The desired log level for this request. Optional.\n\nIf absent, the server MUST NOT send any {@link LoggingMessageNotificationnotifications/message}\nnotifications for this request. The client opts in to log messages by\nexplicitly setting a level. Replaces the former `logging/setLevel` RPC."]
    #[serde(
        rename = "io.modelcontextprotocol/logLevel",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub io_modelcontextprotocol_log_level: ::std::option::Option<LoggingLevel>,
    #[doc = "The MCP Protocol Version being used for this request. Required.\n\nFor the HTTP transport, this value MUST match the `MCP-Protocol-Version`\nheader; otherwise the server MUST return a `400 Bad Request`. If the\nserver does not support the requested version, it MUST return an\n{@link UnsupportedProtocolVersionError}."]
    #[serde(rename = "io.modelcontextprotocol/protocolVersion")]
    pub io_modelcontextprotocol_protocol_version: ::std::string::String,
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by {@link ProgressNotificationnotifications/progress}). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
}
impl RequestMetaObject {
    pub fn builder() -> builder::RequestMetaObject {
        Default::default()
    }
}
#[doc = "Common params for any request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestParams {
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
}
impl RequestParams {
    pub fn builder() -> builder::RequestParams {
        Default::default()
    }
}
#[doc = "A known resource that the server is capable of reading."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Resource {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this resource represents.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\n\nThis can be used by Hosts to display file sizes and estimate context window usage."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<i64>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl Resource {
    pub fn builder() -> builder::Resource {
        Default::default()
    }
}
#[doc = "The contents of a specific resource or sub-resource."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceContents {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ResourceContents {
    pub fn builder() -> builder::ResourceContents {
        Default::default()
    }
}
#[doc = "A resource that the server is capable of reading, included in a prompt or tool call result.\n\nNote: resource links returned by tools are not guaranteed to appear in the results of {@link ListResourcesRequestresources/list} requests."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceLink {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this resource represents.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.\n\nThis can be used by Hosts to display file sizes and estimate context window usage."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<i64>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl ResourceLink {
    pub fn builder() -> builder::ResourceLink {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `resourcesListChanged` filter field."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceListChangedNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<NotificationParams>,
}
impl ResourceListChangedNotification {
    pub fn builder() -> builder::ResourceListChangedNotification {
        Default::default()
    }
}
#[doc = "Common params for resource-related requests."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceRequestParams {
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[doc = "The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl ResourceRequestParams {
    pub fn builder() -> builder::ResourceRequestParams {
        Default::default()
    }
}
#[doc = "A template description for resources available on the server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplate {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[doc = "A description of what this template is for.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "A URI template (according to RFC 6570) that can be used to construct resource URIs."]
    #[serde(rename = "uriTemplate")]
    pub uri_template: ::std::string::String,
}
impl ResourceTemplate {
    pub fn builder() -> builder::ResourceTemplate {
        Default::default()
    }
}
#[doc = "A reference to a resource or resource template definition."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplateReference {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[doc = "The URI or URI template of the resource."]
    pub uri: ::std::string::String,
}
impl ResourceTemplateReference {
    pub fn builder() -> builder::ResourceTemplateReference {
        Default::default()
    }
}
#[doc = "A notification from the server to the client, informing it that a resource has changed and may need to be read again. This is only sent for resources the client opted in to via the `resourceSubscriptions` field of a {@link SubscriptionsListenRequestsubscriptions/listen} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: ResourceUpdatedNotificationParams,
}
impl ResourceUpdatedNotification {
    pub fn builder() -> builder::ResourceUpdatedNotification {
        Default::default()
    }
}
#[doc = "Parameters for a `notifications/resources/updated` notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotificationParams {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
    #[doc = "The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to."]
    pub uri: ::std::string::String,
}
impl ResourceUpdatedNotificationParams {
    pub fn builder() -> builder::ResourceUpdatedNotificationParams {
        Default::default()
    }
}
#[doc = "Common result fields."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Result {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ResultMetaObject>,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl Result {
    pub fn builder() -> builder::Result {
        Default::default()
    }
}
#[doc = "Extends {@link MetaObject} with additional result-specific fields. All key naming rules from `MetaObject` apply."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ResultMetaObject {
    #[doc = "Identifies the server software producing the response. Servers SHOULD\ninclude this field on every response unless specifically configured not\nto do so.\n\nThe {@link Implementation} schema requires `name` and `version`; other\nfields are optional.\n\nThe value is self-reported by the server and is not verified by the\nprotocol. It is intended for display, logging, and debugging. Clients\nSHOULD NOT use it to change their behavior, and SHOULD NOT rely on it for\nsecurity decisions."]
    #[serde(
        rename = "io.modelcontextprotocol/serverInfo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub io_modelcontextprotocol_server_info: ::std::option::Option<Implementation>,
}
impl ResultMetaObject {
    pub fn builder() -> builder::ResultMetaObject {
        Default::default()
    }
}
#[doc = "Indicates the type of a {@link Result} object, allowing the client to\ndetermine how to parse the response.\n\ncomplete - the request completed successfully and the result contains the final content.\ninput_required - the request requires additional input and the result contains an {@link InputRequiredResult} object with instructions for the client to provide additional input before retrying the original request."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct ResultType(pub ::std::string::String);
impl ::std::ops::Deref for ResultType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ResultType> for ::std::string::String {
    fn from(value: ResultType) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ResultType {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ResultType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ResultType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "The sender or recipient of messages and data in a conversation."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a root directory or file that the server can operate on."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Root {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "An optional name for the root. This can be used to provide a human-readable\nidentifier for the root, which may be useful for display purposes or for\nreferencing the root in other parts of the application."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The URI identifying the root. This *must* start with `file://` for now.\nThis restriction may be relaxed in future versions of the protocol to allow\nother URI schemes."]
    pub uri: ::std::string::String,
}
impl Root {
    pub fn builder() -> builder::Root {
        Default::default()
    }
}
#[doc = "Describes a message issued to or received from an LLM API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SamplingMessage {
    pub content: SamplingMessageContent,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    pub role: Role,
}
impl SamplingMessage {
    pub fn builder() -> builder::SamplingMessage {
        Default::default()
    }
}
#[doc = "`SamplingMessageContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SamplingMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ToolUseContent(ToolUseContent),
    ToolResultContent(ToolResultContent),
    Array(::std::vec::Vec<SamplingMessageContentBlock>),
}
impl ::std::convert::From<TextContent> for SamplingMessageContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for SamplingMessageContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for SamplingMessageContent {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
impl ::std::convert::From<ToolUseContent> for SamplingMessageContent {
    fn from(value: ToolUseContent) -> Self {
        Self::ToolUseContent(value)
    }
}
impl ::std::convert::From<ToolResultContent> for SamplingMessageContent {
    fn from(value: ToolResultContent) -> Self {
        Self::ToolResultContent(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<SamplingMessageContentBlock>> for SamplingMessageContent {
    fn from(value: ::std::vec::Vec<SamplingMessageContentBlock>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`SamplingMessageContentBlock`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SamplingMessageContentBlock {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ToolUseContent(ToolUseContent),
    ToolResultContent(ToolResultContent),
}
impl ::std::convert::From<TextContent> for SamplingMessageContentBlock {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ImageContent> for SamplingMessageContentBlock {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<AudioContent> for SamplingMessageContentBlock {
    fn from(value: AudioContent) -> Self {
        Self::AudioContent(value)
    }
}
impl ::std::convert::From<ToolUseContent> for SamplingMessageContentBlock {
    fn from(value: ToolUseContent) -> Self {
        Self::ToolUseContent(value)
    }
}
impl ::std::convert::From<ToolResultContent> for SamplingMessageContentBlock {
    fn from(value: ToolResultContent) -> Self {
        Self::ToolResultContent(value)
    }
}
#[doc = "Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilities {
    #[doc = "Present if the server supports argument autocompletion suggestions."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completions: ::std::option::Option<JsonObject>,
    #[doc = "Experimental, non-standard capabilities that the server supports."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub experimental: ::std::collections::HashMap<::std::string::String, JsonObject>,
    #[doc = "Optional MCP extensions that the server supports. Keys are extension identifiers\n(e.g., \"io.modelcontextprotocol/tasks\"), and values are per-extension settings\nobjects. An empty object indicates support with no settings.\n\nKeys MUST follow the {@link MetaObject`_meta` key naming rules}, with a\nmandatory prefix."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extensions: ::std::collections::HashMap<::std::string::String, JsonObject>,
    #[doc = "Present if the server supports sending log messages to the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<JsonObject>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompts: ::std::option::Option<ServerCapabilitiesPrompts>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<ServerCapabilitiesResources>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ServerCapabilitiesTools>,
}
impl ServerCapabilities {
    pub fn builder() -> builder::ServerCapabilities {
        Default::default()
    }
}
#[doc = "Present if the server offers any prompt templates."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesPrompts {
    #[doc = "Whether this server supports notifications for changes to the prompt list."]
    #[serde(
        rename = "listChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl ServerCapabilitiesPrompts {
    pub fn builder() -> builder::ServerCapabilitiesPrompts {
        Default::default()
    }
}
#[doc = "Present if the server offers any resources to read."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesResources {
    #[doc = "Whether this server supports notifications for changes to the resource list."]
    #[serde(
        rename = "listChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
    #[doc = "Whether this server supports subscribing to resource updates."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subscribe: ::std::option::Option<bool>,
}
impl ServerCapabilitiesResources {
    pub fn builder() -> builder::ServerCapabilitiesResources {
        Default::default()
    }
}
#[doc = "Present if the server offers any tools to call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerCapabilitiesTools {
    #[doc = "Whether this server supports notifications for changes to the tool list."]
    #[serde(
        rename = "listChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl ServerCapabilitiesTools {
    pub fn builder() -> builder::ServerCapabilitiesTools {
        Default::default()
    }
}
#[doc = "`ServerNotification`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerNotification {
    CancelledNotification(CancelledNotification),
    ProgressNotification(ProgressNotification),
    ResourceListChangedNotification(ResourceListChangedNotification),
    SubscriptionsAcknowledgedNotification(SubscriptionsAcknowledgedNotification),
    ResourceUpdatedNotification(ResourceUpdatedNotification),
    PromptListChangedNotification(PromptListChangedNotification),
    ToolListChangedNotification(ToolListChangedNotification),
    LoggingMessageNotification(LoggingMessageNotification),
}
impl ::std::convert::From<CancelledNotification> for ServerNotification {
    fn from(value: CancelledNotification) -> Self {
        Self::CancelledNotification(value)
    }
}
impl ::std::convert::From<ProgressNotification> for ServerNotification {
    fn from(value: ProgressNotification) -> Self {
        Self::ProgressNotification(value)
    }
}
impl ::std::convert::From<ResourceListChangedNotification> for ServerNotification {
    fn from(value: ResourceListChangedNotification) -> Self {
        Self::ResourceListChangedNotification(value)
    }
}
impl ::std::convert::From<SubscriptionsAcknowledgedNotification> for ServerNotification {
    fn from(value: SubscriptionsAcknowledgedNotification) -> Self {
        Self::SubscriptionsAcknowledgedNotification(value)
    }
}
impl ::std::convert::From<ResourceUpdatedNotification> for ServerNotification {
    fn from(value: ResourceUpdatedNotification) -> Self {
        Self::ResourceUpdatedNotification(value)
    }
}
impl ::std::convert::From<PromptListChangedNotification> for ServerNotification {
    fn from(value: PromptListChangedNotification) -> Self {
        Self::PromptListChangedNotification(value)
    }
}
impl ::std::convert::From<ToolListChangedNotification> for ServerNotification {
    fn from(value: ToolListChangedNotification) -> Self {
        Self::ToolListChangedNotification(value)
    }
}
impl ::std::convert::From<LoggingMessageNotification> for ServerNotification {
    fn from(value: LoggingMessageNotification) -> Self {
        Self::LoggingMessageNotification(value)
    }
}
#[doc = "`ServerResult`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ServerResult {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<Result>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<InputRequiredResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<DiscoverResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<ListResourcesResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_4: ::std::option::Option<ListResourceTemplatesResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_5: ::std::option::Option<ReadResourceResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_6: ::std::option::Option<SubscriptionsListenResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_7: ::std::option::Option<ListPromptsResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_8: ::std::option::Option<GetPromptResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_9: ::std::option::Option<ListToolsResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_10: ::std::option::Option<CallToolResult>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_11: ::std::option::Option<CompleteResult>,
}
impl ServerResult {
    pub fn builder() -> builder::ServerResult {
        Default::default()
    }
}
#[doc = "`SingleSelectEnumSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SingleSelectEnumSchema {
    UntitledSingleSelectEnumSchema(UntitledSingleSelectEnumSchema),
    TitledSingleSelectEnumSchema(TitledSingleSelectEnumSchema),
}
impl ::std::convert::From<UntitledSingleSelectEnumSchema> for SingleSelectEnumSchema {
    fn from(value: UntitledSingleSelectEnumSchema) -> Self {
        Self::UntitledSingleSelectEnumSchema(value)
    }
}
impl ::std::convert::From<TitledSingleSelectEnumSchema> for SingleSelectEnumSchema {
    fn from(value: TitledSingleSelectEnumSchema) -> Self {
        Self::TitledSingleSelectEnumSchema(value)
    }
}
#[doc = "`StringSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StringSchema {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<StringSchemaFormat>,
    #[serde(
        rename = "maxLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_length: ::std::option::Option<i64>,
    #[serde(
        rename = "minLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_length: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl StringSchema {
    pub fn builder() -> builder::StringSchema {
        Default::default()
    }
}
#[doc = "`StringSchemaFormat`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StringSchemaFormat {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
}
impl ::std::fmt::Display for StringSchemaFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Date => f.write_str("date"),
            Self::DateTime => f.write_str("date-time"),
            Self::Email => f.write_str("email"),
            Self::Uri => f.write_str("uri"),
        }
    }
}
impl ::std::str::FromStr for StringSchemaFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            "email" => Ok(Self::Email),
            "uri" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StringSchemaFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StringSchemaFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The set of notification types a client may opt in to on a\n{@link SubscriptionsListenRequestsubscriptions/listen} request.\n\nEach notification type is **opt-in**; the server **MUST NOT** send\nnotification types the client has not explicitly requested here."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct SubscriptionFilter {
    #[doc = "If true, receive {@link PromptListChangedNotificationnotifications/prompts/list_changed}."]
    #[serde(
        rename = "promptsListChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prompts_list_changed: ::std::option::Option<bool>,
    #[doc = "Subscribe to {@link ResourceUpdatedNotificationnotifications/resources/updated} for these resource URIs.\nReplaces the former `resources/subscribe` RPC."]
    #[serde(
        rename = "resourceSubscriptions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub resource_subscriptions: ::std::vec::Vec<::std::string::String>,
    #[doc = "If true, receive {@link ResourceListChangedNotificationnotifications/resources/list_changed}."]
    #[serde(
        rename = "resourcesListChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resources_list_changed: ::std::option::Option<bool>,
    #[doc = "If true, receive {@link ToolListChangedNotificationnotifications/tools/list_changed}."]
    #[serde(
        rename = "toolsListChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tools_list_changed: ::std::option::Option<bool>,
}
impl SubscriptionFilter {
    pub fn builder() -> builder::SubscriptionFilter {
        Default::default()
    }
}
#[doc = "Sent by the server to acknowledge that a\n{@link SubscriptionsListenRequestsubscriptions/listen} subscription has been\nestablished and to report which notification types it agreed to honor.\n\nThis notification MUST be the first message the server sends carrying the\nsubscription's ID in `io.modelcontextprotocol/subscriptionId`. The server MUST\nNOT send any notification on the subscription before acknowledging it. On\nstdio, where every subscription shares one channel, this ordering is defined\nper subscription ID and not per channel: messages belonging to other\nsubscriptions MAY be interleaved before it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsAcknowledgedNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: SubscriptionsAcknowledgedNotificationParams,
}
impl SubscriptionsAcknowledgedNotification {
    pub fn builder() -> builder::SubscriptionsAcknowledgedNotification {
        Default::default()
    }
}
#[doc = "Parameters for a {@link SubscriptionsAcknowledgedNotificationnotifications/subscriptions/acknowledged} notification."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsAcknowledgedNotificationParams {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<NotificationMetaObject>,
    #[doc = "The subset of requested notification types the server agreed to honor.\nOnly includes notification types the server actually supports; if the\nclient requested an unsupported type (e.g., `promptsListChanged` when\nthe server has no prompts), it is omitted from this set."]
    pub notifications: SubscriptionFilter,
}
impl SubscriptionsAcknowledgedNotificationParams {
    pub fn builder() -> builder::SubscriptionsAcknowledgedNotificationParams {
        Default::default()
    }
}
#[doc = "Sent from the client to open a long-lived channel for receiving notifications\noutside the context of a specific request. Replaces the previous HTTP GET\nendpoint and ensures consistent behavior between HTTP and STDIO."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsListenRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    pub params: SubscriptionsListenRequestParams,
}
impl SubscriptionsListenRequest {
    pub fn builder() -> builder::SubscriptionsListenRequest {
        Default::default()
    }
}
#[doc = "Parameters for a {@link SubscriptionsListenRequestsubscriptions/listen} request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsListenRequestParams {
    #[serde(rename = "_meta")]
    pub meta: RequestMetaObject,
    #[doc = "The notifications the client opts in to on this stream. The server\n**MUST NOT** send notification types the client has not explicitly\nrequested."]
    pub notifications: SubscriptionFilter,
}
impl SubscriptionsListenRequestParams {
    pub fn builder() -> builder::SubscriptionsListenRequestParams {
        Default::default()
    }
}
#[doc = "The response to a {@link SubscriptionsListenRequestsubscriptions/listen}\nrequest, signalling that the subscription has ended gracefully (for example,\nduring server shutdown). Because the listen stream is long-lived, this result\nis sent only when the server tears the subscription down; an abrupt transport\nclose carries no response. The result body is otherwise empty."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsListenResult {
    #[serde(rename = "_meta")]
    pub meta: SubscriptionsListenResultMetaObject,
    #[doc = "Indicates the type of the result, which allows the client to determine\nhow to parse the result object.\n\nServers implementing this protocol version MUST include this field.\nFor backward compatibility, when a client receives a result from a\nserver implementing an earlier protocol version (which does not include\n`resultType`), the client MUST treat the absent field as `\"complete\"`."]
    #[serde(rename = "resultType")]
    pub result_type: ::std::string::String,
}
impl SubscriptionsListenResult {
    pub fn builder() -> builder::SubscriptionsListenResult {
        Default::default()
    }
}
#[doc = "Extends {@link ResultMetaObject} with the subscription-stream identifier carried by a\n{@link SubscriptionsListenResult}. All key naming rules from `MetaObject` apply."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsListenResultMetaObject {
    #[doc = "Identifies the server software producing the response. Servers SHOULD\ninclude this field on every response unless specifically configured not\nto do so.\n\nThe {@link Implementation} schema requires `name` and `version`; other\nfields are optional.\n\nThe value is self-reported by the server and is not verified by the\nprotocol. It is intended for display, logging, and debugging. Clients\nSHOULD NOT use it to change their behavior, and SHOULD NOT rely on it for\nsecurity decisions."]
    #[serde(
        rename = "io.modelcontextprotocol/serverInfo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub io_modelcontextprotocol_server_info: ::std::option::Option<Implementation>,
    #[doc = "Identifies the subscription stream this response closes, so the client can\ncorrelate it with the originating subscription — mirroring the same key on\nthe stream's notifications. The value is the JSON-RPC ID of the\n`subscriptions/listen` request that opened the stream (and equals this\nresponse's `id`)."]
    #[serde(rename = "io.modelcontextprotocol/subscriptionId")]
    pub io_modelcontextprotocol_subscription_id: RequestId,
}
impl SubscriptionsListenResultMetaObject {
    pub fn builder() -> builder::SubscriptionsListenResultMetaObject {
        Default::default()
    }
}
#[doc = "A successful response from the server for a {@link SubscriptionsListenRequestsubscriptions/listen}\nrequest, sent when the server tears the subscription down gracefully."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscriptionsListenResultResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: SubscriptionsListenResult,
}
impl SubscriptionsListenResultResponse {
    pub fn builder() -> builder::SubscriptionsListenResultResponse {
        Default::default()
    }
}
#[doc = "Text provided to or from an LLM."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextContent {
    #[doc = "Optional annotations for the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<Annotations>,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The text content of the message."]
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl TextContent {
    pub fn builder() -> builder::TextContent {
        Default::default()
    }
}
#[doc = "`TextResourceContents`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextResourceContents {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The text of the item. This must only be set if the item can actually be represented as text (not binary data)."]
    pub text: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl TextResourceContents {
    pub fn builder() -> builder::TextResourceContents {
        Default::default()
    }
}
#[doc = "Schema for multiple-selection enumeration with display titles for each option."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TitledMultiSelectEnumSchema {
    #[doc = "Optional default value."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub default: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional description for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub items: TitledMultiSelectEnumSchemaItems,
    #[doc = "Maximum number of items to select."]
    #[serde(
        rename = "maxItems",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_items: ::std::option::Option<i64>,
    #[doc = "Minimum number of items to select."]
    #[serde(
        rename = "minItems",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_items: ::std::option::Option<i64>,
    #[doc = "Optional title for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl TitledMultiSelectEnumSchema {
    pub fn builder() -> builder::TitledMultiSelectEnumSchema {
        Default::default()
    }
}
#[doc = "Schema for array items with enum options and display labels."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TitledMultiSelectEnumSchemaItems {
    #[doc = "Array of enum options with values and display labels."]
    #[serde(rename = "anyOf")]
    pub any_of: ::std::vec::Vec<TitledMultiSelectEnumSchemaItemsAnyOfItem>,
}
impl TitledMultiSelectEnumSchemaItems {
    pub fn builder() -> builder::TitledMultiSelectEnumSchemaItems {
        Default::default()
    }
}
#[doc = "`TitledMultiSelectEnumSchemaItemsAnyOfItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TitledMultiSelectEnumSchemaItemsAnyOfItem {
    #[doc = "The constant enum value."]
    #[serde(rename = "const")]
    pub const_: ::std::string::String,
    #[doc = "Display title for this option."]
    pub title: ::std::string::String,
}
impl TitledMultiSelectEnumSchemaItemsAnyOfItem {
    pub fn builder() -> builder::TitledMultiSelectEnumSchemaItemsAnyOfItem {
        Default::default()
    }
}
#[doc = "Schema for single-selection enumeration with display titles for each option."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TitledSingleSelectEnumSchema {
    #[doc = "Optional default value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Optional description for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Array of enum options with values and display labels."]
    #[serde(rename = "oneOf")]
    pub one_of: ::std::vec::Vec<TitledSingleSelectEnumSchemaOneOfItem>,
    #[doc = "Optional title for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl TitledSingleSelectEnumSchema {
    pub fn builder() -> builder::TitledSingleSelectEnumSchema {
        Default::default()
    }
}
#[doc = "`TitledSingleSelectEnumSchemaOneOfItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TitledSingleSelectEnumSchemaOneOfItem {
    #[doc = "The enum value."]
    #[serde(rename = "const")]
    pub const_: ::std::string::String,
    #[doc = "Display label for this option."]
    pub title: ::std::string::String,
}
impl TitledSingleSelectEnumSchemaOneOfItem {
    pub fn builder() -> builder::TitledSingleSelectEnumSchemaOneOfItem {
        Default::default()
    }
}
#[doc = "Definition for a tool the client can call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tool {
    #[doc = "Optional additional tool information.\n\nDisplay name precedence order is: `title`, `annotations.title`, then `name`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<ToolAnnotations>,
    #[doc = "A human-readable description of the tool.\n\nThis can be used by clients to improve the LLM's understanding of available tools. It can be thought of like a \"hint\" to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional set of sized icons that the client can display in a user interface.\n\nClients that support rendering icons MUST support at least the following MIME types:\n- `image/png` - PNG images (safe, universal compatibility)\n- `image/jpeg` (and `image/jpg`) - JPEG images (safe, universal compatibility)\n\nClients that support rendering icons SHOULD also support:\n- `image/svg+xml` - SVG images (scalable but requires security precautions)\n- `image/webp` - WebP images (modern, efficient format)"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub icons: ::std::vec::Vec<Icon>,
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present)."]
    pub name: ::std::string::String,
    #[serde(
        rename = "outputSchema",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub output_schema: ::std::option::Option<ToolOutputSchema>,
    #[doc = "Intended for UI and end-user contexts — optimized to be human-readable and easily understood,\neven by those unfamiliar with domain-specific terminology.\n\nIf not provided, the name should be used for display (except for {@link Tool},\nwhere `annotations.title` should be given precedence over using `name`,\nif present)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl Tool {
    pub fn builder() -> builder::Tool {
        Default::default()
    }
}
#[doc = "Additional properties describing a {@link Tool} to clients.\n\nNOTE: all properties in `ToolAnnotations` are **hints**.\nThey are not guaranteed to provide a faithful description of\ntool behavior (including descriptive properties like `title`).\n\nClients should never make tool use decisions based on `ToolAnnotations`\nreceived from untrusted servers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ToolAnnotations {
    #[doc = "If true, the tool may perform destructive updates to its environment.\nIf false, the tool performs only additive updates.\n\n(This property is meaningful only when `readOnlyHint == false`)\n\nDefault: true"]
    #[serde(
        rename = "destructiveHint",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destructive_hint: ::std::option::Option<bool>,
    #[doc = "If true, calling the tool repeatedly with the same arguments\nwill have no additional effect on its environment.\n\n(This property is meaningful only when `readOnlyHint == false`)\n\nDefault: false"]
    #[serde(
        rename = "idempotentHint",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub idempotent_hint: ::std::option::Option<bool>,
    #[doc = "If true, this tool may interact with an \"open world\" of external\nentities. If false, the tool's domain of interaction is closed.\nFor example, the world of a web search tool is open, whereas that\nof a memory tool is not.\n\nDefault: true"]
    #[serde(
        rename = "openWorldHint",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_world_hint: ::std::option::Option<bool>,
    #[doc = "If true, the tool does not modify its environment.\n\nDefault: false"]
    #[serde(
        rename = "readOnlyHint",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub read_only_hint: ::std::option::Option<bool>,
    #[doc = "A human-readable title for the tool."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ToolAnnotations {
    pub fn builder() -> builder::ToolAnnotations {
        Default::default()
    }
}
#[doc = "Controls tool selection behavior for sampling requests."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct ToolChoice {
    #[doc = "Controls the tool use ability of the model:\n- `\"auto\"`: Model decides whether to use tools (default)\n- `\"required\"`: Model MUST use at least one tool before completing\n- `\"none\"`: Model MUST NOT use any tools"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<ToolChoiceMode>,
}
impl ToolChoice {
    pub fn builder() -> builder::ToolChoice {
        Default::default()
    }
}
#[doc = "Controls the tool use ability of the model:\n- `\"auto\"`: Model decides whether to use tools (default)\n- `\"required\"`: Model MUST use at least one tool before completing\n- `\"none\"`: Model MUST NOT use any tools"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolChoiceMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ToolChoiceMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::None => f.write_str("none"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A JSON Schema object defining the expected parameters for the tool.\n\nTool arguments are always JSON objects, so `type: \"object\"` is required at the root.\nBeyond that, any JSON Schema 2020-12 keyword may appear alongside `type` — including\ncomposition keywords (`oneOf`, `anyOf`, `allOf`, `not`), conditional keywords\n(`if`/`then`/`else`), reference keywords (`$ref`, `$defs`, `$anchor`), and any other\nstandard validation or annotation keywords.\n\nProperty schemas may carry an `x-mcp-header` annotation to mirror the\nargument value into an HTTP header on the Streamable HTTP transport. See\nthe Streamable HTTP transport specification for the validity and\nextraction rules.\n\nDefaults to JSON Schema 2020-12 when no explicit `$schema` is provided."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolInputSchema {
    #[serde(
        rename = "$schema",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub schema: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ToolInputSchema {
    pub fn builder() -> builder::ToolInputSchema {
        Default::default()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of tools it offers has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `toolsListChanged` filter field."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolListChangedNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<NotificationParams>,
}
impl ToolListChangedNotification {
    pub fn builder() -> builder::ToolListChangedNotification {
        Default::default()
    }
}
#[doc = "An optional JSON Schema object defining the structure of the tool's output returned in\nthe structuredContent field of a {@link CallToolResult}. This can be any valid JSON Schema 2020-12.\n\nDefaults to JSON Schema 2020-12 when no explicit `$schema` is provided."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolOutputSchema {
    #[serde(
        rename = "$schema",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub schema: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ToolOutputSchema {
    pub fn builder() -> builder::ToolOutputSchema {
        Default::default()
    }
}
#[doc = "The result of a tool use, provided by the user back to the assistant."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolResultContent {
    #[doc = "The unstructured result content of the tool use.\n\nThis has the same format as {@link CallToolResult.content} and can include text, images,\naudio, resource links, and embedded resources."]
    pub content: ::std::vec::Vec<ContentBlock>,
    #[doc = "Whether the tool use resulted in an error.\n\nIf true, the content typically describes the error that occurred.\nDefault: false"]
    #[serde(
        rename = "isError",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_error: ::std::option::Option<bool>,
    #[doc = "Optional metadata about the tool result. Clients SHOULD preserve this field when\nincluding tool results in subsequent sampling requests to enable caching optimizations."]
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "An optional structured result value.\n\nThis can be any JSON value (object, array, string, number, boolean, or null).\nIf the tool defined an {@link Tool.outputSchema}, this SHOULD conform to that schema."]
    #[serde(
        rename = "structuredContent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub structured_content: ::std::option::Option<::serde_json::Value>,
    #[doc = "The ID of the tool use this result corresponds to.\n\nThis MUST match the ID from a previous {@link ToolUseContent}."]
    #[serde(rename = "toolUseId")]
    pub tool_use_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ToolResultContent {
    pub fn builder() -> builder::ToolResultContent {
        Default::default()
    }
}
#[doc = "A request from the assistant to call a tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolUseContent {
    #[doc = "A unique identifier for this tool use.\n\nThis ID is used to match tool results to their corresponding tool uses."]
    pub id: ::std::string::String,
    #[doc = "The arguments to pass to the tool, conforming to the tool's input schema."]
    pub input: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Optional metadata about the tool use. Clients SHOULD preserve this field when\nincluding tool uses in subsequent sampling requests to enable caching optimizations."]
    #[serde(
        rename = "_meta",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<MetaObject>,
    #[doc = "The name of the tool to call."]
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ToolUseContent {
    pub fn builder() -> builder::ToolUseContent {
        Default::default()
    }
}
#[doc = "Returned when the request's protocol version is unknown to the server or\nunsupported (e.g., a known experimental or draft version the server has\nchosen not to implement). For HTTP, the response status code MUST be\n`400 Bad Request`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsupportedProtocolVersionError {
    pub error: UnsupportedProtocolVersionErrorError,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RequestId>,
    pub jsonrpc: ::std::string::String,
}
impl UnsupportedProtocolVersionError {
    pub fn builder() -> builder::UnsupportedProtocolVersionError {
        Default::default()
    }
}
#[doc = "`UnsupportedProtocolVersionErrorError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsupportedProtocolVersionErrorError {
    pub code: UnsupportedProtocolVersionErrorErrorCode,
    pub data: UnsupportedProtocolVersionErrorErrorData,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl UnsupportedProtocolVersionErrorError {
    pub fn builder() -> builder::UnsupportedProtocolVersionErrorError {
        Default::default()
    }
}
#[doc = "`UnsupportedProtocolVersionErrorErrorCode`"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct UnsupportedProtocolVersionErrorErrorCode(i64);
impl ::std::ops::Deref for UnsupportedProtocolVersionErrorErrorCode {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<UnsupportedProtocolVersionErrorErrorCode> for i64 {
    fn from(value: UnsupportedProtocolVersionErrorErrorCode) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for UnsupportedProtocolVersionErrorErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![-32022_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for UnsupportedProtocolVersionErrorErrorCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`UnsupportedProtocolVersionErrorErrorData`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsupportedProtocolVersionErrorErrorData {
    #[doc = "The protocol version that was requested by the client."]
    pub requested: ::std::string::String,
    #[doc = "Protocol versions the server supports. The client should choose a\nmutually supported version from this list and retry."]
    pub supported: ::std::vec::Vec<::std::string::String>,
}
impl UnsupportedProtocolVersionErrorErrorData {
    pub fn builder() -> builder::UnsupportedProtocolVersionErrorErrorData {
        Default::default()
    }
}
#[doc = "Schema for multiple-selection enumeration without display titles for options."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UntitledMultiSelectEnumSchema {
    #[doc = "Optional default value."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub default: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional description for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub items: UntitledMultiSelectEnumSchemaItems,
    #[doc = "Maximum number of items to select."]
    #[serde(
        rename = "maxItems",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_items: ::std::option::Option<i64>,
    #[doc = "Minimum number of items to select."]
    #[serde(
        rename = "minItems",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_items: ::std::option::Option<i64>,
    #[doc = "Optional title for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl UntitledMultiSelectEnumSchema {
    pub fn builder() -> builder::UntitledMultiSelectEnumSchema {
        Default::default()
    }
}
#[doc = "Schema for the array items."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UntitledMultiSelectEnumSchemaItems {
    #[doc = "Array of enum values to choose from."]
    #[serde(rename = "enum")]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl UntitledMultiSelectEnumSchemaItems {
    pub fn builder() -> builder::UntitledMultiSelectEnumSchemaItems {
        Default::default()
    }
}
#[doc = "Schema for single-selection enumeration without display titles for options."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UntitledSingleSelectEnumSchema {
    #[doc = "Optional default value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "Optional description for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Array of enum values to choose from."]
    #[serde(rename = "enum")]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional title for the enum field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl UntitledSingleSelectEnumSchema {
    pub fn builder() -> builder::UntitledSingleSelectEnumSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Annotations {
        audience: ::std::result::Result<::std::vec::Vec<super::Role>, ::std::string::String>,
        last_modified: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Annotations {
        fn default() -> Self {
            Self {
                audience: Ok(Default::default()),
                last_modified: Ok(Default::default()),
                priority: Ok(Default::default()),
            }
        }
    }
    impl Annotations {
        pub fn audience<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Role>>,
            T::Error: ::std::fmt::Display,
        {
            self.audience = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audience: {e}"));
            self
        }
        pub fn last_modified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_modified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_modified: {e}"));
            self
        }
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Annotations> for super::Annotations {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Annotations,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audience: value.audience?,
                last_modified: value.last_modified?,
                priority: value.priority?,
            })
        }
    }
    impl ::std::convert::From<super::Annotations> for Annotations {
        fn from(value: super::Annotations) -> Self {
            Self {
                audience: Ok(value.audience),
                last_modified: Ok(value.last_modified),
                priority: Ok(value.priority),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AudioContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AudioContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                data: Err("no value supplied for data".to_string()),
                meta: Ok(Default::default()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AudioContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AudioContent> for super::AudioContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                data: value.data?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AudioContent> for AudioContent {
        fn from(value: super::AudioContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                data: Ok(value.data),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BaseMetadata {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseMetadata {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
            }
        }
    }
    impl BaseMetadata {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseMetadata> for super::BaseMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseMetadata> for BaseMetadata {
        fn from(value: super::BaseMetadata) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlobResourceContents {
        blob: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BlobResourceContents {
        fn default() -> Self {
            Self {
                blob: Err("no value supplied for blob".to_string()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl BlobResourceContents {
        pub fn blob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.blob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blob: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BlobResourceContents> for super::BlobResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BlobResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                blob: value.blob?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::BlobResourceContents> for BlobResourceContents {
        fn from(value: super::BlobResourceContents) -> Self {
            Self {
                blob: Ok(value.blob),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BooleanSchema {
        default: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BooleanSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl BooleanSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BooleanSchema> for super::BooleanSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BooleanSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::BooleanSchema> for BooleanSchema {
        fn from(value: super::BooleanSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CacheableResult {
        cache_scope: ::std::result::Result<super::CacheableResultCacheScope, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for CacheableResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl CacheableResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CacheableResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CacheableResult> for super::CacheableResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CacheableResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                meta: value.meta?,
                result_type: value.result_type?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::CacheableResult> for CacheableResult {
        fn from(value: super::CacheableResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CallToolRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CallToolRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CallToolRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallToolRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolRequest> for super::CallToolRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolRequest> for CallToolRequest {
        fn from(value: super::CallToolRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolRequestParams {
        arguments: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        input_responses: ::std::result::Result<
            ::std::option::Option<super::InputResponses>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_state: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallToolRequestParams {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                input_responses: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
                name: Err("no value supplied for name".to_string()),
                request_state: Ok(Default::default()),
            }
        }
    }
    impl CallToolRequestParams {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn input_responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputResponses>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_responses: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn request_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolRequestParams> for super::CallToolRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                input_responses: value.input_responses?,
                meta: value.meta?,
                name: value.name?,
                request_state: value.request_state?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolRequestParams> for CallToolRequestParams {
        fn from(value: super::CallToolRequestParams) -> Self {
            Self {
                arguments: Ok(value.arguments),
                input_responses: Ok(value.input_responses),
                meta: Ok(value.meta),
                name: Ok(value.name),
                request_state: Ok(value.request_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolResult {
        content: ::std::result::Result<::std::vec::Vec<super::ContentBlock>, ::std::string::String>,
        is_error: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        structured_content: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CallToolResult {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                is_error: Ok(Default::default()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                structured_content: Ok(Default::default()),
            }
        }
    }
    impl CallToolResult {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ContentBlock>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn is_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_error: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn structured_content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.structured_content = value.try_into().map_err(|e| {
                format!("error converting supplied value for structured_content: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolResult> for super::CallToolResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                is_error: value.is_error?,
                meta: value.meta?,
                result_type: value.result_type?,
                structured_content: value.structured_content?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolResult> for CallToolResult {
        fn from(value: super::CallToolResult) -> Self {
            Self {
                content: Ok(value.content),
                is_error: Ok(value.is_error),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
                structured_content: Ok(value.structured_content),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CallToolResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::CallToolResultResponseResult, ::std::string::String>,
    }
    impl ::std::default::Default for CallToolResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl CallToolResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CallToolResultResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CallToolResultResponse> for super::CallToolResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CallToolResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::CallToolResultResponse> for CallToolResultResponse {
        fn from(value: super::CallToolResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelledNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CancelledNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for CancelledNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CancelledNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelledNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelledNotification> for super::CancelledNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelledNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CancelledNotification> for CancelledNotification {
        fn from(value: super::CancelledNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelledNotificationParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        request_id: ::std::result::Result<super::RequestId, ::std::string::String>,
    }
    impl ::std::default::Default for CancelledNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                reason: Ok(Default::default()),
                request_id: Err("no value supplied for request_id".to_string()),
            }
        }
    }
    impl CancelledNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn request_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.request_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelledNotificationParams> for super::CancelledNotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelledNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                reason: value.reason?,
                request_id: value.request_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelledNotificationParams> for CancelledNotificationParams {
        fn from(value: super::CancelledNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                reason: Ok(value.reason),
                request_id: Ok(value.request_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCapabilities {
        elicitation: ::std::result::Result<
            ::std::option::Option<super::ClientCapabilitiesElicitation>,
            ::std::string::String,
        >,
        experimental: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            ::std::string::String,
        >,
        roots: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        sampling: ::std::result::Result<
            ::std::option::Option<super::ClientCapabilitiesSampling>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClientCapabilities {
        fn default() -> Self {
            Self {
                elicitation: Ok(Default::default()),
                experimental: Ok(Default::default()),
                extensions: Ok(Default::default()),
                roots: Ok(Default::default()),
                sampling: Ok(Default::default()),
            }
        }
    }
    impl ClientCapabilities {
        pub fn elicitation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientCapabilitiesElicitation>>,
            T::Error: ::std::fmt::Display,
        {
            self.elicitation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for elicitation: {e}"));
            self
        }
        pub fn experimental<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.experimental = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experimental: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn roots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.roots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roots: {e}"));
            self
        }
        pub fn sampling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientCapabilitiesSampling>>,
            T::Error: ::std::fmt::Display,
        {
            self.sampling = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCapabilities> for super::ClientCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                elicitation: value.elicitation?,
                experimental: value.experimental?,
                extensions: value.extensions?,
                roots: value.roots?,
                sampling: value.sampling?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCapabilities> for ClientCapabilities {
        fn from(value: super::ClientCapabilities) -> Self {
            Self {
                elicitation: Ok(value.elicitation),
                experimental: Ok(value.experimental),
                extensions: Ok(value.extensions),
                roots: Ok(value.roots),
                sampling: Ok(value.sampling),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCapabilitiesElicitation {
        form:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
        url: ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
    }
    impl ::std::default::Default for ClientCapabilitiesElicitation {
        fn default() -> Self {
            Self {
                form: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ClientCapabilitiesElicitation {
        pub fn form<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.form = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for form: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCapabilitiesElicitation>
        for super::ClientCapabilitiesElicitation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCapabilitiesElicitation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                form: value.form?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCapabilitiesElicitation> for ClientCapabilitiesElicitation {
        fn from(value: super::ClientCapabilitiesElicitation) -> Self {
            Self {
                form: Ok(value.form),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCapabilitiesSampling {
        context:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
        tools:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
    }
    impl ::std::default::Default for ClientCapabilitiesSampling {
        fn default() -> Self {
            Self {
                context: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl ClientCapabilitiesSampling {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCapabilitiesSampling> for super::ClientCapabilitiesSampling {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCapabilitiesSampling,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context: value.context?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCapabilitiesSampling> for ClientCapabilitiesSampling {
        fn from(value: super::ClientCapabilitiesSampling) -> Self {
            Self {
                context: Ok(value.context),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CancelledNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for ClientNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ClientNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelledNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientNotification> for super::ClientNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ClientNotification> for ClientNotification {
        fn from(value: super::ClientNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CompleteRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CompleteRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequest> for super::CompleteRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequest> for CompleteRequest {
        fn from(value: super::CompleteRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParams {
        argument:
            ::std::result::Result<super::CompleteRequestParamsArgument, ::std::string::String>,
        context: ::std::result::Result<
            ::std::option::Option<super::CompleteRequestParamsContext>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        ref_: ::std::result::Result<super::CompleteRequestParamsRef, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequestParams {
        fn default() -> Self {
            Self {
                argument: Err("no value supplied for argument".to_string()),
                context: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl CompleteRequestParams {
        pub fn argument<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParamsArgument>,
            T::Error: ::std::fmt::Display,
        {
            self.argument = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for argument: {e}"));
            self
        }
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CompleteRequestParamsContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteRequestParamsRef>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParams> for super::CompleteRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                argument: value.argument?,
                context: value.context?,
                meta: value.meta?,
                ref_: value.ref_?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParams> for CompleteRequestParams {
        fn from(value: super::CompleteRequestParams) -> Self {
            Self {
                argument: Ok(value.argument),
                context: Ok(value.context),
                meta: Ok(value.meta),
                ref_: Ok(value.ref_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParamsArgument {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteRequestParamsArgument {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl CompleteRequestParamsArgument {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParamsArgument>
        for super::CompleteRequestParamsArgument
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParamsArgument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParamsArgument> for CompleteRequestParamsArgument {
        fn from(value: super::CompleteRequestParamsArgument) -> Self {
            Self {
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteRequestParamsContext {
        arguments: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CompleteRequestParamsContext {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
            }
        }
    }
    impl CompleteRequestParamsContext {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteRequestParamsContext> for super::CompleteRequestParamsContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteRequestParamsContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteRequestParamsContext> for CompleteRequestParamsContext {
        fn from(value: super::CompleteRequestParamsContext) -> Self {
            Self {
                arguments: Ok(value.arguments),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteResult {
        completion: ::std::result::Result<super::CompleteResultCompletion, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteResult {
        fn default() -> Self {
            Self {
                completion: Err("no value supplied for completion".to_string()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl CompleteResult {
        pub fn completion<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteResultCompletion>,
            T::Error: ::std::fmt::Display,
        {
            self.completion = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteResult> for super::CompleteResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion: value.completion?,
                meta: value.meta?,
                result_type: value.result_type?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteResult> for CompleteResult {
        fn from(value: super::CompleteResult) -> Self {
            Self {
                completion: Ok(value.completion),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteResultCompletion {
        has_more: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        total: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        values:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteResultCompletion {
        fn default() -> Self {
            Self {
                has_more: Ok(Default::default()),
                total: Ok(Default::default()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl CompleteResultCompletion {
        pub fn has_more<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_more = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for has_more: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteResultCompletion> for super::CompleteResultCompletion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteResultCompletion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                has_more: value.has_more?,
                total: value.total?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteResultCompletion> for CompleteResultCompletion {
        fn from(value: super::CompleteResultCompletion) -> Self {
            Self {
                has_more: Ok(value.has_more),
                total: Ok(value.total),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompleteResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::CompleteResult, ::std::string::String>,
    }
    impl ::std::default::Default for CompleteResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl CompleteResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CompleteResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompleteResultResponse> for super::CompleteResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompleteResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::CompleteResultResponse> for CompleteResultResponse {
        fn from(value: super::CompleteResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::CreateMessageRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CreateMessageRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequest> for super::CreateMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequest> for CreateMessageRequest {
        fn from(value: super::CreateMessageRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequestParams {
        include_context: ::std::result::Result<
            ::std::option::Option<super::CreateMessageRequestParamsIncludeContext>,
            ::std::string::String,
        >,
        max_tokens: ::std::result::Result<i64, ::std::string::String>,
        messages:
            ::std::result::Result<::std::vec::Vec<super::SamplingMessage>, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
        model_preferences: ::std::result::Result<
            ::std::option::Option<super::ModelPreferences>,
            ::std::string::String,
        >,
        stop_sequences:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        system_prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_choice:
            ::std::result::Result<::std::option::Option<super::ToolChoice>, ::std::string::String>,
        tools: ::std::result::Result<::std::vec::Vec<super::Tool>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageRequestParams {
        fn default() -> Self {
            Self {
                include_context: Ok(Default::default()),
                max_tokens: Err("no value supplied for max_tokens".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                metadata: Ok(Default::default()),
                model_preferences: Ok(Default::default()),
                stop_sequences: Ok(Default::default()),
                system_prompt: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl CreateMessageRequestParams {
        pub fn include_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateMessageRequestParamsIncludeContext>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.include_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include_context: {e}"));
            self
        }
        pub fn max_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_tokens: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SamplingMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn model_preferences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModelPreferences>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_preferences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_preferences: {e}"));
            self
        }
        pub fn stop_sequences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_sequences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_sequences: {e}"));
            self
        }
        pub fn system_prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_prompt: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolChoice>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_choice: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Tool>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequestParams> for super::CreateMessageRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include_context: value.include_context?,
                max_tokens: value.max_tokens?,
                messages: value.messages?,
                metadata: value.metadata?,
                model_preferences: value.model_preferences?,
                stop_sequences: value.stop_sequences?,
                system_prompt: value.system_prompt?,
                temperature: value.temperature?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequestParams> for CreateMessageRequestParams {
        fn from(value: super::CreateMessageRequestParams) -> Self {
            Self {
                include_context: Ok(value.include_context),
                max_tokens: Ok(value.max_tokens),
                messages: Ok(value.messages),
                metadata: Ok(value.metadata),
                model_preferences: Ok(value.model_preferences),
                stop_sequences: Ok(value.stop_sequences),
                system_prompt: Ok(value.system_prompt),
                temperature: Ok(value.temperature),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageResult {
        content: ::std::result::Result<super::CreateMessageResultContent, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
        stop_reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMessageResult {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                meta: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                role: Err("no value supplied for role".to_string()),
                stop_reason: Ok(Default::default()),
            }
        }
    }
    impl CreateMessageResult {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageResultContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn stop_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageResult> for super::CreateMessageResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                meta: value.meta?,
                model: value.model?,
                role: value.role?,
                stop_reason: value.stop_reason?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageResult> for CreateMessageResult {
        fn from(value: super::CreateMessageResult) -> Self {
            Self {
                content: Ok(value.content),
                meta: Ok(value.meta),
                model: Ok(value.model),
                role: Ok(value.role),
                stop_reason: Ok(value.stop_reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscoverRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::RequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for DiscoverRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl DiscoverRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscoverRequest> for super::DiscoverRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscoverRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::DiscoverRequest> for DiscoverRequest {
        fn from(value: super::DiscoverRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscoverResult {
        cache_scope: ::std::result::Result<super::DiscoverResultCacheScope, ::std::string::String>,
        capabilities: ::std::result::Result<super::ServerCapabilities, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        supported_versions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for DiscoverResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                capabilities: Err("no value supplied for capabilities".to_string()),
                instructions: Ok(Default::default()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                supported_versions: Err("no value supplied for supported_versions".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl DiscoverResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DiscoverResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ServerCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self.capabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capabilities: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn supported_versions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_versions = value.try_into().map_err(|e| {
                format!("error converting supplied value for supported_versions: {e}")
            });
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscoverResult> for super::DiscoverResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscoverResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                capabilities: value.capabilities?,
                instructions: value.instructions?,
                meta: value.meta?,
                result_type: value.result_type?,
                supported_versions: value.supported_versions?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::DiscoverResult> for DiscoverResult {
        fn from(value: super::DiscoverResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                capabilities: Ok(value.capabilities),
                instructions: Ok(value.instructions),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
                supported_versions: Ok(value.supported_versions),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscoverResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::DiscoverResult, ::std::string::String>,
    }
    impl ::std::default::Default for DiscoverResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl DiscoverResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DiscoverResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscoverResultResponse> for super::DiscoverResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscoverResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::DiscoverResultResponse> for DiscoverResultResponse {
        fn from(value: super::DiscoverResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ElicitRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ElicitRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequest> for super::ElicitRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequest> for ElicitRequest {
        fn from(value: super::ElicitRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequestFormParams {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        mode: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        requested_schema: ::std::result::Result<
            super::ElicitRequestFormParamsRequestedSchema,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ElicitRequestFormParams {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                mode: Ok(Default::default()),
                requested_schema: Err("no value supplied for requested_schema".to_string()),
            }
        }
    }
    impl ElicitRequestFormParams {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn requested_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitRequestFormParamsRequestedSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.requested_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requested_schema: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequestFormParams> for super::ElicitRequestFormParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequestFormParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                mode: value.mode?,
                requested_schema: value.requested_schema?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequestFormParams> for ElicitRequestFormParams {
        fn from(value: super::ElicitRequestFormParams) -> Self {
            Self {
                message: Ok(value.message),
                mode: Ok(value.mode),
                requested_schema: Ok(value.requested_schema),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequestFormParamsRequestedSchema {
        properties: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::PrimitiveSchemaDefinition>,
            ::std::string::String,
        >,
        required:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        schema: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequestFormParamsRequestedSchema {
        fn default() -> Self {
            Self {
                properties: Err("no value supplied for properties".to_string()),
                required: Ok(Default::default()),
                schema: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ElicitRequestFormParamsRequestedSchema {
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::PrimitiveSchemaDefinition,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequestFormParamsRequestedSchema>
        for super::ElicitRequestFormParamsRequestedSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequestFormParamsRequestedSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                properties: value.properties?,
                required: value.required?,
                schema: value.schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequestFormParamsRequestedSchema>
        for ElicitRequestFormParamsRequestedSchema
    {
        fn from(value: super::ElicitRequestFormParamsRequestedSchema) -> Self {
            Self {
                properties: Ok(value.properties),
                required: Ok(value.required),
                schema: Ok(value.schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitRequestUrlParams {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        mode: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ElicitRequestUrlParams {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
                mode: Err("no value supplied for mode".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl ElicitRequestUrlParams {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitRequestUrlParams> for super::ElicitRequestUrlParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitRequestUrlParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                mode: value.mode?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitRequestUrlParams> for ElicitRequestUrlParams {
        fn from(value: super::ElicitRequestUrlParams) -> Self {
            Self {
                message: Ok(value.message),
                mode: Ok(value.mode),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ElicitResult {
        action: ::std::result::Result<super::ElicitResultAction, ::std::string::String>,
        content: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::ElicitResultContentValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ElicitResult {
        fn default() -> Self {
            Self {
                action: Err("no value supplied for action".to_string()),
                content: Ok(Default::default()),
            }
        }
    }
    impl ElicitResult {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ElicitResultAction>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::ElicitResultContentValue>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ElicitResult> for super::ElicitResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ElicitResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                content: value.content?,
            })
        }
    }
    impl ::std::convert::From<super::ElicitResult> for ElicitResult {
        fn from(value: super::ElicitResult) -> Self {
            Self {
                action: Ok(value.action),
                content: Ok(value.content),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmbeddedResource {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        resource: ::std::result::Result<super::EmbeddedResourceResource, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EmbeddedResource {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                meta: Ok(Default::default()),
                resource: Err("no value supplied for resource".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EmbeddedResource {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EmbeddedResourceResource>,
            T::Error: ::std::fmt::Display,
        {
            self.resource = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmbeddedResource> for super::EmbeddedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmbeddedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                meta: value.meta?,
                resource: value.resource?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EmbeddedResource> for EmbeddedResource {
        fn from(value: super::EmbeddedResource) -> Self {
            Self {
                annotations: Ok(value.annotations),
                meta: Ok(value.meta),
                resource: Ok(value.resource),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnumSchema {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::UntitledSingleSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::TitledSingleSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::UntitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::TitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::LegacyTitledEnumSchema>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EnumSchema {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
            }
        }
    }
    impl EnumSchema {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UntitledSingleSelectEnumSchema>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TitledSingleSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UntitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {e}"));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LegacyTitledEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EnumSchema> for super::EnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
            })
        }
    }
    impl ::std::convert::From<super::EnumSchema> for EnumSchema {
        fn from(value: super::EnumSchema) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Error {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Error {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl Error {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Error> for super::Error {
        type Error = super::error::ConversionError;
        fn try_from(value: Error) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::Error> for Error {
        fn from(value: super::Error) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::GetPromptRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for GetPromptRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl GetPromptRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetPromptRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptRequest> for super::GetPromptRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptRequest> for GetPromptRequest {
        fn from(value: super::GetPromptRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptRequestParams {
        arguments: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        input_responses: ::std::result::Result<
            ::std::option::Option<super::InputResponses>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_state: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetPromptRequestParams {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                input_responses: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
                name: Err("no value supplied for name".to_string()),
                request_state: Ok(Default::default()),
            }
        }
    }
    impl GetPromptRequestParams {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn input_responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputResponses>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_responses: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn request_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptRequestParams> for super::GetPromptRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                input_responses: value.input_responses?,
                meta: value.meta?,
                name: value.name?,
                request_state: value.request_state?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptRequestParams> for GetPromptRequestParams {
        fn from(value: super::GetPromptRequestParams) -> Self {
            Self {
                arguments: Ok(value.arguments),
                input_responses: Ok(value.input_responses),
                meta: Ok(value.meta),
                name: Ok(value.name),
                request_state: Ok(value.request_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptResult {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        messages:
            ::std::result::Result<::std::vec::Vec<super::PromptMessage>, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetPromptResult {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                messages: Err("no value supplied for messages".to_string()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl GetPromptResult {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PromptMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptResult> for super::GetPromptResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                messages: value.messages?,
                meta: value.meta?,
                result_type: value.result_type?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptResult> for GetPromptResult {
        fn from(value: super::GetPromptResult) -> Self {
            Self {
                description: Ok(value.description),
                messages: Ok(value.messages),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetPromptResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::GetPromptResultResponseResult, ::std::string::String>,
    }
    impl ::std::default::Default for GetPromptResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl GetPromptResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetPromptResultResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetPromptResultResponse> for super::GetPromptResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetPromptResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::GetPromptResultResponse> for GetPromptResultResponse {
        fn from(value: super::GetPromptResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HeaderMismatchError {
        error: ::std::result::Result<super::HeaderMismatchErrorError, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::RequestId>, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for HeaderMismatchError {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl HeaderMismatchError {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HeaderMismatchErrorError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HeaderMismatchError> for super::HeaderMismatchError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HeaderMismatchError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::HeaderMismatchError> for HeaderMismatchError {
        fn from(value: super::HeaderMismatchError) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HeaderMismatchErrorError {
        code: ::std::result::Result<super::HeaderMismatchErrorErrorCode, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for HeaderMismatchErrorError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl HeaderMismatchErrorError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HeaderMismatchErrorErrorCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HeaderMismatchErrorError> for super::HeaderMismatchErrorError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HeaderMismatchErrorError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::HeaderMismatchErrorError> for HeaderMismatchErrorError {
        fn from(value: super::HeaderMismatchErrorError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Icon {
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sizes: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        theme:
            ::std::result::Result<::std::option::Option<super::IconTheme>, ::std::string::String>,
    }
    impl ::std::default::Default for Icon {
        fn default() -> Self {
            Self {
                mime_type: Ok(Default::default()),
                sizes: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                theme: Ok(Default::default()),
            }
        }
    }
    impl Icon {
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn sizes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sizes: {e}"));
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {e}"));
            self
        }
        pub fn theme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IconTheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.theme = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for theme: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Icon> for super::Icon {
        type Error = super::error::ConversionError;
        fn try_from(value: Icon) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mime_type: value.mime_type?,
                sizes: value.sizes?,
                src: value.src?,
                theme: value.theme?,
            })
        }
    }
    impl ::std::convert::From<super::Icon> for Icon {
        fn from(value: super::Icon) -> Self {
            Self {
                mime_type: Ok(value.mime_type),
                sizes: Ok(value.sizes),
                src: Ok(value.src),
                theme: Ok(value.theme),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Icons {
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
    }
    impl ::std::default::Default for Icons {
        fn default() -> Self {
            Self {
                icons: Ok(Default::default()),
            }
        }
    }
    impl Icons {
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Icons> for super::Icons {
        type Error = super::error::ConversionError;
        fn try_from(value: Icons) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                icons: value.icons?,
            })
        }
    }
    impl ::std::convert::From<super::Icons> for Icons {
        fn from(value: super::Icons) -> Self {
            Self {
                icons: Ok(value.icons),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ImageContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                data: Err("no value supplied for data".to_string()),
                meta: Ok(Default::default()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ImageContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ImageContent> for super::ImageContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                data: value.data?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ImageContent> for ImageContent {
        fn from(value: super::ImageContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                data: Ok(value.data),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Implementation {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
        website_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Implementation {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
                website_url: Ok(Default::default()),
            }
        }
    }
    impl Implementation {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
        pub fn website_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.website_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for website_url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Implementation> for super::Implementation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Implementation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                icons: value.icons?,
                name: value.name?,
                title: value.title?,
                version: value.version?,
                website_url: value.website_url?,
            })
        }
    }
    impl ::std::convert::From<super::Implementation> for Implementation {
        fn from(value: super::Implementation) -> Self {
            Self {
                description: Ok(value.description),
                icons: Ok(value.icons),
                name: Ok(value.name),
                title: Ok(value.title),
                version: Ok(value.version),
                website_url: Ok(value.website_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputRequiredResult {
        input_requests: ::std::result::Result<
            ::std::option::Option<super::InputRequests>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        request_state: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InputRequiredResult {
        fn default() -> Self {
            Self {
                input_requests: Ok(Default::default()),
                meta: Ok(Default::default()),
                request_state: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl InputRequiredResult {
        pub fn input_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputRequests>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_requests = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_requests: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn request_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_state: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputRequiredResult> for super::InputRequiredResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputRequiredResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_requests: value.input_requests?,
                meta: value.meta?,
                request_state: value.request_state?,
                result_type: value.result_type?,
            })
        }
    }
    impl ::std::convert::From<super::InputRequiredResult> for InputRequiredResult {
        fn from(value: super::InputRequiredResult) -> Self {
            Self {
                input_requests: Ok(value.input_requests),
                meta: Ok(value.meta),
                request_state: Ok(value.request_state),
                result_type: Ok(value.result_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputResponseRequestParams {
        input_responses: ::std::result::Result<
            ::std::option::Option<super::InputResponses>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        request_state: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InputResponseRequestParams {
        fn default() -> Self {
            Self {
                input_responses: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
                request_state: Ok(Default::default()),
            }
        }
    }
    impl InputResponseRequestParams {
        pub fn input_responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputResponses>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_responses: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn request_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputResponseRequestParams> for super::InputResponseRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputResponseRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_responses: value.input_responses?,
                meta: value.meta?,
                request_state: value.request_state?,
            })
        }
    }
    impl ::std::convert::From<super::InputResponseRequestParams> for InputResponseRequestParams {
        fn from(value: super::InputResponseRequestParams) -> Self {
            Self {
                input_responses: Ok(value.input_responses),
                meta: Ok(value.meta),
                request_state: Ok(value.request_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InternalError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InternalError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InternalError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InternalError> for super::InternalError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InternalError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InternalError> for InternalError {
        fn from(value: super::InternalError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvalidParamsError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InvalidParamsError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InvalidParamsError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InvalidParamsError> for super::InvalidParamsError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvalidParamsError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InvalidParamsError> for InvalidParamsError {
        fn from(value: super::InvalidParamsError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvalidRequestError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InvalidRequestError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InvalidRequestError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InvalidRequestError> for super::InvalidRequestError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvalidRequestError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InvalidRequestError> for InvalidRequestError {
        fn from(value: super::InvalidRequestError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcErrorResponse {
        error: ::std::result::Result<super::Error, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::RequestId>, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcErrorResponse {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl JsonrpcErrorResponse {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcErrorResponse> for super::JsonrpcErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcErrorResponse> for JsonrpcErrorResponse {
        fn from(value: super::JsonrpcErrorResponse) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcNotification> for super::JsonrpcNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcNotification> for JsonrpcNotification {
        fn from(value: super::JsonrpcNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcRequest> for super::JsonrpcRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcRequest> for JsonrpcRequest {
        fn from(value: super::JsonrpcRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::Result, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl JsonrpcResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Result>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcResultResponse> for super::JsonrpcResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcResultResponse> for JsonrpcResultResponse {
        fn from(value: super::JsonrpcResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LegacyTitledEnumSchema {
        default: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        enum_names:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LegacyTitledEnumSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                enum_: Err("no value supplied for enum_".to_string()),
                enum_names: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LegacyTitledEnumSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {e}"));
            self
        }
        pub fn enum_names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_names: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LegacyTitledEnumSchema> for super::LegacyTitledEnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LegacyTitledEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                enum_: value.enum_?,
                enum_names: value.enum_names?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LegacyTitledEnumSchema> for LegacyTitledEnumSchema {
        fn from(value: super::LegacyTitledEnumSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                enum_: Ok(value.enum_),
                enum_names: Ok(value.enum_names),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::PaginatedRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ListPromptsRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ListPromptsRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaginatedRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsRequest> for super::ListPromptsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsRequest> for ListPromptsRequest {
        fn from(value: super::ListPromptsRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsResult {
        cache_scope:
            ::std::result::Result<super::ListPromptsResultCacheScope, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompts: ::std::result::Result<::std::vec::Vec<super::Prompt>, ::std::string::String>,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ListPromptsResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                prompts: Err("no value supplied for prompts".to_string()),
                result_type: Err("no value supplied for result_type".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl ListPromptsResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListPromptsResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn prompts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Prompt>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompts: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsResult> for super::ListPromptsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                prompts: value.prompts?,
                result_type: value.result_type?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsResult> for ListPromptsResult {
        fn from(value: super::ListPromptsResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                prompts: Ok(value.prompts),
                result_type: Ok(value.result_type),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPromptsResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::ListPromptsResult, ::std::string::String>,
    }
    impl ::std::default::Default for ListPromptsResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ListPromptsResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListPromptsResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPromptsResultResponse> for super::ListPromptsResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPromptsResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ListPromptsResultResponse> for ListPromptsResultResponse {
        fn from(value: super::ListPromptsResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::PaginatedRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourceTemplatesRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ListResourceTemplatesRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaginatedRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesRequest> for super::ListResourceTemplatesRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesRequest> for ListResourceTemplatesRequest {
        fn from(value: super::ListResourceTemplatesRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesResult {
        cache_scope: ::std::result::Result<
            super::ListResourceTemplatesResultCacheScope,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_templates:
            ::std::result::Result<::std::vec::Vec<super::ResourceTemplate>, ::std::string::String>,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourceTemplatesResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                resource_templates: Err("no value supplied for resource_templates".to_string()),
                result_type: Err("no value supplied for result_type".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl ListResourceTemplatesResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListResourceTemplatesResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn resource_templates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResourceTemplate>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_templates = value.try_into().map_err(|e| {
                format!("error converting supplied value for resource_templates: {e}")
            });
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesResult> for super::ListResourceTemplatesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                resource_templates: value.resource_templates?,
                result_type: value.result_type?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesResult> for ListResourceTemplatesResult {
        fn from(value: super::ListResourceTemplatesResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                resource_templates: Ok(value.resource_templates),
                result_type: Ok(value.result_type),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourceTemplatesResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::ListResourceTemplatesResult, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourceTemplatesResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ListResourceTemplatesResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListResourceTemplatesResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourceTemplatesResultResponse>
        for super::ListResourceTemplatesResultResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourceTemplatesResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourceTemplatesResultResponse>
        for ListResourceTemplatesResultResponse
    {
        fn from(value: super::ListResourceTemplatesResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::PaginatedRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourcesRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ListResourcesRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaginatedRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesRequest> for super::ListResourcesRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesRequest> for ListResourcesRequest {
        fn from(value: super::ListResourcesRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesResult {
        cache_scope:
            ::std::result::Result<super::ListResourcesResultCacheScope, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resources: ::std::result::Result<::std::vec::Vec<super::Resource>, ::std::string::String>,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourcesResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                resources: Err("no value supplied for resources".to_string()),
                result_type: Err("no value supplied for result_type".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl ListResourcesResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListResourcesResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Resource>>,
            T::Error: ::std::fmt::Display,
        {
            self.resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resources: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesResult> for super::ListResourcesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                resources: value.resources?,
                result_type: value.result_type?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesResult> for ListResourcesResult {
        fn from(value: super::ListResourcesResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                resources: Ok(value.resources),
                result_type: Ok(value.result_type),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListResourcesResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::ListResourcesResult, ::std::string::String>,
    }
    impl ::std::default::Default for ListResourcesResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ListResourcesResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListResourcesResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListResourcesResultResponse> for super::ListResourcesResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListResourcesResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ListResourcesResultResponse> for ListResourcesResultResponse {
        fn from(value: super::ListResourcesResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsRequest {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::ListRootsRequestParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListRootsRequest {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ListRootsRequest {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListRootsRequestParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsRequest> for super::ListRootsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsRequest> for ListRootsRequest {
        fn from(value: super::ListRootsRequest) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsRequestParams {
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
    }
    impl ::std::default::Default for ListRootsRequestParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
            }
        }
    }
    impl ListRootsRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsRequestParams> for super::ListRootsRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { meta: value.meta? })
        }
    }
    impl ::std::convert::From<super::ListRootsRequestParams> for ListRootsRequestParams {
        fn from(value: super::ListRootsRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRootsResult {
        roots: ::std::result::Result<::std::vec::Vec<super::Root>, ::std::string::String>,
    }
    impl ::std::default::Default for ListRootsResult {
        fn default() -> Self {
            Self {
                roots: Err("no value supplied for roots".to_string()),
            }
        }
    }
    impl ListRootsResult {
        pub fn roots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Root>>,
            T::Error: ::std::fmt::Display,
        {
            self.roots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roots: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRootsResult> for super::ListRootsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRootsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                roots: value.roots?,
            })
        }
    }
    impl ::std::convert::From<super::ListRootsResult> for ListRootsResult {
        fn from(value: super::ListRootsResult) -> Self {
            Self {
                roots: Ok(value.roots),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::PaginatedRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ListToolsRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ListToolsRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaginatedRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsRequest> for super::ListToolsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsRequest> for ListToolsRequest {
        fn from(value: super::ListToolsRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsResult {
        cache_scope: ::std::result::Result<super::ListToolsResultCacheScope, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        tools: ::std::result::Result<::std::vec::Vec<super::Tool>, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ListToolsResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                tools: Err("no value supplied for tools".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl ListToolsResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListToolsResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Tool>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsResult> for super::ListToolsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                result_type: value.result_type?,
                tools: value.tools?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsResult> for ListToolsResult {
        fn from(value: super::ListToolsResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                result_type: Ok(value.result_type),
                tools: Ok(value.tools),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListToolsResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::ListToolsResult, ::std::string::String>,
    }
    impl ::std::default::Default for ListToolsResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ListToolsResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListToolsResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListToolsResultResponse> for super::ListToolsResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListToolsResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ListToolsResultResponse> for ListToolsResultResponse {
        fn from(value: super::ListToolsResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LoggingMessageNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params:
            ::std::result::Result<super::LoggingMessageNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for LoggingMessageNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl LoggingMessageNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LoggingMessageNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LoggingMessageNotification> for super::LoggingMessageNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LoggingMessageNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::LoggingMessageNotification> for LoggingMessageNotification {
        fn from(value: super::LoggingMessageNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LoggingMessageNotificationParams {
        data: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        level: ::std::result::Result<super::LoggingLevel, ::std::string::String>,
        logger: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LoggingMessageNotificationParams {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                level: Err("no value supplied for level".to_string()),
                logger: Ok(Default::default()),
                meta: Ok(Default::default()),
            }
        }
    }
    impl LoggingMessageNotificationParams {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LoggingLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {e}"));
            self
        }
        pub fn logger<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logger: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LoggingMessageNotificationParams>
        for super::LoggingMessageNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LoggingMessageNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                level: value.level?,
                logger: value.logger?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::LoggingMessageNotificationParams>
        for LoggingMessageNotificationParams
    {
        fn from(value: super::LoggingMessageNotificationParams) -> Self {
            Self {
                data: Ok(value.data),
                level: Ok(value.level),
                logger: Ok(value.logger),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MethodNotFoundError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MethodNotFoundError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl MethodNotFoundError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MethodNotFoundError> for super::MethodNotFoundError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MethodNotFoundError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::MethodNotFoundError> for MethodNotFoundError {
        fn from(value: super::MethodNotFoundError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MissingRequiredClientCapabilityError {
        error: ::std::result::Result<
            super::MissingRequiredClientCapabilityErrorError,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::RequestId>, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MissingRequiredClientCapabilityError {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl MissingRequiredClientCapabilityError {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MissingRequiredClientCapabilityErrorError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MissingRequiredClientCapabilityError>
        for super::MissingRequiredClientCapabilityError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MissingRequiredClientCapabilityError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::MissingRequiredClientCapabilityError>
        for MissingRequiredClientCapabilityError
    {
        fn from(value: super::MissingRequiredClientCapabilityError) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MissingRequiredClientCapabilityErrorError {
        code: ::std::result::Result<
            super::MissingRequiredClientCapabilityErrorErrorCode,
            ::std::string::String,
        >,
        data: ::std::result::Result<
            super::MissingRequiredClientCapabilityErrorErrorData,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MissingRequiredClientCapabilityErrorError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Err("no value supplied for data".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl MissingRequiredClientCapabilityErrorError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MissingRequiredClientCapabilityErrorErrorCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MissingRequiredClientCapabilityErrorErrorData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MissingRequiredClientCapabilityErrorError>
        for super::MissingRequiredClientCapabilityErrorError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MissingRequiredClientCapabilityErrorError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::MissingRequiredClientCapabilityErrorError>
        for MissingRequiredClientCapabilityErrorError
    {
        fn from(value: super::MissingRequiredClientCapabilityErrorError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MissingRequiredClientCapabilityErrorErrorData {
        required_capabilities:
            ::std::result::Result<super::ClientCapabilities, ::std::string::String>,
    }
    impl ::std::default::Default for MissingRequiredClientCapabilityErrorErrorData {
        fn default() -> Self {
            Self {
                required_capabilities: Err(
                    "no value supplied for required_capabilities".to_string()
                ),
            }
        }
    }
    impl MissingRequiredClientCapabilityErrorErrorData {
        pub fn required_capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClientCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self.required_capabilities = value.try_into().map_err(|e| {
                format!("error converting supplied value for required_capabilities: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<MissingRequiredClientCapabilityErrorErrorData>
        for super::MissingRequiredClientCapabilityErrorErrorData
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MissingRequiredClientCapabilityErrorErrorData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                required_capabilities: value.required_capabilities?,
            })
        }
    }
    impl ::std::convert::From<super::MissingRequiredClientCapabilityErrorErrorData>
        for MissingRequiredClientCapabilityErrorErrorData
    {
        fn from(value: super::MissingRequiredClientCapabilityErrorErrorData) -> Self {
            Self {
                required_capabilities: Ok(value.required_capabilities),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelHint {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModelHint {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
            }
        }
    }
    impl ModelHint {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModelHint> for super::ModelHint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelHint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ModelHint> for ModelHint {
        fn from(value: super::ModelHint) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelPreferences {
        cost_priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        hints: ::std::result::Result<::std::vec::Vec<super::ModelHint>, ::std::string::String>,
        intelligence_priority:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        speed_priority: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for ModelPreferences {
        fn default() -> Self {
            Self {
                cost_priority: Ok(Default::default()),
                hints: Ok(Default::default()),
                intelligence_priority: Ok(Default::default()),
                speed_priority: Ok(Default::default()),
            }
        }
    }
    impl ModelPreferences {
        pub fn cost_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cost_priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cost_priority: {e}"));
            self
        }
        pub fn hints<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ModelHint>>,
            T::Error: ::std::fmt::Display,
        {
            self.hints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hints: {e}"));
            self
        }
        pub fn intelligence_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.intelligence_priority = value.try_into().map_err(|e| {
                format!("error converting supplied value for intelligence_priority: {e}")
            });
            self
        }
        pub fn speed_priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed_priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed_priority: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModelPreferences> for super::ModelPreferences {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelPreferences,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cost_priority: value.cost_priority?,
                hints: value.hints?,
                intelligence_priority: value.intelligence_priority?,
                speed_priority: value.speed_priority?,
            })
        }
    }
    impl ::std::convert::From<super::ModelPreferences> for ModelPreferences {
        fn from(value: super::ModelPreferences) -> Self {
            Self {
                cost_priority: Ok(value.cost_priority),
                hints: Ok(value.hints),
                intelligence_priority: Ok(value.intelligence_priority),
                speed_priority: Ok(value.speed_priority),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MultiSelectEnumSchema {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::UntitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::TitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MultiSelectEnumSchema {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl MultiSelectEnumSchema {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UntitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MultiSelectEnumSchema> for super::MultiSelectEnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MultiSelectEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::MultiSelectEnumSchema> for MultiSelectEnumSchema {
        fn from(value: super::MultiSelectEnumSchema) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Notification {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Notification {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl Notification {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Notification> for super::Notification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Notification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::Notification> for Notification {
        fn from(value: super::Notification) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotificationMetaObject {
        io_modelcontextprotocol_subscription_id:
            ::std::result::Result<::std::option::Option<super::RequestId>, ::std::string::String>,
    }
    impl ::std::default::Default for NotificationMetaObject {
        fn default() -> Self {
            Self {
                io_modelcontextprotocol_subscription_id: Ok(Default::default()),
            }
        }
    }
    impl NotificationMetaObject {
        pub fn io_modelcontextprotocol_subscription_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self . io_modelcontextprotocol_subscription_id = value . try_into () . map_err (| e | format ! ("error converting supplied value for io_modelcontextprotocol_subscription_id: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<NotificationMetaObject> for super::NotificationMetaObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotificationMetaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                io_modelcontextprotocol_subscription_id: value
                    .io_modelcontextprotocol_subscription_id?,
            })
        }
    }
    impl ::std::convert::From<super::NotificationMetaObject> for NotificationMetaObject {
        fn from(value: super::NotificationMetaObject) -> Self {
            Self {
                io_modelcontextprotocol_subscription_id: Ok(
                    value.io_modelcontextprotocol_subscription_id
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotificationParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
            }
        }
    }
    impl NotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NotificationParams> for super::NotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { meta: value.meta? })
        }
    }
    impl ::std::convert::From<super::NotificationParams> for NotificationParams {
        fn from(value: super::NotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NumberSchema {
        default: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        maximum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        minimum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::NumberSchemaType, ::std::string::String>,
    }
    impl ::std::default::Default for NumberSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                maximum: Ok(Default::default()),
                minimum: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl NumberSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum: {e}"));
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minimum: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NumberSchemaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NumberSchema> for super::NumberSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NumberSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                maximum: value.maximum?,
                minimum: value.minimum?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::NumberSchema> for NumberSchema {
        fn from(value: super::NumberSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                maximum: Ok(value.maximum),
                minimum: Ok(value.minimum),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::PaginatedRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for PaginatedRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl PaginatedRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaginatedRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedRequest> for super::PaginatedRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedRequest> for PaginatedRequest {
        fn from(value: super::PaginatedRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedRequestParams {
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
    }
    impl ::std::default::Default for PaginatedRequestParams {
        fn default() -> Self {
            Self {
                cursor: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
            }
        }
    }
    impl PaginatedRequestParams {
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedRequestParams> for super::PaginatedRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cursor: value.cursor?,
                meta: value.meta?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedRequestParams> for PaginatedRequestParams {
        fn from(value: super::PaginatedRequestParams) -> Self {
            Self {
                cursor: Ok(value.cursor),
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaginatedResult {
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        next_cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaginatedResult {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                next_cursor: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl PaginatedResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn next_cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_cursor: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaginatedResult> for super::PaginatedResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaginatedResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                next_cursor: value.next_cursor?,
                result_type: value.result_type?,
            })
        }
    }
    impl ::std::convert::From<super::PaginatedResult> for PaginatedResult {
        fn from(value: super::PaginatedResult) -> Self {
            Self {
                meta: Ok(value.meta),
                next_cursor: Ok(value.next_cursor),
                result_type: Ok(value.result_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ParseError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ParseError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl ParseError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ParseError> for super::ParseError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ParseError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::ParseError> for ParseError {
        fn from(value: super::ParseError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PrimitiveSchemaDefinition {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::StringSchema>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::NumberSchema>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::BooleanSchema>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::UntitledSingleSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::TitledSingleSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_5: ::std::result::Result<
            ::std::option::Option<super::UntitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_6: ::std::result::Result<
            ::std::option::Option<super::TitledMultiSelectEnumSchema>,
            ::std::string::String,
        >,
        subtype_7: ::std::result::Result<
            ::std::option::Option<super::LegacyTitledEnumSchema>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PrimitiveSchemaDefinition {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
            }
        }
    }
    impl PrimitiveSchemaDefinition {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NumberSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BooleanSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UntitledSingleSelectEnumSchema>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {e}"));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TitledSingleSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {e}"));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UntitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {e}"));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TitledMultiSelectEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {e}"));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LegacyTitledEnumSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PrimitiveSchemaDefinition> for super::PrimitiveSchemaDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PrimitiveSchemaDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
            })
        }
    }
    impl ::std::convert::From<super::PrimitiveSchemaDefinition> for PrimitiveSchemaDefinition {
        fn from(value: super::PrimitiveSchemaDefinition) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProgressNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ProgressNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for ProgressNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ProgressNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProgressNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProgressNotification> for super::ProgressNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProgressNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ProgressNotification> for ProgressNotification {
        fn from(value: super::ProgressNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProgressNotificationParams {
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
        progress: ::std::result::Result<f64, ::std::string::String>,
        progress_token: ::std::result::Result<super::ProgressToken, ::std::string::String>,
        total: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for ProgressNotificationParams {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
                meta: Ok(Default::default()),
                progress: Err("no value supplied for progress".to_string()),
                progress_token: Err("no value supplied for progress_token".to_string()),
                total: Ok(Default::default()),
            }
        }
    }
    impl ProgressNotificationParams {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn progress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress: {e}"));
            self
        }
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProgressToken>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProgressNotificationParams> for super::ProgressNotificationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProgressNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                meta: value.meta?,
                progress: value.progress?,
                progress_token: value.progress_token?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::ProgressNotificationParams> for ProgressNotificationParams {
        fn from(value: super::ProgressNotificationParams) -> Self {
            Self {
                message: Ok(value.message),
                meta: Ok(value.meta),
                progress: Ok(value.progress),
                progress_token: Ok(value.progress_token),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Prompt {
        arguments:
            ::std::result::Result<::std::vec::Vec<super::PromptArgument>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Prompt {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                meta: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Prompt {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PromptArgument>>,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Prompt> for super::Prompt {
        type Error = super::error::ConversionError;
        fn try_from(value: Prompt) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                description: value.description?,
                icons: value.icons?,
                meta: value.meta?,
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Prompt> for Prompt {
        fn from(value: super::Prompt) -> Self {
            Self {
                arguments: Ok(value.arguments),
                description: Ok(value.description),
                icons: Ok(value.icons),
                meta: Ok(value.meta),
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptArgument {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        required: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptArgument {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                required: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl PromptArgument {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptArgument> for super::PromptArgument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptArgument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                required: value.required?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::PromptArgument> for PromptArgument {
        fn from(value: super::PromptArgument) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                required: Ok(value.required),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptListChangedNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::NotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptListChangedNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl PromptListChangedNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptListChangedNotification>
        for super::PromptListChangedNotification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::PromptListChangedNotification> for PromptListChangedNotification {
        fn from(value: super::PromptListChangedNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptMessage {
        content: ::std::result::Result<super::ContentBlock, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
    }
    impl ::std::default::Default for PromptMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl PromptMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContentBlock>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptMessage> for super::PromptMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::PromptMessage> for PromptMessage {
        fn from(value: super::PromptMessage) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptReference {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PromptReference {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PromptReference {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptReference> for super::PromptReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PromptReference> for PromptReference {
        fn from(value: super::PromptReference) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::ReadResourceRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ReadResourceRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadResourceRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceRequest> for super::ReadResourceRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceRequest> for ReadResourceRequest {
        fn from(value: super::ReadResourceRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceRequestParams {
        input_responses: ::std::result::Result<
            ::std::option::Option<super::InputResponses>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        request_state: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceRequestParams {
        fn default() -> Self {
            Self {
                input_responses: Ok(Default::default()),
                meta: Err("no value supplied for meta".to_string()),
                request_state: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ReadResourceRequestParams {
        pub fn input_responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputResponses>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_responses: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn request_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_state: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceRequestParams> for super::ReadResourceRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_responses: value.input_responses?,
                meta: value.meta?,
                request_state: value.request_state?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceRequestParams> for ReadResourceRequestParams {
        fn from(value: super::ReadResourceRequestParams) -> Self {
            Self {
                input_responses: Ok(value.input_responses),
                meta: Ok(value.meta),
                request_state: Ok(value.request_state),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceResult {
        cache_scope:
            ::std::result::Result<super::ReadResourceResultCacheScope, ::std::string::String>,
        contents: ::std::result::Result<
            ::std::vec::Vec<super::ReadResourceResultContentsItem>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        ttl_ms: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceResult {
        fn default() -> Self {
            Self {
                cache_scope: Err("no value supplied for cache_scope".to_string()),
                contents: Err("no value supplied for contents".to_string()),
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                ttl_ms: Err("no value supplied for ttl_ms".to_string()),
            }
        }
    }
    impl ReadResourceResult {
        pub fn cache_scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadResourceResultCacheScope>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cache_scope: {e}"));
            self
        }
        pub fn contents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ReadResourceResultContentsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.contents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contents: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceResult> for super::ReadResourceResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_scope: value.cache_scope?,
                contents: value.contents?,
                meta: value.meta?,
                result_type: value.result_type?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceResult> for ReadResourceResult {
        fn from(value: super::ReadResourceResult) -> Self {
            Self {
                cache_scope: Ok(value.cache_scope),
                contents: Ok(value.contents),
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadResourceResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result:
            ::std::result::Result<super::ReadResourceResultResponseResult, ::std::string::String>,
    }
    impl ::std::default::Default for ReadResourceResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ReadResourceResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadResourceResultResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReadResourceResultResponse> for super::ReadResourceResultResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadResourceResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ReadResourceResultResponse> for ReadResourceResultResponse {
        fn from(value: super::ReadResourceResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Request {
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Request {
        fn default() -> Self {
            Self {
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl Request {
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Request> for super::Request {
        type Error = super::error::ConversionError;
        fn try_from(value: Request) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::Request> for Request {
        fn from(value: super::Request) -> Self {
            Self {
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestMetaObject {
        io_modelcontextprotocol_client_capabilities:
            ::std::result::Result<super::ClientCapabilities, ::std::string::String>,
        io_modelcontextprotocol_client_info: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
        io_modelcontextprotocol_log_level: ::std::result::Result<
            ::std::option::Option<super::LoggingLevel>,
            ::std::string::String,
        >,
        io_modelcontextprotocol_protocol_version:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        progress_token: ::std::result::Result<
            ::std::option::Option<super::ProgressToken>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RequestMetaObject {
        fn default() -> Self {
            Self {
                io_modelcontextprotocol_client_capabilities: Err(
                    "no value supplied for io_modelcontextprotocol_client_capabilities".to_string(),
                ),
                io_modelcontextprotocol_client_info: Ok(Default::default()),
                io_modelcontextprotocol_log_level: Ok(Default::default()),
                io_modelcontextprotocol_protocol_version: Err(
                    "no value supplied for io_modelcontextprotocol_protocol_version".to_string(),
                ),
                progress_token: Ok(Default::default()),
            }
        }
    }
    impl RequestMetaObject {
        pub fn io_modelcontextprotocol_client_capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClientCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self . io_modelcontextprotocol_client_capabilities = value . try_into () . map_err (| e | format ! ("error converting supplied value for io_modelcontextprotocol_client_capabilities: {e}")) ;
            self
        }
        pub fn io_modelcontextprotocol_client_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.io_modelcontextprotocol_client_info = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for io_modelcontextprotocol_client_info: {e}"
                )
            });
            self
        }
        pub fn io_modelcontextprotocol_log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LoggingLevel>>,
            T::Error: ::std::fmt::Display,
        {
            self.io_modelcontextprotocol_log_level = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for io_modelcontextprotocol_log_level: {e}"
                )
            });
            self
        }
        pub fn io_modelcontextprotocol_protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self . io_modelcontextprotocol_protocol_version = value . try_into () . map_err (| e | format ! ("error converting supplied value for io_modelcontextprotocol_protocol_version: {e}")) ;
            self
        }
        pub fn progress_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProgressToken>>,
            T::Error: ::std::fmt::Display,
        {
            self.progress_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress_token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestMetaObject> for super::RequestMetaObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestMetaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                io_modelcontextprotocol_client_capabilities: value
                    .io_modelcontextprotocol_client_capabilities?,
                io_modelcontextprotocol_client_info: value.io_modelcontextprotocol_client_info?,
                io_modelcontextprotocol_log_level: value.io_modelcontextprotocol_log_level?,
                io_modelcontextprotocol_protocol_version: value
                    .io_modelcontextprotocol_protocol_version?,
                progress_token: value.progress_token?,
            })
        }
    }
    impl ::std::convert::From<super::RequestMetaObject> for RequestMetaObject {
        fn from(value: super::RequestMetaObject) -> Self {
            Self {
                io_modelcontextprotocol_client_capabilities: Ok(
                    value.io_modelcontextprotocol_client_capabilities
                ),
                io_modelcontextprotocol_client_info: Ok(value.io_modelcontextprotocol_client_info),
                io_modelcontextprotocol_log_level: Ok(value.io_modelcontextprotocol_log_level),
                io_modelcontextprotocol_protocol_version: Ok(
                    value.io_modelcontextprotocol_protocol_version
                ),
                progress_token: Ok(value.progress_token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestParams {
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
    }
    impl ::std::default::Default for RequestParams {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
            }
        }
    }
    impl RequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestParams> for super::RequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { meta: value.meta? })
        }
    }
    impl ::std::convert::From<super::RequestParams> for RequestParams {
        fn from(value: super::RequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Resource {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Resource {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                size: Ok(Default::default()),
                title: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl Resource {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Resource> for super::Resource {
        type Error = super::error::ConversionError;
        fn try_from(value: Resource) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                icons: value.icons?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                size: value.size?,
                title: value.title?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::Resource> for Resource {
        fn from(value: super::Resource) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                icons: Ok(value.icons),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                size: Ok(value.size),
                title: Ok(value.title),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceContents {
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceContents {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceContents {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceContents> for super::ResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                mime_type: value.mime_type?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceContents> for ResourceContents {
        fn from(value: super::ResourceContents) -> Self {
            Self {
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceLink {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceLink {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                size: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceLink {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceLink> for super::ResourceLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                icons: value.icons?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                size: value.size?,
                title: value.title?,
                type_: value.type_?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceLink> for ResourceLink {
        fn from(value: super::ResourceLink) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                icons: Ok(value.icons),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                size: Ok(value.size),
                title: Ok(value.title),
                type_: Ok(value.type_),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceListChangedNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::NotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResourceListChangedNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ResourceListChangedNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceListChangedNotification>
        for super::ResourceListChangedNotification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceListChangedNotification>
        for ResourceListChangedNotification
    {
        fn from(value: super::ResourceListChangedNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceRequestParams {
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceRequestParams {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceRequestParams> for super::ResourceRequestParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceRequestParams> for ResourceRequestParams {
        fn from(value: super::ResourceRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceTemplate {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri_template: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceTemplate {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                title: Ok(Default::default()),
                uri_template: Err("no value supplied for uri_template".to_string()),
            }
        }
    }
    impl ResourceTemplate {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn uri_template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri_template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri_template: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceTemplate> for super::ResourceTemplate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceTemplate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                icons: value.icons?,
                meta: value.meta?,
                mime_type: value.mime_type?,
                name: value.name?,
                title: value.title?,
                uri_template: value.uri_template?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceTemplate> for ResourceTemplate {
        fn from(value: super::ResourceTemplate) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                icons: Ok(value.icons),
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                title: Ok(value.title),
                uri_template: Ok(value.uri_template),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceTemplateReference {
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceTemplateReference {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceTemplateReference {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceTemplateReference> for super::ResourceTemplateReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceTemplateReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceTemplateReference> for ResourceTemplateReference {
        fn from(value: super::ResourceTemplateReference) -> Self {
            Self {
                type_: Ok(value.type_),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceUpdatedNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params:
            ::std::result::Result<super::ResourceUpdatedNotificationParams, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceUpdatedNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ResourceUpdatedNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResourceUpdatedNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceUpdatedNotification> for super::ResourceUpdatedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceUpdatedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceUpdatedNotification> for ResourceUpdatedNotification {
        fn from(value: super::ResourceUpdatedNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceUpdatedNotificationParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ResourceUpdatedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceUpdatedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResourceUpdatedNotificationParams>
        for super::ResourceUpdatedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResourceUpdatedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::ResourceUpdatedNotificationParams>
        for ResourceUpdatedNotificationParams
    {
        fn from(value: super::ResourceUpdatedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Result {
        meta: ::std::result::Result<
            ::std::option::Option<super::ResultMetaObject>,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Result {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                result_type: Err("no value supplied for result_type".to_string()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl Result {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResultMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Result> for super::Result {
        type Error = super::error::ConversionError;
        fn try_from(value: Result) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                result_type: value.result_type?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::Result> for Result {
        fn from(value: super::Result) -> Self {
            Self {
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResultMetaObject {
        io_modelcontextprotocol_server_info: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResultMetaObject {
        fn default() -> Self {
            Self {
                io_modelcontextprotocol_server_info: Ok(Default::default()),
            }
        }
    }
    impl ResultMetaObject {
        pub fn io_modelcontextprotocol_server_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.io_modelcontextprotocol_server_info = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for io_modelcontextprotocol_server_info: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ResultMetaObject> for super::ResultMetaObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResultMetaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                io_modelcontextprotocol_server_info: value.io_modelcontextprotocol_server_info?,
            })
        }
    }
    impl ::std::convert::From<super::ResultMetaObject> for ResultMetaObject {
        fn from(value: super::ResultMetaObject) -> Self {
            Self {
                io_modelcontextprotocol_server_info: Ok(value.io_modelcontextprotocol_server_info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Root {
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Root {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                name: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl Root {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Root> for super::Root {
        type Error = super::error::ConversionError;
        fn try_from(value: Root) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                name: value.name?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::Root> for Root {
        fn from(value: super::Root) -> Self {
            Self {
                meta: Ok(value.meta),
                name: Ok(value.name),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SamplingMessage {
        content: ::std::result::Result<super::SamplingMessageContent, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
    }
    impl ::std::default::Default for SamplingMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                meta: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl SamplingMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SamplingMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SamplingMessage> for super::SamplingMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SamplingMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                meta: value.meta?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::SamplingMessage> for SamplingMessage {
        fn from(value: super::SamplingMessage) -> Self {
            Self {
                content: Ok(value.content),
                meta: Ok(value.meta),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilities {
        completions:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
        experimental: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            ::std::string::String,
        >,
        logging:
            ::std::result::Result<::std::option::Option<super::JsonObject>, ::std::string::String>,
        prompts: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesPrompts>,
            ::std::string::String,
        >,
        resources: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::option::Option<super::ServerCapabilitiesTools>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ServerCapabilities {
        fn default() -> Self {
            Self {
                completions: Ok(Default::default()),
                experimental: Ok(Default::default()),
                extensions: Ok(Default::default()),
                logging: Ok(Default::default()),
                prompts: Ok(Default::default()),
                resources: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilities {
        pub fn completions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.completions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completions: {e}"));
            self
        }
        pub fn experimental<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.experimental = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experimental: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::JsonObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn logging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.logging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logging: {e}"));
            self
        }
        pub fn prompts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesPrompts>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompts: {e}"));
            self
        }
        pub fn resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesResources>>,
            T::Error: ::std::fmt::Display,
        {
            self.resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServerCapabilitiesTools>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilities> for super::ServerCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completions: value.completions?,
                experimental: value.experimental?,
                extensions: value.extensions?,
                logging: value.logging?,
                prompts: value.prompts?,
                resources: value.resources?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilities> for ServerCapabilities {
        fn from(value: super::ServerCapabilities) -> Self {
            Self {
                completions: Ok(value.completions),
                experimental: Ok(value.experimental),
                extensions: Ok(value.extensions),
                logging: Ok(value.logging),
                prompts: Ok(value.prompts),
                resources: Ok(value.resources),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesPrompts {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesPrompts {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesPrompts {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesPrompts> for super::ServerCapabilitiesPrompts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesPrompts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesPrompts> for ServerCapabilitiesPrompts {
        fn from(value: super::ServerCapabilitiesPrompts) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesResources {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        subscribe: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesResources {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
                subscribe: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesResources {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
        pub fn subscribe<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.subscribe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscribe: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesResources> for super::ServerCapabilitiesResources {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
                subscribe: value.subscribe?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesResources> for ServerCapabilitiesResources {
        fn from(value: super::ServerCapabilitiesResources) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
                subscribe: Ok(value.subscribe),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilitiesTools {
        list_changed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilitiesTools {
        fn default() -> Self {
            Self {
                list_changed: Ok(Default::default()),
            }
        }
    }
    impl ServerCapabilitiesTools {
        pub fn list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_changed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilitiesTools> for super::ServerCapabilitiesTools {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilitiesTools,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_changed: value.list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilitiesTools> for ServerCapabilitiesTools {
        fn from(value: super::ServerCapabilitiesTools) -> Self {
            Self {
                list_changed: Ok(value.list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerResult {
        subtype_0:
            ::std::result::Result<::std::option::Option<super::Result>, ::std::string::String>,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::InputRequiredResult>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::DiscoverResult>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::ListResourcesResult>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::ListResourceTemplatesResult>,
            ::std::string::String,
        >,
        subtype_5: ::std::result::Result<
            ::std::option::Option<super::ReadResourceResult>,
            ::std::string::String,
        >,
        subtype_6: ::std::result::Result<
            ::std::option::Option<super::SubscriptionsListenResult>,
            ::std::string::String,
        >,
        subtype_7: ::std::result::Result<
            ::std::option::Option<super::ListPromptsResult>,
            ::std::string::String,
        >,
        subtype_8: ::std::result::Result<
            ::std::option::Option<super::GetPromptResult>,
            ::std::string::String,
        >,
        subtype_9: ::std::result::Result<
            ::std::option::Option<super::ListToolsResult>,
            ::std::string::String,
        >,
        subtype_10: ::std::result::Result<
            ::std::option::Option<super::CallToolResult>,
            ::std::string::String,
        >,
        subtype_11: ::std::result::Result<
            ::std::option::Option<super::CompleteResult>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ServerResult {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
                subtype_11: Ok(Default::default()),
            }
        }
    }
    impl ServerResult {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Result>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InputRequiredResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscoverResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListResourcesResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {e}"));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListResourceTemplatesResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {e}"));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReadResourceResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {e}"));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SubscriptionsListenResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {e}"));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListPromptsResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {e}"));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GetPromptResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {e}"));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListToolsResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {e}"));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CallToolResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {e}"));
            self
        }
        pub fn subtype_11<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CompleteResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_11 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_11: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerResult> for super::ServerResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
                subtype_11: value.subtype_11?,
            })
        }
    }
    impl ::std::convert::From<super::ServerResult> for ServerResult {
        fn from(value: super::ServerResult) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
                subtype_11: Ok(value.subtype_11),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StringSchema {
        default: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format: ::std::result::Result<
            ::std::option::Option<super::StringSchemaFormat>,
            ::std::string::String,
        >,
        max_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        min_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StringSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                max_length: Ok(Default::default()),
                min_length: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StringSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StringSchemaFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {e}"));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StringSchema> for super::StringSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StringSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                format: value.format?,
                max_length: value.max_length?,
                min_length: value.min_length?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StringSchema> for StringSchema {
        fn from(value: super::StringSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                format: Ok(value.format),
                max_length: Ok(value.max_length),
                min_length: Ok(value.min_length),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionFilter {
        prompts_list_changed:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        resource_subscriptions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resources_list_changed:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        tools_list_changed:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionFilter {
        fn default() -> Self {
            Self {
                prompts_list_changed: Ok(Default::default()),
                resource_subscriptions: Ok(Default::default()),
                resources_list_changed: Ok(Default::default()),
                tools_list_changed: Ok(Default::default()),
            }
        }
    }
    impl SubscriptionFilter {
        pub fn prompts_list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompts_list_changed = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompts_list_changed: {e}")
            });
            self
        }
        pub fn resource_subscriptions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_subscriptions = value.try_into().map_err(|e| {
                format!("error converting supplied value for resource_subscriptions: {e}")
            });
            self
        }
        pub fn resources_list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.resources_list_changed = value.try_into().map_err(|e| {
                format!("error converting supplied value for resources_list_changed: {e}")
            });
            self
        }
        pub fn tools_list_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools_list_changed = value.try_into().map_err(|e| {
                format!("error converting supplied value for tools_list_changed: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionFilter> for super::SubscriptionFilter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionFilter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompts_list_changed: value.prompts_list_changed?,
                resource_subscriptions: value.resource_subscriptions?,
                resources_list_changed: value.resources_list_changed?,
                tools_list_changed: value.tools_list_changed?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionFilter> for SubscriptionFilter {
        fn from(value: super::SubscriptionFilter) -> Self {
            Self {
                prompts_list_changed: Ok(value.prompts_list_changed),
                resource_subscriptions: Ok(value.resource_subscriptions),
                resources_list_changed: Ok(value.resources_list_changed),
                tools_list_changed: Ok(value.tools_list_changed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsAcknowledgedNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            super::SubscriptionsAcknowledgedNotificationParams,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubscriptionsAcknowledgedNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SubscriptionsAcknowledgedNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionsAcknowledgedNotificationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsAcknowledgedNotification>
        for super::SubscriptionsAcknowledgedNotification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsAcknowledgedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsAcknowledgedNotification>
        for SubscriptionsAcknowledgedNotification
    {
        fn from(value: super::SubscriptionsAcknowledgedNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsAcknowledgedNotificationParams {
        meta: ::std::result::Result<
            ::std::option::Option<super::NotificationMetaObject>,
            ::std::string::String,
        >,
        notifications: ::std::result::Result<super::SubscriptionFilter, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsAcknowledgedNotificationParams {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                notifications: Err("no value supplied for notifications".to_string()),
            }
        }
    }
    impl SubscriptionsAcknowledgedNotificationParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationMetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn notifications<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionFilter>,
            T::Error: ::std::fmt::Display,
        {
            self.notifications = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notifications: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsAcknowledgedNotificationParams>
        for super::SubscriptionsAcknowledgedNotificationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsAcknowledgedNotificationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                notifications: value.notifications?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsAcknowledgedNotificationParams>
        for SubscriptionsAcknowledgedNotificationParams
    {
        fn from(value: super::SubscriptionsAcknowledgedNotificationParams) -> Self {
            Self {
                meta: Ok(value.meta),
                notifications: Ok(value.notifications),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsListenRequest {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params:
            ::std::result::Result<super::SubscriptionsListenRequestParams, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsListenRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SubscriptionsListenRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionsListenRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsListenRequest> for super::SubscriptionsListenRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsListenRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsListenRequest> for SubscriptionsListenRequest {
        fn from(value: super::SubscriptionsListenRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsListenRequestParams {
        meta: ::std::result::Result<super::RequestMetaObject, ::std::string::String>,
        notifications: ::std::result::Result<super::SubscriptionFilter, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsListenRequestParams {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
                notifications: Err("no value supplied for notifications".to_string()),
            }
        }
    }
    impl SubscriptionsListenRequestParams {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn notifications<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionFilter>,
            T::Error: ::std::fmt::Display,
        {
            self.notifications = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notifications: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsListenRequestParams>
        for super::SubscriptionsListenRequestParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsListenRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                notifications: value.notifications?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsListenRequestParams>
        for SubscriptionsListenRequestParams
    {
        fn from(value: super::SubscriptionsListenRequestParams) -> Self {
            Self {
                meta: Ok(value.meta),
                notifications: Ok(value.notifications),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsListenResult {
        meta: ::std::result::Result<
            super::SubscriptionsListenResultMetaObject,
            ::std::string::String,
        >,
        result_type: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsListenResult {
        fn default() -> Self {
            Self {
                meta: Err("no value supplied for meta".to_string()),
                result_type: Err("no value supplied for result_type".to_string()),
            }
        }
    }
    impl SubscriptionsListenResult {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionsListenResultMetaObject>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn result_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsListenResult> for super::SubscriptionsListenResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsListenResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                result_type: value.result_type?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsListenResult> for SubscriptionsListenResult {
        fn from(value: super::SubscriptionsListenResult) -> Self {
            Self {
                meta: Ok(value.meta),
                result_type: Ok(value.result_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsListenResultMetaObject {
        io_modelcontextprotocol_server_info: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
        io_modelcontextprotocol_subscription_id:
            ::std::result::Result<super::RequestId, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsListenResultMetaObject {
        fn default() -> Self {
            Self {
                io_modelcontextprotocol_server_info: Ok(Default::default()),
                io_modelcontextprotocol_subscription_id: Err(
                    "no value supplied for io_modelcontextprotocol_subscription_id".to_string(),
                ),
            }
        }
    }
    impl SubscriptionsListenResultMetaObject {
        pub fn io_modelcontextprotocol_server_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.io_modelcontextprotocol_server_info = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for io_modelcontextprotocol_server_info: {e}"
                )
            });
            self
        }
        pub fn io_modelcontextprotocol_subscription_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self . io_modelcontextprotocol_subscription_id = value . try_into () . map_err (| e | format ! ("error converting supplied value for io_modelcontextprotocol_subscription_id: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsListenResultMetaObject>
        for super::SubscriptionsListenResultMetaObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsListenResultMetaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                io_modelcontextprotocol_server_info: value.io_modelcontextprotocol_server_info?,
                io_modelcontextprotocol_subscription_id: value
                    .io_modelcontextprotocol_subscription_id?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsListenResultMetaObject>
        for SubscriptionsListenResultMetaObject
    {
        fn from(value: super::SubscriptionsListenResultMetaObject) -> Self {
            Self {
                io_modelcontextprotocol_server_info: Ok(value.io_modelcontextprotocol_server_info),
                io_modelcontextprotocol_subscription_id: Ok(
                    value.io_modelcontextprotocol_subscription_id
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscriptionsListenResultResponse {
        id: ::std::result::Result<super::RequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::SubscriptionsListenResult, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionsListenResultResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl SubscriptionsListenResultResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionsListenResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionsListenResultResponse>
        for super::SubscriptionsListenResultResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionsListenResultResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionsListenResultResponse>
        for SubscriptionsListenResultResponse
    {
        fn from(value: super::SubscriptionsListenResultResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextContent {
        annotations:
            ::std::result::Result<::std::option::Option<super::Annotations>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextContent {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                meta: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TextContent {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Annotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TextContent> for super::TextContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                meta: value.meta?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TextContent> for TextContent {
        fn from(value: super::TextContent) -> Self {
            Self {
                annotations: Ok(value.annotations),
                meta: Ok(value.meta),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextResourceContents {
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextResourceContents {
        fn default() -> Self {
            Self {
                meta: Ok(Default::default()),
                mime_type: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl TextResourceContents {
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TextResourceContents> for super::TextResourceContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextResourceContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                meta: value.meta?,
                mime_type: value.mime_type?,
                text: value.text?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::TextResourceContents> for TextResourceContents {
        fn from(value: super::TextResourceContents) -> Self {
            Self {
                meta: Ok(value.meta),
                mime_type: Ok(value.mime_type),
                text: Ok(value.text),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TitledMultiSelectEnumSchema {
        default:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        items:
            ::std::result::Result<super::TitledMultiSelectEnumSchemaItems, ::std::string::String>,
        max_items: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        min_items: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TitledMultiSelectEnumSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                items: Err("no value supplied for items".to_string()),
                max_items: Ok(Default::default()),
                min_items: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TitledMultiSelectEnumSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TitledMultiSelectEnumSchemaItems>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
        pub fn max_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_items: {e}"));
            self
        }
        pub fn min_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_items: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TitledMultiSelectEnumSchema> for super::TitledMultiSelectEnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TitledMultiSelectEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                items: value.items?,
                max_items: value.max_items?,
                min_items: value.min_items?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TitledMultiSelectEnumSchema> for TitledMultiSelectEnumSchema {
        fn from(value: super::TitledMultiSelectEnumSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                items: Ok(value.items),
                max_items: Ok(value.max_items),
                min_items: Ok(value.min_items),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TitledMultiSelectEnumSchemaItems {
        any_of: ::std::result::Result<
            ::std::vec::Vec<super::TitledMultiSelectEnumSchemaItemsAnyOfItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TitledMultiSelectEnumSchemaItems {
        fn default() -> Self {
            Self {
                any_of: Err("no value supplied for any_of".to_string()),
            }
        }
    }
    impl TitledMultiSelectEnumSchemaItems {
        pub fn any_of<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TitledMultiSelectEnumSchemaItemsAnyOfItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.any_of = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for any_of: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TitledMultiSelectEnumSchemaItems>
        for super::TitledMultiSelectEnumSchemaItems
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TitledMultiSelectEnumSchemaItems,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                any_of: value.any_of?,
            })
        }
    }
    impl ::std::convert::From<super::TitledMultiSelectEnumSchemaItems>
        for TitledMultiSelectEnumSchemaItems
    {
        fn from(value: super::TitledMultiSelectEnumSchemaItems) -> Self {
            Self {
                any_of: Ok(value.any_of),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TitledMultiSelectEnumSchemaItemsAnyOfItem {
        const_: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TitledMultiSelectEnumSchemaItemsAnyOfItem {
        fn default() -> Self {
            Self {
                const_: Err("no value supplied for const_".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl TitledMultiSelectEnumSchemaItemsAnyOfItem {
        pub fn const_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.const_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for const_: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TitledMultiSelectEnumSchemaItemsAnyOfItem>
        for super::TitledMultiSelectEnumSchemaItemsAnyOfItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TitledMultiSelectEnumSchemaItemsAnyOfItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                const_: value.const_?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::TitledMultiSelectEnumSchemaItemsAnyOfItem>
        for TitledMultiSelectEnumSchemaItemsAnyOfItem
    {
        fn from(value: super::TitledMultiSelectEnumSchemaItemsAnyOfItem) -> Self {
            Self {
                const_: Ok(value.const_),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TitledSingleSelectEnumSchema {
        default: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        one_of: ::std::result::Result<
            ::std::vec::Vec<super::TitledSingleSelectEnumSchemaOneOfItem>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TitledSingleSelectEnumSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                one_of: Err("no value supplied for one_of".to_string()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TitledSingleSelectEnumSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn one_of<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::TitledSingleSelectEnumSchemaOneOfItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.one_of = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for one_of: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TitledSingleSelectEnumSchema> for super::TitledSingleSelectEnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TitledSingleSelectEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                one_of: value.one_of?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TitledSingleSelectEnumSchema> for TitledSingleSelectEnumSchema {
        fn from(value: super::TitledSingleSelectEnumSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                one_of: Ok(value.one_of),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TitledSingleSelectEnumSchemaOneOfItem {
        const_: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TitledSingleSelectEnumSchemaOneOfItem {
        fn default() -> Self {
            Self {
                const_: Err("no value supplied for const_".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl TitledSingleSelectEnumSchemaOneOfItem {
        pub fn const_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.const_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for const_: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TitledSingleSelectEnumSchemaOneOfItem>
        for super::TitledSingleSelectEnumSchemaOneOfItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TitledSingleSelectEnumSchemaOneOfItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                const_: value.const_?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::TitledSingleSelectEnumSchemaOneOfItem>
        for TitledSingleSelectEnumSchemaOneOfItem
    {
        fn from(value: super::TitledSingleSelectEnumSchemaOneOfItem) -> Self {
            Self {
                const_: Ok(value.const_),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tool {
        annotations: ::std::result::Result<
            ::std::option::Option<super::ToolAnnotations>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icons: ::std::result::Result<::std::vec::Vec<super::Icon>, ::std::string::String>,
        input_schema: ::std::result::Result<super::ToolInputSchema, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        output_schema: ::std::result::Result<
            ::std::option::Option<super::ToolOutputSchema>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Tool {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                description: Ok(Default::default()),
                icons: Ok(Default::default()),
                input_schema: Err("no value supplied for input_schema".to_string()),
                meta: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                output_schema: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Tool {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolAnnotations>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn icons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Icon>>,
            T::Error: ::std::fmt::Display,
        {
            self.icons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icons: {e}"));
            self
        }
        pub fn input_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolInputSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.input_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_schema: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn output_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolOutputSchema>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_schema: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tool> for super::Tool {
        type Error = super::error::ConversionError;
        fn try_from(value: Tool) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                description: value.description?,
                icons: value.icons?,
                input_schema: value.input_schema?,
                meta: value.meta?,
                name: value.name?,
                output_schema: value.output_schema?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Tool> for Tool {
        fn from(value: super::Tool) -> Self {
            Self {
                annotations: Ok(value.annotations),
                description: Ok(value.description),
                icons: Ok(value.icons),
                input_schema: Ok(value.input_schema),
                meta: Ok(value.meta),
                name: Ok(value.name),
                output_schema: Ok(value.output_schema),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolAnnotations {
        destructive_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        idempotent_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        open_world_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        read_only_hint: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolAnnotations {
        fn default() -> Self {
            Self {
                destructive_hint: Ok(Default::default()),
                idempotent_hint: Ok(Default::default()),
                open_world_hint: Ok(Default::default()),
                read_only_hint: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl ToolAnnotations {
        pub fn destructive_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.destructive_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for destructive_hint: {e}"));
            self
        }
        pub fn idempotent_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.idempotent_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for idempotent_hint: {e}"));
            self
        }
        pub fn open_world_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_world_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_world_hint: {e}"));
            self
        }
        pub fn read_only_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.read_only_hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for read_only_hint: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolAnnotations> for super::ToolAnnotations {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolAnnotations,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                destructive_hint: value.destructive_hint?,
                idempotent_hint: value.idempotent_hint?,
                open_world_hint: value.open_world_hint?,
                read_only_hint: value.read_only_hint?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ToolAnnotations> for ToolAnnotations {
        fn from(value: super::ToolAnnotations) -> Self {
            Self {
                destructive_hint: Ok(value.destructive_hint),
                idempotent_hint: Ok(value.idempotent_hint),
                open_world_hint: Ok(value.open_world_hint),
                read_only_hint: Ok(value.read_only_hint),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolChoice {
        mode: ::std::result::Result<
            ::std::option::Option<super::ToolChoiceMode>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolChoice {
        fn default() -> Self {
            Self {
                mode: Ok(Default::default()),
            }
        }
    }
    impl ToolChoice {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolChoiceMode>>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolChoice> for super::ToolChoice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { mode: value.mode? })
        }
    }
    impl ::std::convert::From<super::ToolChoice> for ToolChoice {
        fn from(value: super::ToolChoice) -> Self {
            Self {
                mode: Ok(value.mode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolInputSchema {
        schema: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolInputSchema {
        fn default() -> Self {
            Self {
                schema: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ToolInputSchema {
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolInputSchema> for super::ToolInputSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolInputSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                schema: value.schema?,
                type_: value.type_?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ToolInputSchema> for ToolInputSchema {
        fn from(value: super::ToolInputSchema) -> Self {
            Self {
                schema: Ok(value.schema),
                type_: Ok(value.type_),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolListChangedNotification {
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::std::option::Option<super::NotificationParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolListChangedNotification {
        fn default() -> Self {
            Self {
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl ToolListChangedNotification {
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolListChangedNotification> for super::ToolListChangedNotification {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolListChangedNotification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ToolListChangedNotification> for ToolListChangedNotification {
        fn from(value: super::ToolListChangedNotification) -> Self {
            Self {
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolOutputSchema {
        schema: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ToolOutputSchema {
        fn default() -> Self {
            Self {
                schema: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ToolOutputSchema {
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolOutputSchema> for super::ToolOutputSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolOutputSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                schema: value.schema?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ToolOutputSchema> for ToolOutputSchema {
        fn from(value: super::ToolOutputSchema) -> Self {
            Self {
                schema: Ok(value.schema),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolResultContent {
        content: ::std::result::Result<::std::vec::Vec<super::ContentBlock>, ::std::string::String>,
        is_error: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        structured_content: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        tool_use_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolResultContent {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                is_error: Ok(Default::default()),
                meta: Ok(Default::default()),
                structured_content: Ok(Default::default()),
                tool_use_id: Err("no value supplied for tool_use_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolResultContent {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ContentBlock>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn is_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_error: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn structured_content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.structured_content = value.try_into().map_err(|e| {
                format!("error converting supplied value for structured_content: {e}")
            });
            self
        }
        pub fn tool_use_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_use_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_use_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolResultContent> for super::ToolResultContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolResultContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                is_error: value.is_error?,
                meta: value.meta?,
                structured_content: value.structured_content?,
                tool_use_id: value.tool_use_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolResultContent> for ToolResultContent {
        fn from(value: super::ToolResultContent) -> Self {
            Self {
                content: Ok(value.content),
                is_error: Ok(value.is_error),
                meta: Ok(value.meta),
                structured_content: Ok(value.structured_content),
                tool_use_id: Ok(value.tool_use_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolUseContent {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        input: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        meta:
            ::std::result::Result<::std::option::Option<super::MetaObject>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolUseContent {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                input: Err("no value supplied for input".to_string()),
                meta: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolUseContent {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MetaObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolUseContent> for super::ToolUseContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolUseContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                input: value.input?,
                meta: value.meta?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolUseContent> for ToolUseContent {
        fn from(value: super::ToolUseContent) -> Self {
            Self {
                id: Ok(value.id),
                input: Ok(value.input),
                meta: Ok(value.meta),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsupportedProtocolVersionError {
        error: ::std::result::Result<
            super::UnsupportedProtocolVersionErrorError,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::RequestId>, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnsupportedProtocolVersionError {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl UnsupportedProtocolVersionError {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnsupportedProtocolVersionErrorError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsupportedProtocolVersionError>
        for super::UnsupportedProtocolVersionError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsupportedProtocolVersionError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::UnsupportedProtocolVersionError>
        for UnsupportedProtocolVersionError
    {
        fn from(value: super::UnsupportedProtocolVersionError) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsupportedProtocolVersionErrorError {
        code: ::std::result::Result<
            super::UnsupportedProtocolVersionErrorErrorCode,
            ::std::string::String,
        >,
        data: ::std::result::Result<
            super::UnsupportedProtocolVersionErrorErrorData,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnsupportedProtocolVersionErrorError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Err("no value supplied for data".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl UnsupportedProtocolVersionErrorError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnsupportedProtocolVersionErrorErrorCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnsupportedProtocolVersionErrorErrorData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsupportedProtocolVersionErrorError>
        for super::UnsupportedProtocolVersionErrorError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsupportedProtocolVersionErrorError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::UnsupportedProtocolVersionErrorError>
        for UnsupportedProtocolVersionErrorError
    {
        fn from(value: super::UnsupportedProtocolVersionErrorError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsupportedProtocolVersionErrorErrorData {
        requested: ::std::result::Result<::std::string::String, ::std::string::String>,
        supported:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UnsupportedProtocolVersionErrorErrorData {
        fn default() -> Self {
            Self {
                requested: Err("no value supplied for requested".to_string()),
                supported: Err("no value supplied for supported".to_string()),
            }
        }
    }
    impl UnsupportedProtocolVersionErrorErrorData {
        pub fn requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requested: {e}"));
            self
        }
        pub fn supported<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supported: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsupportedProtocolVersionErrorErrorData>
        for super::UnsupportedProtocolVersionErrorErrorData
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsupportedProtocolVersionErrorErrorData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                requested: value.requested?,
                supported: value.supported?,
            })
        }
    }
    impl ::std::convert::From<super::UnsupportedProtocolVersionErrorErrorData>
        for UnsupportedProtocolVersionErrorErrorData
    {
        fn from(value: super::UnsupportedProtocolVersionErrorErrorData) -> Self {
            Self {
                requested: Ok(value.requested),
                supported: Ok(value.supported),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UntitledMultiSelectEnumSchema {
        default:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        items:
            ::std::result::Result<super::UntitledMultiSelectEnumSchemaItems, ::std::string::String>,
        max_items: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        min_items: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UntitledMultiSelectEnumSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                items: Err("no value supplied for items".to_string()),
                max_items: Ok(Default::default()),
                min_items: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UntitledMultiSelectEnumSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UntitledMultiSelectEnumSchemaItems>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
        pub fn max_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_items: {e}"));
            self
        }
        pub fn min_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_items: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UntitledMultiSelectEnumSchema>
        for super::UntitledMultiSelectEnumSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UntitledMultiSelectEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                items: value.items?,
                max_items: value.max_items?,
                min_items: value.min_items?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UntitledMultiSelectEnumSchema> for UntitledMultiSelectEnumSchema {
        fn from(value: super::UntitledMultiSelectEnumSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                items: Ok(value.items),
                max_items: Ok(value.max_items),
                min_items: Ok(value.min_items),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UntitledMultiSelectEnumSchemaItems {
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UntitledMultiSelectEnumSchemaItems {
        fn default() -> Self {
            Self {
                enum_: Err("no value supplied for enum_".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UntitledMultiSelectEnumSchemaItems {
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UntitledMultiSelectEnumSchemaItems>
        for super::UntitledMultiSelectEnumSchemaItems
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UntitledMultiSelectEnumSchemaItems,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enum_: value.enum_?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UntitledMultiSelectEnumSchemaItems>
        for UntitledMultiSelectEnumSchemaItems
    {
        fn from(value: super::UntitledMultiSelectEnumSchemaItems) -> Self {
            Self {
                enum_: Ok(value.enum_),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UntitledSingleSelectEnumSchema {
        default: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UntitledSingleSelectEnumSchema {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                enum_: Err("no value supplied for enum_".to_string()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UntitledSingleSelectEnumSchema {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UntitledSingleSelectEnumSchema>
        for super::UntitledSingleSelectEnumSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UntitledSingleSelectEnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                enum_: value.enum_?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UntitledSingleSelectEnumSchema>
        for UntitledSingleSelectEnumSchema
    {
        fn from(value: super::UntitledSingleSelectEnumSchema) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                enum_: Ok(value.enum_),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
