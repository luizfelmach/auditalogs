use std::collections::VecDeque;

pub struct TxManager {
    pool: Vec<(String, String)>,
    nonce: VecDeque<u64>,
}

#[cfg(test)]
mod tests {
    // use super::*;
}
