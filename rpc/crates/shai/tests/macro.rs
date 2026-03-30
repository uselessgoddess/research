#![cfg(feature = "macros")]

#[shai::message]
mod farm {
    #[derive(Debug)]
    pub struct Task {
        pub n: u32,
    }
    pub struct Result {
        pub ok: bool,
    }
    pub enum FarmErr {
        Busy,
        Timeout,
    }
}

shai::rpc! {
    0x7000: farm::Task => farm::Result,
}

#[test]
fn message_mod_compiles() {
    let _ = farm::Task { n: 1 };
    let _ = farm::Result { ok: true };
    let _ = farm::FarmErr::Busy;
}

#[test]
fn message_mod_debug() {
    use shai::rpc::Serialize;
    use shai::Archive;

    let task = farm::Task { n: 1 };
    let bytes = task.serialize_to_bytes().unwrap();
    let archived = Archive::<farm::Task>::new(bytes).unwrap();
    assert!(format!("{archived:?}").contains("{ n: 1 }"));
}
