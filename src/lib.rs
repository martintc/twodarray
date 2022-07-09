mod twodarray;

#[cfg(test)]
mod tests {
    use crate::twodarray;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2darray() {
	let arr = twodarray::Array2D::<u8>::new(2, 4);
	let mut result = arr.get_height();
	assert_eq!(result, 4);
	result = arr.get_width();
	assert_eq!(result, 2);
    }
}
