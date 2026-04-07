use thiserror::Error;

/// Errors returned by the UpCloud API client.
#[derive(Debug, Error)]
pub enum Error {
    /// An HTTP transport error (connection refused, timeout, DNS failure, etc.).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The UpCloud API returned a structured error response.
    #[error("API error: {0}")]
    Api(ApiError),

    /// Failed to deserialize a response body as JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Failed to parse a URL.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Invalid client configuration (missing auth, bad base URL, etc.).
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}

impl Error {
    /// Returns the HTTP status code if this is an API error.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Api(e) => Some(e.status),
            _ => None,
        }
    }

    /// Returns `true` if this is a 404 Not Found API error.
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(404)
    }

    /// Returns `true` if this is a 401 or 403 authentication/authorization error.
    pub fn is_unauthorized(&self) -> bool {
        matches!(self.status(), Some(401) | Some(403))
    }

    /// Returns `true` if this is a 409 Conflict API error.
    pub fn is_conflict(&self) -> bool {
        self.status() == Some(409)
    }
}

// ─── ErrorCode ─────────────────────────────────────────────────────────────────

macro_rules! define_error_codes {
    ( $( $variant:ident = $string:literal ),* $(,)? ) => {
        /// Well-known error codes returned by the UpCloud API.
        ///
        /// The API returns machine-readable error codes as strings
        /// (e.g. `"SERVER_NOT_FOUND"`, `"AUTHENTICATION_FAILED"`).
        /// This enum provides typed variants for all documented codes,
        /// with an [`Unknown`](ErrorCode::Unknown) fallback for codes
        /// added after this library was published.
        ///
        /// # Examples
        ///
        /// ```
        /// use upcloud_api::ErrorCode;
        ///
        /// let code = ErrorCode::from("SERVER_NOT_FOUND");
        /// assert_eq!(code, ErrorCode::ServerNotFound);
        /// assert_eq!(code.as_str(), "SERVER_NOT_FOUND");
        ///
        /// let unknown = ErrorCode::from("BRAND_NEW_CODE");
        /// assert!(matches!(unknown, ErrorCode::Unknown(_)));
        /// ```
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum ErrorCode {
            $( $variant, )*
            /// An error code not yet known to this library.
            Unknown(String),
        }

        impl ErrorCode {
            /// Returns the wire-format string (e.g. `"SERVER_NOT_FOUND"`).
            pub fn as_str(&self) -> &str {
                match self {
                    $( Self::$variant => $string, )*
                    Self::Unknown(s) => s,
                }
            }
        }

        impl std::fmt::Display for ErrorCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<String> for ErrorCode {
            fn from(s: String) -> Self {
                match s.as_str() {
                    $( $string => Self::$variant, )*
                    _ => Self::Unknown(s),
                }
            }
        }

        impl From<&str> for ErrorCode {
            fn from(s: &str) -> Self {
                match s {
                    $( $string => Self::$variant, )*
                    _ => Self::Unknown(s.to_string()),
                }
            }
        }
    };
}

define_error_codes! {
    AccessKeyInvalid = "ACCESS_KEY_INVALID",
    AccessKeyMissing = "ACCESS_KEY_MISSING",
    AccountForbidden = "ACCOUNT_FORBIDDEN",
    ActionForbidden = "ACTION_FORBIDDEN",
    ActionInvalid = "ACTION_INVALID",
    ActionMissing = "ACTION_MISSING",
    AddressAttached = "ADDRESS_ATTACHED",
    AddressInvalid = "ADDRESS_INVALID",
    AddressMissing = "ADDRESS_MISSING",
    AddressOrUuidRequired = "ADDRESS_OR_UUID_REQUIRED",
    AlreadyInServerGroup = "ALREADY_IN_SERVER_GROUP",
    AuthenticationFailed = "AUTHENTICATION_FAILED",
    BackupDeletionPolicyInvalid = "BACKUP_DELETION_POLICY_INVALID",
    BackupRuleConflict = "BACKUP_RULE_CONFLICT",
    BackupRuleInvalid = "BACKUP_RULE_INVALID",
    BootOrderInvalid = "BOOT_ORDER_INVALID",
    CanNotCreatePasswordForTheSelectedDistribution = "CAN_NOT_CREATE_PASSWORD_FOR_THE_SELECTED_DISTRIBUTION",
    CannotDeletePrivateAddress = "CANNOT_DELETE_PRIVATE_ADDRESS",
    CdromDeviceInUse = "CDROM_DEVICE_IN_USE",
    CdromEjectFailed = "CDROM_EJECT_FAILED",
    CdromHotplugUnsupported = "CDROM_HOTPLUG_UNSUPPORTED",
    CommentInvalid = "COMMENT_INVALID",
    ConnectionPoolNotFound = "CONNECTION_POOL_NOT_FOUND",
    CoreMemoryUnsupported = "CORE_MEMORY_UNSUPPORTED",
    CoreNumberInvalid = "CORE_NUMBER_INVALID",
    CreatePasswordInvalid = "CREATE_PASSWORD_INVALID",
    DbExists = "DB_EXISTS",
    DbNotFound = "DB_NOT_FOUND",
    DescriptionInvalid = "DESCRIPTION_INVALID",
    DestinationAddressEndInvalid = "DESTINATION_ADDRESS_END_INVALID",
    DestinationAddressOrderIllegal = "DESTINATION_ADDRESS_ORDER_ILLEGAL",
    DestinationAddressStartInvalid = "DESTINATION_ADDRESS_START_INVALID",
    DestinationPortEndInvalid = "DESTINATION_PORT_END_INVALID",
    DestinationPortOrderIllegal = "DESTINATION_PORT_ORDER_ILLEGAL",
    DestinationPortStartInvalid = "DESTINATION_PORT_START_INVALID",
    DeviceAddressInUse = "DEVICE_ADDRESS_IN_USE",
    DeviceAddressNotInUse = "DEVICE_ADDRESS_NOT_IN_USE",
    DirectionInvalid = "DIRECTION_INVALID",
    DirectionMissing = "DIRECTION_MISSING",
    DualStackInterface = "DUAL_STACK_INTERFACE",
    DuplicateLabelKeys = "DUPLICATE_LABEL_KEYS",
    DuplicateResource = "DUPLICATE_RESOURCE",
    DuplicateRoute = "DUPLICATE_ROUTE",
    FailedToAddMemory = "FAILED_TO_ADD_MEMORY",
    FirewallInvalid = "FIREWALL_INVALID",
    FirewallRuleExists = "FIREWALL_RULE_EXISTS",
    FirewallRuleLimitReached = "FIREWALL_RULE_LIMIT_REACHED",
    FirewallRuleNotFound = "FIREWALL_RULE_NOT_FOUND",
    FloatingIpNotAvailable = "FLOATING_IP_NOT_AVAILABLE",
    GroupForbidden = "GROUP_FORBIDDEN",
    GroupInvalid = "GROUP_INVALID",
    GroupNotFound = "GROUP_NOT_FOUND",
    HostForbidden = "HOST_FORBIDDEN",
    HostNotFound = "HOST_NOT_FOUND",
    HostnameInvalid = "HOSTNAME_INVALID",
    HostnameMissing = "HOSTNAME_MISSING",
    HotResizeNotEnabled = "HOT_RESIZE_NOT_ENABLED",
    HotResizeUnavailable = "HOT_RESIZE_UNAVAILABLE",
    HotplugFailed = "HOTPLUG_FAILED",
    HourInvalid = "HOUR_INVALID",
    HourMissing = "HOUR_MISSING",
    IcmpTypeInvalid = "ICMP_TYPE_INVALID",
    IcmpTypeProtocolMismatch = "ICMP_TYPE_PROTOCOL_MISMATCH",
    IdeHotplugUnsupported = "IDE_HOTPLUG_UNSUPPORTED",
    ImportUnavailable = "IMPORT_UNAVAILABLE",
    InsufficientCredits = "INSUFFICIENT_CREDITS",
    InterfaceExists = "INTERFACE_EXISTS",
    InterfaceForbidden = "INTERFACE_FORBIDDEN",
    InterfaceNotFound = "INTERFACE_NOT_FOUND",
    IntervalInvalid = "INTERVAL_INVALID",
    IntervalMissing = "INTERVAL_MISSING",
    InvalidLabelKey = "INVALID_LABEL_KEY",
    InvalidLabelValue = "INVALID_LABEL_VALUE",
    InvalidOptions = "INVALID_OPTIONS",
    InvalidRequest = "INVALID_REQUEST",
    InvalidRoute = "INVALID_ROUTE",
    InvalidRouteFamily = "INVALID_ROUTE_FAMILY",
    InvalidUsername = "INVALID_USERNAME",
    IpAddressForbidden = "IP_ADDRESS_FORBIDDEN",
    IpAddressInvalid = "IP_ADDRESS_INVALID",
    IpAddressLimitReached = "IP_ADDRESS_LIMIT_REACHED",
    IpAddressNotFound = "IP_ADDRESS_NOT_FOUND",
    IpAddressResourcesUnavailable = "IP_ADDRESS_RESOURCES_UNAVAILABLE",
    LocalNetworkNoRouter = "LOCAL_NETWORK_NO_ROUTER",
    MacInvalid = "MAC_INVALID",
    MaxiopsStorageLimitReached = "MAXIOPS_STORAGE_LIMIT_REACHED",
    MemoryAmountInvalid = "MEMORY_AMOUNT_INVALID",
    MetadataDisabledOnCloudInit = "METADATA_DISABLED_ON_CLOUD-INIT",
    MethodNotAllowed = "METHOD_NOT_ALLOWED",
    MsLmaRequired = "MS_LMA_REQUIRED",
    MultipleTemplates = "MULTIPLE_TEMPLATES",
    NameInvalid = "NAME_INVALID",
    NetworkForbidden = "NETWORK_FORBIDDEN",
    NetworkInvalid = "NETWORK_INVALID",
    NetworkNameInvalid = "NETWORK_NAME_INVALID",
    NetworkNameMissing = "NETWORK_NAME_MISSING",
    NetworkNotEmpty = "NETWORK_NOT_EMPTY",
    NetworkNotFound = "NETWORK_NOT_FOUND",
    NetworkTypeInvalid = "NETWORK_TYPE_INVALID",
    NetworkTypeMissing = "NETWORK_TYPE_MISSING",
    NexthopInvalid = "NEXTHOP_INVALID",
    NicModelInvalid = "NIC_MODEL_INVALID",
    NoAvailableMemorySlots = "NO_AVAILABLE_MEMORY_SLOTS",
    NoCdromDevice = "NO_CDROM_DEVICE",
    NoStoragesAttached = "NO_STORAGES_ATTACHED",
    NotFound = "NOT_FOUND",
    ObjectStorageForbidden = "OBJECT_STORAGE_FORBIDDEN",
    ObjectStorageNotFound = "OBJECT_STORAGE_NOT_FOUND",
    PasswordDeliveryInvalid = "PASSWORD_DELIVERY_INVALID",
    PeerNetworkNotFound = "PEER_NETWORK_NOT_FOUND",
    PeeringAccountInvalid = "PEERING_ACCOUNT_INVALID",
    PeeringConflict = "PEERING_CONFLICT",
    PeeringNotDisabled = "PEERING_NOT_DISABLED",
    PeeringNotFound = "PEERING_NOT_FOUND",
    PlanCoreNumberIllegal = "PLAN_CORE_NUMBER_ILLEGAL",
    PlanMemoryAmountIllegal = "PLAN_MEMORY_AMOUNT_ILLEGAL",
    PortProtocolMismatch = "PORT_PROTOCOL_MISMATCH",
    PositionInvalid = "POSITION_INVALID",
    ProtocolInvalid = "PROTOCOL_INVALID",
    PtrRecordInvalid = "PTR_RECORD_INVALID",
    PtrRecordNotSupported = "PTR_RECORD_NOT_SUPPORTED",
    PublicStorageAttach = "PUBLIC_STORAGE_ATTACH",
    RequestInvalid = "REQUEST_INVALID",
    ResizeFailed = "RESIZE_FAILED",
    ResourceAlreadyExists = "RESOURCE_ALREADY_EXISTS",
    ResourceNotFound = "RESOURCE_NOT_FOUND",
    RetentionInvalid = "RETENTION_INVALID",
    RetentionMissing = "RETENTION_MISSING",
    RouterAttached = "ROUTER_ATTACHED",
    RouterNameMissing = "ROUTER_NAME_MISSING",
    RouterNotFound = "ROUTER_NOT_FOUND",
    SecretKeyInvalid = "SECRET_KEY_INVALID",
    SecretKeyMissing = "SECRET_KEY_MISSING",
    ServerCoreLimitReached = "SERVER_CORE_LIMIT_REACHED",
    ServerCreatingLimitReached = "SERVER_CREATING_LIMIT_REACHED",
    ServerForbidden = "SERVER_FORBIDDEN",
    ServerInvalid = "SERVER_INVALID",
    ServerIpLimitReached = "SERVER_IP_LIMIT_REACHED",
    ServerMemoryLimitReached = "SERVER_MEMORY_LIMIT_REACHED",
    ServerNotFound = "SERVER_NOT_FOUND",
    ServerResourcesUnavailable = "SERVER_RESOURCES_UNAVAILABLE",
    ServerStateIllegal = "SERVER_STATE_ILLEGAL",
    ServerTitleInvalid = "SERVER_TITLE_INVALID",
    ServerTitleMissing = "SERVER_TITLE_MISSING",
    ServiceError = "SERVICE_ERROR",
    ServiceExists = "SERVICE_EXISTS",
    ServiceNotFound = "SERVICE_NOT_FOUND",
    SimpleBackupInvalid = "SIMPLE_BACKUP_INVALID",
    SizeInvalid = "SIZE_INVALID",
    SizeMissing = "SIZE_MISSING",
    SourceAddressEndInvalid = "SOURCE_ADDRESS_END_INVALID",
    SourceAddressOrderIllegal = "SOURCE_ADDRESS_ORDER_ILLEGAL",
    SourceAddressStartInvalid = "SOURCE_ADDRESS_START_INVALID",
    SourcePortEndInvalid = "SOURCE_PORT_END_INVALID",
    SourcePortOrderIllegal = "SOURCE_PORT_ORDER_ILLEGAL",
    SourcePortStartInvalid = "SOURCE_PORT_START_INVALID",
    StaticRouteLimitReached = "STATIC_ROUTE_LIMIT_REACHED",
    StaticRouteTargetInvalid = "STATIC_ROUTE_TARGET_INVALID",
    StopTypeInvalid = "STOP_TYPE_INVALID",
    StorageAttached = "STORAGE_ATTACHED",
    StorageAttachedAsCdrom = "STORAGE_ATTACHED_AS_CDROM",
    StorageAttachedAsDisk = "STORAGE_ATTACHED_AS_DISK",
    StorageDeletionPolicyInvalid = "STORAGE_DELETION_POLICY_INVALID",
    StorageDeviceInvalid = "STORAGE_DEVICE_INVALID",
    StorageDeviceLimitReached = "STORAGE_DEVICE_LIMIT_REACHED",
    StorageDeviceMissing = "STORAGE_DEVICE_MISSING",
    StorageDevicesInvalid = "STORAGE_DEVICES_INVALID",
    StorageDevicesMissing = "STORAGE_DEVICES_MISSING",
    StorageForbidden = "STORAGE_FORBIDDEN",
    StorageImportNotFound = "STORAGE_IMPORT_NOT_FOUND",
    StorageImportNotInProgress = "STORAGE_IMPORT_NOT_IN_PROGRESS",
    StorageInUse = "STORAGE_IN_USE",
    StorageInconsistent = "STORAGE_INCONSISTENT",
    StorageInvalid = "STORAGE_INVALID",
    StorageMissing = "STORAGE_MISSING",
    StorageNotFound = "STORAGE_NOT_FOUND",
    StorageResourcesUnavailable = "STORAGE_RESOURCES_UNAVAILABLE",
    StorageStateIllegal = "STORAGE_STATE_ILLEGAL",
    StorageTierIllegal = "STORAGE_TIER_ILLEGAL",
    StorageTitleInvalid = "STORAGE_TITLE_INVALID",
    StorageTitleMissing = "STORAGE_TITLE_MISSING",
    StorageTypeIllegal = "STORAGE_TYPE_ILLEGAL",
    StrictAntiAffinityNotMet = "STRICT_ANTI_AFFINITY_NOT_MET",
    TagExists = "TAG_EXISTS",
    TagForbidden = "TAG_FORBIDDEN",
    TagInvalid = "TAG_INVALID",
    TagNotFound = "TAG_NOT_FOUND",
    TargetIdentifierInvalid = "TARGET_IDENTIFIER_INVALID",
    TargetTypeInvalid = "TARGET_TYPE_INVALID",
    TierInvalid = "TIER_INVALID",
    TimeoutActionInvalid = "TIMEOUT_ACTION_INVALID",
    TimeoutInvalid = "TIMEOUT_INVALID",
    TimeoutMissing = "TIMEOUT_MISSING",
    TimezoneInvalid = "TIMEZONE_INVALID",
    TitleInvalid = "TITLE_INVALID",
    TitleMissing = "TITLE_MISSING",
    TooManyBootDisks = "TOO_MANY_BOOT_DISKS",
    TrialPlan = "TRIAL_PLAN",
    TypeInvalid = "TYPE_INVALID",
    UnableToCancel = "UNABLE_TO_CANCEL",
    UnknownAttribute = "UNKNOWN_ATTRIBUTE",
    UserDataInvalid = "USER_DATA_INVALID",
    UserInvalid = "USER_INVALID",
    UserNotFound = "USER_NOT_FOUND",
    ValidationError = "VALIDATION_ERROR",
    VideoModelInvalid = "VIDEO_MODEL_INVALID",
    VncInvalid = "VNC_INVALID",
    VncPasswordInvalid = "VNC_PASSWORD_INVALID",
    WindowsNotAvailable = "WINDOWS_NOT_AVAILABLE",
    ZoneHostForbidden = "ZONE_HOST_FORBIDDEN",
    ZoneInvalid = "ZONE_INVALID",
    ZoneMismatch = "ZONE_MISMATCH",
    ZoneMissing = "ZONE_MISSING",
    ZoneNotFound = "ZONE_NOT_FOUND",
}

// ─── ApiError ──────────────────────────────────────────────────────────────────

/// An error returned by the UpCloud API.
///
/// Contains the HTTP status code and the structured error body from the API
/// response envelope: `{"error": {"error_code": "...", "error_message": "..."}}`.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// Machine-readable error code (e.g. [`ErrorCode::ServerNotFound`]).
    pub error_code: ErrorCode,
    /// Human-readable error description.
    pub error_message: String,
    /// HTTP status code (e.g. 404, 409).
    pub status: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.error_message, self.error_code)
    }
}

impl std::error::Error for ApiError {}

/// The UpCloud error response envelope: `{ "error": { "error_code": ..., "error_message": ... } }`.
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorBody {
    pub error_code: String,
    pub error_message: String,
}
