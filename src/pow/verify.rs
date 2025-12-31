use crate::chain::header::BlockHeader;
use crate::chain::hash::hash_header;
use crate::pow::target::bits_to_target;

pub fn verify_pow(header: &BlockHeader) -> bool {
    let hash = hash_header(header);
    let target = bits_to_target(header.bits);

    hash <= target
}
