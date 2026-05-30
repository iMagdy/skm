use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::init::already_exists))]
pub struct InitAlreadyExists {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::init::path_not_found))]
pub struct InitPathNotFound {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::manifest::not_found))]
pub struct ManifestNotFound {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::manifest::invalid_json))]
pub struct ManifestInvalidJson {
    pub message: String,
    #[source_code]
    pub content: String,
    #[label("error here")]
    pub span: miette::SourceSpan,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::manifest::duplicate_name))]
pub struct ManifestDuplicateName {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::manifest::invalid_name))]
pub struct ManifestInvalidName {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::lockfile::not_found))]
pub struct LockfileNotFound {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::lockfile::invalid))]
pub struct LockfileInvalid {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::git::clone_failed))]
pub struct GitCloneFailed {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::git::fetch_failed))]
pub struct GitFetchFailed {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::git::checkout_failed))]
pub struct GitCheckoutFailed {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::git::rev_parse_failed))]
pub struct GitRevParseFailed {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::skill::copy_failed))]
pub struct SkillCopyFailed {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::skill::not_found))]
pub struct SkillNotFound {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::install::invalid_format))]
pub struct InstallInvalidFormat {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::install::already_exists))]
pub struct InstallAlreadyExists {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::general))]
pub struct GeneralError {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::discovery::error))]
pub struct DiscoveryError {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::discovery::skills_directory_empty))]
pub struct SkillsDirectoryEmpty {
    pub message: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("{}", message)]
#[diagnostic(code(skm::discovery::user_cancelled))]
pub struct UserCancelled {
    pub message: String,
}
