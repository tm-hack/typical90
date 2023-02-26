mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"3 3
1 1 1
1 1 1
1 1 1
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "5 5 5
5 5 5
5 5 5
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
                r#"4 4
3 1 4 1
5 9 2 6
5 3 5 8
9 7 9 3
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "28 28 25 26
39 33 40 34
38 38 36 31
41 41 39 43
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
                r#"2 10
31 41 59 26 53 58 97 93 23 84
62 64 33 83 27 95 2 88 41 97
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "627 629 598 648 592 660 567 653 606 662
623 633 651 618 645 650 689 685 615 676
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
