use lmc_assembly::{self, Output, LMCIO};

struct TestIO {
    input_buffer: Vec<i16>,
    output_buffer: Vec<Output>,
}

impl LMCIO for TestIO {
    fn get_input(&mut self) -> i16 {
        self.input_buffer.pop().unwrap()
    }

    fn print_output(&mut self, val: Output) {
        self.output_buffer.push(val);
    }
}

fn get_program(path: &str) -> [i16; 100] {
    // read the ./examples/sum.lmc file
    let code = std::fs::read_to_string(path).unwrap();

    // parse the code
    let program = lmc_assembly::parse(&code, false);

    // assemble the program
    lmc_assembly::assemble(program)
}

#[test]
fn test_sum_1() {
    let assembled = get_program("./examples/sum.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![1, 2],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(io_handler.output_buffer, vec![Output::Int(3)]);
}

#[test]
fn test_sum_2() {
    let assembled = get_program("./examples/sum.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![3, 4],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(io_handler.output_buffer, vec![Output::Int(7)]);
}

#[test]
fn test_fibonacci_1() {
    let assembled = get_program("./examples/fibonacci.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![10],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(
        io_handler.output_buffer,
        vec![
            Output::Int(0),
            Output::Int(1),
            Output::Int(1),
            Output::Int(2),
            Output::Int(3),
            Output::Int(5),
            Output::Int(8)
        ]
    );
}

#[test]
fn test_fibonacci_2() {
    let assembled = get_program("./examples/fibonacci.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![30],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(
        io_handler.output_buffer,
        vec![
            Output::Int(0),
            Output::Int(1),
            Output::Int(1),
            Output::Int(2),
            Output::Int(3),
            Output::Int(5),
            Output::Int(8),
            Output::Int(13),
            Output::Int(21)
        ]
    );
}

#[test]
fn test_countdown_1() {
    let assembled = get_program("./examples/countdown.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![10],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    let mut expected = vec![];

    for i in (0..=10).rev() {
        expected.push(Output::Int(i));
    }

    // check the output
    assert_eq!(io_handler.output_buffer, expected);
}

#[test]
fn test_countdown_2() {
    let assembled = get_program("./examples/countdown.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![30],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    let mut expected = vec![];

    for i in (0..=30).rev() {
        expected.push(Output::Int(i));
    }

    // check the output
    assert_eq!(io_handler.output_buffer, expected);
}

#[test]
fn test_multiplication_1() {
    let assembled = get_program("./examples/multiplication.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![2, 3],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(io_handler.output_buffer, vec![Output::Int(6)]);
}

#[test]
fn test_multiplication_2() {
    let assembled = get_program("./examples/multiplication.lmc");

    // create a new TestIO instance
    let mut io_handler = TestIO {
        input_buffer: vec![5, 7],
        output_buffer: vec![],
    };

    // run the program
    lmc_assembly::run(assembled, &mut io_handler, false);

    // check the output
    assert_eq!(io_handler.output_buffer, vec![Output::Int(35)]);
}
