//! Partner types.

use serde::{Deserialize, Serialize};

/// A partner-managed account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartnerAccount {
    /// Account username.
    pub username: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal / zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State (populated when country is `USA`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 alpha-3 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number in international format (`+countrycode.nationalpart`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

/// Request body for creating a partner account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartnerAccountRequest {
    /// Account username.
    pub username: String,
    /// Account password.
    pub password: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal / zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State (required when country is `USA`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Three-character country code (ISO 3166-1 alpha-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// E-mail address.
    pub email: String,
    /// Phone number in international format (`+countrycode.nationalpart`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Whether the user agreed to the terms.
    pub terms_of_service_agreed: bool,
    /// Enable tag cloud features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_cloud: Option<bool>,
    /// Account contact details. If omitted, contact details are copied from the partner account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<PartnerContactDetails>,
}

/// Contact details for a partner account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartnerContactDetails {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal / zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State (required when country is `USA`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Three-character country code (ISO 3166-1 alpha-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number in international format (`+countrycode.nationalpart`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

/// Request body for modifying a partner account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyPartnerAccountRequest {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal / zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State (populated when country is `USA`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 alpha-3 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number in international format (`+countrycode.nationalpart`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}
