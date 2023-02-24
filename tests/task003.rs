mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"3
1 2
2 3
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "3
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn case2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"5
1 2
2 3
3 4
3 5
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "4
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn case3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"10
1 2
1 3
2 4
4 5
4 6
3 7
7 8
8 9
8 10
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "8
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
