// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct UpsertUserParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub email: T1,
    pub display_name: T2,
}
#[derive(Debug)]
pub struct UpsertProjectParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub owner_id: i64,
    pub key: T1,
    pub name: T2,
}
#[derive(Debug)]
pub struct CreateTaskParams<T1: crate::StringSql> {
    pub project_id: i64,
    pub assignee_id: i64,
    pub title: T1,
    pub priority: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpsertUser {
    pub id: i64,
    pub email: String,
    pub display_name: String,
}
pub struct UpsertUserBorrowed<'a> {
    pub id: i64,
    pub email: &'a str,
    pub display_name: &'a str,
}
impl<'a> From<UpsertUserBorrowed<'a>> for UpsertUser {
    fn from(
        UpsertUserBorrowed {
            id,
            email,
            display_name,
        }: UpsertUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            display_name: display_name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpsertProject {
    pub id: i64,
    pub key: String,
    pub name: String,
}
pub struct UpsertProjectBorrowed<'a> {
    pub id: i64,
    pub key: &'a str,
    pub name: &'a str,
}
impl<'a> From<UpsertProjectBorrowed<'a>> for UpsertProject {
    fn from(UpsertProjectBorrowed { id, key, name }: UpsertProjectBorrowed<'a>) -> Self {
        Self {
            id,
            key: key.into(),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTask {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub priority: i32,
}
pub struct CreateTaskBorrowed<'a> {
    pub id: i64,
    pub title: &'a str,
    pub status: &'a str,
    pub priority: i32,
}
impl<'a> From<CreateTaskBorrowed<'a>> for CreateTask {
    fn from(
        CreateTaskBorrowed {
            id,
            title,
            status,
            priority,
        }: CreateTaskBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            status: status.into(),
            priority,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CompleteTask {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub priority: i32,
}
pub struct CompleteTaskBorrowed<'a> {
    pub id: i64,
    pub title: &'a str,
    pub status: &'a str,
    pub priority: i32,
}
impl<'a> From<CompleteTaskBorrowed<'a>> for CompleteTask {
    fn from(
        CompleteTaskBorrowed {
            id,
            title,
            status,
            priority,
        }: CompleteTaskBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            status: status.into(),
            priority,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TaskBoard {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub priority: i32,
    pub assignee_name: String,
}
pub struct TaskBoardBorrowed<'a> {
    pub id: i64,
    pub title: &'a str,
    pub status: &'a str,
    pub priority: i32,
    pub assignee_name: &'a str,
}
impl<'a> From<TaskBoardBorrowed<'a>> for TaskBoard {
    fn from(
        TaskBoardBorrowed {
            id,
            title,
            status,
            priority,
            assignee_name,
        }: TaskBoardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            status: status.into(),
            priority,
            assignee_name: assignee_name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectSummary {
    pub key: String,
    pub name: String,
    pub total_tasks: i64,
    pub open_tasks: i64,
    pub done_tasks: i64,
}
pub struct ProjectSummaryBorrowed<'a> {
    pub key: &'a str,
    pub name: &'a str,
    pub total_tasks: i64,
    pub open_tasks: i64,
    pub done_tasks: i64,
}
impl<'a> From<ProjectSummaryBorrowed<'a>> for ProjectSummary {
    fn from(
        ProjectSummaryBorrowed {
            key,
            name,
            total_tasks,
            open_tasks,
            done_tasks,
        }: ProjectSummaryBorrowed<'a>,
    ) -> Self {
        Self {
            key: key.into(),
            name: name.into(),
            total_tasks,
            open_tasks,
            done_tasks,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UpsertUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpsertUserBorrowed, tokio_postgres::Error>,
    mapper: fn(UpsertUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpsertUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpsertUserBorrowed) -> R,
    ) -> UpsertUserQuery<'c, 'a, 's, C, R, N> {
        UpsertUserQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UpsertProjectQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpsertProjectBorrowed, tokio_postgres::Error>,
    mapper: fn(UpsertProjectBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpsertProjectQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpsertProjectBorrowed) -> R,
    ) -> UpsertProjectQuery<'c, 'a, 's, C, R, N> {
        UpsertProjectQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreateTaskQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateTaskBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateTaskBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateTaskQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateTaskBorrowed) -> R,
    ) -> CreateTaskQuery<'c, 'a, 's, C, R, N> {
        CreateTaskQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CompleteTaskQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CompleteTaskBorrowed, tokio_postgres::Error>,
    mapper: fn(CompleteTaskBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CompleteTaskQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CompleteTaskBorrowed) -> R,
    ) -> CompleteTaskQuery<'c, 'a, 's, C, R, N> {
        CompleteTaskQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct TaskBoardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TaskBoardBorrowed, tokio_postgres::Error>,
    mapper: fn(TaskBoardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TaskBoardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(TaskBoardBorrowed) -> R) -> TaskBoardQuery<'c, 'a, 's, C, R, N> {
        TaskBoardQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct ProjectSummaryQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ProjectSummaryBorrowed, tokio_postgres::Error>,
    mapper: fn(ProjectSummaryBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProjectSummaryQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProjectSummaryBorrowed) -> R,
    ) -> ProjectSummaryQuery<'c, 'a, 's, C, R, N> {
        ProjectSummaryQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct DeleteProjectByKeyStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_project_by_key() -> DeleteProjectByKeyStmt {
    DeleteProjectByKeyStmt("DELETE FROM projects WHERE key = $1", None)
}
impl DeleteProjectByKeyStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        key: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[key]).await
    }
}
pub struct UpsertUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert_user() -> UpsertUserStmt {
    UpsertUserStmt(
        "INSERT INTO app_users (email, display_name) VALUES ($1, $2) ON CONFLICT (email) DO UPDATE SET display_name = EXCLUDED.display_name RETURNING id, email, display_name",
        None,
    )
}
impl UpsertUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        email: &'a T1,
        display_name: &'a T2,
    ) -> UpsertUserQuery<'c, 'a, 's, C, UpsertUser, 2> {
        UpsertUserQuery {
            client,
            params: [email, display_name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpsertUserBorrowed, tokio_postgres::Error> {
                    Ok(UpsertUserBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        display_name: row.try_get(2)?,
                    })
                },
            mapper: |it| UpsertUser::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpsertUserParams<T1, T2>,
        UpsertUserQuery<'c, 'a, 's, C, UpsertUser, 2>,
        C,
    > for UpsertUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpsertUserParams<T1, T2>,
    ) -> UpsertUserQuery<'c, 'a, 's, C, UpsertUser, 2> {
        self.bind(client, &params.email, &params.display_name)
    }
}
pub struct UpsertProjectStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert_project() -> UpsertProjectStmt {
    UpsertProjectStmt(
        "INSERT INTO projects (owner_id, key, name) VALUES ($1, $2, $3) ON CONFLICT (key) DO UPDATE SET name = EXCLUDED.name RETURNING id, key, name",
        None,
    )
}
impl UpsertProjectStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        owner_id: &'a i64,
        key: &'a T1,
        name: &'a T2,
    ) -> UpsertProjectQuery<'c, 'a, 's, C, UpsertProject, 3> {
        UpsertProjectQuery {
            client,
            params: [owner_id, key, name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpsertProjectBorrowed, tokio_postgres::Error> {
                    Ok(UpsertProjectBorrowed {
                        id: row.try_get(0)?,
                        key: row.try_get(1)?,
                        name: row.try_get(2)?,
                    })
                },
            mapper: |it| UpsertProject::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpsertProjectParams<T1, T2>,
        UpsertProjectQuery<'c, 'a, 's, C, UpsertProject, 3>,
        C,
    > for UpsertProjectStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpsertProjectParams<T1, T2>,
    ) -> UpsertProjectQuery<'c, 'a, 's, C, UpsertProject, 3> {
        self.bind(client, &params.owner_id, &params.key, &params.name)
    }
}
pub struct CreateTaskStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_task() -> CreateTaskStmt {
    CreateTaskStmt(
        "INSERT INTO tasks (project_id, assignee_id, title, priority) VALUES ($1, $2, $3, $4) RETURNING id, title, status, priority",
        None,
    )
}
impl CreateTaskStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        project_id: &'a i64,
        assignee_id: &'a i64,
        title: &'a T1,
        priority: &'a i32,
    ) -> CreateTaskQuery<'c, 'a, 's, C, CreateTask, 4> {
        CreateTaskQuery {
            client,
            params: [project_id, assignee_id, title, priority],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateTaskBorrowed, tokio_postgres::Error> {
                    Ok(CreateTaskBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        status: row.try_get(2)?,
                        priority: row.try_get(3)?,
                    })
                },
            mapper: |it| CreateTask::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateTaskParams<T1>,
        CreateTaskQuery<'c, 'a, 's, C, CreateTask, 4>,
        C,
    > for CreateTaskStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateTaskParams<T1>,
    ) -> CreateTaskQuery<'c, 'a, 's, C, CreateTask, 4> {
        self.bind(
            client,
            &params.project_id,
            &params.assignee_id,
            &params.title,
            &params.priority,
        )
    }
}
pub struct CompleteTaskStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn complete_task() -> CompleteTaskStmt {
    CompleteTaskStmt(
        "UPDATE tasks SET status = 'done', completed_at = now() WHERE id = $1 RETURNING id, title, status, priority",
        None,
    )
}
impl CompleteTaskStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        task_id: &'a i64,
    ) -> CompleteTaskQuery<'c, 'a, 's, C, CompleteTask, 1> {
        CompleteTaskQuery {
            client,
            params: [task_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CompleteTaskBorrowed, tokio_postgres::Error> {
                    Ok(CompleteTaskBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        status: row.try_get(2)?,
                        priority: row.try_get(3)?,
                    })
                },
            mapper: |it| CompleteTask::from(it),
        }
    }
}
pub struct TaskBoardStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn task_board() -> TaskBoardStmt {
    TaskBoardStmt(
        "SELECT tasks.id, tasks.title, tasks.status, tasks.priority, COALESCE(app_users.display_name, 'unassigned') AS assignee_name FROM tasks LEFT JOIN app_users ON app_users.id = tasks.assignee_id WHERE tasks.project_id = $1 ORDER BY tasks.status, tasks.priority, tasks.id",
        None,
    )
}
impl TaskBoardStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        project_id: &'a i64,
    ) -> TaskBoardQuery<'c, 'a, 's, C, TaskBoard, 1> {
        TaskBoardQuery {
            client,
            params: [project_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TaskBoardBorrowed, tokio_postgres::Error> {
                    Ok(TaskBoardBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        status: row.try_get(2)?,
                        priority: row.try_get(3)?,
                        assignee_name: row.try_get(4)?,
                    })
                },
            mapper: |it| TaskBoard::from(it),
        }
    }
}
pub struct ProjectSummaryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn project_summary() -> ProjectSummaryStmt {
    ProjectSummaryStmt(
        "SELECT projects.key, projects.name, COUNT(tasks.id)::BIGINT AS total_tasks, COUNT(tasks.id) FILTER (WHERE tasks.status <> 'done')::BIGINT AS open_tasks, COUNT(tasks.id) FILTER (WHERE tasks.status = 'done')::BIGINT AS done_tasks FROM projects LEFT JOIN tasks ON tasks.project_id = projects.id WHERE projects.id = $1 GROUP BY projects.id, projects.key, projects.name",
        None,
    )
}
impl ProjectSummaryStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        project_id: &'a i64,
    ) -> ProjectSummaryQuery<'c, 'a, 's, C, ProjectSummary, 1> {
        ProjectSummaryQuery {
            client,
            params: [project_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ProjectSummaryBorrowed, tokio_postgres::Error> {
                Ok(ProjectSummaryBorrowed {
                    key: row.try_get(0)?,
                    name: row.try_get(1)?,
                    total_tasks: row.try_get(2)?,
                    open_tasks: row.try_get(3)?,
                    done_tasks: row.try_get(4)?,
                })
            },
            mapper: |it| ProjectSummary::from(it),
        }
    }
}
