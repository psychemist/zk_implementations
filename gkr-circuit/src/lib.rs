use ark_ff::PrimeField;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Null,
}

impl<F: PrimeField> From<F> for Operation {
    fn from(value: F) -> Self {
        match value {
            0 => Operation::Add,
            1 => Operation::Mul,
            2 => Operation::Null,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<F: PrimeField> {
    output_index: F,
    left: F,
    right: F,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct Layer<F: PrimeField> {
    gates: Vec<Gate>,
    // outputs: Vec<F>,
}

#[derive(Debug, Clone)]
struct Circuit<F: PrimeField> {
    layers: Vec<Vec<Gate>>,
    // layers: Vec<Layer>,
    evaluations: Vec<Vec<F>>,
}

impl Circuit {
    fn new() -> Self {
        // // let height = (inputs.len() as f64).sqrt();
        // // let mut layers = vec![vec![]; (2_u32.pow(height as F) - 1).try_into().unwrap()];
        // let mut layers = vec![];

        // for i in 0..height {
        //     let mut layer = vec![];

        //     for j in 0..2_u32.pow(i) {
        //         let o_index = if i == 0 {
        //             0
        //         } else {
        //             j / i
        //         };

        //         let gate = Gate {
        //             output_index: o_index,
        //             left: 0,
        //             right: 1,
        //             operation: Operation::Null
        //         };

        //         layer.push(gate);
        //     }

        //     layers.push(layer);
        // }

        Circuit {
            layers: vec![],
            evaluations: vec![],
        }
    }

    fn build_circuit<F: PrimeField>(layers: Vec<Vec<F>>) {
        let mut circuit =  Circuit::new();

        for (i, layer) in layers.iter().enumerate() {
            let mut gate  =  Gate {
                output_index: layer[0],
                left: layer[1],
                right: layer[2],
                operation: Operation::from(layer[3]),
            };
        }
    }

    // fn evaluate(inputs: Vec<F>) {
    //     
    // }

    fn print_circuit332(&self) {
        for (layer_index, layer) in self.layers.iter().enumerate() {
            println!("Layer {}:", layer_index);
            for (gate_index, gate) in layer.iter().enumerate() {
                let op = match gate.operation {
                    Operation::Add => "+",
                    Operation::Mul => "*",
                    Operation::Null => "",
                };
                let left_value = gate.left;
                let right_value = gate.right;
                let output_index = gate.output_index;
                println!(
                    "  Gate {}: {} {} {} = {}",
                    gate_index,
                    left_value,
                    right_value,
                    output_index,
                    op,
                );
            }
            println!();
        }
    }
}

fn main() {
    let circuit = Circuit::new();

    circuit.print_circuit();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let circuit = Circuit::new();



        circuit.print_circuit();
    }
}