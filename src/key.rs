use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{client::Client, errors::Error};

/// Represent a [meilisearch key](https://docs.meilisearch.com/reference/api/keys.html#returned-fields)
/// You can get a [Key] from the [Client::get_key] method.
/// Or you can create a [Key] with the [KeyBuilder::create] or [Client::create_key] methods.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<String>,
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    /// Update the description of the key.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_description("My not so little lovely test key");
    /// # assert_eq!(key.description, "My not so little lovely test key".to_string());
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_description(&mut self, desc: impl AsRef<str>) -> &mut Self {
        self.description = Some(desc.as_ref().to_string());
        self
    }

    /// Update the name of the key.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_name("lovely key");
    /// # assert_eq!(key.name, "lovely key".to_string());
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_name(&mut self, desc: impl AsRef<str>) -> &mut Self {
        self.name = Some(desc.as_ref().to_string());
        self
    }

    /// Update the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, client::Client};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// let mut key = KeyBuilder::new("My little lovely test key")
    ///   .create(&client).await.unwrap();
    ///
    /// # assert_eq!(key.description, "My little lovely test key");
    ///
    /// key.with_description("My not so little lovely test key");
    /// let key = key.update(&client).await.unwrap();
    ///
    /// # assert_eq!(key.description, "My not so little lovely test key".to_string());
    ///
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn update(&self, client: &Client) -> Result<Key, Error> {
        client.update_key(self).await
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.key
    }
}

impl AsRef<Key> for Key {
    fn as_ref(&self) -> &Key {
        self
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeysQuery<'a> {
    #[serde(skip_serializing)]
    pub client: &'a Client,
    /// The number of documents to skip.
    /// If the value of the parameter `offset` is `n`, the `n` first documents (ordered by relevance) will not be returned.
    /// This is helpful for pagination.
    ///
    /// Example: If you want to skip the first document, set offset to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// The maximum number of documents returned.
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` documents in the response.
    /// This is helpful for pagination.
    ///
    /// Example: If you don't want to get more than two documents, set limit to `2`.
    /// Default: `20`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<'a> KeysQuery<'a> {
    pub fn new(client: &'a Client) -> KeysQuery<'a> {
        KeysQuery {
            client,
            offset: None,
            limit: None,
        }
    }

    pub fn with_offset<'b>(&'b mut self, offset: usize) -> &'b mut KeysQuery<'a> {
        self.offset = Some(offset);
        self
    }
    pub fn with_limit<'b>(&'b mut self, limit: usize) -> &'b mut KeysQuery<'a> {
        self.limit = Some(limit);
        self
    }

    /// Execute the query and fetch the results.
    pub async fn execute(&'a self) -> Result<KeysResults, Error> {
        self.client.execute_get_keys(self).await
    }
}

/// The [KeyBuilder] is an analog to the [Key] type but without all the fields managed by Meilisearch.
/// It's used to create [Key].
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
/// #
/// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # futures::executor::block_on(async move {
/// let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
///
/// let key = KeyBuilder::new("My little lovely test key")
///   .with_action(Action::DocumentsAdd)
///   .with_index("*")
///   .create(&client).await.unwrap();
///
/// assert_eq!(key.description, "My little lovely test key");
/// # client.delete_key(key).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyBuilder {
    pub actions: Vec<Action>,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    pub indexes: Vec<String>,
}

impl KeyBuilder {
    /// Create a [KeyBuilder] with only a description.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let builder = KeyBuilder::new("My little lovely test key");
    /// ```
    pub fn new() -> KeyBuilder {
        Self {
            actions: Vec::new(),
            description: None,
            expires_at: None,
            indexes: Vec::new(),
        }
    }

    /// Declare a set of actions the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::key::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_actions(vec![Action::Search, Action::DocumentsAdd]);
    /// ```
    pub fn with_actions(&mut self, actions: impl IntoIterator<Item = Action>) -> &mut Self {
        self.actions.extend(actions);
        self
    }

    /// Add one action the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::key::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_action(Action::DocumentsAdd);
    /// ```
    pub fn with_action(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    /// Set the expiration date of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// use time::{OffsetDateTime, Duration};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// // create a key that expires in two weeks from now
    /// builder.with_expires_at(OffsetDateTime::now_utc() + Duration::WEEK * 2);
    /// ```
    pub fn with_expires_at(&mut self, expires_at: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set the indexes the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_indexes(vec!["test", "movies"]);
    /// ```
    pub fn with_indexes(
        &mut self,
        indexes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.indexes = indexes
            .into_iter()
            .map(|index| index.as_ref().to_string())
            .collect();
        self
    }

    /// Add one index the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_index("test");
    /// ```
    pub fn with_index(&mut self, index: impl AsRef<str>) -> &mut Self {
        self.indexes.push(index.as_ref().to_string());
        self
    }

    // TODO: with_description

    /// Create a [Key] from the builder.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, client::Client};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// let key = KeyBuilder::new("My little lovely test key")
    ///   .create(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, "My little lovely test key");
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&self, client: &Client) -> Result<Key, Error> {
        client.create_key(self).await
    }
}

impl AsRef<KeyBuilder> for KeyBuilder {
    fn as_ref(&self) -> &KeyBuilder {
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Provides access to everything.
    #[serde(rename = "*")]
    All,
    /// Provides access to both [`POST`](https://docs.meilisearch.com/reference/api/search.md#search-in-an-index-with-post-route) and [`GET`](https://docs.meilisearch.com/reference/api/search.md#search-in-an-index-with-get-route) search endpoints on authorized indexes.
    #[serde(rename = "search")]
    Search,
    /// Provides access to the [add documents](https://docs.meilisearch.com/reference/api/documents.md#add-or-replace-documents) and [update documents](https://docs.meilisearch.com/reference/api/documents.md#add-or-update-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.add")]
    DocumentsAdd,
    /// Provides access to the [get one document](https://docs.meilisearch.com/reference/api/documents.md#get-one-document) and [get documents](https://docs.meilisearch.com/reference/api/documents.md#get-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.get")]
    DocumentsGet,
    /// Provides access to the [delete one document](https://docs.meilisearch.com/reference/api/documents.md#delete-one-document), [delete all documents](https://docs.meilisearch.com/reference/api/documents.md#delete-all-documents), and [batch delete](https://docs.meilisearch.com/reference/api/documents.md#delete-documents-by-batch) endpoints on authorized indexes.
    #[serde(rename = "documents.delete")]
    DocumentsDelete,
    /// Provides access to the [create index](https://docs.meilisearch.com/reference/api/indexes.md#create-an-index) endpoint.
    #[serde(rename = "indexes.create")]
    IndexesCreate,
    /// Provides access to the [get one index](https://docs.meilisearch.com/reference/api/indexes.md#get-one-index) and [list all indexes](https://docs.meilisearch.com/reference/api/indexes.md#list-all-indexes) endpoints. **Non-authorized `indexes` will be omitted from the response**.
    #[serde(rename = "indexes.get")]
    IndexesGet,
    /// Provides access to the [update index](https://docs.meilisearch.com/reference/api/indexes.md#update-an-index) endpoint.
    #[serde(rename = "indexes.update")]
    IndexesUpdate,
    /// Provides access to the [delete index](https://docs.meilisearch.com/reference/api/indexes.md#delete-an-index) endpoint.
    #[serde(rename = "indexes.delete")]
    IndexesDelete,
    /// Provides access to the [get one task](https://docs.meilisearch.com/reference/api/tasks.md#get-task) and [get all tasks](https://docs.meilisearch.com/reference/api/tasks.md#get-all-tasks) endpoints. **Tasks from non-authorized `indexes` will be omitted from the response**. Also provides access to the [get one task by index](https://docs.meilisearch.com/reference/api/tasks.md#get-task-by-index) and [get all tasks by index](https://docs.meilisearch.com/reference/api/tasks.md#get-all-tasks-by-index) endpoints on authorized indexes.
    #[serde(rename = "tasks.get")]
    TasksGet,
    /// Provides access to the [get settings](https://docs.meilisearch.com/reference/api/settings.md#get-settings) endpoint and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.get")]
    SettingsGet,
    /// Provides access to the [update settings](https://docs.meilisearch.com/reference/api/settings.md#update-settings) and [reset settings](https://docs.meilisearch.com/reference/api/settings.md#reset-settings) endpoints and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.update")]
    SettingsUpdate,
    /// Provides access to the [get stats of an index](https://docs.meilisearch.com/reference/api/stats.md#get-stats-of-an-index) endpoint and the [get stats of all indexes](https://docs.meilisearch.com/reference/api/stats.md#get-stats-of-all-indexes) endpoint. For the latter, **non-authorized `indexes` are omitted from the response**.
    #[serde(rename = "stats.get")]
    StatsGet,
    /// Provides access to the [create dump](https://docs.meilisearch.com/reference/api/dump.md#create-a-dump) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.create")]
    DumpsCreate,
    /// Provides access to the [get dump status](https://docs.meilisearch.com/reference/api/dump.md#get-dump-status) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.get")]
    DumpsGet,
    /// Provides access to the [get Meilisearch version](https://docs.meilisearch.com/reference/api/version.md#get-version-of-meilisearch) endpoint.
    #[serde(rename = "version")]
    Version,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeysResults {
    pub results: Vec<Key>,
    pub limit: u32,
    pub offset: u32,
}
