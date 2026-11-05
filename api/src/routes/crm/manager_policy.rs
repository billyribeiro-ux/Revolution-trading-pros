//! Committed CRM policy shared by Rust and SvelteKit. No unknown-route fallback.
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    sync::LazyLock,
};

const COMMITTED: &str = include_str!("../../../assets/policies/crm-permissions.json");
static CURRENT: LazyLock<Result<CompiledPolicy, PolicyError>> =
    LazyLock::new(|| CompiledPolicy::parse(COMMITTED));

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ErrorKind {
    InvalidPolicy,
    Denied,
    InvalidFacts,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PolicyError {
    pub(super) kind: ErrorKind,
    pub(super) message: String,
}
impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}
impl std::error::Error for PolicyError {}
fn invalid(message: impl Into<String>) -> PolicyError {
    PolicyError {
        kind: ErrorKind::InvalidPolicy,
        message: message.into(),
    }
}
fn denied(message: impl Into<String>) -> PolicyError {
    PolicyError {
        kind: ErrorKind::Denied,
        message: message.into(),
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(super) enum Action {
    View,
    Create,
    Edit,
    Delete,
    Manage,
}
impl Action {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::View => "view",
            Self::Create => "create",
            Self::Edit => "edit",
            Self::Delete => "delete",
            Self::Manage => "manage",
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub(super) enum Method {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
}
impl Method {
    pub(super) fn parse(value: &str) -> Option<Self> {
        match value {
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "POST" => Some(Self::Post),
            "PUT" => Some(Self::Put),
            "PATCH" => Some(Self::Patch),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Parameter {
    PositiveInteger,
    Uuid,
    Slug,
}
impl Parameter {
    fn matches(self, value: &str) -> bool {
        match self {
            Self::PositiveInteger => {
                value
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| matches!(byte, b'1'..=b'9'))
                    && value.bytes().all(|byte| byte.is_ascii_digit())
                    && value.parse::<i64>().is_ok_and(|number| number > 0)
            }
            Self::Uuid => {
                value.len() == 36
                    && value.bytes().enumerate().all(|(position, byte)| {
                        if matches!(position, 8 | 13 | 18 | 23) {
                            byte == b'-'
                        } else {
                            byte.is_ascii_hexdigit()
                        }
                    })
            }
            Self::Slug => {
                !value.is_empty()
                    && value.len() <= 100
                    && value
                        .as_bytes()
                        .first()
                        .is_some_and(u8::is_ascii_alphabetic)
                    && value
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
            }
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(super) enum Context {
    Body,
    Record,
    History,
    ImportPlan,
    ExportRecord,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum PageMode {
    All,
    Any,
}

fn unique_map<'de, D, T>(deserializer: D) -> Result<BTreeMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Unique<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> de::Visitor<'de> for Unique<T> {
        type Value = BTreeMap<String, T>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an object without duplicate keys")
        }
        fn visit_map<M: de::MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
            let mut result = BTreeMap::new();
            while let Some((key, value)) = access.next_entry::<String, T>()? {
                if result.insert(key.clone(), value).is_some() {
                    return Err(de::Error::custom(format!("duplicate policy key: {key}")));
                }
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(Unique(std::marker::PhantomData))
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub(super) struct Capability {
    pub(super) module: String,
    pub(super) action: Action,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Permission {
    pub(super) module: String,
    pub(super) action: String,
    pub(super) allowed: bool,
}
/// Use with Axum Query::try_from_uri; duplicate selector keys must fail before
/// choosing a route requirement. Do not collect query pairs into a lossy map.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(super) struct Selectors(
    #[serde(deserialize_with = "unique_map")] pub(super) BTreeMap<String, String>,
);
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Module {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) notes: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Fact {
    pub(super) id: String,
    pub(super) context: Context,
    pub(super) source_fields: Vec<String>,
    pub(super) description: String,
    pub(super) all: Vec<Capability>,
    pub(super) global_admin_only: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    pub(super) path: String,
    pub(super) methods: Vec<Method>,
    pub(super) all: Vec<Capability>,
    #[serde(deserialize_with = "unique_map")]
    pub(super) params: BTreeMap<String, Parameter>,
    #[serde(deserialize_with = "unique_map")]
    pub(super) query_equals: BTreeMap<String, String>,
    pub(super) conditional: Vec<String>,
    pub(super) global_admin_only: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Operation {
    pub(super) method: Method,
    pub(super) path: String,
    #[serde(deserialize_with = "unique_map")]
    pub(super) query_equals: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Page {
    pub(super) path: String,
    #[serde(deserialize_with = "unique_map")]
    pub(super) params: BTreeMap<String, Parameter>,
    pub(super) mode: PageMode,
    pub(super) operations: Vec<Operation>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Document {
    pub(super) policy_version: u32,
    pub(super) api_prefix: String,
    pub(super) capabilities_path: String,
    pub(super) actions: Vec<Action>,
    pub(super) manage_implies: Vec<Action>,
    pub(super) modules: Vec<Module>,
    pub(super) facts: Vec<Fact>,
    pub(super) routes: Vec<Route>,
    pub(super) pages: Vec<Page>,
}
pub(super) struct CompiledPolicy {
    pub(super) document: Document,
    pub(super) policy_hash: String,
}
pub(super) fn current() -> Result<&'static CompiledPolicy, &'static PolicyError> {
    CURRENT.as_ref()
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty()
        && value.chars().count() <= maximum
        && !value.chars().any(char::is_control)
}
fn variable(segment: &str) -> Option<&str> {
    segment.strip_prefix('{')?.strip_suffix('}')
}
fn segments(path: &str) -> Vec<&str> {
    path.strip_prefix('/').unwrap_or(path).split('/').collect()
}
fn validate_path(path: &str, parameters: &BTreeMap<String, Parameter>) -> Result<(), PolicyError> {
    if !path.starts_with('/')
        || path.len() > 250
        || path.ends_with('/')
        || path.contains(['?', '#', '%', '\\'])
    {
        return Err(invalid(format!("noncanonical policy path: {path}")));
    }
    let mut expected = BTreeSet::new();
    for segment in segments(path) {
        if let Some(parameter) = variable(segment) {
            if !identifier(parameter) || !expected.insert(parameter) {
                return Err(invalid(format!("invalid/repeated path parameter: {path}")));
            }
        } else if segment.is_empty()
            || !segment
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_lowercase)
            || !segment.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
            })
        {
            return Err(invalid(format!("invalid static path segment: {path}")));
        }
    }
    if expected != parameters.keys().map(String::as_str).collect() {
        return Err(invalid(format!(
            "missing/unexpected typed parameters: {path}"
        )));
    }
    Ok(())
}
fn parameters_overlap(left: Parameter, right: Parameter) -> bool {
    !matches!(
        (left, right),
        (
            Parameter::PositiveInteger,
            Parameter::Uuid | Parameter::Slug
        ) | (
            Parameter::Uuid | Parameter::Slug,
            Parameter::PositiveInteger
        )
    )
}
fn paths_overlap(
    left: &str,
    left_params: &BTreeMap<String, Parameter>,
    right: &str,
    right_params: &BTreeMap<String, Parameter>,
) -> bool {
    let left = segments(left);
    let right = segments(right);
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| match (variable(left), variable(right)) {
                (Some(left), Some(right)) => {
                    parameters_overlap(left_params[left], right_params[right])
                }
                (Some(left), None) => left_params[left].matches(right),
                (None, Some(right)) => right_params[right].matches(left),
                (None, None) => *left == right,
            })
}
fn concrete_path_matches(
    template: &str,
    parameters: &BTreeMap<String, Parameter>,
    path: &str,
) -> bool {
    let expected = segments(template);
    let actual = segments(path);
    expected.len() == actual.len()
        && expected.iter().zip(actual).all(|(expected, actual)| {
            variable(expected).map_or(*expected == actual, |parameter| {
                parameters[parameter].matches(actual)
            })
        })
}
fn query_matches(expected: &BTreeMap<String, String>, actual: &BTreeMap<String, String>) -> bool {
    expected
        .iter()
        .all(|(key, value)| actual.get(key) == Some(value))
}
fn queries_overlap(left: &BTreeMap<String, String>, right: &BTreeMap<String, String>) -> bool {
    !left
        .iter()
        .any(|(key, value)| right.get(key).is_some_and(|other| other != value))
}

impl CompiledPolicy {
    pub(super) fn parse(source: &str) -> Result<Self, PolicyError> {
        let document: Document = serde_json::from_str(source)
            .map_err(|error| invalid(format!("invalid CRM policy JSON: {error}")))?;
        document.validate()?;
        let value: Value =
            serde_json::from_str(source).map_err(|error| invalid(error.to_string()))?;
        let mut canonical = Vec::new();
        write_canonical(&value, &mut canonical)?;
        Ok(Self {
            document,
            policy_hash: hex::encode(Sha256::digest(canonical)),
        })
    }
}
fn write_canonical(value: &Value, output: &mut Vec<u8>) -> Result<(), PolicyError> {
    match value {
        Value::Object(values) => {
            output.push(b'{');
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                serde_json::to_writer(&mut *output, key)
                    .map_err(|error| invalid(error.to_string()))?;
                output.push(b':');
                write_canonical(value, output)?;
            }
            output.push(b'}');
        }
        Value::Array(values) => {
            output.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_canonical(value, output)?;
            }
            output.push(b']');
        }
        _ => serde_json::to_writer(output, value).map_err(|error| invalid(error.to_string()))?,
    }
    Ok(())
}
impl Document {
    fn capabilities_valid(&self, values: &[Capability], allow_empty: bool) -> bool {
        let mut unique = BTreeSet::new();
        (allow_empty || !values.is_empty())
            && values.len() <= 100
            && values.iter().all(|value| {
                self.modules.iter().any(|module| module.id == value.module)
                    && self.actions.contains(&value.action)
                    && unique.insert(value)
            })
    }
    fn validate(&self) -> Result<(), PolicyError> {
        if self.policy_version != 1
            || self.api_prefix != "/api/admin/crm"
            || self.capabilities_path != "/permissions"
            || self.actions
                != [
                    Action::View,
                    Action::Create,
                    Action::Edit,
                    Action::Delete,
                    Action::Manage,
                ]
            || self.manage_implies != [Action::View, Action::Create, Action::Edit, Action::Delete]
            || !(1..=64).contains(&self.modules.len())
            || !(1..=512).contains(&self.routes.len())
            || self.facts.len() > 100
            || self.pages.len() > 250
        {
            return Err(invalid(
                "unsupported CRM policy version, namespace, actions or document bounds",
            ));
        }
        let mut modules = BTreeSet::new();
        for module in &self.modules {
            if !identifier(&module.id)
                || !modules.insert(&module.id)
                || !bounded_text(&module.name, 100)
                || module.notes.len() > 20
                || module.notes.iter().any(|note| !bounded_text(note, 2000))
            {
                return Err(invalid("invalid/duplicate CRM module metadata"));
            }
        }
        let mut facts = BTreeSet::new();
        for fact in &self.facts {
            if !identifier(&fact.id)
                || !facts.insert(&fact.id)
                || !bounded_text(&fact.description, 2000)
                || !(1..=10).contains(&fact.source_fields.len())
                || fact.source_fields.iter().any(|field| {
                    !field.starts_with('/')
                        || field.len() > 200
                        || field.chars().any(char::is_control)
                })
                || !self.capabilities_valid(&fact.all, fact.global_admin_only)
            {
                return Err(invalid("invalid/duplicate conditional CRM requirement"));
            }
        }
        let mut used_facts = BTreeSet::new();
        for (index, route) in self.routes.iter().enumerate() {
            validate_path(&route.path, &route.params)?;
            if route.path == self.capabilities_path
                || route.methods.is_empty()
                || route.methods.len() > 6
                || route.methods.iter().collect::<BTreeSet<_>>().len() != route.methods.len()
                || route.methods.contains(&Method::Get) != route.methods.contains(&Method::Head)
                || !self.capabilities_valid(&route.all, route.global_admin_only)
                || route.query_equals.len() > 4
                || route
                    .query_equals
                    .iter()
                    .any(|(key, value)| !identifier(key) || !bounded_text(value, 100))
                || route.conditional.iter().collect::<BTreeSet<_>>().len()
                    != route.conditional.len()
            {
                return Err(invalid(format!(
                    "invalid CRM route requirement: {}",
                    route.path
                )));
            }
            for identity in &route.conditional {
                let fact = self
                    .fact(identity)
                    .ok_or_else(|| invalid(format!("unregistered CRM fact: {identity}")))?;
                if fact.context == Context::Body
                    && route
                        .methods
                        .iter()
                        .any(|method| matches!(method, Method::Get | Method::Head | Method::Delete))
                {
                    return Err(invalid(format!(
                        "body requirement on a bodyless route: {}",
                        route.path
                    )));
                }
                used_facts.insert(identity);
            }
            for previous in &self.routes[..index] {
                if route
                    .methods
                    .iter()
                    .any(|method| previous.methods.contains(method))
                    && paths_overlap(&route.path, &route.params, &previous.path, &previous.params)
                    && queries_overlap(&route.query_equals, &previous.query_equals)
                {
                    return Err(invalid(format!(
                        "ambiguous CRM route rules: {} and {}",
                        route.path, previous.path
                    )));
                }
            }
        }
        if used_facts != facts {
            return Err(invalid("unused conditional CRM requirement"));
        }
        for (index, page) in self.pages.iter().enumerate() {
            validate_path(&page.path, &page.params)?;
            if !(1..=50).contains(&page.operations.len()) {
                return Err(invalid("CRM pages require explicit operation references"));
            }
            let mut operations = BTreeSet::new();
            for operation in &page.operations {
                if self.operation(operation).is_none()
                    || !operations.insert((
                        operation.method,
                        &operation.path,
                        &operation.query_equals,
                    ))
                {
                    return Err(invalid(format!(
                        "unknown/duplicate page operation: {}",
                        page.path
                    )));
                }
            }
            if self.pages[..index].iter().any(|previous| {
                paths_overlap(&page.path, &page.params, &previous.path, &previous.params)
            }) {
                return Err(invalid(format!("ambiguous CRM page rule: {}", page.path)));
            }
        }
        Ok(())
    }
    pub(super) fn fact(&self, identity: &str) -> Option<&Fact> {
        self.facts.iter().find(|fact| fact.id == identity)
    }
    pub(super) fn operation(&self, operation: &Operation) -> Option<&Route> {
        self.routes.iter().find(|route| {
            route.path == operation.path
                && route.methods.contains(&operation.method)
                && route.query_equals == operation.query_equals
        })
    }
    pub(super) fn matched_route(
        &self,
        method: &str,
        registered_path: &str,
        query: &BTreeMap<String, String>,
    ) -> Option<&Route> {
        let method = Method::parse(method)?;
        let relative = registered_path.strip_prefix(&self.api_prefix)?;
        self.routes.iter().find(|route| {
            route.path == relative
                && route.methods.contains(&method)
                && query_matches(&route.query_equals, query)
        })
    }
    pub(super) fn concrete_route(
        &self,
        method: &str,
        path: &str,
        query: &BTreeMap<String, String>,
    ) -> Option<&Route> {
        let method = Method::parse(method)?;
        let relative = path.strip_prefix(&self.api_prefix)?;
        self.routes.iter().find(|route| {
            route.methods.contains(&method)
                && concrete_path_matches(&route.path, &route.params, relative)
                && query_matches(&route.query_equals, query)
        })
    }
    pub(super) fn page(&self, path: &str) -> Option<&Page> {
        self.pages
            .iter()
            .find(|page| concrete_path_matches(&page.path, &page.params, path))
    }
    pub(super) fn valid_permissions(&self, permissions: &[Permission]) -> bool {
        let mut unique = BTreeSet::new();
        permissions.len() <= 100
            && permissions.iter().all(|permission| {
                self.modules
                    .iter()
                    .any(|module| module.id == permission.module)
                    && self
                        .actions
                        .iter()
                        .any(|action| action.as_str() == permission.action)
                    && unique.insert((&permission.module, &permission.action))
            })
    }
    pub(super) fn allows(&self, permissions: &[Permission], module: &str, action: Action) -> bool {
        self.valid_permissions(permissions)
            && permissions.iter().any(|permission| {
                permission.allowed
                    && permission.module == module
                    && (permission.action == action.as_str()
                        || permission.action == "manage" && self.manage_implies.contains(&action))
            })
    }
    pub(super) fn authorize_base(
        &self,
        route: &Route,
        global: bool,
        permissions: &[Permission],
    ) -> Result<(), PolicyError> {
        if global {
            return Ok(());
        }
        if route.global_admin_only
            || !route
                .all
                .iter()
                .all(|required| self.allows(permissions, &required.module, required.action))
        {
            return Err(denied("CRM permission denied"));
        }
        Ok(())
    }
    /// Call with actual typed facts, including false values for every expected
    /// condition in this context. Missing/extra facts are implementation errors,
    /// never an authorization fallback. No client-supplied capabilities are used.
    pub(super) fn authorize_facts(
        &self,
        route: &Route,
        context: Context,
        actual: &BTreeMap<String, bool>,
        global: bool,
        permissions: &[Permission],
    ) -> Result<(), PolicyError> {
        let expected = route
            .conditional
            .iter()
            .filter_map(|identity| self.fact(identity))
            .filter(|fact| fact.context == context)
            .collect::<Vec<_>>();
        if actual.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != expected.iter().map(|fact| fact.id.as_str()).collect()
        {
            return Err(PolicyError {
                kind: ErrorKind::InvalidFacts,
                message: "CRM conditional facts are incomplete or unexpected".into(),
            });
        }
        for fact in expected {
            if actual[&fact.id]
                && !global
                && (fact.global_admin_only
                    || !fact.all.iter().all(|required| {
                        self.allows(permissions, &required.module, required.action)
                    }))
            {
                return Err(denied(format!("CRM permission denied for {}", fact.id)));
            }
        }
        Ok(())
    }
    pub(super) fn authorize_page(
        &self,
        page: &Page,
        global: bool,
        permissions: &[Permission],
    ) -> bool {
        let allowed = |operation: &Operation| {
            self.operation(operation)
                .is_some_and(|route| self.authorize_base(route, global, permissions).is_ok())
        };
        match page.mode {
            PageMode::All => page.operations.iter().all(allowed),
            PageMode::Any => page.operations.iter().any(allowed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn policy() -> CompiledPolicy {
        CompiledPolicy::parse(COMMITTED).unwrap()
    }
    fn altered(change: impl FnOnce(&mut Value)) -> Result<CompiledPolicy, PolicyError> {
        let mut value: Value = serde_json::from_str(COMMITTED).unwrap();
        change(&mut value);
        CompiledPolicy::parse(&serde_json::to_string(&value).unwrap())
    }
    fn grant(module: &str, action: &str) -> Permission {
        Permission {
            module: module.into(),
            action: action.into(),
            allowed: true,
        }
    }
    fn route<'a>(policy: &'a Document, method: &str, path: &str) -> &'a Route {
        policy
            .matched_route(method, &format!("/api/admin/crm{path}"), &BTreeMap::new())
            .unwrap()
    }
    fn body<'a>(policy: &'a Document, path: &str) -> (&'a Route, BTreeMap<String, bool>) {
        let route = route(policy, "POST", path);
        let facts = route
            .conditional
            .iter()
            .filter_map(|identity| policy.fact(identity))
            .filter(|fact| fact.context == Context::Body)
            .map(|fact| (fact.id.clone(), false))
            .collect();
        (route, facts)
    }
    #[test]
    fn committed_policy_validates_and_canonical_hash_ignores_object_key_order_and_whitespace() {
        let compiled = policy();
        assert_eq!(compiled.document.modules.len(), 20);
        assert_eq!(compiled.policy_hash.len(), 64);
        let value: Value = serde_json::from_str(COMMITTED).unwrap();
        assert_eq!(
            compiled.policy_hash,
            CompiledPolicy::parse(&serde_json::to_string(&value).unwrap())
                .unwrap()
                .policy_hash
        );
        assert!(current().is_ok());
    }
    #[test]
    fn dashboard_aggregate_requires_the_actual_cross_module_read_bundle() {
        let compiled = policy();
        let document = &compiled.document;
        let selected = route(document, "GET", "/stats");
        let grants = vec![Permission {
            module: "dashboard".into(),
            action: "manage".into(),
            allowed: true,
        }];
        assert!(document.authorize_base(selected, false, &grants).is_err());
        let complete = ["dashboard", "contacts", "deals", "campaigns"]
            .into_iter()
            .map(|module| Permission {
                module: module.into(),
                action: "view".into(),
                allowed: true,
            })
            .collect::<Vec<_>>();
        document.authorize_base(selected, false, &complete).unwrap();
        let current = BTreeMap::from([("record_member_audience".into(), true)]);
        assert!(
            document
                .authorize_facts(selected, Context::Record, &current, false, &complete)
                .is_err()
        );
        let captured = BTreeMap::from([("history_member_audience".into(), true)]);
        assert!(
            document
                .authorize_facts(selected, Context::History, &captured, false, &complete)
                .is_err()
        );
    }
    #[test]
    fn schema_rejects_unknown_fields_keys_actions_and_routes_that_overlap() {
        assert!(altered(|value| value["unexpected"] = Value::Bool(true)).is_err());
        assert!(
            altered(|value| value["routes"][0]["all"][0]["module"] = Value::String("*".into()))
                .is_err()
        );
        assert!(
            altered(|value| value["routes"][0]["all"][0]["action"] = Value::String("send".into()))
                .is_err()
        );
        assert!(
            altered(|value| {
                let first = value["routes"][0].clone();
                value["routes"].as_array_mut().unwrap().push(first);
            })
            .is_err()
        );
        let duplicated = COMMITTED.replacen(
            "\"policy_version\": 1",
            "\"policy_version\": 1, \"policy_version\": 1",
            1,
        );
        assert!(CompiledPolicy::parse(&duplicated).is_err());
        assert!(
            altered(|value| {
                for route in value["routes"].as_array_mut().unwrap() {
                    if route["path"] == "/contacts/{id}" {
                        route["params"]["id"] = Value::String("slug".into());
                    }
                }
            })
            .is_err()
        );
    }
    #[test]
    fn duplicate_selector_maps_and_unknown_facts_fail_closed() {
        let duplicated = COMMITTED.replacen(
            "\"kind\": \"lists\"",
            "\"kind\": \"lists\", \"kind\": \"tags\"",
            1,
        );
        assert!(CompiledPolicy::parse(&duplicated).is_err());
        assert!(serde_json::from_str::<Selectors>(r#"{"kind":"tags","kind":"lists"}"#).is_err());
        assert_eq!(
            serde_json::from_str::<Selectors>(r#"{"kind":"tags"}"#)
                .unwrap()
                .0["kind"],
            "tags"
        );
        assert!(
            altered(
                |value| value["routes"][0]["conditional"] = serde_json::json!(["not_registered"])
            )
            .is_err()
        );
        assert!(
            altered(|value| value["pages"][0]["operations"][0]["path"] =
                Value::String("/future-unregistered".into()))
            .is_err()
        );
    }
    #[test]
    fn exact_typed_paths_and_query_selectors_have_no_unknown_fallback() {
        let compiled = policy();
        let policy = &compiled.document;
        assert!(
            policy
                .concrete_route("GET", "/api/admin/crm/contacts/123", &BTreeMap::new())
                .is_some()
        );
        for path in [
            "/api/admin/crm/contacts/0",
            "/api/admin/crm/contacts/01",
            "/api/admin/crm/contacts/abc",
            "/api/admin/crm/contacts/123/future",
            "/api/admin/email/sequences/123",
        ] {
            assert!(
                policy
                    .concrete_route("GET", path, &BTreeMap::new())
                    .is_none(),
                "{path}"
            );
        }
        assert!(
            policy
                .concrete_route(
                    "POST",
                    "/api/admin/crm/contacts/123/emails",
                    &BTreeMap::new()
                )
                .is_none()
        );
        assert!(
            policy
                .concrete_route(
                    "GET",
                    "/api/admin/crm/imports/options",
                    &BTreeMap::from([("kind".into(), "tags".into())])
                )
                .is_some()
        );
        assert!(
            policy
                .concrete_route(
                    "GET",
                    "/api/admin/crm/imports/options",
                    &BTreeMap::from([("kind".into(), "members".into())])
                )
                .is_none()
        );
    }
    #[test]
    fn manage_is_module_local_and_invalid_stored_policies_deny() {
        let compiled = policy();
        let policy = &compiled.document;
        let grants = vec![grant("contacts", "manage")];
        for action in &policy.actions {
            assert!(policy.allows(&grants, "contacts", *action));
        }
        assert!(!policy.allows(&grants, "templates", Action::View));
        assert!(!policy.allows(&[grant("*", "manage")], "contacts", Action::View));
        assert!(!policy.allows(
            &[grant("contacts", "view"), grant("contacts", "view")],
            "contacts",
            Action::View
        ));
    }
    #[test]
    fn custom_composition_does_not_inherit_native_template_rights() {
        let compiled = policy();
        let policy = &compiled.document;
        let grants = vec![grant("contacts", "manage")];
        let (selected, mut facts) = body(policy, "/contacts/{id}/send-email");
        assert!(policy.authorize_base(selected, false, &grants).is_ok());
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &grants)
                .is_ok()
        );
        facts.insert("body_native_template".into(), true);
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &grants)
                .is_err()
        );
        assert!(
            policy
                .authorize_base(
                    route(policy, "GET", "/contacts/native-templates"),
                    false,
                    &grants
                )
                .is_err()
        );
        let complete = vec![grant("contacts", "manage"), grant("templates", "view")];
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &complete)
                .is_ok()
        );
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &BTreeMap::new(), false, &complete)
                .is_err()
        );
        assert_eq!(
            policy
                .authorize_facts(selected, Context::Body, &BTreeMap::new(), false, &complete)
                .unwrap_err()
                .kind,
            ErrorKind::InvalidFacts
        );
        assert_eq!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &grants)
                .unwrap_err()
                .kind,
            ErrorKind::Denied
        );
    }
    #[test]
    fn member_selection_is_global_but_custom_events_are_not_directory_selection() {
        let compiled = policy();
        let policy = &compiled.document;
        let grants = vec![grant("sequences", "manage"), grant("contacts", "view")];
        assert!(
            policy
                .authorize_base(route(policy, "POST", "/sequences/events"), false, &grants)
                .is_ok()
        );
        let (sequence, facts) = body(policy, "/sequences");
        assert!(
            policy
                .authorize_facts(sequence, Context::Body, &facts, false, &grants)
                .is_ok()
        );
        let mut selected = facts;
        selected.insert("body_member_audience".into(), true);
        assert!(
            policy
                .authorize_facts(sequence, Context::Body, &selected, false, &grants)
                .is_err()
        );
        assert!(
            policy
                .authorize_facts(sequence, Context::Body, &selected, true, &grants)
                .is_ok()
        );
        let history = route(policy, "GET", "/sequences/{id}/tests");
        assert!(
            policy
                .authorize_facts(
                    history,
                    Context::History,
                    &BTreeMap::from([("history_member_audience".into(), true)]),
                    false,
                    &grants
                )
                .is_err()
        );
    }
    #[test]
    fn generic_sequence_authoring_cannot_smuggle_runtime_status_controls() {
        let compiled = policy();
        let policy = &compiled.document;
        let selected = route(policy, "PUT", "/sequences/{id}");
        let permissions = vec![grant("sequences", "view"), grant("sequences", "edit")];
        assert!(policy.authorize_base(selected, false, &permissions).is_ok());
        let mut facts = selected
            .conditional
            .iter()
            .filter_map(|identity| policy.fact(identity))
            .filter(|fact| fact.context == Context::Body)
            .map(|fact| (fact.id.clone(), false))
            .collect::<BTreeMap<_, _>>();
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &permissions)
                .is_ok()
        );
        facts.insert("body_sequence_execution_status".into(), true);
        assert_eq!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &permissions)
                .unwrap_err()
                .kind,
            ErrorKind::Denied
        );
        let manager = vec![grant("sequences", "manage")];
        assert!(
            policy
                .authorize_facts(selected, Context::Body, &facts, false, &manager)
                .is_ok()
        );
    }
    #[test]
    fn transfers_and_security_management_do_not_implicitly_grant_other_modules() {
        let compiled = policy();
        let policy = &compiled.document;
        let imports = route(policy, "POST", "/imports/{id}/start");
        let grants = vec![grant("imports", "manage"), grant("contacts", "view")];
        assert!(policy.authorize_base(imports, false, &grants).is_ok());
        let facts = BTreeMap::from([
            ("import_plan_creates".into(), true),
            ("import_plan_updates".into(), false),
            ("import_plan_tags".into(), false),
            ("import_plan_lists".into(), false),
        ]);
        assert!(
            policy
                .authorize_facts(imports, Context::ImportPlan, &facts, false, &grants)
                .is_err()
        );
        assert!(
            policy
                .authorize_base(
                    route(policy, "GET", "/exports/{id}/download"),
                    false,
                    &[grant("exports", "manage")]
                )
                .is_err()
        );
        assert!(
            policy
                .authorize_base(
                    route(policy, "POST", "/manager-roles"),
                    false,
                    &[grant("managers", "manage")]
                )
                .is_err()
        );
        assert!(
            policy
                .authorize_base(
                    route(policy, "GET", "/managers/users"),
                    false,
                    &[grant("managers", "manage")]
                )
                .is_err()
        );
    }
    #[test]
    fn pages_reuse_canonical_operations_and_global_roster_constraints() {
        let compiled = policy();
        let policy = &compiled.document;
        let grants = vec![grant("managers", "manage")];
        let editor = policy.page("/admin/crm/managers/123/edit").unwrap();
        assert!(!policy.authorize_page(editor, false, &grants));
        assert!(policy.authorize_page(editor, true, &[]));
        assert!(policy.page("/admin/email/sequences/123").is_none());
        assert!(!policy.authorize_page(policy.page("/admin/crm").unwrap(), false, &[]));
    }
}
