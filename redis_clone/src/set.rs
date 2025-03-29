use crate::storage_result::{StorageError, StorageResult};

#[derive(Debug, PartialEq)]
pub enum KeyExistence {
    NX,
    XX,
}

#[derive(Debug, PartialEq)]
pub enum KeyExpiry {
    EX(u64),
    PX(u64),
}

#[derive(Debug, PartialEq)]
pub struct SetArgs {
    pub expiry: Option<KeyExpiry>,
    pub existence: Option<KeyExistence>,
    pub get: bool,
}

impl SetArgs {
    pub fn new() -> Self {
        SetArgs {
            expiry: None,
            existence: None,
            get: false,
        }
    }
}

pub fn parse_set_arguments(arguments: &Vec<String>) -> StorageResult<SetArgs> {
    let mut args = SetArgs::new();

    let mut idx: usize = 0;

    loop {
        if idx >= arguments.len() {
            break;
        }

        match arguments[idx].to_lowercase().as_str() {
            "nx" => {
                if args.existence == Some(KeyExistence::XX) {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                args.existence = Some(KeyExistence::NX);
                idx += 1;
            }
            "xx" => {
                if args.existence == Some(KeyExistence::NX) {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                args.existence = Some(KeyExistence::XX);
                idx += 1;
            }
            "get" => {
                args.get = true;
                idx += 1;
            }

            "ex" => {
                if let Some(KeyExpiry::PX(_)) = args.expiry {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                if idx + 1 == arguments.len() {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                let value: u64 = arguments[idx + 1]
                    .parse()
                    .map_err(|_| StorageError::CommandSyntaxError(arguments.join(" ")))?;

                args.expiry = Some(KeyExpiry::EX(value));
                idx += 2;
            }
            "px" => {
                if let Some(KeyExpiry::EX(_)) = args.expiry {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                if idx + 1 == arguments.len() {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                let value: u64 = arguments[idx + 1]
                    .parse()
                    .map_err(|_| StorageError::CommandSyntaxError(arguments.join(" ")))?;

                args.expiry = Some(KeyExpiry::PX(value));
                idx += 2;
            }
            _ => {
                return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
            }
        }
    }
    Ok(args)
}
