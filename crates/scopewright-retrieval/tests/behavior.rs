use scopewright_evidence::seal;
use scopewright_persistence::EvidenceStore;
use scopewright_retrieval::replay;
#[test]
fn replays_saved_event() {
    let p = std::env::temp_dir().join(format!(
        "scopewright-retrieval-{}.jsonl",
        std::process::id()
    ));
    let s = EvidenceStore::open(&p).unwrap();
    s.append(&seal("1", "c", "done", "genesis").unwrap())
        .unwrap();
    assert_eq!(replay(&s, "c").unwrap().events, 1);
    let _ = std::fs::remove_file(p);
}
