predicate;

use std::{
    constants::ZERO_B256,
    outputs::{
        output_asset_to,
        output_count,
    },
};

configurable {
    SENDER: b256 = ZERO_B256,
    RECIPIENT: b256 = ZERO_B256,
}

fn main() -> bool {
    let count = output_count();
    let mut i = 0;

    while i < count {
        match output_asset_to(i) {
            Some(recipient) => {
                if recipient != RECIPIENT && recipient != SENDER {
                    return false;
                }
            },
            None => (),
        }
        i += 1;
    }
    return true;
}
