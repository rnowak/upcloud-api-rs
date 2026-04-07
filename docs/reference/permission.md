# Permissions

Permissions are used to manage access to resources by subaccounts. A main account
user can choose to expose only certain resources to certain subaccounts. Wildcard
permission entries (`*`) are also possible to give a subaccount access to all
resources within a specific type of service.

When a subaccount creates new resources it automatically receives permission
entries for those particular resources.

---

## Get permissions

Returns a list of permission objects. For main accounts all defined permissions
are returned. For subaccounts only the applicable permissions are returned.

### Request

```
GET /1.3/permission HTTP/1.1
```

### Normal response — main account (`main_account_user1`)

```
HTTP/1.1 200 OK
```

```json
{
  "permissions": {
    "permission": [
      {
        "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
        "target_type": "managed_loadbalancer",
        "user": "sub_account_user1"
      },
      {
        "target_identifier": "0973a187-3ede-4jze-133e-85ea3e61b5bc",
        "target_type": "managed_database",
        "user": "sub_account_user1"
      },
      {
        "target_identifier": "1203a187-3ede-4aae-883e-85ea3e69babc",
        "target_type": "managed_object_storage",
        "user": "sub_account_user1"
      },
      {
        "options": {
          "storage": "yes"
        },
        "target_identifier": "tag1",
        "target_type": "tag_access",
        "user": "sub_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "router",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "server",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "storage",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "managed_database",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "managed_loadbalancer",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "tag_access",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "managed_object_storage",
        "user": "main_account_user1"
      },
      {
        "target_identifier": "*",
        "target_type": "network",
        "user": "main_account_user1"
      }
    ]
  }
}
```

For descriptions of the attributes refer to [Grant permission](#grant-permission).

### Normal response — subaccount (`sub_account_user1`)

```
HTTP/1.1 200 OK
```

```json
{
  "permissions": {
    "permission": [
      {
        "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
        "target_type": "managed_loadbalancer",
        "user": "sub_account_user1"
      },
      {
        "target_identifier": "1203a187-3ede-4lwe-884v-25ea3e69babc",
        "target_type": "managed_object_storage",
        "user": "sub_account_user1"
      },
      {
        "options": {
          "storage": "yes"
        },
        "target_identifier": "tag1",
        "target_type": "tag_access",
        "user": "sub_account_user1"
      }
    ]
  }
}
```

For descriptions of the attributes refer to [Grant permission](#grant-permission).

---

## Grant permission

Grants a permission to a subaccount. Only the main account user can grant
permissions.

The call is idempotent — if a permission with matching target type, identifier,
and user already exists it will be replaced. Omitting the `options` attribute
means "don't change options".

Grants where the `user` attribute refers to a main account have no effect because
the main account has implicit access to all resources.

It is currently not possible to grant permissions to users that are not part of
the main account used to manage the permissions.

### Request

```
POST /1.3/permission/grant HTTP/1.1
```

```json
{
  "permission": {
    "options": {},
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}
```

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "permission": {
    "options": {},
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `target_identifier` | Valid UUID / `*` | yes | Specifies a resource to grant access to or a wildcard |
| `target_type` | Valid target type, see below | yes | Specifies the target type to grant access to |
| `user` | Valid subaccount username | yes | Specifies the user to grant access to |
| `options` | Valid options object, see below | no | Specifies target type specific options |

### Valid target types

| Target type | Description |
|---|---|
| `server` | Virtual server |
| `storage` | Storage for a virtual server |
| `network` | Private SDN Network |
| `router` | Private SDN Router |
| `network_gateway` | Network gateway |
| `managed_database` | Managed Database |
| `managed_kubernetes` | Managed Kubernetes |
| `managed_loadbalancer` | Managed Load Balancer |
| `managed_object_storage` | Managed Object Storage |
| `tag_access` | Accounts with access to this tag automatically have access to virtual servers with this tag |

### Valid options

| Option | Applicable target type | Accepted value | Description |
|---|---|---|---|
| `storage` | `tag_access` | `yes` / `no` | Whether the tag-referenced server's storages are accessible |

### Error responses

| HTTP status | Error code | Description |
|---|---|---|
| 400 Bad Request | `TARGET_TYPE_INVALID` | Invalid target type. |
| 400 Bad Request | `TARGET_IDENTIFIER_INVALID` | Invalid target identifier. |
| 400 Bad Request | `USER_INVALID` | Invalid user name. |
| 400 Bad Request | `INVALID_OPTIONS` | Either unknown or not supported options. |
| 403 Forbidden | `ACTION_FORBIDDEN` | A subaccount cannot manage permissions. |
| 403 Forbidden | `ACCOUNT_FORBIDDEN` | The account referenced by `user` either doesn't exist or is not part of the main account. |

---

## Revoke permission

Revokes a permission from a subaccount user account. Only the main account can
revoke permissions.

The call is idempotent — if no existing permission matches the target type,
identifier, and user, the call does nothing. Revoking a wildcard permission does
not remove more specific ones. The `options` attribute has no meaning when
revoking.

Revokes where the `user` attribute refers to a main account have no effect
because the main account has implicit access to all resources.

### Request

```
POST /1.3/permission/revoke HTTP/1.1
```

```json
{
  "permission": {
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}
```

### Normal response

```
HTTP/1.1 204 No Content
```

### Attributes

For descriptions of the attributes refer to [Grant permission](#grant-permission).
