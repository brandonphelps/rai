

// struct Species {
//     excess_coeff: f64;
//     weight_diff_coeff: f64;
//     compat_threashold: f64;
// }


// impl Species {
//     pub fn same_species(&self, other: &nn::Network) {
        
//     }

//     pub fn get_excess_disjoing(one: &nn::Network, two: &nn::Network) {
//         let mut matching = 0.0;
//         for one_edge in one.edges.iter() {
//             for two_edge in two.edges.iter() {
                
//             }
//         }
//     }
// }

#[derive(Debug)]
pub struct InnovationHistory {
    global_inno_id: usize,
    pub conn_history: Vec<ConnHistory>,
}

impl InnovationHistory {
    pub fn get_inno_number(&mut self, network_inno_ids: &Vec<u64>, from_node: usize, to_node: usize) -> usize {
        let mut is_new = true;
        // todo: change zero to next conn number.
        let mut connect_inno_num = self.global_inno_id;
        self.global_inno_id += 1;
        for conn_history in self.conn_history.iter() {
            match conn_history.inno_numbers.iter().position(|inno_num|
                                                           conn_history.matches(network_inno_ids,
                                                                                from_node,
                                                                                to_node)) {
                Some(t) => {
                    is_new = false;
                    connect_inno_num = conn_history.inno_number;
                    break;
                },
                None => (),
            }
        }

        if is_new { 
            let mut new_inno_nums = Vec::<u64>::new();
            for edge in network_inno_ids.iter() {
                new_inno_nums.push(edge.clone());
            }

            self.conn_history.push(ConnHistory::new(from_node, to_node, connect_inno_num, new_inno_nums));
        }

        return connect_inno_num;
    }

}


#[derive(Debug)]
pub struct ConnHistory {
    from_node: usize,
    to_node: usize,
    inno_number: usize,
    inno_numbers: Vec<u64>,
}


impl ConnHistory {
    pub fn new(from_node: usize, to_node: usize, inno_number: usize, inno_numbers: Vec<u64>) -> ConnHistory {
        return ConnHistory{from_node: from_node,
                           to_node: to_node,
                           inno_number: inno_number,
                           inno_numbers: inno_numbers };
    }

    /// inserts and returns a new inno_id or if already existing returns current one.
    pub fn matches(&self, network_inno_ids: &Vec<u64>, from_node: usize, to_node: usize) -> bool {
        // ConnHistory must have the same number of numbers as edges
        if network_inno_ids.len() == self.inno_numbers.len() {
            if from_node == self.from_node && to_node == self.to_node {
                for inno_index in 0..network_inno_ids.len() {
                    let net_inno_id = &network_inno_ids[inno_index];
                    if ! self.inno_numbers.iter().any(|inno_id| net_inno_id == inno_id) {
                        return false;
                    }
                }
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let mut network: Vec<u64> = Vec::new();
        for edge_count in 0..12 {
            network.push(edge_count);
        }
        let global_inno_id = 2;

        let mut innovation_history = InnovationHistory {
            global_inno_id: global_inno_id,
            conn_history: vec![]
        };

        let new_inno_num = innovation_history.get_inno_number(&network, 0, 2);
        assert_eq!(new_inno_num, 2);
        let conn_history = &innovation_history.conn_history[0];

        println!("{:#?}", conn_history);
        println!("{:#?}", innovation_history);

        assert!(conn_history.matches(&network, 0, 2));


        // new connection that hasn't been seen before. 
        let new_inno_num = innovation_history.get_inno_number(&network, 0, 20);
        assert_eq!(new_inno_num, global_inno_id+1);
        
    }
}
