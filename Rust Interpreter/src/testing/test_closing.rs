#[cfg(test)]
mod tests_assign_var {
    use crate::testing::*;
    use ntest::timeout;
    #[test]
    #[timeout(1000)]
    fn test_ellipsis_6_close() {
        let data = run_parser!(b"It was sub sub sub sub sub sub one...");
        check_lisp!(
            data,
            "(assign@3,4,5$34$$3 \"sub\"@7 (-@11,12,13$34$$3 (-@15,16,17$34$$3 \
            (-@19,20,21$34$$3 (-@23,24,25$34$$3 (-@27,28,29$34$$3 (litnum 1@31$$3)))))))"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_2_peirod() {
        let data = run_parser!(b"It was sub sub one..");
        check_lisp!(
            data,
            "(assign@3,4,5$19 \"sub\"@7 (-@11,12,13$18 (litnum 1@15$$3)))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_ellipsis_overload_12() {
        let data = run_parser!(b"It was sub sub sub sub sub sub sub sub sub sub sub sub one......");
        check_lisp!(data,
            "(assign@3,4,5$61$$3 \"sub\"@7 (-@11,12,13$61$$3 (-@15,16,17$58$$3 (-@19,20,21$58$$3 (-@23,24,25$58$$3 \
            (-@27,28,29$58$$3 (-@31,32,33$58$$3 (-@35,36,37$58$$3 (-@39,40,41$58$$3 (-@43,44,45$58$$3 \
            (-@47,48,49$58$$3 (-@51,52,53$58$$3 (litnum 1@55$$3)))))))))))))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_dashes() {
        let data = run_parser!(b"was tu-tu twelve hi---hi");
        check_lisp!(data, "(assign@0,1,2$19$$3 \"tutu\"@4|2 (litnum 12@10$$6))");
    }
}
