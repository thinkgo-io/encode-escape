use encode::types::EncodeOperation;
use shared::prelude::*;
use shared::validate::Validator;

use crate::domain::encode::data::get_encodings;
use crate::domain::encode::types::Encoding;
use crate::domain::encode::types::Operation;

fn get_encoding(encode_operation: &EncodeOperation, encodings: &Vec<Encoding>) -> Option<Encoding> {
    for encoding in encodings {
        if encoding.name == encode_operation.encoding {
            return Some(encoding.clone());
        }
    }
    None
}

fn get_encoding_and_operation(
    encode_operation: &EncodeOperation,
) -> (Option<Encoding>, Option<Operation>) {
    let encodings = get_encodings();
    let encoding = get_encoding(&encode_operation, &encodings);

    let operation = match &encoding {
        Some(encoding) => get_operation(&encode_operation, &encoding),
        None => None,
    };
    (encoding, operation)
}

fn get_operation(encode_operation: &EncodeOperation, encoding: &Encoding) -> Option<Operation> {
    for operation in &encoding.operations {
        if operation.name == encode_operation.operation {
            return Some(operation.clone());
        }
    }
    None
}

pub fn normalize_encode_operation(encode_operation: &EncodeOperation) -> EncodeOperation {
    EncodeOperation::new(
        &encode_operation.encoding.to_lowercase(),
        &encode_operation.operation.to_lowercase(),
    )
}

pub fn to_encode_operation_titles(encode_operation: &EncodeOperation) -> Option<EncodeOperation> {
    let (encoding, operation) = get_encoding_and_operation(&encode_operation);

    match (encoding, operation) {
        (Some(encoding), Some(operation)) => {
            Some(EncodeOperation::new(&encoding.label, &operation.label))
        }
        _ => None,
    }
}

pub fn validate_encode_operation(encode_operation: &EncodeOperation) -> ResultOk {
    let (encoding, operation) = get_encoding_and_operation(&encode_operation);

    let mut validator = Validator::default();

    if validator.check_not_none("Encoding", &encoding) {
        validator.check_not_none("Operation", &operation);
    }

    validator.into_result()
}
