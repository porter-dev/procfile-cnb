//! Integration tests using libcnb-test.
//!
//! All integration tests are skipped by default (using the `ignore` attribute),
//! since performing builds is slow. To run the tests use: `cargo test -- --ignored`

// Enable Clippy lints that are disabled by default.
#![warn(clippy::pedantic)]

use libcnb_test::{assert_contains, BuildpackReference, IntegrationTest};
use std::io;

#[test]
#[ignore]
fn test1() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test2() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test3() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test4() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test5() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test6() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test7() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test8() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test9() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

#[test]
#[ignore]
fn test10() {
    IntegrationTest::new("heroku/buildpacks:20", "tests/fixtures/app_with_procfile")
        .buildpacks(vec![BuildpackReference::Crate])
        .run_test(|context| {
            assert_contains!(context.pack_stdout, "[Discovering process types]");
            assert_contains!(
                context.pack_stdout,
                "Procfile declares types -> web, worker"
            );
            assert_contains!(context.pack_stdout, "Setting default process type 'web'");
        });
}

fn _call_test_fixture_service(addr: std::net::SocketAddr, payload: &str) -> io::Result<String> {
    let req = ureq::get(&format!(
        "http://{}:{}/?payload={}",
        addr.ip(),
        addr.port(),
        payload
    ));
    req.call().unwrap().into_string()
}
