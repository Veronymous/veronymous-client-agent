use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum VeronymousClientError {
    #[error("Connect error. {0}")]
    ConnectError(String),

    #[error("User authentication is required")]
    AuthRequired(),

    #[error("Subscription is required.")]
    SubscriptionRequired(),

    #[error("OIDC error. {0}")]
    OidcError(String),

    #[error("Deserialization error. {0}")]
    DeserializationError(String),

    #[error("Decoding error. {0}")]
    DecodingError(String),

    #[error("VPN domain is already in use.")]
    DomainInUseError(),

    #[error("Token client error. {0}")]
    TokenClientError(String),

    #[error("Parse error. {0}")]
    ParseError(String),

    #[error("Missing token error. {0}")]
    MissingTokenError(String),

    #[error("Missing issuer info")]
    MissingIssuerInfoError(),

    #[error("Token error. {0}")]
    TokenError(String),

    #[error("Http error. {0}")]
    HttpError(String),

    #[error("Command error. {0}")]
    CommandError(String),

    #[error("Illegal argument. {0}")]
    IllegalArgumentError(String),

    #[error("Not found. {0}")]
    NotFoundError(String),

    #[error("Veronymous error. {0}")]
    VeronymousError(String),

}
