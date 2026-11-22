use scopewright_evidence::{seal, verify};
#[test]
fn tampering_is_rejected() {
    let mut r = seal("r-1", "corr-1", "completed", "genesis").unwrap();
    r.payload = "changed".into();
    assert!(verify(&r).is_err());
}
