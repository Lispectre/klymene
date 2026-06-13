use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub trait ProtocolMessage: Sized {
    const CODE: u32;

    type Request;
    type Success;
    type Failure;

    fn encode_request(req: &Self::Request) -> Vec<u8>;
    fn decode_success(data: &[u8]) -> Self::Success;
    fn decode_failure(data: &[u8]) -> Self::Failure;
}

pub trait BinaryEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait BinaryDecode {
    fn decode(data: &[u8]) -> Self;
}

impl BinaryEncode for u32 {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(self.to_le_bytes());
        buf
    }
}

impl BinaryEncode for String {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(u32::encode(&(self.len() as u32)));
        buf.extend(self.as_bytes());
        buf
    }
}

#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub version_number: u32,
    pub hash: String,
    pub minor_version: u32,
}

#[derive(Debug)]
pub struct LoginSuccess {
    greet: String,
    own_ip_address: u32,
    hash: String,
    is_supporter: bool,
}

impl BinaryDecode for LoginSuccess {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum LoginFailure {
    InvalidUsername(UsernameRejectionReason),
    Other,
}

impl BinaryDecode for LoginFailure {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum UsernameRejectionReason {
    Empty,
    TooLong,
    InvalidCharacters,
    LeadingOrTrailingSpaces,
}

impl BinaryEncode for LoginRequest {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(String::encode(&self.username));
        buf.extend(String::encode(&self.password));
        buf.extend(u32::encode(&self.version_number));
        buf.extend(String::encode(&self.hash));
        buf.extend(u32::encode(&self.minor_version));
        buf
    }
}

#[macro_export]
macro_rules! protocol_message {
    (
        $name:ident,
        code = $code:expr,
        request = $req:ty,
        success = $success:ty,
        failure = $failure:ty
    ) => {
        pub struct $name;

        impl ProtocolMessage for $name {
            const CODE: u32 = $code;

            type Request = $req;
            type Success = $success;
            type Failure = $failure;

            fn encode_request(req: &Self::Request) -> Vec<u8> {
                let mut buf = Vec::new();
                buf.extend(Self::CODE.encode());
                buf.extend(req.encode());
                buf
            }

            fn decode_success(data: &[u8]) -> Self::Success {
                <$success as BinaryDecode>::decode(data)
            }
            fn decode_failure(data: &[u8]) -> Self::Failure {
                <$failure as BinaryDecode>::decode(data)
            }
        }
    };
}

protocol_message!(
    Login,
    code = 0x01,
    request = LoginRequest,
    success = LoginSuccess,
    failure = LoginFailure
);
