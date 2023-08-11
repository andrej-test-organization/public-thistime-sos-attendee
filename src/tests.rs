#[cfg(test)]
mod test {
    use crate::secret;

    #[test]
    fn it_works() {
        let x_in: f64 = 1.0;
        let y_in: f64 = 5.0;
        let mut example = secret::Compute::new(&x_in, &y_in);

        assert_eq!(example.do_adition(), 6.0);

        example.change_x(&8.0);
        example.change_y(&8.0);

        assert_eq!(example.do_adition(), 16.0);
    }
}
