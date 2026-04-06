use super::*;

const PARTNER_ACCOUNT_LIST_JSON: &str = r#"[
  {
"address": "Some street",
"city": "Some city",
"company": "Some company",
"country": "FIN",
"email": "someuser@example.com",
"first_name": "Some",
"last_name": "User",
"phone": "+358.91234567",
"postal_code": "00100",
"state": "",
"username": "someuser",
"vat_number": ""
  },
  {
"address": "Another street",
"city": "Another city",
"company": "Another company",
"country": "FIN",
"email": "anotheruser@example.com",
"first_name": "Another",
"last_name": "User",
"phone": "+358.97654321",
"postal_code": "90100",
"state": "",
"username": "anotheruser",
"vat_number": ""
  }
]"#;

const PARTNER_ACCOUNT_CREATE_JSON: &str = r#"{
  "address": "",
  "city": "",
  "company": "",
  "country": "FIN",
  "email": "newuser@example.com",
  "first_name": "New",
  "last_name": "User",
  "phone": "+358.91111111",
  "postal_code": "",
  "state": "",
  "username": "newuser",
  "vat_number": ""
}"#;

#[test]
fn deserialize_partner_account_list() {
    let accounts: Vec<PartnerAccount> = serde_json::from_str(PARTNER_ACCOUNT_LIST_JSON).unwrap();
    assert_eq!(accounts.len(), 2);

    assert_eq!(accounts[0].username, "someuser");
    assert_eq!(accounts[0].first_name.as_deref(), Some("Some"));
    assert_eq!(accounts[0].last_name.as_deref(), Some("User"));
    assert_eq!(accounts[0].company.as_deref(), Some("Some company"));
    assert_eq!(accounts[0].address.as_deref(), Some("Some street"));
    assert_eq!(accounts[0].postal_code.as_deref(), Some("00100"));
    assert_eq!(accounts[0].city.as_deref(), Some("Some city"));
    assert_eq!(accounts[0].state.as_deref(), Some(""));
    assert_eq!(accounts[0].country.as_deref(), Some("FIN"));
    assert_eq!(accounts[0].email.as_deref(), Some("someuser@example.com"));
    assert_eq!(accounts[0].phone.as_deref(), Some("+358.91234567"));
    assert_eq!(accounts[0].vat_number.as_deref(), Some(""));

    assert_eq!(accounts[1].username, "anotheruser");
    assert_eq!(accounts[1].first_name.as_deref(), Some("Another"));
    assert_eq!(accounts[1].last_name.as_deref(), Some("User"));
    assert_eq!(accounts[1].company.as_deref(), Some("Another company"));
    assert_eq!(accounts[1].address.as_deref(), Some("Another street"));
    assert_eq!(accounts[1].postal_code.as_deref(), Some("90100"));
    assert_eq!(accounts[1].city.as_deref(), Some("Another city"));
    assert_eq!(accounts[1].state.as_deref(), Some(""));
    assert_eq!(accounts[1].country.as_deref(), Some("FIN"));
    assert_eq!(
        accounts[1].email.as_deref(),
        Some("anotheruser@example.com")
    );
    assert_eq!(accounts[1].phone.as_deref(), Some("+358.97654321"));
    assert_eq!(accounts[1].vat_number.as_deref(), Some(""));
}

#[test]
fn deserialize_partner_account_create_response() {
    let account: PartnerAccount = serde_json::from_str(PARTNER_ACCOUNT_CREATE_JSON).unwrap();
    assert_eq!(account.username, "newuser");
    assert_eq!(account.first_name.as_deref(), Some("New"));
    assert_eq!(account.last_name.as_deref(), Some("User"));
    assert_eq!(account.company.as_deref(), Some(""));
    assert_eq!(account.address.as_deref(), Some(""));
    assert_eq!(account.postal_code.as_deref(), Some(""));
    assert_eq!(account.city.as_deref(), Some(""));
    assert_eq!(account.state.as_deref(), Some(""));
    assert_eq!(account.country.as_deref(), Some("FIN"));
    assert_eq!(account.email.as_deref(), Some("newuser@example.com"));
    assert_eq!(account.phone.as_deref(), Some("+358.91111111"));
    assert_eq!(account.vat_number.as_deref(), Some(""));
}

const CREATE_REQUEST_WITH_CONTACT_JSON: &str = r#"{
  "username": "newuser",
  "password": "superSecret123",
  "email": "newuser@example.com",
  "terms_of_service_agreed": true,
  "contact_details": {
"country": "FIN",
"email": "newuser@example.com",
"first_name": "New",
"last_name": "User",
"phone": "+358.91111111"
  }
}"#;

#[test]
fn deserialize_create_request_with_contact_details() {
    let req: CreatePartnerAccountRequest =
        serde_json::from_str(CREATE_REQUEST_WITH_CONTACT_JSON).unwrap();
    assert_eq!(req.username, "newuser");
    assert_eq!(req.password, "superSecret123");
    assert_eq!(req.email, "newuser@example.com");
    assert!(req.terms_of_service_agreed);
    assert!(req.first_name.is_none());
    assert!(req.last_name.is_none());
    assert!(req.company.is_none());
    assert!(req.address.is_none());
    assert!(req.postal_code.is_none());
    assert!(req.city.is_none());
    assert!(req.state.is_none());
    assert!(req.country.is_none());
    assert!(req.phone.is_none());
    assert!(req.vat_number.is_none());
    assert!(req.tag_cloud.is_none());

    assert!(req.contact_details.is_some());
    let cd = req.contact_details.unwrap();
    assert_eq!(cd.country.as_deref(), Some("FIN"));
    assert_eq!(cd.email.as_deref(), Some("newuser@example.com"));
    assert_eq!(cd.first_name.as_deref(), Some("New"));
    assert_eq!(cd.last_name.as_deref(), Some("User"));
    assert_eq!(cd.phone.as_deref(), Some("+358.91111111"));
    assert!(cd.company.is_none());
    assert!(cd.address.is_none());
    assert!(cd.postal_code.is_none());
    assert!(cd.city.is_none());
    assert!(cd.state.is_none());
    assert!(cd.vat_number.is_none());
}

const CREATE_REQUEST_WITHOUT_CONTACT_JSON: &str = r#"{
  "username": "newuser",
  "password": "superSecret123",
  "email": "newuser@example.com",
  "terms_of_service_agreed": true
}"#;

#[test]
fn deserialize_create_request_without_contact_details() {
    let req: CreatePartnerAccountRequest =
        serde_json::from_str(CREATE_REQUEST_WITHOUT_CONTACT_JSON).unwrap();
    assert_eq!(req.username, "newuser");
    assert_eq!(req.password, "superSecret123");
    assert_eq!(req.email, "newuser@example.com");
    assert!(req.terms_of_service_agreed);
    assert!(req.first_name.is_none());
    assert!(req.last_name.is_none());
    assert!(req.company.is_none());
    assert!(req.address.is_none());
    assert!(req.postal_code.is_none());
    assert!(req.city.is_none());
    assert!(req.state.is_none());
    assert!(req.country.is_none());
    assert!(req.phone.is_none());
    assert!(req.vat_number.is_none());
    assert!(req.tag_cloud.is_none());
    assert!(req.contact_details.is_none());
}

// ─── Partner account detail / modify tests (official docs, page 23) ──────

const PARTNER_ACCOUNT_DETAIL_JSON: &str = r#"{
  "address": "Some street",
  "city": "Some city",
  "company": "Some company",
  "country": "FIN",
  "email": "someuser@example.com",
  "first_name": "Some",
  "last_name": "User",
  "phone": "+358.91234567",
  "postal_code": "00100",
  "state": "",
  "username": "someuser",
  "vat_number": ""
}"#;

const MODIFY_PARTNER_ACCOUNT_REQUEST_JSON: &str = r#"{
  "first_name": "Updated",
  "last_name": "User",
  "company": "Updated company",
  "email": "updated@example.com"
}"#;

const MODIFY_PARTNER_ACCOUNT_RESPONSE_JSON: &str = r#"{
  "address": "Some street",
  "city": "Some city",
  "company": "Updated company",
  "country": "FIN",
  "email": "updated@example.com",
  "first_name": "Updated",
  "last_name": "User",
  "phone": "+358.91234567",
  "postal_code": "00100",
  "state": "",
  "username": "someuser",
  "vat_number": ""
}"#;

#[test]
fn deserialize_partner_account_detail() {
    let account: PartnerAccount = serde_json::from_str(PARTNER_ACCOUNT_DETAIL_JSON).unwrap();
    assert_eq!(account.username, "someuser");
    assert_eq!(account.first_name.as_deref(), Some("Some"));
    assert_eq!(account.last_name.as_deref(), Some("User"));
    assert_eq!(account.company.as_deref(), Some("Some company"));
    assert_eq!(account.address.as_deref(), Some("Some street"));
    assert_eq!(account.postal_code.as_deref(), Some("00100"));
    assert_eq!(account.city.as_deref(), Some("Some city"));
    assert_eq!(account.state.as_deref(), Some(""));
    assert_eq!(account.country.as_deref(), Some("FIN"));
    assert_eq!(account.email.as_deref(), Some("someuser@example.com"));
    assert_eq!(account.phone.as_deref(), Some("+358.91234567"));
    assert_eq!(account.vat_number.as_deref(), Some(""));
}

#[test]
fn deserialize_modify_partner_account_request() {
    let req: ModifyPartnerAccountRequest =
        serde_json::from_str(MODIFY_PARTNER_ACCOUNT_REQUEST_JSON).unwrap();
    assert_eq!(req.first_name.as_deref(), Some("Updated"));
    assert_eq!(req.last_name.as_deref(), Some("User"));
    assert_eq!(req.company.as_deref(), Some("Updated company"));
    assert_eq!(req.email.as_deref(), Some("updated@example.com"));
    assert!(req.address.is_none());
    assert!(req.postal_code.is_none());
    assert!(req.city.is_none());
    assert!(req.state.is_none());
    assert!(req.country.is_none());
    assert!(req.phone.is_none());
    assert!(req.vat_number.is_none());
}

#[test]
fn deserialize_modify_partner_account_response() {
    let account: PartnerAccount =
        serde_json::from_str(MODIFY_PARTNER_ACCOUNT_RESPONSE_JSON).unwrap();
    assert_eq!(account.username, "someuser");
    assert_eq!(account.first_name.as_deref(), Some("Updated"));
    assert_eq!(account.last_name.as_deref(), Some("User"));
    assert_eq!(account.company.as_deref(), Some("Updated company"));
    assert_eq!(account.address.as_deref(), Some("Some street"));
    assert_eq!(account.postal_code.as_deref(), Some("00100"));
    assert_eq!(account.city.as_deref(), Some("Some city"));
    assert_eq!(account.state.as_deref(), Some(""));
    assert_eq!(account.country.as_deref(), Some("FIN"));
    assert_eq!(account.email.as_deref(), Some("updated@example.com"));
    assert_eq!(account.phone.as_deref(), Some("+358.91234567"));
    assert_eq!(account.vat_number.as_deref(), Some(""));
}
