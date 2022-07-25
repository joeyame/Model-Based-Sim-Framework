pub mod model {
    pub trait SimModel {
        fn initialize( &mut self ) -> bool;
        fn update( &mut self ) -> bool;
        fn finalize( &mut self ) -> bool;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
