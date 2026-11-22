# Cloud Run configuration and rollback

This repository contains configuration only. No deployment is authorized by its presence. Before an operator deploys, the immutable image must include or securely bind the seven exact-pinned upstream runtimes; replace the placeholder project, source SHA and bucket with approved values; and verify the durable shared-volume policy.

Rollback is revision-based: inspect traffic, route 100% to the last independently verified revision, verify `/healthz` and a read-only audit retrieval, then retain the failed revision and external evidence for investigation. Do not delete evidence storage during rollback. A rollback never changes the Git source or evidence binding used by the publication gate.
