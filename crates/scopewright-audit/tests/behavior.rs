use scopewright_audit::AuditChain;
#[test]
fn verifies_ordered_chain() {
    let mut c = AuditChain::default();
    c.append("1", "c", "started").unwrap();
    c.append("2", "c", "completed").unwrap();
    c.verify_all().unwrap();
    assert_eq!(c.records().len(), 2);
}
