// can't use this here ?
//#[macro_use]
// extern crate more_asserts;
use rand::prelude::*;
use crate::neat::InnovationHistory;

#[derive(Debug)]
pub struct Node {
    input_sum: f64,
    output_sum: f64,
    pub layer: u64,
}

impl Node {
    pub fn clone(&self) -> Node {
        Node{input_sum:0.0,
             output_sum:0.0,
             layer:self.layer}
    }
}

fn sigmoid(value: f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-1.0 * value));
}

#[derive(Debug)]
pub struct Edge {
    pub from_node: u64,
    to_node: u64,
    pub weight: f64,
    pub enabled: bool,
    pub inno_id: u64
}

impl Edge {
    pub fn clone(&self) -> Edge {
        return Edge{from_node: self.from_node,
                    to_node: self.to_node,
                    weight: self.weight,
                    enabled: self.enabled,
                    inno_id: self.inno_id};
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
    pub bias_node_id: u64,
}

impl Network {


    pub fn new(input_node_count: u32, output_node_count: u32, fully_connect: bool ) -> Network {
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

        if fully_connect {
            let mut local_inno_id = 0;
            for _input_n in 0..input_node_count {
                for _output_n in 0..output_node_count {
                    network.edges.push(Edge {
                        from_node: _input_n as u64,
                        to_node: (_output_n + input_node_count) as u64,
                        // todo: could be random or zero.
                        weight: 0.5,
                        enabled: true,
                        inno_id: local_inno_id});
                    local_inno_id += 1;
                }
            }
            // connect bias nodes up 
            for _output_n in 0..output_node_count {
                network.edges.push(Edge {
                    from_node: network.bias_node_id,
                    to_node: (_output_n + input_node_count)as u64,
                    weight: 1.0,
                    enabled: true,
                    inno_id: local_inno_id});
                local_inno_id += 1;
            }
        }

        return network;
    }

    pub fn is_fully_connected(&self) -> bool {
        let mut max_connections = 0;
        let mut num_nodes_in_layer: Vec<u64> = Vec::new();

        for n in 0..self.layer_count {
            num_nodes_in_layer.push(0);
        }

        for node in self.nodes.iter() {
            num_nodes_in_layer[node.layer as usize] += 1;
        }

        for i in 0..self.layer_count-1 {
            let mut nodes_in_front = 0;
            for j in i+1..self.layer_count {
                nodes_in_front += num_nodes_in_layer[j as usize];
            }
            max_connections += num_nodes_in_layer[i as usize] * nodes_in_front;
        }

        return (max_connections as usize) == self.edges.len();
    }

    pub fn pretty_print(&self) -> () {
        for layer in 0..self.layer_count {
            for node in self.nodes.iter() {
                if node.layer == layer.into() {
                    println!("{:#?}", node);
                }
            }
        }
        for layer in 0..self.layer_count {
            println!("Layer: {}", layer);
            for edge in self.edges.iter() {
                if edge.enabled && self.nodes[edge.from_node as usize].layer == layer.into() { 
                    println!("{:#?}", edge);
                }
            }
        }
    }


    pub fn feed_input(&mut self, inputs: Vec<f64> ) -> Vec<f64> {

        let mut output = Vec::new();
        for i in 0..self.input_node_count {
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        // set bias to true
        self.nodes[self.bias_node_id as usize].output_sum = 1.0;

        // self.pretty_print();

        for layer in 0..self.layer_count {
            // println!("Feeding layer: {}", layer);
            for node_index in 0..self.nodes.len() {
                if self.nodes[node_index].layer == layer.into() {
                    // set the output sum of the node so it can be used as input for next layer
                    if layer != 0 {
                        // println!("output: {} of {} {}", sigmoid(self.nodes[node_index].input_sum), node_index, self.nodes[node_index].input_sum);
                        self.nodes[node_index].output_sum = sigmoid(self.nodes[node_index].input_sum);
                    }

                    // 
                    for edge in self.edges.iter() {
                        if edge.from_node as usize == node_index {
                            if edge.enabled {
                                let tmp_p = edge.weight * self.nodes[node_index].output_sum;

                                self.nodes[edge.to_node as usize].input_sum += tmp_p;
                                // println!("{} -> {}, ({}) = {}", edge.from_node, edge.to_node, tmp_p, self.nodes[edge.to_node as usize].input_sum);
                            }
                        }
                    }
                }
            }
            // self.pretty_print()
        }

        for output_i in 0..self.output_node_count { 
            let o_node = &self.nodes[(output_i + self.input_node_count) as usize];
            output.push(o_node.output_sum);
        }

        // reset all the input sums for next feed
        for node in self.nodes.iter_mut() {
            node.input_sum = 0.0;
        }

        return output;
    }

    pub fn new_node(&mut self, layer: u64) -> u64 {
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: layer};
        self.nodes.push(m);
        return (self.nodes.len() - 1) as u64;
    }
    
    pub fn random_node(&self) -> usize {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        return (y * (self.nodes.len()-1) as f64) as usize;
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

    fn get_inno_ids(&self) -> Vec<u64> {
        let mut inno_ids: Vec<u64> = Vec::new();
        
        for edge in self.edges.iter() {
            inno_ids.push(edge.inno_id);
        }
        return inno_ids;
    }
        

    /// Takes an edge and inserts a node inline. 
    pub fn add_node(&mut self, _edge_index: usize, edge1_w: f64, edge2_w: f64, inno_handler: Option<&mut InnovationHistory>) -> u64 {
        self.edges[_edge_index].enabled = false;

        let edge = &self.edges[_edge_index];
        let outgoing_node_id = edge.to_node;

        // get teh node the edge we are breaking up was pointing to. 
        let node = &self.nodes[edge.from_node as usize];
        let current_node_layer = node.layer + 1;
        
        // new node. 
        let m = Node {
            input_sum: 0.0,
            output_sum: 0.0,
            layer: current_node_layer
        };
        self.nodes.push(m);
        
        let new_inno_id = match inno_handler {
            Some(inno_history) => {
                let edge_to_node_id = edge.to_node as usize;
                let edge_from_node_id = edge.from_node as usize;
                let network_inno_ids = self.get_inno_ids();
                inno_history.get_inno_number(&network_inno_ids,
                                             edge_from_node_id,
                                             edge_to_node_id)
            },
            None => {
                2
            },
        };


        let edge1 = Edge{from_node: edge.from_node,
                         to_node: (self.nodes.len() - 1) as u64,
                         weight: edge1_w,
                         enabled: true,
                         inno_id: new_inno_id as u64};
        {
            self.edges.push(edge1);
        }

        let edge2 = Edge{from_node: (self.nodes.len() - 1) as u64,
                         to_node: edge.to_node,
                         weight: edge2_w,
                         enabled: true,
                         inno_id: new_inno_id as u64};


        self.edges.push(edge2);
        
        if self.nodes[outgoing_node_id as usize].layer == current_node_layer.into() {
            for node_i in 0..self.nodes.len()-1 {
                let node_t = &mut self.nodes[node_i];
                if node_t.layer >= current_node_layer {
                    node_t.layer += 1;
                }
            }
            self.layer_count += 1;
        }

        return (self.nodes.len() - 1) as u64;
    }

    pub fn are_connected(&self, _node_one: usize, _node_two: usize) -> bool {
        let pos_check = self.edges.iter().position(|edge| edge.to_node == _node_one as u64
                                                   && edge.from_node == _node_two as u64);
        match pos_check {
            Some(T) => return true,
            None => return false,
        }
    }

    pub fn add_connection(&mut self, _node_one: usize, _node_two: usize, weight: f64, inno_hist: Option<&mut InnovationHistory>) -> usize {
        // todo: don't add in edges if the edge already exists.
        let node_one = &self.nodes[_node_one];
        let node_two = &self.nodes[_node_two];
        if node_one.layer == node_two.layer {
            return 0;
        }
        let edge;
        // allow for nodes to be in reverse order, so if node 1 layer is greater than node 2, swap. 
        if node_one.layer > node_two.layer {
            let pos_check = self.edges.iter().position(|edge| edge.to_node == _node_one as u64
                                                       && edge.from_node == _node_two as u64);
            match pos_check {
                Some(T) => return T,
                None => (),
            }


            edge = Edge{from_node: _node_two as u64,
                        to_node: _node_one as u64,
                        weight: weight,
                        enabled: true,
                        inno_id: 1};
        } else {

            let pos_check = self.edges.iter().position(|edge| edge.to_node == _node_two as u64
                                                       && edge.from_node == _node_one as u64);
            match pos_check {
                Some(T) => return T,
                None => (),
            }


            edge = Edge{from_node: _node_one as u64,
                        to_node: _node_two as u64,
                        weight: weight,
                        enabled: true,
                        inno_id: 1};
        }
        
        self.edges.push(edge);
        return self.edges.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_and_network() -> Network {
        let mut network = Network::new(2, 1, false);
        let edge1 = network.add_connection(0, 2, 20.0, None);
        // let node1 = network.add_node(edge1, 20.0, 20.0) as usize;
        let edge2 = network.add_connection(1, 2, 20.0, None);
        let edge3 = network.add_connection(3, 2, -30.0, None);
        return network;
    }

    fn construct_xor_network() -> Network {
        let mut network = Network::new(2, 1, false);
        
        // node 0 - > end node
        let edge1 = network.add_connection(0, 2, 1.0, None);
        // node 1 - > end node
        let edge2 = network.add_connection(1, 2, -1.0, None);
        
        // node 0 -> node 2 - > end node
        let node1_index = network.add_node(edge1, 20.0, 20.0, None);

        // node 1 -> node 3 - > end node
        let node2_index = network.add_node(edge2, -20.0, 20.0, None);

        network.add_connection(3, node1_index as usize, -10.0, None);
        network.add_connection(3, node2_index as usize, 30.0, None);
        network.add_connection(3, 2 as usize, -30.0, None);
        
        // node 0 -> node 3 -> end node
        let edge3 = network.add_connection(0, node2_index as usize, -20.0, None);

        // node 1 -> node 2 -> end node
        let edge4 = network.add_connection(1, node1_index as usize, 20.0, None);
        return network;
    }

    #[test]
    fn test_add_node_layer_checking() {
        let mut network = Network::new(1, 3, true);
        let mut output = network.feed_input(vec![0.3]);
        assert_eq!(network.layer_count, 2);
        assert_eq!(output.len(), 3);
        network.add_node(2, 1.0, 2.0, None);
        output = network.feed_input(vec![0.4]);
        assert_eq!(network.layer_count, 3);
        assert_eq!(output.len(), 3);
        println!("Output of graph");
        network.pretty_print();
    }


    #[test]
    fn test_simple_add() {
        let mut network = Network::new(1, 1, false);

        network.add_connection(0, 1, 0.5, None);

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

    // looks to test that all connections go forward rather than backwards. 
    #[test]
    fn test_connections_forward_construction() {
        let network = Network::new(4, 400, true);

        for edge in network.edges.iter() {
            let from_node = &network.nodes[edge.from_node as usize];
            let to_node = &network.nodes[edge.to_node as usize];
            println!("Checking: {} -> {}", edge.from_node, edge.to_node);
            assert!(from_node.layer < to_node.layer);
        }
    }


    #[test]
    // check to see as we add in nodes that edges preserve
    // forward feed direction. 
    fn test_connections_add_node_forward() {
        let mut network = Network::new(4, 400, true);
        let mut node_count = 4 + 400 + 1;
        assert_eq!(network.nodes.len(), node_count);

        for i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0, None);
            node_count += 1;
            assert_eq!(network.nodes.len(), node_count);
            for edge in network.edges.iter() {
                let from_node = &network.nodes[edge.from_node as usize];
                let to_node = &network.nodes[edge.to_node as usize];
                println!("Checking: {} -> {}", edge.from_node, edge.to_node);
                assert!(from_node.layer < to_node.layer);
            }
        }
    }

    #[test]
    // check to see as we add in nodes that edges preserve
    // forward feed direction. 
    fn test_connections_add_connection_forward() {
        let mut network = Network::new(4, 400, true);
        let mut node_count = 4 + 400 + 1;
        assert_eq!(network.nodes.len(), node_count);

        for i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0, None);
            node_count += 1;
            assert_eq!(network.nodes.len(), node_count);
            for edge in network.edges.iter() {
                let from_node = &network.nodes[edge.from_node as usize];
                let to_node = &network.nodes[edge.to_node as usize];
                println!("Checking: {} -> {}", edge.from_node, edge.to_node);
                assert!(from_node.layer < to_node.layer);
            }
        }
    }

    macro_rules! network_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (mut network, input, expected) = $value;
                    let output = network.feed_input(input);
                    network.pretty_print();
                    assert_eq!(output.len(), 1);
                    assert!(output[0] > expected[0]);
                    assert!(output[0] < expected[1]);
                }
            )*
        }
    }

    network_test! {
        xor_one_one: (construct_xor_network(), vec![1.0, 1.0], vec![-0.1, 0.1]),
        xor_zero_zero: (construct_xor_network(), vec![0.0, 0.0], vec![-0.1, 0.1]),
        xor_one_zero: (construct_xor_network(), vec![1.0, 0.0], vec![0.9, 1.1]),
        xor_zero_one: (construct_xor_network(), vec![0.0, 1.0], vec![0.9, 1.1]),

        and_one_one: (construct_and_network(), vec![1.0, 1.0], vec![0.9, 1.1]),
        and_one_zero: (construct_and_network(), vec![1.0, 0.0], vec![-0.1, 0.1]),
        and_zero_one: (construct_and_network(), vec![0.0, 1.0], vec![-0.1, 0.1]),
        and_zero_zero: (construct_and_network(), vec![0.0, 0.0], vec![-0.1, 0.1]),
            
    }

    #[test]
    fn test_fully_connected() {
        let network = Network::new(2, 40, true);
        assert!(network.is_fully_connected());

        // todo: add more neteworks with connections.

        let mut net = Network::new(3, 5, true);
        net.add_node(2, 1.0, 2.0, None);
        println!("{:#?}", net);
        assert!(!net.is_fully_connected());
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


