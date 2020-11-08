
// can't use this here ?
//#[macro_use]
// extern crate more_asserts;
use rand::prelude::*;

#[derive(Debug)]
pub struct Node {
    input_sum: f64,
    output_sum: f64,
    layer: u64,
}

fn sigmoid(value: f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-1.0 * value));
}

impl Node {
    fn engage(&mut self) -> () {
        // iterate over the inbound items and do the sigmoid
        println!("Engage of node: {}", self.layer)
    }
}

#[derive(Debug)]
pub struct Edge {
    from_node: u64,
    to_node: u64,
    inno_id: u64,
    weight: f64,
    enabled: bool
}

impl Edge {
    pub fn clone(&self) -> Edge {
        return Edge{from_node: self.from_node,
                    to_node: self.to_node,
                    weight: self.weight,
                    inno_id: self.inno_id,
                    enabled: self.enabled};
    }
}

#[derive(Debug)]
pub struct Network {

    // first I nodes are input nodes
    // after which the output nodes are next.
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub input_node_count: u32,
    pub output_node_count: u32,
    pub layer_count: u32,
    pub bias_node_id: u64
}

impl Network {
    pub fn new(input_node_count: u32, output_node_count: u32) -> Network {
        let mut network = Network{nodes: Vec::new(),
                                  edges: Vec::new(),
                                  input_node_count: input_node_count,
                                  output_node_count: output_node_count,
                                  layer_count: 2,
                                  bias_node_id: 0};
        for _input_n in 0..input_node_count {
            network.new_node(0);
        }

        for _output_n in 0..output_node_count {
            network.new_node(1);
        }

        // bias node.
        network.new_node(0);

        network.bias_node_id = (network.nodes.len()-1) as u64;

        return network;
    }

    pub fn feed_input(&mut self, inputs: Vec<f64> ) -> Vec<f64> {
        let mut output = Vec::new();
        for i in 0..self.input_node_count {
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        // set bias to true
        self.nodes[self.bias_node_id as usize].output_sum = 1.0;

        for layer in 0..self.layer_count {
            for node_index in 0..self.nodes.len() {
                if self.nodes[node_index].layer == layer.into() {
                    // set the output sum of the node so it can be used as input for next layer
                    if layer != 0 {
                        self.nodes[node_index].output_sum = sigmoid(self.nodes[node_index].input_sum);
                    }

                    // 
                    for edge in self.edges.iter() {
                        if edge.from_node as usize == node_index {
                            if edge.enabled {
                                let tmp_p = edge.weight * self.nodes[node_index].output_sum;
                                self.nodes[edge.to_node as usize].input_sum += tmp_p;
                            }
                        }
                    }
                }
            }
        }

        for node in self.nodes.iter() {
            if node.layer == (self.layer_count-1).into() {
                output.push(node.output_sum);
            }
        }

        // reset all the input sums for next feed
        for node in self.nodes.iter_mut() {
            node.input_sum = 0.0;
        }

        return output;
    }

    fn new_node(&mut self, layer: u64) -> () {
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: layer};
        self.nodes.push(m);
    }
    
    pub fn random_edge(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        return (y * (self.edges.len()-1) as f64) as u64;
    }

    pub fn random_non_bias_edge(&self) -> u64 {
        let mut edge_index: u64 = self.random_edge();

        while self.edges[edge_index as usize].from_node == self.bias_node_id  {
            edge_index = self.random_edge();
        }
        return edge_index;
    }


    /// Takes an edge and inserts a node inline. 
    pub fn add_node(&mut self, _edge_index: usize, edge1_w: f64, edge2_w: f64) -> u64 {
        let edge = &mut self.edges[_edge_index];
        edge.enabled = false;

        let node = &self.nodes[edge.from_node as usize];
        let current_node_layer = node.layer + 1;
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: current_node_layer};
        self.nodes.push(m);

        let edge1 = Edge{from_node: edge.from_node,
                         to_node: (self.nodes.len() - 1) as u64,
                         inno_id: 2,
                         weight: edge1_w,
                         enabled: true};

        let edge2 = Edge{from_node: (self.nodes.len() - 1) as u64,
                         to_node: edge.to_node,
                         inno_id: 2,
                         weight: edge2_w,
                         enabled: true};
        let outgoing_node_id = edge.to_node;

        self.edges.push(edge1);
        self.edges.push(edge2);

        for node_i in 0..self.nodes.len()-1 {
            let node_t = &mut self.nodes[node_i];
            if node_t.layer >= current_node_layer {
                node_t.layer += 1;
            }
        }
        self.layer_count += 1;
        return (self.nodes.len() - 1) as u64;
    }

    pub fn add_connection(&mut self, _node_one: usize, _node_two: usize, weight: f64) -> usize {
        let edge = Edge{from_node: _node_one as u64,
                        to_node: _node_two as u64,
                        inno_id: 1,
                        weight: weight,
                        enabled: true};
        self.edges.push(edge);
        return self.edges.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut network = Network::new(1, 1);

        network.add_connection(0, 1, 0.5);

        let input_value = vec![1.0, 2.0];
        println!("First evaulation");
        let mut output_values = network.feed_input(input_value);

        // simplest network
        // 1.0 -> node 1 -(0.5)> node 2 -> output (0.5)

        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.6224593312018546);

        println!("Second evaulation");
        output_values = network.feed_input(vec![1.0]);
        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.6224593312018546);
    }

    fn construct_xor_network() -> Network {
        let mut network = Network::new(2, 1);
        
        // node 0 - > end node
        let edge1 = network.add_connection(0, 2, 1.0);
        // node 1 - > end node
        let edge2 = network.add_connection(1, 2, -1.0);
        
        // node 0 -> node 2 - > end node
        let node1_index = network.add_node(edge1, 20.0, 20.0);

        // node 1 -> node 3 - > end node
        let node2_index = network.add_node(edge2, -20.0, 20.0);

        network.add_connection(3, node1_index as usize, -10.0);
        network.add_connection(3, node2_index as usize, 30.0);
        network.add_connection(3, 2 as usize, -30.0);
        
        // node 0 -> node 3 -> end node
        let edge3 = network.add_connection(0, node2_index as usize, -20.0);

        // node 1 -> node 2 -> end node
        let edge4 = network.add_connection(1, node1_index as usize, 20.0);
        return network;
    }

    #[test]
    fn test_xor_network_one_one() {
        let mut network = construct_xor_network();
        let mut output_values = network.feed_input(vec![1.0, 1.0]);
        println!("{:?}", output_values);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 0.1);
        assert!(output_values[0] > -0.1);

        output_values = network.feed_input(vec![1.0, 1.0]);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 0.1);
        assert!(output_values[0] > -0.1);

    }

    #[test]
    fn test_xor_network_zero_zero() {
        let mut network = construct_xor_network();
        let output_values = network.feed_input(vec![0.0, 0.0]);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 0.1);
        assert!(output_values[0] > -0.1);

    }

    #[test]
    fn test_xor_network_zero_one() {
        let mut network = construct_xor_network();
        let output_values = network.feed_input(vec![0.0, 1.0]);
        assert_eq!(output_values.len(), 1);
        println!("output_values: {:?}", output_values);
        assert!(output_values[0] < 1.1);
        assert!(output_values[0] > 0.9);


    }

    #[test]
    fn test_xor_network_one_zero() {
        let mut network = construct_xor_network();
        let mut output_values = network.feed_input(vec![1.0, 0.0]);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 1.1);
        assert!(output_values[0] > 0.9);

        output_values = network.feed_input(vec![1.0, 0.0]);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 1.1);
        assert!(output_values[0] > 0.9);
    }

    #[test]
    fn test_sigmoid() {

        let r = sigmoid(0.0);
        println!("Sigmoid of 0 is: {}", r);
        if r > 0.52 {
            assert!(false);
        }
        if r < 0.48 {
            println!("Result of r: {}", r);
            assert!(false);
        }
        
    }
    
}
