use scopewright_evidence::{seal, verify};
#[test]
fn sealed_record_verifies() {
    let r = seal("r-1", "corr-1", "completed", "genesis").unwrap();
    verify(&r).unwrap();
    assert_eq!(r.hash.len(), 64);
}
