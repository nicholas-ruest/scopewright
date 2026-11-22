use scopewright_audit::AuditChain;
#[test]
fn empty_payload_is_rejected() {
    let mut c = AuditChain::default();
    assert!(c.append("1", "c", "").is_err());
}
