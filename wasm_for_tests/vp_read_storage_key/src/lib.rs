use namada_vp_prelude::*;

#[validity_predicate(gas = 1000)]
fn validate_tx(
    ctx: &Ctx,
    tx_data: Tx,
    _addr: Address,
    _keys_changed: BTreeSet<storage::Key>,
    _verifiers: BTreeSet<Address>,
) -> VpResult {
    // Allocates a memory of size given from the `tx_data (usize)`
    let key =
        storage::Key::try_from_slice(&tx_data.data().as_ref().unwrap()[..])
            .unwrap();
    log_string(format!("key {}", key));
    let _result: Vec<u8> = ctx.read_pre(&key)?.unwrap();
    accept()
}
