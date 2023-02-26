mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"4
4000 4400 5000 3200
3
3312
2992
4229
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "112
208
171
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
                r#"1
4000
10
3582
3538
3320
3312
3296
3257
3111
3040
3013
2994
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "418
462
680
688
704
743
889
960
987
1006
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
869120000 998244353 777777777 123456789 100100100 464646464 987654321 252525252 869120001 1000000000
10
4229
20210406
1
268435456
3582
536870912
1000000000
869120
99999999
869120001
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "100095871
79889694
100100099
15910204
100096518
72224448
0
99230980
100101
0
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
