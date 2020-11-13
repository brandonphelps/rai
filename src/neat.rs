
use crate::nn;

struct Species {
    
}


#[derive(Debug)]
struct ConnHistory {
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
    pub fn get_inno_number(global_innovation_num: &mut usize, innovation_hist: &mut Vec<ConnHistory>, network_inno_ids: &Vec<u64>, from_node: usize, to_node: usize) -> usize {

        let mut is_new = true;

        // todo: change zero to next conn number.
        let mut connect_inno_num = global_innovation_num.clone();
        *global_innovation_num += 1;
        for conn_history in innovation_hist.iter() {
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

            innovation_hist.push(ConnHistory::new(from_node, to_node, connect_inno_num, new_inno_nums));
        }

        return connect_inno_num;
    }
    

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
        let network: Vec<u64> = Vec::new();
        let mut innovation_history = vec![];
        let mut global_inno_num = 2;
        
        let new_inno_num = ConnHistory::get_inno_number(&mut global_inno_num, &mut innovation_history, &network, 0, 12);
        
        let conn_history = &innovation_history[0];

        println!("{:#?}", conn_history);

        assert!(conn_history.matches(&network, 0, 4));
        
    }
}
