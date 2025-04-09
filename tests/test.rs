
    use rust_cli_lib::command::Command;

    #[test]
    fn it_works() {
        Command { }.print_usage();
        assert_eq!(1, 1);
    }
