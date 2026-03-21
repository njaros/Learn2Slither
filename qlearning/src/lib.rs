/// Je sais pas ou je vais avec mon States
struct States {
    cardinals: Vec<usize>
}

struct Agent {
    q_table: Vec<Vec<f64>>,
    best_score: f64,
    best_q_table: Vec<Vec<f64>>
}

impl Agent {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
