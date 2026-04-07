# Timezones

> Source: https://developers.upcloud.com/1.3/6-timezones

A cloud server's hardware clock is set to a specific timezone. Some operating
systems, such as Microsoft Windows, rely on the hardware clock being in local
time. Others, such as Linux, use a default setting of a hardware clock in
Coordinated Universal Time (UTC). The server's timezone setting can be adjusted
to a local time if required by the operating system.

## List timezones

Returns a list of available timezones. Timezones are used to set the hardware
clock for servers.

The default server timezone is UTC unless configured to one of the timezone
identifiers returned by this operation.

### Request

`GET /1.3/timezone`

### Response

```json
{
  "timezones": {
    "timezone": [
      "Africa/Abidjan",
      "Africa/Accra",
      "Africa/Addis_Ababa",
      "...",
      "Pacific/Truk",
      "Pacific/Wake",
      "Pacific/Wallis",
      "UTC"
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| timezones | object | Wrapper object for the timezone list. |
| timezones.timezone | array of strings | List of available timezone identifiers (e.g. `Africa/Abidjan`, `UTC`). The default timezone is `UTC`. |
