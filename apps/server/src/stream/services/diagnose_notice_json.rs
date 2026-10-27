use crate::stream::{DIAGNOSE_NOTICE_KIND, DiagnoseNotice};

pub fn diagnose_notice_json(run: u64) -> String {
    serde_json::to_string(&DiagnoseNotice {
        kind: DIAGNOSE_NOTICE_KIND,
        run,
    })
    .expect("a notice made of a number and a plain string always serializes")
}
