# Partner API

> **Source:** <https://developers.upcloud.com/1.3/23-partner-api>

Partner API gives [UpCloud partners](https://upcloud.com/partners) the ability to
create new accounts for their end users.

> **Note:** Use of this API is available only to accounts provisioned as partner
> accounts. Contact your account representative for details.

---

## Get account list

Returns the list of accounts associated with the partner.

### Request

```
GET /1.3/partner/accounts HTTP/1.1
```

### Normal response

```
HTTP/1.1 200 OK
```

```json
[
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
]
```

### Response attributes

| Attribute     | Description                                                                 |
|---------------|-----------------------------------------------------------------------------|
| `username`    | Account username.                                                           |
| `first_name`  | First name.                                                                 |
| `last_name`   | Last name.                                                                  |
| `company`     | Company name.                                                               |
| `address`     | Street address.                                                             |
| `postal_code` | Postal / zip code.                                                          |
| `city`        | City.                                                                       |
| `state`       | State (populated when country is `USA`).                                    |
| `country`     | ISO 3166-1 alpha-3 country code.                                            |
| `email`       | E-mail address.                                                             |
| `phone`       | Phone number in international format (`+countrycode.nationalpart`).         |
| `vat_number`  | VAT number.                                                                 |

---

## Create new account

Creates a new UpCloud account linked to the partner's existing invoicing.

### Request (without contact details)

```
POST /1.3/partner/accounts HTTP/1.1
```

```json
{
   "username": "newuser",
   "password": "superSecret123"
}
```

### Request (with minimal contact details)

```
POST /1.3/partner/accounts HTTP/1.1
```

```json
{
   "username": "newuser",
   "password": "superSecret123",
   "contact_details": {
      "country": "FIN",
      "email": "newuser@example.com",
      "first_name": "New",
      "last_name": "User",
      "phone": "+358.91111111"
   }
}
```

### Request attributes

| Attribute          | Accepted value | Required | Description |
|--------------------|----------------|----------|-------------|
| `username`         | 4-64 characters matching `^[a-z0-9]+(_[a-z0-9]+)*$` | yes | Account username. |
| `password`         | 8-256 characters; at least one lowercase letter (`a-z`), one uppercase letter (`A-Z`) and one digit (`0-9`) | yes | Account password. |
| `contact_details`  | JSON object (see below) | no | Account contact details. If omitted, contact details are copied from the partner account. |

### Contact details attributes

| Attribute      | Accepted value | Required | Description |
|----------------|----------------|----------|-------------|
| `first_name`   | 1-50 characters | yes | First name. |
| `last_name`    | 1-50 characters | yes | Last name. |
| `company`      | 1-100 characters | no | Company name. |
| `address`      | 1 or 2 lines, each 1-100 characters | no | Street address. |
| `postal_code`  | 1-20 characters | no | Postal / zip code. |
| `city`         | 1-100 characters | no | City. |
| `state`        | Valid U.S. state | yes if country is `USA` | State. |
| `country`      | [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3) code | yes | Three-character country code. |
| `phone`        | `+countrycode.nationalpart` | yes | Phone number in international format. |
| `email`        | Valid e-mail address | yes | E-mail address. |
| `vat_number`   | Valid VAT number | no | VAT number for the contact's country. |

### Normal response

```
HTTP/1.1 201 Created
```

```json
{
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
}
```

### Error responses

| HTTP status      | Error code                        | Description |
|------------------|-----------------------------------|-------------|
| 403 Forbidden    | `ACCOUNT_CREATION_LIMIT_REACHED`  | Partner-specific account creation limit has been reached. |
| 409 Conflict     | `ACCOUNT_EXISTS`                  | An account with the given `username` already exists. |
| 400 Bad Request  | `ADDRESS_INVALID`                 | The attribute `address` has an invalid value. |
| 400 Bad Request  | `CITY_INVALID`                    | The attribute `city` has an invalid value. |
| 400 Bad Request  | `COMPANY_INVALID`                 | The attribute `company` has an invalid value. |
| 400 Bad Request  | `COUNTRY_INVALID`                 | The attribute `country` has an invalid value. |
| 400 Bad Request  | `COUNTRY_MISSING`                 | The attribute `country` is missing. |
| 400 Bad Request  | `EMAIL_INVALID`                   | The attribute `email` has an invalid value. |
| 400 Bad Request  | `EMAIL_MISSING`                   | The attribute `email` is missing. |
| 400 Bad Request  | `FIRST_NAME_INVALID`              | The attribute `first_name` has an invalid value. |
| 400 Bad Request  | `FIRST_NAME_MISSING`              | The attribute `first_name` is missing. |
| 400 Bad Request  | `LAST_NAME_INVALID`               | The attribute `last_name` has an invalid value. |
| 400 Bad Request  | `LAST_NAME_MISSING`               | The attribute `last_name` is missing. |
| 400 Bad Request  | `PASSWORD_INVALID`                | The attribute `password` has an invalid value. |
| 400 Bad Request  | `PASSWORD_MISSING`                | The attribute `password` is missing. |
| 400 Bad Request  | `PHONE_INVALID`                   | The attribute `phone` has an invalid value. |
| 400 Bad Request  | `PHONE_MISSING`                   | The attribute `phone` is missing. |
| 400 Bad Request  | `POSTAL_CODE_INVALID`             | The attribute `postal_code` has an invalid value. |
| 400 Bad Request  | `STATE_INVALID`                   | The attribute `state` has an invalid value. |
| 400 Bad Request  | `STATE_MISSING`                   | The attribute `state` is missing. |
| 400 Bad Request  | `USERNAME_INVALID`                | The attribute `username` has an invalid value. |
| 400 Bad Request  | `USERNAME_MISSING`                | The attribute `username` is missing. |
| 400 Bad Request  | `VAT_NUMBER_INVALID`              | The attribute `vat_number` has an invalid value. |

---

## Access account billing summary

Partners can access the billing summary of associated accounts using the
[Get monthly billing summary with resource details](https://developers.upcloud.com/1.3/3-accounts/#get-monthly-billing-summary-with-resource-details)
request with the URL query parameter `username`.

### Request

```
GET /1.3/account/billing/summary/{year}-{month}?username={username} HTTP/1.1
```

### Example

Retrieve the May 2025 billing summary for account `someuser`:

```
GET /1.3/account/billing/summary/2025-05?username=someuser HTTP/1.1
```
