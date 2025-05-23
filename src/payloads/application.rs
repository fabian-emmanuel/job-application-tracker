use crate::enums::application::{ApplicationType, InterviewType, Status, TestType};
use crate::models::application::{Application, ApplicationStatus};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ApplicationFilter {
    pub search: Option<String>,
    pub status: Option<Status>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ApplicationRequest {
    #[validate(length(min = 1, message = "Company name cannot be empty"))]
    pub company: String,

    #[validate(length(min = 1, message = "Position cannot be empty"))]
    pub position: String,

    pub website: Option<String>,

    #[serde(rename = "applicationType")]
    pub application_type: Option<ApplicationType>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationsResponse {
    pub id: i64,
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    #[serde(rename = "applicationType")]
    pub application_type: Option<ApplicationType>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "createdBy")]
    pub created_by: i64,
    pub status: Status,
    #[serde(rename = "statusHistory")]
    pub status_history: Vec<ApplicationStatusResponse>,
}

impl ApplicationsResponse {
    pub fn from_application_and_status(
        application: &Application,
        statuses: &Vec<ApplicationStatus>,
    ) -> Self {
        Self {
            id: application.id.clone(),
            company: application.company.clone(),
            position: application.position.clone(),
            website: application.website.clone(),
            application_type: application.application_type.clone(),
            created_at: application.created_at.clone(),
            created_by: application.created_by.clone(),
            status: statuses.last().unwrap().status_type.clone(),
            status_history: statuses
                .iter()
                .map(|status| ApplicationStatusResponse::from_application_status(status))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationStatusResponse {
    pub id: i64,
    #[serde(rename = "applicationId")]
    pub application_id: i64,
    #[serde(rename = "createdBy")]
    pub created_by: i64,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "testType")]
    pub test_type: Option<TestType>,
    #[serde(rename = "interviewType")]
    pub interview_type: Option<InterviewType>,
    pub notes: Option<String>,
}

impl ApplicationStatusResponse {
    pub fn from_application_status(application_status: &ApplicationStatus) -> Self {
        Self {
            id: application_status.id.clone(),
            application_id: application_status.application_id.clone(),
            created_by: application_status.created_by.clone(),
            status: application_status.status_type.clone(),
            created_at: application_status.created_at.clone(),
            test_type: application_status.test_type.clone(),
            interview_type: application_status.interview_type.clone(),
            notes: application_status.notes.clone(),
        }
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ApplicationStatusRequest {
    #[serde(rename = "applicationId")]
    pub application_id: i64,
    #[serde(rename = "status")]
    pub status_type: Status,
    #[serde(rename = "testType")]
    pub test_type: Option<TestType>,
    #[serde(rename = "interviewType")]
    pub interview_type: Option<InterviewType>,
    pub notes: Option<String>,
}
