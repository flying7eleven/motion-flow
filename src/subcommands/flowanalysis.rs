use log::trace;

pub struct FlowAnalysis();

impl FlowAnalysis {
    pub fn new() -> std::option::Option<FlowAnalysis> {
        trace!("NEW");
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
