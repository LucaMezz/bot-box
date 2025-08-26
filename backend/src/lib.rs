#![allow(dead_code)]

use chrono::{DateTime, Utc};

/// Represents the current state of a `Bot`.
///
/// - `Stopped`: The bot is not running.
/// - `Starting`: The bot is in the process of starting.
/// - `Running`: The bot is currently running.
/// - `Stopping`: The bot is in the process of stopping.
/// - `Unknown`: The bot's state could not be determined.
pub enum BotStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Unknown,
}

/// Represents the current lifecycle status of a job.
///
/// - `Pending`: The job has been created but not yet started.
/// - `Running`: The job is actively executing.
/// - `Completed`: The job finished successfully.
/// - `Failed(reason)`: The job failed, with `reason` describing why.
/// - `Cancelled`: The job was cancelled before completion.
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed(String),
}

/// Represents the outcome of a job.
///
/// - `Success`: The job completed successfully.
/// - `Failure(reason)`: The job failed, with `reason` describing why.
/// - `Skipped(reason)`: The job was skipped, with `reason` explaining why.
pub enum JobResult {
    Success,
    Failure(String),
    Skipped(String),
}

/// Describes the seriousness of an alert condition.
///
/// Severity levels are often used to control routing, escalation,
/// and prioritization of alerts.
pub enum Severity {
    /// Informational only — no action required.
    Info,
    /// Something unusual, but not immediately harmful.
    Warning,
    /// A serious issue that requires prompt attention.
    Critical,
}

/// A `Bot` represents a controllable automated agent.
///
/// This trait defines the basic lifecycle operations for a bot,
/// including starting, stopping, and querying its current status.
pub struct Bot {
    id: String,
    name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    tags: Vec<String>,
}

/// Represents a unit of work in the system.
///
/// A `Job` is an executable, cancellable, retryable, and auditable task.
/// It does not define its own methods, but instead groups together
/// the traits that define a job's capabilities.
///
/// # Traits it inherits
/// - [`Executable`]: allows the job to be executed.
/// - [`Cancellable`]: allows the job to be canceled before completion.
/// - [`Retryable`]: allows the job to be retried upon failure.
/// - [`Auditable`]: allows auditing/logging of job activity.
/// - [`Queryable`]: allows the state of the job to be queried
pub trait Job: Cancellable + Retryable + Queryable<State = JobStatus> {
    /// Executes the entity and returns the result.
    fn execute(&self) -> JobResult;
}

/// A trait for entities whose current state can be queried.
///
/// This is useful for monitoring and orchestration, where external
/// systems may need to know the status of a job, bot, or deployment.
pub trait Queryable {
    /// The type of state that can be returned.
    ///
    /// For example, `JobStatus` for jobs, or `BotStatus` for bots.
    type State;

    /// Returns the current state of the entity.
    fn state(&self) -> Self::State;
}

/// A trait for entities that can be canceled or stopped before completion.
///
/// Implementors define how cancellation is handled.
pub trait Cancellable {
    fn cancel(&self);
}

/// A trait for entities that support retrying operations upon failure.
///
/// Implementors define the retry logic and conditions.
pub trait Retryable {
    fn retry(&self);
}

/// A trait for entities that can accept subscriptions from listeners.
///
/// Implementors allow other components to subscribe to events or updates,
/// so they can be notified when something of interest occurs.
pub trait Subscribable {
    fn subscribe(&self);
}

/// Responsible for managing bot deployments.
///
/// A `Worker` can deploy or undeploy any type that implements the `Bot` trait.
pub trait Worker<'a> {
    /// Deploys the given bot.
    fn deploy(&self, bot: Bot) -> Deployment<'a>;

    /// Undeploys the given bot.
    fn undeploy(&self, deployment: Deployment);
}

/// A BotBox is a container for bots, users, and related resources.
///
/// Conceptually, it is a "Box" for managing all bots under a single
/// ownership or organization.
pub struct BotBox<'a> {
    /// Unique identifier for the workspace.
    id: String,

    /// Human-readable name for the workspace.
    name: String,

    /// Optional description of the workspace’s purpose.
    description: Option<String>,

    /// Users who belong to this workspace.
    members: Vec<User>,

    /// The bots managed under this box.
    bots: Vec<Bot>,

    /// Deployments associated with this project.
    deployments: Vec<Deployment<'a>>,

    /// Roles assigned to users in the workspace (for access control).
    roles: Vec<Role>,

    /// Timestamps for auditing and tracking changes.
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    /// Optional metadata or tags for categorization.
    tags: Vec<String>,
}

/// Represents a deployment of a bot or project.
///
/// Deployments track the lifecycle and status of a deployed project or bot.
pub struct Deployment<'a> {
    id: String,
    // Add DeploymentStatus
    bot: &'a Bot,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a user in the system.
///
/// Users may own projects, execute jobs, or manage bots.
pub struct User {}

/// Represents the role of a user within a workspace, project, or team.
///
/// Roles define the permissions and responsibilities that a user has,
/// such as `Admin`, `Developer`, or `Viewer`.
pub enum Role {}

/// Represents a group of users collaborating on shared projects or workspaces.
///
/// A team aggregates users under a common identity, making it easier to manage
/// access control, permissions, and resource sharing.
pub struct Team {}

/// A record of an auditable event that occurred within the system.
///
/// Audit records provide traceability for critical actions such as deployments,
/// job executions, cancellations, or configuration changes. They can be used
/// for debugging, compliance, and monitoring.
pub struct AuditRecord {}

/// Represents an invitation sent to a user to join a workspace, project, or team.
///
/// Invitations capture metadata about who sent the invite, the target user,
/// the role being offered, and the current status (pending, accepted, declined).
pub struct Invitation {}

/// Represents a notification triggered by a monitoring or evaluation condition.
///
/// An `Alert` typically captures information about the condition that was violated,
/// the entity or resource affected, the severity of the issue, and any relevant metadata
/// for handling or escalation.
///
/// Alerts can be consumed by notification systems, dashboards, or automated responses.
pub struct Alert {
    pub severity: Severity,
    pub timestamp: DateTime<Utc>,
    pub details: AlertKind,
}

pub enum AlertKind {
    BotUnhealthy {
        bot_id: String,
        reason: String,
    },
    JobFailure {
        job_id: String,
        error_message: String,
        retry_count: u32,
    },
    DeploymentFailure {
        deployment_id: String,
        environment: String,
        reason: String,
    },
}

/// A message intended to inform a user, team, or system component
/// about an event or change of state.
///
/// Unlike [`Alert`], which signals conditions requiring attention or
/// intervention, a `Notification` is generally informational in nature.
/// Examples include user invitations, deployment completions, or
/// team membership changes.
pub struct Notification {
    pub timestamp: DateTime<Utc>,
    pub details: NotificationKind,
}

/// The type of event a `Notification` represents.
///
/// Used to categorize notifications and allow filtering or routing
/// based on their semantic meaning.
pub enum NotificationKind {
    /// A user was invited to join a team or workspace.
    Invitation,
    /// A user's role or permissions were updated.
    RoleChanged,
    /// A new deployment was started.
    DeploymentStarted,
    /// A deployment completed successfully.
    DeploymentSucceeded,
    /// A deployment failed.
    DeploymentFailed,
    /// A job began execution.
    JobStarted,
    /// A job finished successfully.
    JobSucceeded,
    /// A job failed or was cancelled.
    JobFailed,
    /// A new project or workspace was created.
    ProjectCreated,
    /// General informational message not tied to a specific entity.
    Info,
}
