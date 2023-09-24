use crate::actions;
use crate::handles;
use crate::nix;

#[derive(Debug)]
pub enum Error {
    AccessDenied,
    ActionError(actions::Error),
    BadJobsetDecl(String),
    BuildNotFound(handles::Build),
    BuildNotRunning(handles::Build),
    EvaluationNotFound(handles::Evaluation),
    EvaluationNotRunning(handles::Evaluation),
    IllegalProjectHandle(handles::Project),
    JobNotFound(handles::Job),
    JobNotRunning(handles::Job),
    JobsetNotFound(handles::Jobset),
    LogNotFound(handles::Log),
    NixError(nix::Error),
    ProjectAlreadyExists(handles::Project),
    ProjectNotFound(handles::Project),
    Todo,
    UnexpectedDatabaseError(diesel::result::Error),
    LoginError,
}

impl Error {
    pub fn is_internal(&self) -> bool {
        match self {
            Error::UnexpectedDatabaseError(_) | Error::Todo => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            AccessDenied => write!(f, "Access denied"),
            ActionError(e) => write!(f, "Action error: {}", e),
            BadJobsetDecl(s) => write!(f, "Bad jobset declaration: {}", s),
            BuildNotFound(build_handle) => write!(f, "Build {} not found", build_handle),
            BuildNotRunning(build_handle) => write!(f, "Build {} is not running", build_handle),
            IllegalProjectHandle(handle) => {
                write!(f, "The project name [{}] is illegal. Legal project names are sequences of alphanumerical characters that may contains dashes [-] or underscores [_].", handle.project)
            }
            JobNotFound(job_handle) => {
                write!(f, "Job {} not found", job_handle)
            }
            JobNotRunning(job_handle) => {
                write!(f, "Job {} is not running", job_handle)
            }
            JobsetNotFound(jobset_handle) => {
                write!(f, "Jobset {} not found", jobset_handle)
            }
            LogNotFound(log_handle) => {
                write!(f, "Log {} not found", log_handle)
            }
            EvaluationNotFound(evaluation_handle) => {
                write!(f, "Evaluation {} not found", evaluation_handle)
            }
            EvaluationNotRunning(evaluation_handle) => {
                write!(f, "Evaluation {} is not running", evaluation_handle)
            }
            ProjectAlreadyExists(project_handle) => {
                write!(f, "Project {} already exists", project_handle)
            }
            ProjectNotFound(project_handle) => write!(f, "Project {} not found", project_handle),
            NixError(e) => write!(f, "Nix error: {}", e),
            LoginError => write!(f, "Login error"),
            Todo => write!(f, "Unspecified error"),
            UnexpectedDatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        Error::UnexpectedDatabaseError(e)
    }
}

impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Error {
        Error::NixError(e)
    }
}

impl From<actions::Error> for Error {
    fn from(e: actions::Error) -> Error {
        Error::ActionError(e)
    }
}

impl Into<typhon_types::responses::ResponseError> for Error {
    fn into(self) -> typhon_types::responses::ResponseError {
        use {typhon_types::responses::ResponseError::*, Error::*};
        match self {
            BuildNotFound(_)
            | EvaluationNotFound(_)
            | JobNotFound(_)
            | JobsetNotFound(_)
            | ProjectNotFound(_) => ResourceNotFound(format!("{}", self)),
            AccessDenied
            | ActionError(_)
            | BadJobsetDecl(_)
            | BuildNotRunning(_)
            | EvaluationNotRunning(_)
            | JobNotRunning(_)
            | IllegalProjectHandle(_)
            | NixError(_)
            | ProjectAlreadyExists(_)
            | LoginError
            | LogNotFound(_) => BadRequest(format!("{}", self)),
            Todo | UnexpectedDatabaseError(_) => InternalError,
        }
    }
}
