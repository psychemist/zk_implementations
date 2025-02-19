// #[derive(Debug, Clone, Copy)]
// enum Operation {
//     Add,
//     Multiply,
// }

// #[derive(Debug, Clone)]
// struct Gate {
//     left: usize,
//     right: usize,
//     operation: Operation,
//     output: usize,
// }

// #[derive(Debug)]
// struct Layer {
//     gates: Vec<Gate>,
//     outputs: Vec<usize>,
// }

// #[derive(Debug)]
// struct Circuit {
//     layers: Vec<Layer>,
//     results: Vec<f64>,
// }

// impl Circuit {
//     fn new() -> Self {
//         Circuit {
//             layers: Vec::new(),
//             results: Vec::new(),
//         }
//     }

//     fn add_layer(&mut self, layer: Layer) {
//         self.layers.push(layer);
//     }

//     fn execute(&mut self) {
//         for layer_index in 0..self.layers.len() {
//             let layer = &mut self.layers[layer_index];
//             for (gate_index, gate) in layer.gates.iter_mut().enumerate() {
//                 let left_value = self.results[gate.left];
//                 let right_value = self.results[gate.right];
//                 let result = match gate.operation {
//                     Operation::Add => left_value + right_value,
//                     Operation::Multiply => left_value * right_value,
//                 };

//                 gate.output = Some(self.results.len());
//                 self.results.push(result);
//                 layer.outputs.push(self.results.len() - 1);
//             }
//         }
//     }

//     fn print_circuit(&self) {
//         for (layer_index, layer) in self.layers.iter().enumerate() {
//             println!("Layer {}:", layer_index + 1);
//             for (gate_index, gate) in layer.gates.iter().enumerate() {
//                 let op = match gate.operation {
//                     Operation::Add => "+",
//                     Operation::Multiply => "*",
//                 };
//                 let left_value = self.results[gate.left];
//                 let right_value = self.results[gate.right];
//                 let output_value = self.results[gate.output.unwrap()];
//                 println!(
//                     "  Gate {}: {} {} {} = {}",
//                     gate_index,
//                     left_value,
//                     op,
//                     right_value,
//                     output_value
//                 );
//             }
//             println!();
//         }
//     }
// }

// // Test the Circuit implementation
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic_circuit() {
//         // Initialize the circuit
//         let mut circuit = Circuit::new();

//         // Layer 1 (Root) - 1 gate: Multiply (2 * 3)
//         let layer1 = Layer {
//             gates: vec![Gate {
//                 left: 0,
//                 right: 1,
//                 operation: Operation::Multiply,
//                 output: None,
//             }],
//             outputs: Vec::new(),
//         };
//         circuit.add_layer(layer1);

//         // Layer 2 - 2 gates: Multiply (left of layer 1) and Add
//         let layer2 = Layer {
//             gates: vec![
//                 Gate {
//                     left: 0,
//                     right: 1,
//                     operation: Operation::Multiply,
//                     output: None,
//                 },
//                 Gate {
//                     left: 2,
//                     right: 3,
//                     operation: Operation::Add,
//                     output: None,
//                 },
//             ],
//             outputs: Vec::new(),
//         };
//         circuit.add_layer(layer2);

//         // Layer 3 - 4 gates with dynamic inputs
//         let layer3 = Layer {
//             gates: vec![
//                 Gate {
//                     left: 1,
//                     right: 2,
//                     operation: Operation::Add,
//                     output: None,
//                 },
//                 Gate {
//                     left: 3,
//                     right: 4,
//                     operation: Operation::Multiply,
//                     output: None,
//                 },
//                 Gate {
//                     left: 5,
//                     right: 6,
//                     operation: Operation::Add,
//                     output: None,
//                 },
//                 Gate {
//                     left: 7,
//                     right: 8,
//                     operation: Operation::Multiply,
//                     output: None,
//                 },
//             ],
//             outputs: Vec::new(),
//         };
//         circuit.add_layer(layer3);

//         // Test execution
//         circuit.execute();

//         // Test visualization
//         circuit.print_circuit();

//         // Check the results
//         assert_eq!(circuit.results.len(), 9); // Expect 9 results (3 layers with several gates)
//         assert_eq!(circuit.results[0], 6.0); // Layer 1, gate 0 (2 * 3)
//         assert_eq!(circuit.results[1], 7.0); // Layer 1, gate 1 (4 + 3)
//         assert_eq!(circuit.results[2], 42.0); // Layer 2, gate 0 (2 * 3)
//     }

//     #[test]
//     fn test_empty_circuit() {
//         // Test with no layers and gates
//         let mut circuit = Circuit::new();
//         circuit.execute();
//         assert_eq!(circuit.results.len(), 0);
//     }

//     #[test]
//     fn test_layer_agnostic() {
//         // Initialize the circuit
//         let mut circuit = Circuit::new();

//         // Single layer with a multiply gate
//         let layer = Layer {
//             gates: vec![Gate {
//                 left: 0,
//                 right: 1,
//                 operation: Operation::Multiply,
//                 output: None,
//             }],
//             outputs: Vec::new(),
//         };
//         circuit.add_layer(layer);

//         // Execute and check
//         circuit.execute();
//         assert_eq!(circuit.results[0], 6.0); // 2 * 3 = 6
//     }
// }


#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Null,
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    output_index: u32,
    left: u32,
    right: u32,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct Layer {
    gates: Vec<Gate>,
    // outputs: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Circuit {
    layers: Vec<Vec<Gate>>,
    // layers: Vec<Layer>,
    evaluations: Vec<Vec<u32>>,
}

impl Circuit {
    fn new() -> Self {
        // // let height = (inputs.len() as f64).sqrt();
        // // let mut layers = vec![vec![]; (2_u32.pow(height as u32) - 1).try_into().unwrap()];
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

    fn initialize(layers: u32) {
        
    }

    // fn evaluate(inputs: Vec<u32>) {
    //     
    // }

    fn print_circuit(&self) {
        for (layer_index, layer) in self.layers.iter().enumerate() {
            println!("Layer {}:", layer_index);
            for (gate_index, gate) in layer.iter().enumerate() {
                let op = match gate.operation {
                    Operation::Add => "+",
                    Operation::Multiply => "*",
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