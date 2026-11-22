use scopewright_persistence::EvidenceStore;
use scopewright_retrieval::replay;
#[test]
fn missing_factory_run_fails() {
    let p = std::env::temp_dir().join(format!("scopewright-none-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    assert!(replay(&s, "missing").is_err());
    let _ = std::fs::remove_file(p);
}
