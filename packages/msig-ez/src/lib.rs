pub mod api;
pub mod bash;
pub mod errors;
pub mod key;
pub mod multi;

#[cfg(test)]
pub mod test {
    use crate::{api::tests::TEST_RESOURCE_MUTEX, bash};

    #[test]
    fn main_runs() {
        let _guard = TEST_RESOURCE_MUTEX.lock().unwrap();

        let res = bash::run_cmd("cargo b".to_string());
        assert!(res.is_ok(), "err: {:?}", res);
        let mut cmd = assert_cmd::Command::cargo_bin("msig").unwrap();
        cmd.assert().success();
    }
}
