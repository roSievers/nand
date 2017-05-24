extern crate rand;

use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct Nand {
    in1: usize,
    in2: usize,
}

impl Nand {
    pub fn new(in1: usize, in2: usize) -> Nand {
        Nand { in1: in1, in2: in2 }
    }
    pub fn eval(&self, values: &Vec<bool>) -> bool {
        !(values[self.in1] && values[self.in2])
    }
}

#[derive(Debug)]
struct Stack {
    // Prime example of dependent types. I would want inputs to be a compile time variable.
    inputs: usize,
    content: Vec<Nand>,
}

impl Stack {
    fn random(inputs: usize, nand_count: usize) -> Self {
        assert!(inputs > 0, "You need at least one input!");

        let content = Vec::with_capacity(nand_count);
        let mut result = Stack { inputs, content };

        for _ in 0..nand_count {
            result.add_nand()
        }

        result
    }
    fn add_nand(&mut self) {
        let mut rng = rand::thread_rng();
        let range = Range::new(0, self.len());

        let nand = Nand::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng));

        self.content.push(nand);
    }
    fn len(&self) -> usize {
        self.inputs + self.content.len()
    }
    fn eval(&self, mut inputs: Vec<bool>) -> bool {
        assert!(inputs.len() == self.inputs,
                "The number of inputs is incorrect.");


        for nand in &self.content {
            let nand_value = nand.eval(&inputs);
            inputs.push(nand_value);
        }
        // .unwrap can't fail because inputs contains at least self.inputs entries
        // and self.inputs is strictly positive.
        inputs.pop().unwrap()
    }
    fn test<F>(&self, target: F) -> bool
        where F: Fn(&Vec<bool>) -> bool
    {
        let iterator = (0..(self.inputs * self.inputs)).map(|n| as_bool_vec(n, self.inputs));

        for input in iterator {
            let target_value = target(&input);
            let actual_value = self.eval(input);
            if target_value != actual_value {
                return false;
            }
        }

        true
    }
    fn from_target_function<F>(inputs: usize,
                               size: usize,
                               target: F,
                               timeout: usize)
                               -> Option<Stack>
        where F: Fn(&Vec<bool>) -> bool
    {
        for _ in 0..timeout {
            let stack = Stack::random(inputs, size);
            if stack.test(&target) {
                return Some(stack);
            }
        }
        None
    }
}

fn as_bool_vec(mut number: usize, bits: usize) -> Vec<bool> {
    // Returns a little-endian representation of the input number.
    let mut result = Vec::with_capacity(bits);
    for _ in 0..bits {
        result.push(number % 2 == 1);
        number /= 2;
    }
    result
}

fn xor_test_function(input: &Vec<bool>) -> bool {
    input[0] ^ input[1]
}

fn mux_test_function(input: &Vec<bool>) -> bool {
    if input[2] {
        input[1]
    } else {
        input[0]
    }
}

fn main() {
    let xor = Stack::from_target_function(2, 4, xor_test_function, 10000);
    println!("XOR: {:?}", xor);

    let mux = Stack::from_target_function(3, 4, mux_test_function, 10000);
    println!("MUX: {:?}", mux);

}
