# Zones

> Source: https://developers.upcloud.com/1.3/5-zones

A zone identifies the physical site where the cloud services are located. Different zones may be used to provide geographical and logical separation of servers.

All zones have their own storage network. Storages can only be attached to servers located in the same zone. In order to use storages from other zones, storages can be transferred between zones by cloning.

## List available zones

Returns a list of available zones.

All servers and storages must set their zone attributes to one of the zone IDs returned by this operation.

Private cloud zones have their `public` attribute set to `no`.

Private cloud zones have their parent zones set to whatever the zone containing them is.

### Request

`GET /1.3/zone`

### Response

```json
{
  "zones": {
    "zone": [
      {
        "description": "Frankfurt #1",
        "id": "de-fra1",
        "public": "yes"
      },
      {
        "description": "Helsinki #1",
        "id": "fi-hel1",
        "public": "yes"
      },
      {
        "description": "Fake private cloud",
        "id": "fi-fake-priv-cloud",
        "parent_zone": "fi-hel1",
        "public": "no"
      },
      {
        "description": "Helsinki #2",
        "id": "fi-hel2",
        "public": "yes"
      },
      {
        "description": "Amsterdam #1",
        "id": "nl-ams1",
        "public": "yes"
      },
      {
        "description": "Singapore #1",
        "id": "sg-sin1",
        "public": "yes"
      },
      {
        "description": "London #1",
        "id": "uk-lon1",
        "public": "yes"
      },
      {
        "description": "Chicago #1",
        "id": "us-chi1",
        "public": "yes"
      },
      {
        "description": "San Jose #1",
        "id": "us-sjo1",
        "public": "yes"
      }
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `id` | String | Unique zone identifier (e.g. `de-fra1`). Used when creating servers and storages. |
| `description` | String | Human-readable description of the zone (e.g. `Helsinki #1`). |
| `public` | String (`yes` / `no`) | Whether the zone is publicly available. Private cloud zones have this set to `no`. |
| `parent_zone` | String (optional) | The ID of the parent zone. Only present for private cloud zones; identifies the public zone that contains this private zone. |
