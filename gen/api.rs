pub use progenitor_client::{ByteStream, Error, ResponseValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenLookupAccessorBody {
        #[doc = "Accessor of the token to look up (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accessor: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenLookupBody {
        #[doc = "Token to lookup (POST request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenLookupSelfBody {
        #[doc = "Token to look up (unused, does not need to be set)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRenewAccessorBody {
        #[doc = "Accessor of the token to renew (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accessor: Option<String>,
        #[doc = "The desired increment in seconds to the token expiration"]
        #[serde(default)]
        pub increment: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRenewBody {
        #[doc = "The desired increment in seconds to the token expiration"]
        #[serde(default)]
        pub increment: u64,
        #[doc = "Token to renew (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRenewSelfBody {
        #[doc = "The desired increment in seconds to the token expiration"]
        #[serde(default)]
        pub increment: u64,
        #[doc = "Token to renew (unused, does not need to be set)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRevokeAccessorBody {
        #[doc = "Accessor of the token (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accessor: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRevokeBody {
        #[doc = "Token to revoke (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRevokeOrphanBody {
        #[doc = "Token to revoke (request body)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostAuthTokenRolesRoleNameBody {
        #[doc = "String or JSON list of allowed entity aliases. If set, specifies the entity aliases which are allowed to be used during token generation. This field supports globbing."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_entity_aliases: Vec<String>,
        #[doc = "If set, tokens can be created with any subset of the policies in this list, rather than the normal semantics of tokens being a subset of the calling token's policies. The parameter is a comma-delimited string of policy names."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_policies: Vec<String>,
        #[doc = "Use 'token_bound_cidrs' instead."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub bound_cidrs: Vec<String>,
        #[doc = "If set, successful token creation via this role will require that no policies in the given list are requested. The parameter is a comma-delimited string of policy names."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub disallowed_policies: Vec<String>,
        #[doc = "Use 'token_explicit_max_ttl' instead."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub explicit_max_ttl: Option<u64>,
        #[doc = "If true, tokens created via this role will be orphan tokens (have no parent)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub orphan: Option<bool>,
        #[doc = "If set, tokens created via this role will contain the given suffix as a part of their path. This can be used to assist use of the 'revoke-prefix' endpoint later on. The given suffix must match the regular expression.\\w[\\w-.]+\\w"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path_suffix: Option<String>,
        #[doc = "Use 'token_period' instead."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub period: Option<u64>,
        #[doc = "Tokens created via this role will be renewable or not according to this value. Defaults to \"true\"."]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub renewable: bool,
        #[doc = "Comma separated string or JSON list of CIDR blocks. If set, specifies the blocks of IP addresses which are allowed to use the generated token."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub token_bound_cidrs: Vec<String>,
        #[doc = "If set, tokens created via this role carry an explicit maximum TTL. During renewal, the current maximum TTL values of the role and the mount are not checked for changes, and any updates to these values will have no effect on the token being renewed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_explicit_max_ttl: Option<u64>,
        #[doc = "If true, the 'default' policy will not automatically be added to generated tokens"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_no_default_policy: Option<bool>,
        #[doc = "The maximum number of times a token may be used, a value of zero means unlimited"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_num_uses: Option<i64>,
        #[doc = "If set, tokens created via this role will have no max lifetime; instead, their renewal period will be fixed to this value. This takes an integer number of seconds, or a string duration (e.g. \"24h\")."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_period: Option<u64>,
        #[doc = "The type of token to generate, service or batch"]
        #[serde(default = "post_auth_token_roles_role_name_body_token_type")]
        pub token_type: String,
    }

    fn post_auth_token_roles_role_name_body_token_type() -> String {
        "default-service".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityAliasBody {
        #[doc = "Entity ID to which this alias belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "Entity ID to which this alias belongs to. This field is deprecated in favor of 'canonical_id'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[doc = "ID of the alias"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Mount accessor to which this alias belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Name of the alias"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityAliasIdIdBody {
        #[doc = "Entity ID to which this alias should be tied to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "Entity ID to which this alias should be tied to. This field is deprecated in favor of 'canonical_id'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[doc = "Mount accessor to which this alias belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Name of the alias"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityAliasBody {
        #[doc = "Entity ID to which this alias belongs"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "Entity ID to which this alias belongs. This field is deprecated, use canonical_id."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[doc = "ID of the entity alias. If set, updates the corresponding entity alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Mount accessor to which this alias belongs to; unused for a modify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Name of the alias; unused for a modify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityAliasIdIdBody {
        #[doc = "Entity ID to which this alias should be tied to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "Entity ID to which this alias belongs to. This field is deprecated, use canonical_id."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[doc = "(Unused)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "(Unused)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityBatchDeleteBody {
        #[doc = "Entity IDs to delete"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub entity_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityBody {
        #[doc = "If set true, tokens tied to this identity will not be able to be used (but will not be revoked)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[doc = "ID of the entity. If set, updates the corresponding existing entity."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the entity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Policies to be tied to the entity."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityIdIdBody {
        #[doc = "If set true, tokens tied to this identity will not be able to be used (but will not be revoked)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the entity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Policies to be tied to the entity."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityMergeBody {
        #[doc = "Setting this will follow the 'mine' strategy for merging MFA secrets. If there are secrets of the same type both in entities that are merged from and in entity into which all others are getting merged, secrets in the destination will be unaltered. If not set, this API will throw an error containing all the conflicts."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub force: Option<bool>,
        #[doc = "Entity IDs which needs to get merged"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub from_entity_ids: Vec<String>,
        #[doc = "Entity ID into which all the other entities need to get merged"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to_entity_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityEntityNameNameBody {
        #[doc = "If set true, tokens tied to this identity will not be able to be used (but will not be revoked)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[doc = "ID of the entity. If set, updates the corresponding existing entity."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Policies to be tied to the entity."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityGroupAliasBody {
        #[doc = "ID of the group to which this is an alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "ID of the group alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Mount accessor to which this alias belongs to."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Alias of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityGroupAliasIdIdBody {
        #[doc = "ID of the group to which this is an alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub canonical_id: Option<String>,
        #[doc = "Mount accessor to which this alias belongs to."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Alias of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityGroupBody {
        #[doc = "ID of the group. If set, updates the corresponding existing group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Entity IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_entity_ids: Vec<String>,
        #[doc = "Group IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_group_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Policies to be tied to the group."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
        #[doc = "Type of the group, 'internal' or 'external'. Defaults to 'internal'"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityGroupIdIdBody {
        #[doc = "Entity IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_entity_ids: Vec<String>,
        #[doc = "Group IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_group_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Policies to be tied to the group."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
        #[doc = "Type of the group, 'internal' or 'external'. Defaults to 'internal'"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityGroupNameNameBody {
        #[doc = "ID of the group. If set, updates the corresponding existing group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Entity IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_entity_ids: Vec<String>,
        #[doc = "Group IDs to be assigned as group members."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub member_group_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Policies to be tied to the group."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
        #[doc = "Type of the group, 'internal' or 'external'. Defaults to 'internal'"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityLookupEntityBody {
        #[doc = "ID of the alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_id: Option<String>,
        #[doc = "Accessor of the mount to which the alias belongs to. This should be supplied in conjunction with 'alias_name'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_mount_accessor: Option<String>,
        #[doc = "Name of the alias. This should be supplied in conjunction with 'alias_mount_accessor'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_name: Option<String>,
        #[doc = "ID of the entity."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Name of the entity."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityLookupGroupBody {
        #[doc = "ID of the alias."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_id: Option<String>,
        #[doc = "Accessor of the mount to which the alias belongs to. This should be supplied in conjunction with 'alias_name'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_mount_accessor: Option<String>,
        #[doc = "Name of the alias. This should be supplied in conjunction with 'alias_mount_accessor'."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias_name: Option<String>,
        #[doc = "ID of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Name of the group."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityOidcConfigBody {
        #[doc = "Issuer URL to be used in the iss claim of the token. If not set, Vault's app_addr will be used."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityOidcIntrospectBody {
        #[doc = "Optional client_id to verify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[doc = "Token to verify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityOidcKeyNameBody {
        #[doc = "Signing algorithm to use. This will default to RS256."]
        #[serde(default = "post_identity_oidc_key_name_body_algorithm")]
        pub algorithm: String,
        #[doc = "Comma separated string or array of role client ids allowed to use this key for signing. If empty no roles are allowed. If \"*\" all roles are allowed."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_client_ids: Vec<String>,
        #[doc = "How often to generate a new keypair."]
        #[serde(default = "super::defaults::default_u64::<86400>")]
        pub rotation_period: u64,
        #[doc = "Controls how long the public portion of a key will be available for verification after being rotated."]
        #[serde(default = "super::defaults::default_u64::<86400>")]
        pub verification_ttl: u64,
    }

    fn post_identity_oidc_key_name_body_algorithm() -> String {
        "RS256".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityOidcKeyNameRotateBody {
        #[doc = "Controls how long the public portion of a key will be available for verification after being rotated. Setting verification_ttl here will override the verification_ttl set on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub verification_ttl: Option<u64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityOidcRoleNameBody {
        #[doc = "Optional client_id"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[doc = "The OIDC key to use for generating tokens. The specified key must already exist."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[doc = "The template string to use for generating tokens. This may be in string-ified JSON or base64 format."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<String>,
        #[doc = "TTL of the tokens generated against the role."]
        #[serde(default = "super::defaults::default_u64::<86400>")]
        pub ttl: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityPersonaBody {
        #[doc = "Entity ID to which this persona belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[doc = "ID of the persona"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Mount accessor to which this persona belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Name of the persona"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostIdentityPersonaIdIdBody {
        #[doc = "Entity ID to which this persona should be tied to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Mount accessor to which this persona belongs to"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_accessor: Option<String>,
        #[doc = "Name of the persona"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretConfigBody {
        #[doc = "If true, the backend will require the cas parameter to be set for each write"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cas_required: Option<bool>,
        #[doc = "If set, the length of time before a version is deleted. A negative duration disables the use of delete_version_after on all keys. A zero duration clears the current setting. Accepts a Go duration format string."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_version_after: Option<u64>,
        #[doc = "The number of versions to keep for each key. Defaults to 10"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_versions: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretDataPathBody {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub data: std::collections::HashMap<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "If provided during a read, the value at the version number will be returned"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretDeletePathBody {
        #[doc = "The versions to be archived. The versioned data will not be deleted, but it will no longer be returned in normal get requests."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretDestroyPathBody {
        #[doc = "The versions to destroy. Their data will be permanently deleted."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretMetadataPathBody {
        #[doc = "If true the key will require the cas parameter to be set on all write requests. If false, the backend???s configuration will be used."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cas_required: Option<bool>,
        #[doc = "The length of time before a version is deleted. If not set, the backend's configured delete_version_after is used. Cannot be greater than the backend's delete_version_after. A zero duration clears the current setting. A negative duration will cause an error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_version_after: Option<u64>,
        #[doc = "The number of versions to keep. If not set, the backend???s configured max version is used."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_versions: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSecretUndeletePathBody {
        #[doc = "The versions to unarchive. The versions will be restored and their data will be returned on normal get requests."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysAuditHashPathBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysAuditPathBody {
        #[doc = "User-friendly description for this audit backend."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Mark the mount as a local mount, which is not replicated and is unaffected by replication."]
        #[serde(default)]
        pub local: bool,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "The type of the backend. Example: \"mysql\""]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysAuthPathBody {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "User-friendly description for this credential backend."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Whether to give the mount access to Vault's external entropy."]
        #[serde(default)]
        pub external_entropy_access: bool,
        #[doc = "Mark the mount as a local mount, which is not replicated and is unaffected by replication."]
        #[serde(default)]
        pub local: bool,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the auth plugin to use based from the name in the plugin catalog."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plugin_name: Option<String>,
        #[doc = "Whether to turn on seal wrapping for the mount."]
        #[serde(default)]
        pub seal_wrap: bool,
        #[doc = "The type of the backend. Example: \"userpass\""]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysAuthPathTuneBody {
        #[doc = "A list of headers to whitelist and allow a plugin to set on responses."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_response_headers: Vec<String>,
        #[doc = "The list of keys in the request data object that will not be HMAC'ed by audit devices."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub audit_non_hmac_request_keys: Vec<String>,
        #[doc = "The list of keys in the response data object that will not be HMAC'ed by audit devices."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub audit_non_hmac_response_keys: Vec<String>,
        #[doc = "The default lease TTL for this mount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_lease_ttl: Option<String>,
        #[doc = "User-friendly description for this credential backend."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Determines the visibility of the mount in the UI-specific listing endpoint. Accepted value are 'unauth' and ''."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub listing_visibility: Option<String>,
        #[doc = "The max lease TTL for this mount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_lease_ttl: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "A list of headers to whitelist and pass from the request to the plugin."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub passthrough_request_headers: Vec<String>,
        #[doc = "The type of token to issue (service or batch)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_type: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysCapabilitiesAccessorBody {
        #[doc = "Accessor of the token for which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accessor: Option<String>,
        #[doc = "Use 'paths' instead."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub path: Vec<String>,
        #[doc = "Paths on which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub paths: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysCapabilitiesBody {
        #[doc = "Use 'paths' instead."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub path: Vec<String>,
        #[doc = "Paths on which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub paths: Vec<String>,
        #[doc = "Token for which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysCapabilitiesSelfBody {
        #[doc = "Use 'paths' instead."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub path: Vec<String>,
        #[doc = "Paths on which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub paths: Vec<String>,
        #[doc = "Token for which capabilities are being queried."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysConfigAuditingRequestHeadersHeaderBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hmac: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysConfigCorsBody {
        #[doc = "A comma-separated string or array of strings indicating headers that are allowed on cross-origin requests."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_headers: Vec<String>,
        #[doc = "A comma-separated string or array of strings indicating origins that may make cross-origin requests."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_origins: Vec<String>,
        #[doc = "Enables or disables CORS headers on requests."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysConfigUiHeadersHeaderBody {
        #[doc = "Returns multiple values if true"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multivalue: Option<bool>,
        #[doc = "The values to set the header."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub values: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysGenerateRootAttemptBody {
        #[doc = "Specifies a base64-encoded PGP public key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pgp_key: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysGenerateRootBody {
        #[doc = "Specifies a base64-encoded PGP public key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pgp_key: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysGenerateRootUpdateBody {
        #[doc = "Specifies a single master key share."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[doc = "Specifies the nonce of the attempt."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysInitBody {
        #[doc = "Specifies an array of PGP public keys used to encrypt the output unseal keys. Ordering is preserved. The keys must be base64-encoded from their original binary representation. The size of this array must be the same as `secret_shares`."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pgp_keys: Vec<String>,
        #[doc = "Specifies an array of PGP public keys used to encrypt the output recovery keys. Ordering is preserved. The keys must be base64-encoded from their original binary representation. The size of this array must be the same as `recovery_shares`."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub recovery_pgp_keys: Vec<String>,
        #[doc = "Specifies the number of shares to split the recovery key into."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recovery_shares: Option<i64>,
        #[doc = "Specifies the number of shares required to reconstruct the recovery key. This must be less than or equal to `recovery_shares`."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recovery_threshold: Option<i64>,
        #[doc = "Specifies a PGP public key used to encrypt the initial root token. The key must be base64-encoded from its original binary representation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root_token_pgp_key: Option<String>,
        #[doc = "Specifies the number of shares to split the master key into."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret_shares: Option<i64>,
        #[doc = "Specifies the number of shares required to reconstruct the master key. This must be less than or equal secret_shares. If using Vault HSM with auto-unsealing, this value must be the same as `secret_shares`."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret_threshold: Option<i64>,
        #[doc = "Specifies the number of shares that should be encrypted by the HSM and stored for auto-unsealing. Currently must be the same as `secret_shares`."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stored_shares: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysInternalCountersConfigBody {
        #[doc = "Number of months to report if no start date specified."]
        #[serde(default = "super::defaults::default_i64::<12>")]
        pub default_report_months: i64,
        #[doc = "Enable or disable collection of client count: enable, disable, or default."]
        #[serde(default = "post_sys_internal_counters_config_body_enabled")]
        pub enabled: String,
        #[doc = "Number of months of client data to retain. Setting to 0 will clear all existing data."]
        #[serde(default = "super::defaults::default_i64::<24>")]
        pub retention_months: i64,
    }

    fn post_sys_internal_counters_config_body_enabled() -> String {
        "default".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesLookupBody {
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesRenewBody {
        #[doc = "The desired increment in seconds to the lease"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub increment: Option<u64>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url_lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesRenewUrlLeaseIdBody {
        #[doc = "The desired increment in seconds to the lease"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub increment: Option<u64>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesRevokeBody {
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url_lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesRevokePrefixPrefixBody {
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysLeasesRevokeUrlLeaseIdBody {
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysMountsPathBody {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "User-friendly description for this mount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Whether to give the mount access to Vault's external entropy."]
        #[serde(default)]
        pub external_entropy_access: bool,
        #[doc = "Mark the mount as a local mount, which is not replicated and is unaffected by replication."]
        #[serde(default)]
        pub local: bool,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "Name of the plugin to mount based from the name registered in the plugin catalog."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plugin_name: Option<String>,
        #[doc = "Whether to turn on seal wrapping for the mount."]
        #[serde(default)]
        pub seal_wrap: bool,
        #[doc = "The type of the backend. Example: \"passthrough\""]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysMountsPathTuneBody {
        #[doc = "A list of headers to whitelist and allow a plugin to set on responses."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allowed_response_headers: Vec<String>,
        #[doc = "The list of keys in the request data object that will not be HMAC'ed by audit devices."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub audit_non_hmac_request_keys: Vec<String>,
        #[doc = "The list of keys in the response data object that will not be HMAC'ed by audit devices."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub audit_non_hmac_response_keys: Vec<String>,
        #[doc = "The default lease TTL for this mount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_lease_ttl: Option<String>,
        #[doc = "User-friendly description for this credential backend."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "Determines the visibility of the mount in the UI-specific listing endpoint. Accepted value are 'unauth' and ''."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub listing_visibility: Option<String>,
        #[doc = "The max lease TTL for this mount."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_lease_ttl: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub options: std::collections::HashMap<String, serde_json::Value>,
        #[doc = "A list of headers to whitelist and pass from the request to the plugin."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub passthrough_request_headers: Vec<String>,
        #[doc = "The type of token to issue (service or batch)."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token_type: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPluginsCatalogNameBody {
        #[doc = "The args passed to plugin command."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub args: Vec<String>,
        #[doc = "The command used to start the plugin. The executable defined in this command must exist in vault's plugin directory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command: Option<String>,
        #[doc = "The environment variables passed to plugin command. Each entry is of the form \"key=value\"."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub env: Vec<String>,
        #[doc = "The SHA256 sum of the executable used in the command field. This should be HEX encoded."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sha256: Option<String>,
        #[doc = "The SHA256 sum of the executable used in the command field. This should be HEX encoded."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sha_256: Option<String>,
        #[doc = "The type of the plugin, may be auth, secret, or database"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPluginsCatalogTypeNameBody {
        #[doc = "The args passed to plugin command."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub args: Vec<String>,
        #[doc = "The command used to start the plugin. The executable defined in this command must exist in vault's plugin directory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command: Option<String>,
        #[doc = "The environment variables passed to plugin command. Each entry is of the form \"key=value\"."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub env: Vec<String>,
        #[doc = "The SHA256 sum of the executable used in the command field. This should be HEX encoded."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sha256: Option<String>,
        #[doc = "The SHA256 sum of the executable used in the command field. This should be HEX encoded."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sha_256: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPluginsReloadBackendBody {
        #[doc = "The mount paths of the plugin backends to reload."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub mounts: Vec<String>,
        #[doc = "The name of the plugin to reload, as registered in the plugin catalog."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plugin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scope: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPoliciesAclNameBody {
        #[doc = "The rules of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPoliciesPasswordNameBody {
        #[doc = "The password policy"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysPolicyNameBody {
        #[doc = "The rules of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy: Option<String>,
        #[doc = "The rules of the policy."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rules: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysQuotasConfigBody {
        #[doc = "If set, starts audit logging of requests that get rejected due to rate limit quota rule violations."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable_rate_limit_audit_logging: Option<bool>,
        #[doc = "If set, additional rate limit quota HTTP headers will be added to responses."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable_rate_limit_response_headers: Option<bool>,
        #[doc = "Specifies the list of exempt paths from all rate limit quotas. If empty no paths will be exempt."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rate_limit_exempt_paths: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysQuotasRateLimitNameBody {
        #[doc = "If set, when a client reaches a rate limit threshold, the client will be prohibited from any further requests until after the 'block_interval' has elapsed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block_interval: Option<u64>,
        #[doc = "The duration to enforce rate limiting for (default '1s')."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<u64>,
        #[doc = "Path of the mount or namespace to apply the quota. A blank path configures a global quota. For example namespace1/ adds a quota to a full namespace, namespace1/auth/userpass adds a quota to userpass in namespace1."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rate: Option<f64>,
        #[doc = "Type of the quota rule."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRawBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRawPathBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRekeyInitBody {
        #[doc = "Specifies if using PGP-encrypted keys, whether Vault should also store a plaintext backup of the PGP-encrypted keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<bool>,
        #[doc = "Specifies an array of PGP public keys used to encrypt the output unseal keys. Ordering is preserved. The keys must be base64-encoded from their original binary representation. The size of this array must be the same as secret_shares."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pgp_keys: Vec<String>,
        #[doc = "Turns on verification functionality"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_verification: Option<bool>,
        #[doc = "Specifies the number of shares to split the master key into."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret_shares: Option<i64>,
        #[doc = "Specifies the number of shares required to reconstruct the master key. This must be less than or equal secret_shares. If using Vault HSM with auto-unsealing, this value must be the same as secret_shares."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret_threshold: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRekeyUpdateBody {
        #[doc = "Specifies a single master key share."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[doc = "Specifies the nonce of the rekey attempt."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRekeyVerifyBody {
        #[doc = "Specifies a single master share key from the new set of shares."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[doc = "Specifies the nonce of the rekey verification operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRemountBody {
        #[doc = "The previous mount point."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        #[doc = "The new mount point."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRenewBody {
        #[doc = "The desired increment in seconds to the lease"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub increment: Option<u64>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url_lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRenewUrlLeaseIdBody {
        #[doc = "The desired increment in seconds to the lease"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub increment: Option<u64>,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRevokeBody {
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url_lease_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRevokePrefixPrefixBody {
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRevokeUrlLeaseIdBody {
        #[doc = "The lease identifier to renew. This is included with a lease."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_id: Option<String>,
        #[doc = "Whether or not to perform the revocation synchronously"]
        #[serde(default = "super::defaults::default_bool::<false>")]
        pub sync: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysRotateConfigBody {
        #[doc = "Whether automatic rotation is enabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "How long after installation of an active key term that the key will be automatically rotated."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<u64>,
        #[doc = "The number of encryption operations performed before the barrier key is automatically rotated."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_operations: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysToolsHashBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_sys_tools_hash_body_algorithm")]
        pub algorithm: String,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"hex\"."]
        #[serde(default = "post_sys_tools_hash_body_format")]
        pub format: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "Algorithm to use (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlalgorithm: Option<String>,
    }

    fn post_sys_tools_hash_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_sys_tools_hash_body_format() -> String {
        "hex".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysToolsHashUrlalgorithmBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_sys_tools_hash_urlalgorithm_body_algorithm")]
        pub algorithm: String,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"hex\"."]
        #[serde(default = "post_sys_tools_hash_urlalgorithm_body_format")]
        pub format: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
    }

    fn post_sys_tools_hash_urlalgorithm_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_sys_tools_hash_urlalgorithm_body_format() -> String {
        "hex".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysToolsRandomBody {
        #[doc = "The number of bytes to generate (POST body parameter). Defaults to 32 (256 bits)."]
        #[serde(default = "super::defaults::default_i64::<32>")]
        pub bytes: i64,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"base64\"."]
        #[serde(default = "post_sys_tools_random_body_format")]
        pub format: String,
        #[doc = "The number of bytes to generate (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlbytes: Option<String>,
    }

    fn post_sys_tools_random_body_format() -> String {
        "base64".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysToolsRandomUrlbytesBody {
        #[doc = "The number of bytes to generate (POST body parameter). Defaults to 32 (256 bits)."]
        #[serde(default = "super::defaults::default_i64::<32>")]
        pub bytes: i64,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"base64\"."]
        #[serde(default = "post_sys_tools_random_urlbytes_body_format")]
        pub format: String,
    }

    fn post_sys_tools_random_urlbytes_body_format() -> String {
        "base64".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysUnsealBody {
        #[doc = "Specifies a single master key share. This is required unless reset is true."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[doc = "Specifies if previously-provided unseal keys are discarded and the unseal process is reset."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reset: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysWrappingLookupBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysWrappingRewrapBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostSysWrappingUnwrapBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitCacheConfigBody {
        #[doc = "Size of cache, use 0 for an unlimited cache size, defaults to 0"]
        #[serde(default)]
        pub size: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitDatakeyPlaintextNameBody {
        #[doc = "Number of bits for the key; currently 128, 256, and 512 bits are supported. Defaults to 256."]
        #[serde(default = "super::defaults::default_i64::<256>")]
        pub bits: i64,
        #[doc = "Context for key derivation. Required for derived keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "The version of the Vault key to use for encryption of the data key. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "Nonce for when convergent encryption v1 is used (only in Vault 0.6.1)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitDecryptNameBody {
        #[doc = "The ciphertext to decrypt, provided as returned by encrypt."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ciphertext: Option<String>,
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Base64 encoded nonce value used during encryption. Must be provided if convergent encryption is enabled for this key and the key was generated with Vault 0.6.1. Not required for keys created in 0.6.2+."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitEncryptNameBody {
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "This parameter will only be used when a key is expected to be created. Whether to support convergent encryption. This is only supported when using a key with key derivation enabled and will require all requests to carry both a context and 96-bit (12-byte) nonce. The given nonce will be used in place of a randomly generated nonce. As a result, when the same context and nonce are supplied, the same ciphertext is generated. It is *very important* when using this mode that you ensure that all nonces are unique for a given context. Failing to do so will severely impact the ciphertext's security."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub convergent_encryption: Option<bool>,
        #[doc = "The version of the key to use for encryption. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "Base64 encoded nonce value. Must be provided if convergent encryption is enabled for this key and the key was generated with Vault 0.6.1. Not required for keys created in 0.6.2+. The value must be exactly 96 bits (12 bytes) long and the user must ensure that for any given context (and thus, any given encryption key) this nonce value is **never reused**."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        #[doc = "Base64 encoded plaintext value to be encrypted"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plaintext: Option<String>,
        #[doc = "This parameter is required when encryption key is expected to be created. When performing an upsert operation, the type of key to create. Currently, \"aes128-gcm96\" (symmetric) and \"aes256-gcm96\" (symmetric) are the only types supported. Defaults to \"aes256-gcm96\"."]
        #[serde(rename = "type", default = "post_transit_encrypt_name_body_type")]
        pub type_: String,
    }

    fn post_transit_encrypt_name_body_type() -> String {
        "aes256-gcm96".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitHashBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_transit_hash_body_algorithm")]
        pub algorithm: String,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"hex\"."]
        #[serde(default = "post_transit_hash_body_format")]
        pub format: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "Algorithm to use (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlalgorithm: Option<String>,
    }

    fn post_transit_hash_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_hash_body_format() -> String {
        "hex".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitHashUrlalgorithmBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_transit_hash_urlalgorithm_body_algorithm")]
        pub algorithm: String,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"hex\"."]
        #[serde(default = "post_transit_hash_urlalgorithm_body_format")]
        pub format: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
    }

    fn post_transit_hash_urlalgorithm_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_hash_urlalgorithm_body_format() -> String {
        "hex".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitHmacNameBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_transit_hmac_name_body_algorithm")]
        pub algorithm: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The version of the key to use for generating the HMAC. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "Algorithm to use (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlalgorithm: Option<String>,
    }

    fn post_transit_hmac_name_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitHmacNameUrlalgorithmBody {
        #[doc = "Algorithm to use (POST body parameter). Valid values are: * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\"."]
        #[serde(default = "post_transit_hmac_name_urlalgorithm_body_algorithm")]
        pub algorithm: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The version of the key to use for generating the HMAC. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
    }

    fn post_transit_hmac_name_urlalgorithm_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitKeysNameBody {
        #[doc = "Enables taking a backup of the named key in plaintext format. Once set, this cannot be disabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allow_plaintext_backup: Option<bool>,
        #[doc = "Base64 encoded context for key derivation. When reading a key with key derivation enabled, if the key type supports public keys, this will return the public key for the given context."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Whether to support convergent encryption. This is only supported when using a key with key derivation enabled and will require all requests to carry both a context and 96-bit (12-byte) nonce. The given nonce will be used in place of a randomly generated nonce. As a result, when the same context and nonce are supplied, the same ciphertext is generated. It is *very important* when using this mode that you ensure that all nonces are unique for a given context. Failing to do so will severely impact the ciphertext's security."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub convergent_encryption: Option<bool>,
        #[doc = "Enables key derivation mode. This allows for per-transaction unique keys for encryption operations."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub derived: Option<bool>,
        #[doc = "Enables keys to be exportable. This allows for all the valid keys in the key ring to be exported."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[doc = "The type of key to create. Currently, \"aes128-gcm96\" (symmetric), \"aes256-gcm96\" (symmetric), \"ecdsa-p256\" (asymmetric), \"ecdsa-p384\" (asymmetric), \"ecdsa-p521\" (asymmetric), \"ed25519\" (asymmetric), \"rsa-2048\" (asymmetric), \"rsa-3072\" (asymmetric), \"rsa-4096\" (asymmetric) are supported. Defaults to \"aes256-gcm96\"."]
        #[serde(rename = "type", default = "post_transit_keys_name_body_type")]
        pub type_: String,
    }

    fn post_transit_keys_name_body_type() -> String {
        "aes256-gcm96".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitKeysNameConfigBody {
        #[doc = "Enables taking a backup of the named key in plaintext format. Once set, this cannot be disabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allow_plaintext_backup: Option<bool>,
        #[doc = "Whether to allow deletion of the key"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deletion_allowed: Option<bool>,
        #[doc = "Enables export of the key. Once set, this cannot be disabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exportable: Option<bool>,
        #[doc = "If set, the minimum version of the key allowed to be decrypted. For signing keys, the minimum version allowed to be used for verification."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_decryption_version: Option<i64>,
        #[doc = "If set, the minimum version of the key allowed to be used for encryption; or for signing keys, to be used for signing. If set to zero, only the latest version of the key is allowed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_encryption_version: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitKeysNameTrimBody {
        #[doc = "The minimum available version for the key ring. All versions before this version will be permanently deleted. This value can at most be equal to the lesser of 'min_decryption_version' and 'min_encryption_version'. This is not allowed to be set when either 'min_encryption_version' or 'min_decryption_version' is set to zero."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_available_version: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitRandomBody {
        #[doc = "The number of bytes to generate (POST body parameter). Defaults to 32 (256 bits)."]
        #[serde(default = "super::defaults::default_i64::<32>")]
        pub bytes: i64,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"base64\"."]
        #[serde(default = "post_transit_random_body_format")]
        pub format: String,
        #[doc = "The number of bytes to generate (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlbytes: Option<String>,
    }

    fn post_transit_random_body_format() -> String {
        "base64".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitRandomUrlbytesBody {
        #[doc = "The number of bytes to generate (POST body parameter). Defaults to 32 (256 bits)."]
        #[serde(default = "super::defaults::default_i64::<32>")]
        pub bytes: i64,
        #[doc = "Encoding format to use. Can be \"hex\" or \"base64\". Defaults to \"base64\"."]
        #[serde(default = "post_transit_random_urlbytes_body_format")]
        pub format: String,
    }

    fn post_transit_random_urlbytes_body_format() -> String {
        "base64".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitRestoreBody {
        #[doc = "Backed up key data to be restored. This should be the output from the 'backup/' endpoint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<String>,
        #[doc = "If set and a key by the given name exists, force the restore operation and override the key."]
        #[serde(default)]
        pub force: bool,
        #[doc = "If set, this will be the name of the restored key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitRestoreNameBody {
        #[doc = "Backed up key data to be restored. This should be the output from the 'backup/' endpoint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<String>,
        #[doc = "If set and a key by the given name exists, force the restore operation and override the key."]
        #[serde(default)]
        pub force: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitRewrapNameBody {
        #[doc = "Ciphertext value to rewrap"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ciphertext: Option<String>,
        #[doc = "Base64 encoded context for key derivation. Required for derived keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "The version of the key to use for encryption. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "Nonce for when convergent encryption is used"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitSignNameBody {
        #[doc = "Deprecated: use \"hash_algorithm\" instead."]
        #[serde(default = "post_transit_sign_name_body_algorithm")]
        pub algorithm: String,
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled; currently only available with ed25519 keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Hash algorithm to use (POST body parameter). Valid values are: * sha1 * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\". Not valid for all key types, including ed25519."]
        #[serde(default = "post_transit_sign_name_body_hash_algorithm")]
        pub hash_algorithm: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The version of the key to use for signing. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "The method by which to marshal the signature. The default is 'asn1' which is used by openssl and X.509. It can also be set to 'jws' which is used for JWT signatures; setting it to this will also cause the encoding of the signature to be url-safe base64 instead of using standard base64 encoding. Currently only valid for ECDSA P-256 key types\"."]
        #[serde(default = "post_transit_sign_name_body_marshaling_algorithm")]
        pub marshaling_algorithm: String,
        #[doc = "Set to 'true' when the input is already hashed. If the key type is 'rsa-2048', 'rsa-3072' or 'rsa-4096', then the algorithm used to hash the input should be indicated by the 'algorithm' parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prehashed: Option<bool>,
        #[doc = "The signature algorithm to use for signing. Currently only applies to RSA key types. Options are 'pss' or 'pkcs1v15'. Defaults to 'pss'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature_algorithm: Option<String>,
        #[doc = "Hash algorithm to use (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlalgorithm: Option<String>,
    }

    fn post_transit_sign_name_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_sign_name_body_hash_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_sign_name_body_marshaling_algorithm() -> String {
        "asn1".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitSignNameUrlalgorithmBody {
        #[doc = "Deprecated: use \"hash_algorithm\" instead."]
        #[serde(default = "post_transit_sign_name_urlalgorithm_body_algorithm")]
        pub algorithm: String,
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled; currently only available with ed25519 keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Hash algorithm to use (POST body parameter). Valid values are: * sha1 * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\". Not valid for all key types, including ed25519."]
        #[serde(default = "post_transit_sign_name_urlalgorithm_body_hash_algorithm")]
        pub hash_algorithm: String,
        #[doc = "The base64-encoded input data"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The version of the key to use for signing. Must be 0 (for latest) or a value greater than or equal to the min_encryption_version configured on the key."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_version: Option<i64>,
        #[doc = "The method by which to marshal the signature. The default is 'asn1' which is used by openssl and X.509. It can also be set to 'jws' which is used for JWT signatures; setting it to this will also cause the encoding of the signature to be url-safe base64 instead of using standard base64 encoding. Currently only valid for ECDSA P-256 key types\"."]
        #[serde(default = "post_transit_sign_name_urlalgorithm_body_marshaling_algorithm")]
        pub marshaling_algorithm: String,
        #[doc = "Set to 'true' when the input is already hashed. If the key type is 'rsa-2048', 'rsa-3072' or 'rsa-4096', then the algorithm used to hash the input should be indicated by the 'algorithm' parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prehashed: Option<bool>,
        #[doc = "The signature algorithm to use for signing. Currently only applies to RSA key types. Options are 'pss' or 'pkcs1v15'. Defaults to 'pss'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature_algorithm: Option<String>,
    }

    fn post_transit_sign_name_urlalgorithm_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_sign_name_urlalgorithm_body_hash_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_sign_name_urlalgorithm_body_marshaling_algorithm() -> String {
        "asn1".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitVerifyNameBody {
        #[doc = "Deprecated: use \"hash_algorithm\" instead."]
        #[serde(default = "post_transit_verify_name_body_algorithm")]
        pub algorithm: String,
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled; currently only available with ed25519 keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Hash algorithm to use (POST body parameter). Valid values are: * sha1 * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\". Not valid for all key types."]
        #[serde(default = "post_transit_verify_name_body_hash_algorithm")]
        pub hash_algorithm: String,
        #[doc = "The HMAC, including vault header/key version"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hmac: Option<String>,
        #[doc = "The base64-encoded input data to verify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The method by which to unmarshal the signature when verifying. The default is 'asn1' which is used by openssl and X.509; can also be set to 'jws' which is used for JWT signatures in which case the signature is also expected to be url-safe base64 encoding instead of standard base64 encoding. Currently only valid for ECDSA P-256 key types\"."]
        #[serde(default = "post_transit_verify_name_body_marshaling_algorithm")]
        pub marshaling_algorithm: String,
        #[doc = "Set to 'true' when the input is already hashed. If the key type is 'rsa-2048', 'rsa-3072' or 'rsa-4096', then the algorithm used to hash the input should be indicated by the 'algorithm' parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prehashed: Option<bool>,
        #[doc = "The signature, including vault header/key version"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature: Option<String>,
        #[doc = "The signature algorithm to use for signature verification. Currently only applies to RSA key types. Options are 'pss' or 'pkcs1v15'. Defaults to 'pss'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature_algorithm: Option<String>,
        #[doc = "Hash algorithm to use (POST URL parameter)"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub urlalgorithm: Option<String>,
    }

    fn post_transit_verify_name_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_verify_name_body_hash_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_verify_name_body_marshaling_algorithm() -> String {
        "asn1".to_string()
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PostTransitVerifyNameUrlalgorithmBody {
        #[doc = "Deprecated: use \"hash_algorithm\" instead."]
        #[serde(default = "post_transit_verify_name_urlalgorithm_body_algorithm")]
        pub algorithm: String,
        #[doc = "Base64 encoded context for key derivation. Required if key derivation is enabled; currently only available with ed25519 keys."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[doc = "Hash algorithm to use (POST body parameter). Valid values are: * sha1 * sha2-224 * sha2-256 * sha2-384 * sha2-512 Defaults to \"sha2-256\". Not valid for all key types."]
        #[serde(default = "post_transit_verify_name_urlalgorithm_body_hash_algorithm")]
        pub hash_algorithm: String,
        #[doc = "The HMAC, including vault header/key version"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hmac: Option<String>,
        #[doc = "The base64-encoded input data to verify"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        #[doc = "The method by which to unmarshal the signature when verifying. The default is 'asn1' which is used by openssl and X.509; can also be set to 'jws' which is used for JWT signatures in which case the signature is also expected to be url-safe base64 encoding instead of standard base64 encoding. Currently only valid for ECDSA P-256 key types\"."]
        #[serde(default = "post_transit_verify_name_urlalgorithm_body_marshaling_algorithm")]
        pub marshaling_algorithm: String,
        #[doc = "Set to 'true' when the input is already hashed. If the key type is 'rsa-2048', 'rsa-3072' or 'rsa-4096', then the algorithm used to hash the input should be indicated by the 'algorithm' parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prehashed: Option<bool>,
        #[doc = "The signature, including vault header/key version"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature: Option<String>,
        #[doc = "The signature algorithm to use for signature verification. Currently only applies to RSA key types. Options are 'pss' or 'pkcs1v15'. Defaults to 'pss'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signature_algorithm: Option<String>,
    }

    fn post_transit_verify_name_urlalgorithm_body_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_verify_name_urlalgorithm_body_hash_algorithm() -> String {
        "sha2-256".to_string()
    }

    fn post_transit_verify_name_urlalgorithm_body_marshaling_algorithm() -> String {
        "asn1".to_string()
    }
}

#[derive(Clone)]
pub struct Client {
    baseurl: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(baseurl: &str) -> Self {
        let dur = std::time::Duration::from_secs(15);
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(dur)
            .timeout(dur)
            .build()
            .unwrap();
        Self::new_with_client(baseurl, client)
    }

    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    #[doc = "List token accessors, which can then be\nbe used to iterate and discover their properties\nor revoke them. Because this can be used to\ncause a denial of service, this endpoint\nrequires 'sudo' capability in addition to\n'list'\n\nSends a `GET` request to `/auth/token/accessors`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_auth_token_accessors<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/accessors", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "The token create path is used to create new tokens\n\nSends a `POST` request to `/auth/token/create`"]
    pub async fn post_auth_token_create<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/create", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "The token create path is used to create new orphan tokens\n\nSends a `POST` request to `/auth/token/create-orphan`"]
    pub async fn post_auth_token_create_orphan<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/create-orphan", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This token create path is used to create new tokens adhering to the given role\n\nSends a `POST` request to `/auth/token/create/{role_name}`\n\nArguments:\n- `role_name`: Name of the role\n"]
    pub async fn post_auth_token_create_role_name<'a>(
        &'a self,
        role_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/auth/token/create/{}",
            self.baseurl,
            progenitor_client::encode_path(&role_name.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will lookup a token and its properties\n\nSends a `GET` request to `/auth/token/lookup`"]
    pub async fn get_auth_token_lookup<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/lookup", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will lookup a token and its properties\n\nSends a `POST` request to `/auth/token/lookup`"]
    pub async fn post_auth_token_lookup<'a>(
        &'a self,
        body: &'a types::PostAuthTokenLookupBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/lookup", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will lookup a token associated with the given accessor and its properties. Response will not contain the token ID\n\nSends a `POST` request to `/auth/token/lookup-accessor`"]
    pub async fn post_auth_token_lookup_accessor<'a>(
        &'a self,
        body: &'a types::PostAuthTokenLookupAccessorBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/lookup-accessor", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will lookup a token and its properties\n\nSends a `GET` request to `/auth/token/lookup-self`"]
    pub async fn get_auth_token_lookup_self<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/lookup-self", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will lookup a token and its properties\n\nSends a `POST` request to `/auth/token/lookup-self`"]
    pub async fn post_auth_token_lookup_self<'a>(
        &'a self,
        body: &'a types::PostAuthTokenLookupSelfBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/lookup-self", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will renew the given token and prevent expiration\n\nSends a `POST` request to `/auth/token/renew`"]
    pub async fn post_auth_token_renew<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRenewBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/renew", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will renew a token associated with the given accessor and its properties. Response will not contain the token ID\n\nSends a `POST` request to `/auth/token/renew-accessor`"]
    pub async fn post_auth_token_renew_accessor<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRenewAccessorBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/renew-accessor", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will renew the token used to call it and prevent expiration\n\nSends a `POST` request to `/auth/token/renew-self`"]
    pub async fn post_auth_token_renew_self<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRenewSelfBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/renew-self", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will delete the given token and all of its child tokens\n\nSends a `POST` request to `/auth/token/revoke`"]
    pub async fn post_auth_token_revoke<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRevokeBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/revoke", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will delete the token associated with the accessor and all of its child tokens\n\nSends a `POST` request to `/auth/token/revoke-accessor`"]
    pub async fn post_auth_token_revoke_accessor<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRevokeAccessorBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/revoke-accessor", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will delete the token and orphan its child tokens\n\nSends a `POST` request to `/auth/token/revoke-orphan`"]
    pub async fn post_auth_token_revoke_orphan<'a>(
        &'a self,
        body: &'a types::PostAuthTokenRevokeOrphanBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/revoke-orphan", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint will delete the token used to call it and all of its child tokens\n\nSends a `POST` request to `/auth/token/revoke-self`"]
    pub async fn post_auth_token_revoke_self<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/revoke-self", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint lists configured roles\n\nSends a `GET` request to `/auth/token/roles`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_auth_token_roles<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/roles", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/auth/token/roles/{role_name}`\n\nArguments:\n- `role_name`: Name of the role\n"]
    pub async fn get_auth_token_roles_role_name<'a>(
        &'a self,
        role_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/auth/token/roles/{}",
            self.baseurl,
            progenitor_client::encode_path(&role_name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/auth/token/roles/{role_name}`\n\nArguments:\n- `role_name`: Name of the role\n- `body`\n"]
    pub async fn post_auth_token_roles_role_name<'a>(
        &'a self,
        role_name: &'a str,
        body: &'a types::PostAuthTokenRolesRoleNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/auth/token/roles/{}",
            self.baseurl,
            progenitor_client::encode_path(&role_name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `DELETE` request to `/auth/token/roles/{role_name}`\n\nArguments:\n- `role_name`: Name of the role\n"]
    pub async fn delete_auth_token_roles_role_name<'a>(
        &'a self,
        role_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/auth/token/roles/{}",
            self.baseurl,
            progenitor_client::encode_path(&role_name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint performs cleanup tasks that can be run if certain error\nconditions have occurred\n\nSends a `POST` request to `/auth/token/tidy`"]
    pub async fn post_auth_token_tidy<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/token/tidy", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve the secret at the specified location\n\nSends a `GET` request to `/cubbyhole/{path}`\n\nArguments:\n- `path`: Specifies the path of the secret.\n- `list`: Return a list if `true`\n"]
    pub async fn get_cubbyhole_path<'a>(
        &'a self,
        path: &'a str,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/cubbyhole/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Store a secret at the specified location\n\nSends a `POST` request to `/cubbyhole/{path}`\n\nArguments:\n- `path`: Specifies the path of the secret.\n"]
    pub async fn post_cubbyhole_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/cubbyhole/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Deletes the secret at the specified location\n\nSends a `DELETE` request to `/cubbyhole/{path}`\n\nArguments:\n- `path`: Specifies the path of the secret.\n"]
    pub async fn delete_cubbyhole_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/cubbyhole/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Create a new alias\n\nSends a `POST` request to `/identity/alias`"]
    pub async fn post_identity_alias<'a>(
        &'a self,
        body: &'a types::PostIdentityAliasBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/alias", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the alias IDs\n\nSends a `GET` request to `/identity/alias/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_alias_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/alias/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `GET` request to `/identity/alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n"]
    pub async fn get_identity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `POST` request to `/identity/alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n- `body`\n"]
    pub async fn post_identity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityAliasIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `DELETE` request to `/identity/alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n"]
    pub async fn delete_identity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Create a new entity\n\nSends a `POST` request to `/identity/entity`"]
    pub async fn post_identity_entity<'a>(
        &'a self,
        body: &'a types::PostIdentityEntityBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Create a new alias\n\nSends a `POST` request to `/identity/entity-alias`"]
    pub async fn post_identity_entity_alias<'a>(
        &'a self,
        body: &'a types::PostIdentityEntityAliasBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity-alias", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the alias IDs\n\nSends a `GET` request to `/identity/entity-alias/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_entity_alias_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity-alias/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `GET` request to `/identity/entity-alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n"]
    pub async fn get_identity_entity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `POST` request to `/identity/entity-alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n- `body`\n"]
    pub async fn post_identity_entity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityEntityAliasIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `DELETE` request to `/identity/entity-alias/id/{id}`\n\nArguments:\n- `id`: ID of the alias\n"]
    pub async fn delete_identity_entity_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete all of the entities provided\n\nSends a `POST` request to `/identity/entity/batch-delete`"]
    pub async fn post_identity_entity_batch_delete<'a>(
        &'a self,
        body: &'a types::PostIdentityEntityBatchDeleteBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity/batch-delete", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the entity IDs\n\nSends a `GET` request to `/identity/entity/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_entity_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity ID\n\nSends a `GET` request to `/identity/entity/id/{id}`\n\nArguments:\n- `id`: ID of the entity. If set, updates the corresponding existing entity.\n"]
    pub async fn get_identity_entity_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity ID\n\nSends a `POST` request to `/identity/entity/id/{id}`\n\nArguments:\n- `id`: ID of the entity. If set, updates the corresponding existing entity.\n- `body`\n"]
    pub async fn post_identity_entity_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityEntityIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity ID\n\nSends a `DELETE` request to `/identity/entity/id/{id}`\n\nArguments:\n- `id`: ID of the entity. If set, updates the corresponding existing entity.\n"]
    pub async fn delete_identity_entity_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Merge two or more entities together\n\nSends a `POST` request to `/identity/entity/merge`"]
    pub async fn post_identity_entity_merge<'a>(
        &'a self,
        body: &'a types::PostIdentityEntityMergeBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity/merge", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the entity names\n\nSends a `GET` request to `/identity/entity/name`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_entity_name<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/entity/name", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity name\n\nSends a `GET` request to `/identity/entity/name/{name}`\n\nArguments:\n- `name`: Name of the entity\n"]
    pub async fn get_identity_entity_name_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity name\n\nSends a `POST` request to `/identity/entity/name/{name}`\n\nArguments:\n- `name`: Name of the entity\n- `body`\n"]
    pub async fn post_identity_entity_name_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostIdentityEntityNameNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an entity using entity name\n\nSends a `DELETE` request to `/identity/entity/name/{name}`\n\nArguments:\n- `name`: Name of the entity\n"]
    pub async fn delete_identity_entity_name_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/entity/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Create a new group\n\nSends a `POST` request to `/identity/group`"]
    pub async fn post_identity_group<'a>(
        &'a self,
        body: &'a types::PostIdentityGroupBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/group", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Creates a new group alias, or updates an existing one\n\nSends a `POST` request to `/identity/group-alias`"]
    pub async fn post_identity_group_alias<'a>(
        &'a self,
        body: &'a types::PostIdentityGroupAliasBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/group-alias", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the group alias IDs\n\nSends a `GET` request to `/identity/group-alias/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_group_alias_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/group-alias/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/identity/group-alias/id/{id}`\n\nArguments:\n- `id`: ID of the group alias.\n"]
    pub async fn get_identity_group_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/identity/group-alias/id/{id}`\n\nArguments:\n- `id`: ID of the group alias.\n- `body`\n"]
    pub async fn post_identity_group_alias_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityGroupAliasIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `DELETE` request to `/identity/group-alias/id/{id}`\n\nArguments:\n- `id`: ID of the group alias.\n"]
    pub async fn delete_identity_group_alias_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group-alias/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the group IDs\n\nSends a `GET` request to `/identity/group/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_group_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/group/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update or delete an existing group using its ID\n\nSends a `GET` request to `/identity/group/id/{id}`\n\nArguments:\n- `id`: ID of the group. If set, updates the corresponding existing group.\n"]
    pub async fn get_identity_group_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update or delete an existing group using its ID\n\nSends a `POST` request to `/identity/group/id/{id}`\n\nArguments:\n- `id`: ID of the group. If set, updates the corresponding existing group.\n- `body`\n"]
    pub async fn post_identity_group_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityGroupIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update or delete an existing group using its ID\n\nSends a `DELETE` request to `/identity/group/id/{id}`\n\nArguments:\n- `id`: ID of the group. If set, updates the corresponding existing group.\n"]
    pub async fn delete_identity_group_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/identity/group/name`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_group_name<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/group/name", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/identity/group/name/{name}`\n\nArguments:\n- `name`: Name of the group.\n"]
    pub async fn get_identity_group_name_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/identity/group/name/{name}`\n\nArguments:\n- `name`: Name of the group.\n- `body`\n"]
    pub async fn post_identity_group_name_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostIdentityGroupNameNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `DELETE` request to `/identity/group/name/{name}`\n\nArguments:\n- `name`: Name of the group.\n"]
    pub async fn delete_identity_group_name_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/group/name/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Query entities based on various properties\n\nSends a `POST` request to `/identity/lookup/entity`"]
    pub async fn post_identity_lookup_entity<'a>(
        &'a self,
        body: &'a types::PostIdentityLookupEntityBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/lookup/entity", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Query groups based on various properties\n\nSends a `POST` request to `/identity/lookup/group`"]
    pub async fn post_identity_lookup_group<'a>(
        &'a self,
        body: &'a types::PostIdentityLookupGroupBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/lookup/group", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve public keys\n\nSends a `GET` request to `/identity/oidc/.well-known/keys`"]
    pub async fn get_identity_oidc_well_known_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/.well-known/keys", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Query OIDC configurations\n\nSends a `GET` request to `/identity/oidc/.well-known/openid-configuration`"]
    pub async fn get_identity_oidc_well_known_openid_configuration<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/.well-known/openid-configuration",
            self.baseurl,
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "OIDC configuration\n\nSends a `GET` request to `/identity/oidc/config`"]
    pub async fn get_identity_oidc_config<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "OIDC configuration\n\nSends a `POST` request to `/identity/oidc/config`"]
    pub async fn post_identity_oidc_config<'a>(
        &'a self,
        body: &'a types::PostIdentityOidcConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Verify the authenticity of an OIDC token\n\nSends a `POST` request to `/identity/oidc/introspect`"]
    pub async fn post_identity_oidc_introspect<'a>(
        &'a self,
        body: &'a types::PostIdentityOidcIntrospectBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/introspect", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List OIDC keys\n\nSends a `GET` request to `/identity/oidc/key`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_oidc_key<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/key", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations for OIDC keys\n\nSends a `GET` request to `/identity/oidc/key/{name}`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn get_identity_oidc_key_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/key/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations for OIDC keys\n\nSends a `POST` request to `/identity/oidc/key/{name}`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_identity_oidc_key_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostIdentityOidcKeyNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/key/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations for OIDC keys\n\nSends a `DELETE` request to `/identity/oidc/key/{name}`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn delete_identity_oidc_key_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/key/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Rotate a named OIDC key\n\nSends a `POST` request to `/identity/oidc/key/{name}/rotate`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_identity_oidc_key_name_rotate<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostIdentityOidcKeyNameRotateBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/key/{}/rotate",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List configured OIDC roles\n\nSends a `GET` request to `/identity/oidc/role`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_oidc_role<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/oidc/role", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations on OIDC Roles\n\nSends a `GET` request to `/identity/oidc/role/{name}`\n\nArguments:\n- `name`: Name of the role\n"]
    pub async fn get_identity_oidc_role_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/role/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations on OIDC Roles\n\nSends a `POST` request to `/identity/oidc/role/{name}`\n\nArguments:\n- `name`: Name of the role\n- `body`\n"]
    pub async fn post_identity_oidc_role_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostIdentityOidcRoleNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/role/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "CRUD operations on OIDC Roles\n\nSends a `DELETE` request to `/identity/oidc/role/{name}`\n\nArguments:\n- `name`: Name of the role\n"]
    pub async fn delete_identity_oidc_role_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/role/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate an OIDC token\n\nSends a `GET` request to `/identity/oidc/token/{name}`\n\nArguments:\n- `name`: Name of the role\n"]
    pub async fn get_identity_oidc_token_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/oidc/token/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Create a new alias\n\nSends a `POST` request to `/identity/persona`"]
    pub async fn post_identity_persona<'a>(
        &'a self,
        body: &'a types::PostIdentityPersonaBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/persona", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List all the alias IDs\n\nSends a `GET` request to `/identity/persona/id`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_identity_persona_id<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/identity/persona/id", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `GET` request to `/identity/persona/id/{id}`\n\nArguments:\n- `id`: ID of the persona\n"]
    pub async fn get_identity_persona_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/persona/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `POST` request to `/identity/persona/id/{id}`\n\nArguments:\n- `id`: ID of the persona\n- `body`\n"]
    pub async fn post_identity_persona_id_id<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::PostIdentityPersonaIdIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/persona/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update, read or delete an alias ID\n\nSends a `DELETE` request to `/identity/persona/id/{id}`\n\nArguments:\n- `id`: ID of the persona\n"]
    pub async fn delete_identity_persona_id_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/identity/persona/id/{}",
            self.baseurl,
            progenitor_client::encode_path(&id.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the backend level settings\n\nSends a `GET` request to `/secret/config`"]
    pub async fn get_secret_config<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/secret/config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configure backend level settings that are applied to every key in the key-value store\n\nSends a `POST` request to `/secret/config`"]
    pub async fn post_secret_config<'a>(
        &'a self,
        body: &'a types::PostSecretConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/secret/config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Write, Read, and Delete data in the Key-Value Store\n\nSends a `GET` request to `/secret/data/{path}`\n\nArguments:\n- `path`: Location of the secret.\n"]
    pub async fn get_secret_data_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/data/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Write, Read, and Delete data in the Key-Value Store\n\nSends a `POST` request to `/secret/data/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `body`\n"]
    pub async fn post_secret_data_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSecretDataPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/data/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Write, Read, and Delete data in the Key-Value Store\n\nSends a `DELETE` request to `/secret/data/{path}`\n\nArguments:\n- `path`: Location of the secret.\n"]
    pub async fn delete_secret_data_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/data/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Marks one or more versions as deleted in the KV store\n\nSends a `POST` request to `/secret/delete/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `body`\n"]
    pub async fn post_secret_delete_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSecretDeletePathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/delete/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Permanently removes one or more versions in the KV store\n\nSends a `POST` request to `/secret/destroy/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `body`\n"]
    pub async fn post_secret_destroy_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSecretDestroyPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/destroy/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configures settings for the KV store\n\nSends a `GET` request to `/secret/metadata/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `list`: Return a list if `true`\n"]
    pub async fn get_secret_metadata_path<'a>(
        &'a self,
        path: &'a str,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/metadata/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configures settings for the KV store\n\nSends a `POST` request to `/secret/metadata/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `body`\n"]
    pub async fn post_secret_metadata_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSecretMetadataPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/metadata/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configures settings for the KV store\n\nSends a `DELETE` request to `/secret/metadata/{path}`\n\nArguments:\n- `path`: Location of the secret.\n"]
    pub async fn delete_secret_metadata_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/metadata/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Undeletes one or more versions from the KV store\n\nSends a `POST` request to `/secret/undelete/{path}`\n\nArguments:\n- `path`: Location of the secret.\n- `body`\n"]
    pub async fn post_secret_undelete_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSecretUndeletePathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/secret/undelete/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the enabled audit devices\n\nSends a `GET` request to `/sys/audit`"]
    pub async fn get_sys_audit<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/audit", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "The hash of the given string via the given audit backend\n\nSends a `POST` request to `/sys/audit-hash/{path}`\n\nArguments:\n- `path`: The name of the backend. Cannot be delimited. Example: \"mysql\"\n- `body`\n"]
    pub async fn post_sys_audit_hash_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysAuditHashPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/audit-hash/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enable a new audit device at the supplied path\n\nSends a `POST` request to `/sys/audit/{path}`\n\nArguments:\n- `path`: The name of the backend. Cannot be delimited. Example: \"mysql\"\n- `body`\n"]
    pub async fn post_sys_audit_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysAuditPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/audit/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Disable the audit device at the given path\n\nSends a `DELETE` request to `/sys/audit/{path}`\n\nArguments:\n- `path`: The name of the backend. Cannot be delimited. Example: \"mysql\"\n"]
    pub async fn delete_sys_audit_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/audit/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the currently enabled credential backends\n\nSends a `GET` request to `/sys/auth`"]
    pub async fn get_sys_auth<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/auth", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enables a new auth method\n\nAfter enabling, the auth method can be accessed and configured via the auth path specified as part of the URL. This auth path will be nested under the auth prefix.\n\nFor example, enable the \"foo\" auth method will make it accessible at /auth/foo.\n\nSends a `POST` request to `/sys/auth/{path}`\n\nArguments:\n- `path`: The path to mount to. Cannot be delimited. Example: \"user\"\n- `body`\n"]
    pub async fn post_sys_auth_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysAuthPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/auth/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Disable the auth method at the given auth path\n\nSends a `DELETE` request to `/sys/auth/{path}`\n\nArguments:\n- `path`: The path to mount to. Cannot be delimited. Example: \"user\"\n"]
    pub async fn delete_sys_auth_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/auth/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Reads the given auth path's configuration\n\nThis endpoint requires sudo capability on the final path, but the same functionality can be achieved without sudo via `sys/mounts/auth/[auth-path]/tune`.\n\nSends a `GET` request to `/sys/auth/{path}/tune`\n\nArguments:\n- `path`: Tune the configuration parameters for an auth path.\n"]
    pub async fn get_sys_auth_path_tune<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/auth/{}/tune",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Tune configuration parameters for a given auth path\n\nThis endpoint requires sudo capability on the final path, but the same functionality can be achieved without sudo via `sys/mounts/auth/[auth-path]/tune`.\n\nSends a `POST` request to `/sys/auth/{path}/tune`\n\nArguments:\n- `path`: Tune the configuration parameters for an auth path.\n- `body`\n"]
    pub async fn post_sys_auth_path_tune<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysAuthPathTuneBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/auth/{}/tune",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetches the capabilities of the given token on the given path\n\nSends a `POST` request to `/sys/capabilities`"]
    pub async fn post_sys_capabilities<'a>(
        &'a self,
        body: &'a types::PostSysCapabilitiesBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/capabilities", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetches the capabilities of the token associated with the given token, on the given path\n\nSends a `POST` request to `/sys/capabilities-accessor`"]
    pub async fn post_sys_capabilities_accessor<'a>(
        &'a self,
        body: &'a types::PostSysCapabilitiesAccessorBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/capabilities-accessor", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetches the capabilities of the given token on the given path\n\nSends a `POST` request to `/sys/capabilities-self`"]
    pub async fn post_sys_capabilities_self<'a>(
        &'a self,
        body: &'a types::PostSysCapabilitiesSelfBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/capabilities-self", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the request headers that are configured to be audited\n\nSends a `GET` request to `/sys/config/auditing/request-headers`"]
    pub async fn get_sys_config_auditing_request_headers<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/auditing/request-headers", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the information for the given request header\n\nSends a `GET` request to `/sys/config/auditing/request-headers/{header}`"]
    pub async fn get_sys_config_auditing_request_headers_header<'a>(
        &'a self,
        header: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/auditing/request-headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enable auditing of a header\n\nSends a `POST` request to `/sys/config/auditing/request-headers/{header}`"]
    pub async fn post_sys_config_auditing_request_headers_header<'a>(
        &'a self,
        header: &'a str,
        body: &'a types::PostSysConfigAuditingRequestHeadersHeaderBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/auditing/request-headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Disable auditing of the given request header\n\nSends a `DELETE` request to `/sys/config/auditing/request-headers/{header}`"]
    pub async fn delete_sys_config_auditing_request_headers_header<'a>(
        &'a self,
        header: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/auditing/request-headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return the current CORS settings\n\nSends a `GET` request to `/sys/config/cors`"]
    pub async fn get_sys_config_cors<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/cors", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configure the CORS settings\n\nSends a `POST` request to `/sys/config/cors`"]
    pub async fn post_sys_config_cors<'a>(
        &'a self,
        body: &'a types::PostSysConfigCorsBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/cors", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Remove any CORS settings\n\nSends a `DELETE` request to `/sys/config/cors`"]
    pub async fn delete_sys_config_cors<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/cors", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Reload the given subsystem\n\nSends a `POST` request to `/sys/config/reload/{subsystem}`"]
    pub async fn post_sys_config_reload_subsystem<'a>(
        &'a self,
        subsystem: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/reload/{}",
            self.baseurl,
            progenitor_client::encode_path(&subsystem.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return a sanitized version of the Vault server configuration\n\nThe sanitized output strips configuration values in the storage, HA storage, and seals stanzas, which may contain sensitive values such as API tokens. It also removes any token or secret fields in other stanzas, such as the circonus_api_token from telemetry.\n\nSends a `GET` request to `/sys/config/state/sanitized`"]
    pub async fn get_sys_config_state_sanitized<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/state/sanitized", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return a list of configured UI headers\n\nSends a `GET` request to `/sys/config/ui/headers`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_config_ui_headers<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/config/ui/headers", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return the given UI header's configuration\n\nSends a `GET` request to `/sys/config/ui/headers/{header}`\n\nArguments:\n- `header`: The name of the header.\n"]
    pub async fn get_sys_config_ui_headers_header<'a>(
        &'a self,
        header: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/ui/headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configure the values to be returned for the UI header\n\nSends a `POST` request to `/sys/config/ui/headers/{header}`\n\nArguments:\n- `header`: The name of the header.\n- `body`\n"]
    pub async fn post_sys_config_ui_headers_header<'a>(
        &'a self,
        header: &'a str,
        body: &'a types::PostSysConfigUiHeadersHeaderBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/ui/headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Remove a UI header\n\nSends a `DELETE` request to `/sys/config/ui/headers/{header}`\n\nArguments:\n- `header`: The name of the header.\n"]
    pub async fn delete_sys_config_ui_headers_header<'a>(
        &'a self,
        header: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/config/ui/headers/{}",
            self.baseurl,
            progenitor_client::encode_path(&header.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the configuration and progress of the current root generation attempt\n\nSends a `GET` request to `/sys/generate-root`"]
    pub async fn get_sys_generate_root<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Initializes a new root generation attempt\n\nOnly a single root generation attempt can take place at a time. One (and only one) of otp or pgp_key are required.\n\nSends a `POST` request to `/sys/generate-root`"]
    pub async fn post_sys_generate_root<'a>(
        &'a self,
        body: &'a types::PostSysGenerateRootBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Cancels any in-progress root generation attempt\n\nSends a `DELETE` request to `/sys/generate-root`"]
    pub async fn delete_sys_generate_root<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the configuration and progress of the current root generation attempt\n\nSends a `GET` request to `/sys/generate-root/attempt`"]
    pub async fn get_sys_generate_root_attempt<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root/attempt", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Initializes a new root generation attempt\n\nOnly a single root generation attempt can take place at a time. One (and only one) of otp or pgp_key are required.\n\nSends a `POST` request to `/sys/generate-root/attempt`"]
    pub async fn post_sys_generate_root_attempt<'a>(
        &'a self,
        body: &'a types::PostSysGenerateRootAttemptBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root/attempt", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Cancels any in-progress root generation attempt\n\nSends a `DELETE` request to `/sys/generate-root/attempt`"]
    pub async fn delete_sys_generate_root_attempt<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root/attempt", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enter a single master key share to progress the root generation attempt\n\nIf the threshold number of master key shares is reached, Vault will complete the root generation and issue the new token. Otherwise, this API must be called multiple times until that threshold is met. The attempt nonce must be provided with each call.\n\nSends a `POST` request to `/sys/generate-root/update`"]
    pub async fn post_sys_generate_root_update<'a>(
        &'a self,
        body: &'a types::PostSysGenerateRootUpdateBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/generate-root/update", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the health status of Vault\n\nSends a `GET` request to `/sys/health`"]
    pub async fn get_sys_health<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/health", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            429u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            472u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            501u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            503u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Information about the host instance that this Vault server is running on\n\nInformation about the host instance that this Vault server is running on.\n\t\tThe information that gets collected includes host hardware information, and CPU,\n\t\tdisk, and memory utilization\n\nSends a `GET` request to `/sys/host-info`"]
    pub async fn get_sys_host_info<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/host-info", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the initialization status of Vault\n\nSends a `GET` request to `/sys/init`"]
    pub async fn get_sys_init<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/init", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Initialize a new Vault\n\nThe Vault must not have been previously initialized. The recovery options, as well as the stored shares option, are only available when using Vault HSM.\n\nSends a `POST` request to `/sys/init`"]
    pub async fn post_sys_init<'a>(
        &'a self,
        body: &'a types::PostSysInitBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/init", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Report the client count metrics, for this namespace and all child namespaces\n\nSends a `GET` request to `/sys/internal/counters/activity`"]
    pub async fn get_sys_internal_counters_activity<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/counters/activity", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Report the number of clients for this month, for this namespace and all child namespaces\n\nSends a `GET` request to `/sys/internal/counters/activity/monthly`"]
    pub async fn get_sys_internal_counters_activity_monthly<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/counters/activity/monthly", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the client count tracking configuration\n\nSends a `GET` request to `/sys/internal/counters/config`"]
    pub async fn get_sys_internal_counters_config<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/counters/config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enable or disable collection of client count, set retention period, or set default reporting period\n\nSends a `POST` request to `/sys/internal/counters/config`"]
    pub async fn post_sys_internal_counters_config<'a>(
        &'a self,
        body: &'a types::PostSysInternalCountersConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/counters/config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate an OpenAPI 3 document of all mounted paths\n\nSends a `GET` request to `/sys/internal/specs/openapi`"]
    pub async fn get_sys_internal_specs_openapi<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/specs/openapi", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Lists enabled feature flags\n\nSends a `GET` request to `/sys/internal/ui/feature-flags`"]
    pub async fn get_sys_internal_ui_feature_flags<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/ui/feature-flags", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Lists all enabled and visible auth and secrets mounts\n\nSends a `GET` request to `/sys/internal/ui/mounts`"]
    pub async fn get_sys_internal_ui_mounts<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/internal/ui/mounts", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return information about the given mount\n\nSends a `GET` request to `/sys/internal/ui/mounts/{path}`\n\nArguments:\n- `path`: The path of the mount.\n"]
    pub async fn get_sys_internal_ui_mounts_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/internal/ui/mounts/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Provides information about the backend encryption key\n\nSends a `GET` request to `/sys/key-status`"]
    pub async fn get_sys_key_status<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/key-status", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the high availability status and current leader instance of Vault\n\nSends a `GET` request to `/sys/leader`"]
    pub async fn get_sys_leader<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leader", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List leases associated with this Vault cluster\n\nSends a `GET` request to `/sys/leases`"]
    pub async fn get_sys_leases<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Count of leases associated with this Vault cluster\n\nSends a `GET` request to `/sys/leases/count`"]
    pub async fn get_sys_leases_count<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/count", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve lease metadata\n\nSends a `POST` request to `/sys/leases/lookup`"]
    pub async fn post_sys_leases_lookup<'a>(
        &'a self,
        body: &'a types::PostSysLeasesLookupBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/lookup", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns a list of lease ids\n\nSends a `GET` request to `/sys/leases/lookup`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_leases_lookup<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/lookup", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns a list of lease ids\n\nSends a `GET` request to `/sys/leases/lookup/{prefix}`\n\nArguments:\n- `prefix`: The path to list leases under. Example: \"aws/creds/deploy\"\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_leases_lookup_prefix<'a>(
        &'a self,
        prefix: &'a str,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/leases/lookup/{}",
            self.baseurl,
            progenitor_client::encode_path(&prefix.to_string()),
        );
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Renews a lease, requesting to extend the lease\n\nSends a `POST` request to `/sys/leases/renew`"]
    pub async fn post_sys_leases_renew<'a>(
        &'a self,
        body: &'a types::PostSysLeasesRenewBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/renew", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Renews a lease, requesting to extend the lease\n\nSends a `POST` request to `/sys/leases/renew/{url_lease_id}`\n\nArguments:\n- `url_lease_id`: The lease identifier to renew. This is included with a lease.\n- `body`\n"]
    pub async fn post_sys_leases_renew_url_lease_id<'a>(
        &'a self,
        url_lease_id: &'a str,
        body: &'a types::PostSysLeasesRenewUrlLeaseIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/leases/renew/{}",
            self.baseurl,
            progenitor_client::encode_path(&url_lease_id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes a lease immediately\n\nSends a `POST` request to `/sys/leases/revoke`"]
    pub async fn post_sys_leases_revoke<'a>(
        &'a self,
        body: &'a types::PostSysLeasesRevokeBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/revoke", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes all secrets or tokens generated under a given prefix immediately\n\nUnlike `/sys/leases/revoke-prefix`, this path ignores backend errors encountered during revocation. This is potentially very dangerous and should only be used in specific emergency situations where errors in the backend or the connected backend service prevent normal revocation.\n\nBy ignoring these errors, Vault abdicates responsibility for ensuring that the issued credentials or secrets are properly revoked and/or cleaned up. Access to this endpoint should be tightly controlled.\n\nSends a `POST` request to `/sys/leases/revoke-force/{prefix}`\n\nArguments:\n- `prefix`: The path to revoke keys under. Example: \"prod/aws/ops\"\n"]
    pub async fn post_sys_leases_revoke_force_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/leases/revoke-force/{}",
            self.baseurl,
            progenitor_client::encode_path(&prefix.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes all secrets (via a lease ID prefix) or tokens (via the tokens' path property) generated under a given prefix immediately\n\nSends a `POST` request to `/sys/leases/revoke-prefix/{prefix}`\n\nArguments:\n- `prefix`: The path to revoke keys under. Example: \"prod/aws/ops\"\n- `body`\n"]
    pub async fn post_sys_leases_revoke_prefix_prefix<'a>(
        &'a self,
        prefix: &'a str,
        body: &'a types::PostSysLeasesRevokePrefixPrefixBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/leases/revoke-prefix/{}",
            self.baseurl,
            progenitor_client::encode_path(&prefix.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes a lease immediately\n\nSends a `POST` request to `/sys/leases/revoke/{url_lease_id}`\n\nArguments:\n- `url_lease_id`: The lease identifier to renew. This is included with a lease.\n- `body`\n"]
    pub async fn post_sys_leases_revoke_url_lease_id<'a>(
        &'a self,
        url_lease_id: &'a str,
        body: &'a types::PostSysLeasesRevokeUrlLeaseIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/leases/revoke/{}",
            self.baseurl,
            progenitor_client::encode_path(&url_lease_id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "This endpoint performs cleanup tasks that can be run if certain error\nconditions have occurred\n\nSends a `POST` request to `/sys/leases/tidy`"]
    pub async fn post_sys_leases_tidy<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/leases/tidy", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Export the metrics aggregated for telemetry purpose\n\nSends a `GET` request to `/sys/metrics`\n\nArguments:\n- `format`: Format to export metrics into. Currently accepts only \"prometheus\".\n"]
    pub async fn get_sys_metrics<'a>(
        &'a self,
        format: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/metrics", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &format {
            query.push(("format", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/monitor`\n\nArguments:\n- `log_level`: Log level to view system logs at. Currently supported values are \"trace\", \"debug\", \"info\", \"warn\", \"error\".\n"]
    pub async fn get_sys_monitor<'a>(
        &'a self,
        log_level: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/monitor", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &log_level {
            query.push(("log_level", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the currently mounted backends\n\nSends a `GET` request to `/sys/mounts`"]
    pub async fn get_sys_mounts<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/mounts", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enable a new secrets engine at the given path\n\nSends a `POST` request to `/sys/mounts/{path}`\n\nArguments:\n- `path`: The path to mount to. Example: \"aws/east\"\n- `body`\n"]
    pub async fn post_sys_mounts_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysMountsPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/mounts/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Disable the mount point specified at the given path\n\nSends a `DELETE` request to `/sys/mounts/{path}`\n\nArguments:\n- `path`: The path to mount to. Example: \"aws/east\"\n"]
    pub async fn delete_sys_mounts_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/mounts/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Tune backend configuration parameters for this mount\n\nSends a `GET` request to `/sys/mounts/{path}/tune`\n\nArguments:\n- `path`: The path to mount to. Example: \"aws/east\"\n"]
    pub async fn get_sys_mounts_path_tune<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/mounts/{}/tune",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Tune backend configuration parameters for this mount\n\nSends a `POST` request to `/sys/mounts/{path}/tune`\n\nArguments:\n- `path`: The path to mount to. Example: \"aws/east\"\n- `body`\n"]
    pub async fn post_sys_mounts_path_tune<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysMountsPathTuneBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/mounts/{}/tune",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Lists all the plugins known to Vault\n\nSends a `GET` request to `/sys/plugins/catalog`"]
    pub async fn get_sys_plugins_catalog<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/plugins/catalog", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return the configuration data for the plugin with the given name\n\nSends a `GET` request to `/sys/plugins/catalog/{name}`\n\nArguments:\n- `name`: The name of the plugin\n"]
    pub async fn get_sys_plugins_catalog_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Register a new plugin, or updates an existing one with the supplied name\n\nSends a `POST` request to `/sys/plugins/catalog/{name}`\n\nArguments:\n- `name`: The name of the plugin\n- `body`\n"]
    pub async fn post_sys_plugins_catalog_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostSysPluginsCatalogNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Remove the plugin with the given name\n\nSends a `DELETE` request to `/sys/plugins/catalog/{name}`\n\nArguments:\n- `name`: The name of the plugin\n"]
    pub async fn delete_sys_plugins_catalog_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the plugins in the catalog\n\nSends a `GET` request to `/sys/plugins/catalog/{type}`\n\nArguments:\n- `type_`: The type of the plugin, may be auth, secret, or database\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_plugins_catalog_type<'a>(
        &'a self,
        type_: &'a str,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
        );
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return the configuration data for the plugin with the given name\n\nSends a `GET` request to `/sys/plugins/catalog/{type}/{name}`\n\nArguments:\n- `type_`: The type of the plugin, may be auth, secret, or database\n- `name`: The name of the plugin\n"]
    pub async fn get_sys_plugins_catalog_type_name<'a>(
        &'a self,
        type_: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Register a new plugin, or updates an existing one with the supplied name\n\nSends a `POST` request to `/sys/plugins/catalog/{type}/{name}`\n\nArguments:\n- `type_`: The type of the plugin, may be auth, secret, or database\n- `name`: The name of the plugin\n- `body`\n"]
    pub async fn post_sys_plugins_catalog_type_name<'a>(
        &'a self,
        type_: &'a str,
        name: &'a str,
        body: &'a types::PostSysPluginsCatalogTypeNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Remove the plugin with the given name\n\nSends a `DELETE` request to `/sys/plugins/catalog/{type}/{name}`\n\nArguments:\n- `type_`: The type of the plugin, may be auth, secret, or database\n- `name`: The name of the plugin\n"]
    pub async fn delete_sys_plugins_catalog_type_name<'a>(
        &'a self,
        type_: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/plugins/catalog/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Reload mounted plugin backends\n\nEither the plugin name (`plugin`) or the desired plugin backend mounts (`mounts`) must be provided, but not both. In the case that the plugin name is provided, all mounted paths that use that plugin backend will be reloaded.  If (`scope`) is provided and is (`global`), the plugin(s) are reloaded globally.\n\nSends a `POST` request to `/sys/plugins/reload/backend`"]
    pub async fn post_sys_plugins_reload_backend<'a>(
        &'a self,
        body: &'a types::PostSysPluginsReloadBackendBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/plugins/reload/backend", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the configured access control policies\n\nSends a `GET` request to `/sys/policies/acl`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_policies_acl<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/policies/acl", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve information about the named ACL policy\n\nSends a `GET` request to `/sys/policies/acl/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n"]
    pub async fn get_sys_policies_acl_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/acl/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Add a new or update an existing ACL policy\n\nSends a `POST` request to `/sys/policies/acl/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n- `body`\n"]
    pub async fn post_sys_policies_acl_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostSysPoliciesAclNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/acl/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete the ACL policy with the given name\n\nSends a `DELETE` request to `/sys/policies/acl/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n"]
    pub async fn delete_sys_policies_acl_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/acl/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve an existing password policy\n\nSends a `GET` request to `/sys/policies/password/{name}`\n\nArguments:\n- `name`: The name of the password policy.\n"]
    pub async fn get_sys_policies_password_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/password/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Add a new or update an existing password policy\n\nSends a `POST` request to `/sys/policies/password/{name}`\n\nArguments:\n- `name`: The name of the password policy.\n- `body`\n"]
    pub async fn post_sys_policies_password_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostSysPoliciesPasswordNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/password/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete a password policy\n\nSends a `DELETE` request to `/sys/policies/password/{name}`\n\nArguments:\n- `name`: The name of the password policy.\n"]
    pub async fn delete_sys_policies_password_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/password/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a password from an existing password policy\n\nSends a `GET` request to `/sys/policies/password/{name}/generate`\n\nArguments:\n- `name`: The name of the password policy.\n"]
    pub async fn get_sys_policies_password_name_generate<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policies/password/{}/generate",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List the configured access control policies\n\nSends a `GET` request to `/sys/policy`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_policy<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/policy", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieve the policy body for the named policy\n\nSends a `GET` request to `/sys/policy/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n"]
    pub async fn get_sys_policy_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policy/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Add a new or update an existing policy\n\nSends a `POST` request to `/sys/policy/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n- `body`\n"]
    pub async fn post_sys_policy_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostSysPolicyNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policy/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete the policy with the given name\n\nSends a `DELETE` request to `/sys/policy/{name}`\n\nArguments:\n- `name`: The name of the policy. Example: \"ops\"\n"]
    pub async fn delete_sys_policy_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/policy/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns an HTML page listing the available profiles\n\nReturns an HTML page listing the available \nprofiles. This should be mainly accessed via browsers or applications that can \nrender pages.\n\nSends a `GET` request to `/sys/pprof`"]
    pub async fn get_sys_pprof<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns a sampling of all past memory allocations\n\nReturns a sampling of all past memory allocations.\n\nSends a `GET` request to `/sys/pprof/allocs`"]
    pub async fn get_sys_pprof_allocs<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/allocs", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns stack traces that led to blocking on synchronization primitives\n\nReturns stack traces that led to blocking on synchronization primitives\n\nSends a `GET` request to `/sys/pprof/block`"]
    pub async fn get_sys_pprof_block<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/block", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the running program's command line\n\nReturns the running program's command line, with arguments separated by NUL bytes.\n\nSends a `GET` request to `/sys/pprof/cmdline`"]
    pub async fn get_sys_pprof_cmdline<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/cmdline", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns stack traces of all current goroutines\n\nReturns stack traces of all current goroutines.\n\nSends a `GET` request to `/sys/pprof/goroutine`"]
    pub async fn get_sys_pprof_goroutine<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/goroutine", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns a sampling of memory allocations of live object\n\nReturns a sampling of memory allocations of live object.\n\nSends a `GET` request to `/sys/pprof/heap`"]
    pub async fn get_sys_pprof_heap<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/heap", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns stack traces of holders of contended mutexes\n\nReturns stack traces of holders of contended mutexes\n\nSends a `GET` request to `/sys/pprof/mutex`"]
    pub async fn get_sys_pprof_mutex<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/mutex", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns a pprof-formatted cpu profile payload\n\nReturns a pprof-formatted cpu profile payload. Profiling lasts for duration specified in seconds GET parameter, or for 30 seconds if not specified.\n\nSends a `GET` request to `/sys/pprof/profile`"]
    pub async fn get_sys_pprof_profile<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/profile", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the program counters listed in the request\n\nReturns the program counters listed in the request.\n\nSends a `GET` request to `/sys/pprof/symbol`"]
    pub async fn get_sys_pprof_symbol<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/symbol", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns stack traces that led to the creation of new OS threads\n\nReturns stack traces that led to the creation of new OS threads\n\nSends a `GET` request to `/sys/pprof/threadcreate`"]
    pub async fn get_sys_pprof_threadcreate<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/threadcreate", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the execution trace in binary form\n\nReturns  the execution trace in binary form. Tracing lasts for duration specified in seconds GET parameter, or for 1 second if not specified.\n\nSends a `GET` request to `/sys/pprof/trace`"]
    pub async fn get_sys_pprof_trace<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/pprof/trace", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/quotas/config`"]
    pub async fn get_sys_quotas_config<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/quotas/config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/sys/quotas/config`"]
    pub async fn post_sys_quotas_config<'a>(
        &'a self,
        body: &'a types::PostSysQuotasConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/quotas/config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/quotas/rate-limit`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_quotas_rate_limit<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/quotas/rate-limit", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/quotas/rate-limit/{name}`\n\nArguments:\n- `name`: Name of the quota rule.\n"]
    pub async fn get_sys_quotas_rate_limit_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/quotas/rate-limit/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/sys/quotas/rate-limit/{name}`\n\nArguments:\n- `name`: Name of the quota rule.\n- `body`\n"]
    pub async fn post_sys_quotas_rate_limit_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostSysQuotasRateLimitNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/quotas/rate-limit/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `DELETE` request to `/sys/quotas/rate-limit/{name}`\n\nArguments:\n- `name`: Name of the quota rule.\n"]
    pub async fn delete_sys_quotas_rate_limit_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/quotas/rate-limit/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the value of the key at the given path\n\nSends a `GET` request to `/sys/raw`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_raw<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/raw", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update the value of the key at the given path\n\nSends a `POST` request to `/sys/raw`"]
    pub async fn post_sys_raw<'a>(
        &'a self,
        body: &'a types::PostSysRawBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/raw", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete the key with given path\n\nSends a `DELETE` request to `/sys/raw`"]
    pub async fn delete_sys_raw<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/raw", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the value of the key at the given path\n\nSends a `GET` request to `/sys/raw/{path}`\n\nArguments:\n- `path`\n- `list`: Return a list if `true`\n"]
    pub async fn get_sys_raw_path<'a>(
        &'a self,
        path: &'a str,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/raw/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Update the value of the key at the given path\n\nSends a `POST` request to `/sys/raw/{path}`"]
    pub async fn post_sys_raw_path<'a>(
        &'a self,
        path: &'a str,
        body: &'a types::PostSysRawPathBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/raw/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete the key with given path\n\nSends a `DELETE` request to `/sys/raw/{path}`"]
    pub async fn delete_sys_raw_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/raw/{}",
            self.baseurl,
            progenitor_client::encode_path(&path.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Return the backup copy of PGP-encrypted unseal keys\n\nSends a `GET` request to `/sys/rekey/backup`"]
    pub async fn get_sys_rekey_backup<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/backup", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Delete the backup copy of PGP-encrypted unseal keys\n\nSends a `DELETE` request to `/sys/rekey/backup`"]
    pub async fn delete_sys_rekey_backup<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/backup", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Reads the configuration and progress of the current rekey attempt\n\nSends a `GET` request to `/sys/rekey/init`"]
    pub async fn get_sys_rekey_init<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/init", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Initializes a new rekey attempt\n\nOnly a single rekey attempt can take place at a time, and changing the parameters of a rekey requires canceling and starting a new rekey, which will also provide a new nonce.\n\nSends a `POST` request to `/sys/rekey/init`"]
    pub async fn post_sys_rekey_init<'a>(
        &'a self,
        body: &'a types::PostSysRekeyInitBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/init", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Cancels any in-progress rekey\n\nThis clears the rekey settings as well as any progress made. This must be called to change the parameters of the rekey. Note: verification is still a part of a rekey. If rekeying is canceled during the verification flow, the current unseal keys remain valid.\n\nSends a `DELETE` request to `/sys/rekey/init`"]
    pub async fn delete_sys_rekey_init<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/init", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Allows fetching or deleting the backup of the rotated unseal keys\n\nSends a `GET` request to `/sys/rekey/recovery-key-backup`"]
    pub async fn get_sys_rekey_recovery_key_backup<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/recovery-key-backup", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Allows fetching or deleting the backup of the rotated unseal keys\n\nSends a `DELETE` request to `/sys/rekey/recovery-key-backup`"]
    pub async fn delete_sys_rekey_recovery_key_backup<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/recovery-key-backup", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enter a single master key share to progress the rekey of the Vault\n\nSends a `POST` request to `/sys/rekey/update`"]
    pub async fn post_sys_rekey_update<'a>(
        &'a self,
        body: &'a types::PostSysRekeyUpdateBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/update", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Read the configuration and progress of the current rekey verification attempt\n\nSends a `GET` request to `/sys/rekey/verify`"]
    pub async fn get_sys_rekey_verify<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/verify", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Enter a single new key share to progress the rekey verification operation\n\nSends a `POST` request to `/sys/rekey/verify`"]
    pub async fn post_sys_rekey_verify<'a>(
        &'a self,
        body: &'a types::PostSysRekeyVerifyBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/verify", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Cancel any in-progress rekey verification operation\n\nThis clears any progress made and resets the nonce. Unlike a `DELETE` against `sys/rekey/init`, this only resets the current verification operation, not the entire rekey atttempt.\n\nSends a `DELETE` request to `/sys/rekey/verify`"]
    pub async fn delete_sys_rekey_verify<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rekey/verify", self.baseurl,);
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Move the mount point of an already-mounted backend\n\nSends a `POST` request to `/sys/remount`"]
    pub async fn post_sys_remount<'a>(
        &'a self,
        body: &'a types::PostSysRemountBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/remount", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Renews a lease, requesting to extend the lease\n\nSends a `POST` request to `/sys/renew`"]
    pub async fn post_sys_renew<'a>(
        &'a self,
        body: &'a types::PostSysRenewBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/renew", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Renews a lease, requesting to extend the lease\n\nSends a `POST` request to `/sys/renew/{url_lease_id}`\n\nArguments:\n- `url_lease_id`: The lease identifier to renew. This is included with a lease.\n- `body`\n"]
    pub async fn post_sys_renew_url_lease_id<'a>(
        &'a self,
        url_lease_id: &'a str,
        body: &'a types::PostSysRenewUrlLeaseIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/renew/{}",
            self.baseurl,
            progenitor_client::encode_path(&url_lease_id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/replication/status`"]
    pub async fn get_sys_replication_status<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/replication/status", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes a lease immediately\n\nSends a `POST` request to `/sys/revoke`"]
    pub async fn post_sys_revoke<'a>(
        &'a self,
        body: &'a types::PostSysRevokeBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/revoke", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes all secrets or tokens generated under a given prefix immediately\n\nUnlike `/sys/leases/revoke-prefix`, this path ignores backend errors encountered during revocation. This is potentially very dangerous and should only be used in specific emergency situations where errors in the backend or the connected backend service prevent normal revocation.\n\nBy ignoring these errors, Vault abdicates responsibility for ensuring that the issued credentials or secrets are properly revoked and/or cleaned up. Access to this endpoint should be tightly controlled.\n\nSends a `POST` request to `/sys/revoke-force/{prefix}`\n\nArguments:\n- `prefix`: The path to revoke keys under. Example: \"prod/aws/ops\"\n"]
    pub async fn post_sys_revoke_force_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/revoke-force/{}",
            self.baseurl,
            progenitor_client::encode_path(&prefix.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes all secrets (via a lease ID prefix) or tokens (via the tokens' path property) generated under a given prefix immediately\n\nSends a `POST` request to `/sys/revoke-prefix/{prefix}`\n\nArguments:\n- `prefix`: The path to revoke keys under. Example: \"prod/aws/ops\"\n- `body`\n"]
    pub async fn post_sys_revoke_prefix_prefix<'a>(
        &'a self,
        prefix: &'a str,
        body: &'a types::PostSysRevokePrefixPrefixBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/revoke-prefix/{}",
            self.baseurl,
            progenitor_client::encode_path(&prefix.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Revokes a lease immediately\n\nSends a `POST` request to `/sys/revoke/{url_lease_id}`\n\nArguments:\n- `url_lease_id`: The lease identifier to renew. This is included with a lease.\n- `body`\n"]
    pub async fn post_sys_revoke_url_lease_id<'a>(
        &'a self,
        url_lease_id: &'a str,
        body: &'a types::PostSysRevokeUrlLeaseIdBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/revoke/{}",
            self.baseurl,
            progenitor_client::encode_path(&url_lease_id.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Rotates the backend encryption key used to persist data\n\nSends a `POST` request to `/sys/rotate`"]
    pub async fn post_sys_rotate<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rotate", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/sys/rotate/config`"]
    pub async fn get_sys_rotate_config<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rotate/config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/sys/rotate/config`"]
    pub async fn post_sys_rotate_config<'a>(
        &'a self,
        body: &'a types::PostSysRotateConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/rotate/config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Seal the Vault\n\nSends a `POST` request to `/sys/seal`"]
    pub async fn post_sys_seal<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/seal", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Check the seal status of a Vault\n\nSends a `GET` request to `/sys/seal-status`"]
    pub async fn get_sys_seal_status<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/seal-status", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Cause the node to give up active status\n\nThis endpoint forces the node to give up active status. If the node does not have active status, this endpoint does nothing. Note that the node will sleep for ten seconds before attempting to grab the active lock again, but if no standby nodes grab the active lock in the interim, the same node may become the active node again.\n\nSends a `POST` request to `/sys/step-down`"]
    pub async fn post_sys_step_down<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/step-down", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a hash sum for input data\n\nSends a `POST` request to `/sys/tools/hash`"]
    pub async fn post_sys_tools_hash<'a>(
        &'a self,
        body: &'a types::PostSysToolsHashBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/tools/hash", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a hash sum for input data\n\nSends a `POST` request to `/sys/tools/hash/{urlalgorithm}`\n\nArguments:\n- `urlalgorithm`: Algorithm to use (POST URL parameter)\n- `body`\n"]
    pub async fn post_sys_tools_hash_urlalgorithm<'a>(
        &'a self,
        urlalgorithm: &'a str,
        body: &'a types::PostSysToolsHashUrlalgorithmBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/tools/hash/{}",
            self.baseurl,
            progenitor_client::encode_path(&urlalgorithm.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate random bytes\n\nSends a `POST` request to `/sys/tools/random`"]
    pub async fn post_sys_tools_random<'a>(
        &'a self,
        body: &'a types::PostSysToolsRandomBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/tools/random", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate random bytes\n\nSends a `POST` request to `/sys/tools/random/{urlbytes}`\n\nArguments:\n- `urlbytes`: The number of bytes to generate (POST URL parameter)\n- `body`\n"]
    pub async fn post_sys_tools_random_urlbytes<'a>(
        &'a self,
        urlbytes: &'a str,
        body: &'a types::PostSysToolsRandomUrlbytesBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/sys/tools/random/{}",
            self.baseurl,
            progenitor_client::encode_path(&urlbytes.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Unseal the Vault\n\nSends a `POST` request to `/sys/unseal`"]
    pub async fn post_sys_unseal<'a>(
        &'a self,
        body: &'a types::PostSysUnsealBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/unseal", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Look up wrapping properties for the requester's token\n\nSends a `GET` request to `/sys/wrapping/lookup`"]
    pub async fn get_sys_wrapping_lookup<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/wrapping/lookup", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Look up wrapping properties for the given token\n\nSends a `POST` request to `/sys/wrapping/lookup`"]
    pub async fn post_sys_wrapping_lookup<'a>(
        &'a self,
        body: &'a types::PostSysWrappingLookupBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/wrapping/lookup", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Rotates a response-wrapped token\n\nSends a `POST` request to `/sys/wrapping/rewrap`"]
    pub async fn post_sys_wrapping_rewrap<'a>(
        &'a self,
        body: &'a types::PostSysWrappingRewrapBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/wrapping/rewrap", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Unwraps a response-wrapped token\n\nSends a `POST` request to `/sys/wrapping/unwrap`"]
    pub async fn post_sys_wrapping_unwrap<'a>(
        &'a self,
        body: &'a types::PostSysWrappingUnwrapBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/wrapping/unwrap", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Response-wraps an arbitrary JSON object\n\nSends a `POST` request to `/sys/wrapping/wrap`"]
    pub async fn post_sys_wrapping_wrap<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/sys/wrapping/wrap", self.baseurl,);
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Backup the named key\n\nSends a `GET` request to `/transit/backup/{name}`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn get_transit_backup_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/backup/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the size of the active cache\n\nSends a `GET` request to `/transit/cache-config`"]
    pub async fn get_transit_cache_config<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/cache-config", self.baseurl,);
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configures a new cache of the specified size\n\nSends a `POST` request to `/transit/cache-config`"]
    pub async fn post_transit_cache_config<'a>(
        &'a self,
        body: &'a types::PostTransitCacheConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/cache-config", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a data key\n\nSends a `POST` request to `/transit/datakey/{plaintext}/{name}`\n\nArguments:\n- `plaintext`: \"plaintext\" will return the key in both plaintext and ciphertext; \"wrapped\" will return the ciphertext only.\n- `name`: The backend key used for encrypting the data key\n- `body`\n"]
    pub async fn post_transit_datakey_plaintext_name<'a>(
        &'a self,
        plaintext: &'a str,
        name: &'a str,
        body: &'a types::PostTransitDatakeyPlaintextNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/datakey/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&plaintext.to_string()),
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Decrypt a ciphertext value using a named key\n\nSends a `POST` request to `/transit/decrypt/{name}`\n\nArguments:\n- `name`: Name of the policy\n- `body`\n"]
    pub async fn post_transit_decrypt_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitDecryptNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/decrypt/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Encrypt a plaintext value or a batch of plaintext\nblocks using a named key\n\nSends a `POST` request to `/transit/encrypt/{name}`\n\nArguments:\n- `name`: Name of the policy\n- `body`\n"]
    pub async fn post_transit_encrypt_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitEncryptNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/encrypt/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Export named encryption or signing key\n\nSends a `GET` request to `/transit/export/{type}/{name}`\n\nArguments:\n- `type_`: Type of key to export (encryption-key, signing-key, hmac-key)\n- `name`: Name of the key\n"]
    pub async fn get_transit_export_type_name<'a>(
        &'a self,
        type_: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/export/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Export named encryption or signing key\n\nSends a `GET` request to `/transit/export/{type}/{name}/{version}`\n\nArguments:\n- `type_`: Type of key to export (encryption-key, signing-key, hmac-key)\n- `name`: Name of the key\n- `version`: Version of the key\n"]
    pub async fn get_transit_export_type_name_version<'a>(
        &'a self,
        type_: &'a str,
        name: &'a str,
        version: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/export/{}/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&type_.to_string()),
            progenitor_client::encode_path(&name.to_string()),
            progenitor_client::encode_path(&version.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a hash sum for input data\n\nSends a `POST` request to `/transit/hash`"]
    pub async fn post_transit_hash<'a>(
        &'a self,
        body: &'a types::PostTransitHashBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/hash", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a hash sum for input data\n\nSends a `POST` request to `/transit/hash/{urlalgorithm}`\n\nArguments:\n- `urlalgorithm`: Algorithm to use (POST URL parameter)\n- `body`\n"]
    pub async fn post_transit_hash_urlalgorithm<'a>(
        &'a self,
        urlalgorithm: &'a str,
        body: &'a types::PostTransitHashUrlalgorithmBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/hash/{}",
            self.baseurl,
            progenitor_client::encode_path(&urlalgorithm.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate an HMAC for input data using the named key\n\nSends a `POST` request to `/transit/hmac/{name}`\n\nArguments:\n- `name`: The key to use for the HMAC function\n- `body`\n"]
    pub async fn post_transit_hmac_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitHmacNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/hmac/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate an HMAC for input data using the named key\n\nSends a `POST` request to `/transit/hmac/{name}/{urlalgorithm}`\n\nArguments:\n- `name`: The key to use for the HMAC function\n- `urlalgorithm`: Algorithm to use (POST URL parameter)\n- `body`\n"]
    pub async fn post_transit_hmac_name_urlalgorithm<'a>(
        &'a self,
        name: &'a str,
        urlalgorithm: &'a str,
        body: &'a types::PostTransitHmacNameUrlalgorithmBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/hmac/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
            progenitor_client::encode_path(&urlalgorithm.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Managed named encryption keys\n\nSends a `GET` request to `/transit/keys`\n\nArguments:\n- `list`: Return a list if `true`\n"]
    pub async fn get_transit_keys<'a>(
        &'a self,
        list: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/keys", self.baseurl,);
        let mut query = Vec::new();
        if let Some(v) = &list {
            query.push(("list", v.to_string()));
        }

        let request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Managed named encryption keys\n\nSends a `GET` request to `/transit/keys/{name}`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn get_transit_keys_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Managed named encryption keys\n\nSends a `POST` request to `/transit/keys/{name}`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_transit_keys_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitKeysNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Managed named encryption keys\n\nSends a `DELETE` request to `/transit/keys/{name}`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn delete_transit_keys_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Configure a named encryption key\n\nSends a `POST` request to `/transit/keys/{name}/config`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_transit_keys_name_config<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitKeysNameConfigBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}/config",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Rotate named encryption key\n\nSends a `POST` request to `/transit/keys/{name}/rotate`\n\nArguments:\n- `name`: Name of the key\n"]
    pub async fn post_transit_keys_name_rotate<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}/rotate",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Trim key versions of a named key\n\nSends a `POST` request to `/transit/keys/{name}/trim`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_transit_keys_name_trim<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitKeysNameTrimBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/keys/{}/trim",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate random bytes\n\nSends a `POST` request to `/transit/random`"]
    pub async fn post_transit_random<'a>(
        &'a self,
        body: &'a types::PostTransitRandomBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/random", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate random bytes\n\nSends a `POST` request to `/transit/random/{urlbytes}`\n\nArguments:\n- `urlbytes`: The number of bytes to generate (POST URL parameter)\n- `body`\n"]
    pub async fn post_transit_random_urlbytes<'a>(
        &'a self,
        urlbytes: &'a str,
        body: &'a types::PostTransitRandomUrlbytesBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/random/{}",
            self.baseurl,
            progenitor_client::encode_path(&urlbytes.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Restore the named key\n\nSends a `POST` request to `/transit/restore`"]
    pub async fn post_transit_restore<'a>(
        &'a self,
        body: &'a types::PostTransitRestoreBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/transit/restore", self.baseurl,);
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Restore the named key\n\nSends a `POST` request to `/transit/restore/{name}`\n\nArguments:\n- `name`: If set, this will be the name of the restored key.\n- `body`\n"]
    pub async fn post_transit_restore_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitRestoreNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/restore/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Rewrap ciphertext\n\nSends a `POST` request to `/transit/rewrap/{name}`\n\nArguments:\n- `name`: Name of the key\n- `body`\n"]
    pub async fn post_transit_rewrap_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitRewrapNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/rewrap/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a signature for input data using the named key\n\nSends a `POST` request to `/transit/sign/{name}`\n\nArguments:\n- `name`: The key to use\n- `body`\n"]
    pub async fn post_transit_sign_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitSignNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/sign/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Generate a signature for input data using the named key\n\nSends a `POST` request to `/transit/sign/{name}/{urlalgorithm}`\n\nArguments:\n- `name`: The key to use\n- `urlalgorithm`: Hash algorithm to use (POST URL parameter)\n- `body`\n"]
    pub async fn post_transit_sign_name_urlalgorithm<'a>(
        &'a self,
        name: &'a str,
        urlalgorithm: &'a str,
        body: &'a types::PostTransitSignNameUrlalgorithmBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/sign/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
            progenitor_client::encode_path(&urlalgorithm.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Verify a signature or HMAC for input data created using the named key\n\nSends a `POST` request to `/transit/verify/{name}`\n\nArguments:\n- `name`: The key to use\n- `body`\n"]
    pub async fn post_transit_verify_name<'a>(
        &'a self,
        name: &'a str,
        body: &'a types::PostTransitVerifyNameBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/verify/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Verify a signature or HMAC for input data created using the named key\n\nSends a `POST` request to `/transit/verify/{name}/{urlalgorithm}`\n\nArguments:\n- `name`: The key to use\n- `urlalgorithm`: Hash algorithm to use (POST URL parameter)\n- `body`\n"]
    pub async fn post_transit_verify_name_urlalgorithm<'a>(
        &'a self,
        name: &'a str,
        urlalgorithm: &'a str,
        body: &'a types::PostTransitVerifyNameUrlalgorithmBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/transit/verify/{}/{}",
            self.baseurl,
            progenitor_client::encode_path(&name.to_string()),
            progenitor_client::encode_path(&urlalgorithm.to_string()),
        );
        let request = self.client.post(url).json(body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
