// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Errors
//!
//! This module contains types and utility functions for error handling

use tea_codec::error::{
    new_common_error_code, new_wascc_error_code, CommonCode, TeaError, WasccCode,
};

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

pub(crate) fn new(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

#[derive(Debug)]
pub enum ErrorKind {
    KeyValueError(String),
    MessagingError(String),
    MiscError(Box<dyn ::std::error::Error>),
    EnvVar(std::env::VarError),
    UTF8(std::string::FromUtf8Error),
    UTF8Str(std::str::Utf8Error),
    JsonMarshaling(serde_json::Error),
    HostError(String),
    BadDispatch(String),
    WapcError(wapc::errors::Error),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl Into<TeaError> for Error {
    fn into(self) -> TeaError {
        match *self.0 {
            ErrorKind::KeyValueError(s) => {
                new_wascc_error_code(WasccCode::KeyValueError).to_error_code(Some(s), None)
            }
            ErrorKind::UTF8(e) => new_common_error_code(CommonCode::UTF8EncodingError)
                .to_error_code(Some(format!("{:?}", e)), None),
            ErrorKind::MessagingError(s) => {
                new_wascc_error_code(WasccCode::MessagingError).to_error_code(Some(s), None)
            }
            ErrorKind::EnvVar(e) => new_wascc_error_code(WasccCode::EnvVarError)
                .to_error_code(Some(format!("{:?}", e)), None),
            ErrorKind::JsonMarshaling(e) => new_common_error_code(CommonCode::JsonMarshalingError)
                .to_error_code(Some(format!("{:?}", e)), None),
            ErrorKind::UTF8Str(e) => new_common_error_code(CommonCode::Utf8StrEncodingError)
                .to_error_code(Some(format!("{:?}", e)), None),
            ErrorKind::HostError(s) => {
                new_wascc_error_code(WasccCode::GeneralHostError).to_error_code(Some(s), None)
            }
            ErrorKind::BadDispatch(s) => {
                new_wascc_error_code(WasccCode::BadDispatch).to_error_code(Some(s), None)
            }
            ErrorKind::WapcError(e) => {
                let inner: TeaError = e.into();
                new_wascc_error_code(WasccCode::WapcGeneralError)
                    .to_error_code(None, inner.parse_error_code())
            }
            ErrorKind::MiscError(e) => new_wascc_error_code(WasccCode::WasmMisc)
                .to_error_code(Some(format!("{:?}", e)), None),
        }
    }
}

impl From<wapc::errors::Error> for Error {
    fn from(source: wapc::errors::Error) -> Error {
        new(ErrorKind::WapcError(source))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(source: std::str::Utf8Error) -> Error {
        Error(Box::new(ErrorKind::UTF8Str(source)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Error {
        Error(Box::new(ErrorKind::JsonMarshaling(source)))
    }
}

impl From<std::env::VarError> for Error {
    fn from(source: std::env::VarError) -> Error {
        Error(Box::new(ErrorKind::EnvVar(source)))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Error {
        Error(Box::new(ErrorKind::UTF8(source)))
    }
}

impl From<Box<dyn ::std::error::Error>> for Error {
    fn from(source: Box<dyn ::std::error::Error>) -> Error {
        Error(Box::new(ErrorKind::MiscError(source)))
    }
}
