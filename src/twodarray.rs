#[derive(Clone, Debug, Default)]
pub struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + std::clone::Clone> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
	if width < 1 || height < 1 {
	    return Array2D {
		data: vec![],
		width: 0,
		height: 0,
	    }
	}
        let size = (width * height) as usize;
        Array2D {
            data: vec![Default::default(); size],
            width,
            height,
        }
    }

    pub fn populate_array_with_data(&mut self, data: Vec<T>) -> Result<(), &str> {
        if data.len() > self.data.len() {
            //panic!("Data to populate array is larger than the array");
	    return Err("Data to populate array is larger than the array");
        }
	for (pos, e) in data.iter().enumerate() {
	    self.data[pos] = e.clone();
	}
	Ok(())
    }

    pub fn get_size(&self) -> usize {
        self.data.len() as usize
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_value_at(&self, row: usize, column: usize) -> Result<T, &str>
    where
        T: Copy,
    {
	if row > self.height {
	    return Err("Invalid row for data access");
	}
	if column > self.width {
	    return Err("Invalid column for data access");
	}
        let index: usize = self.get_index(row, column);
        Ok(self.data[index])
    }

    pub fn set_value_at(&mut self, row: usize, column: usize, data: T) {
        let index: usize = self.get_index(row, column);
        self.data[index] = data;
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        self.width * row + column
    }
}

#[cfg(test)]
mod tests {
    use super::Array2D;

    #[test]
    fn test_new_array() {
        let _arr = Array2D::<u8>::new(4, 4);
        assert!(true)
    }

    #[test]
    fn test_array_size() {
        let arr = Array2D::<u8>::new(4, 5);
        assert_eq!(arr.get_size(), (4 * 5));
    }

    #[test]
    fn test_array_populate_with_data() {
        let mut arr = Array2D::<u8>::new(2, 2);
        let data: [u8; 4] = [2; 4];
	match arr.populate_array_with_data(data.to_vec()) {
	    Ok(_) => assert!(true),
	    Err(_) => assert!(false),
	}
    }

    #[test]
    fn test_array_populate_with_data_error() {
	let mut array = Array2D::<u8>::new(2, 2);
	let data: [u8; 5] = [2; 5];
	match array.populate_array_with_data(data.to_vec()) {
	    Ok(_) => assert!(false),
	    Err(_) => assert!(true),
	}
    }

    #[test]
    fn test_get_height() {
        let arr = Array2D::<u8>::new(80, 100);
        assert_eq!(arr.get_height(), 100)
    }

    #[test]
    fn test_get_width() {
        let arr = Array2D::<u8>::new(80, 100);
        assert_eq!(arr.get_width(), 80);
    }

    #[test]
    fn test_get_value_at() {
        let test_data = vec![1, 2, 3, 4];
        let mut arr = Array2D::<i8>::new(2, 2);
        match arr.populate_array_with_data(test_data) {
	    Ok(_) => assert!(true),
	    Err(_) => assert!(false),
	}
        let mut result: i8 = match arr.get_value_at(0, 0) {
	    Ok(r) => r,
	    Err(_) => -1,
	};
        assert_eq!(result, 1);
        result = match arr.get_value_at(0, 1) {
	    Ok(r) => r,
	    Err(_) => -1,
	};
        assert_eq!(result, 2);
        result = match arr.get_value_at(1, 0) {
	    Ok(r) => r,
	    Err(_) => -1,
	};
        assert_eq!(result, 3);
        result = match arr.get_value_at(1, 1) {
	    Ok(r) => r,
	    Err(_) => -1,
	};
        assert_eq!(result, 4);
    }

    #[test]
    fn test_set_value_at() {
        let mut arr = Array2D::<i8>::new(4, 5);
        arr.set_value_at(2, 3, 5);
        let result = match arr.get_value_at(2, 3) {
	    Ok(r) => r,
	    Err(_) => -1,
	};
        assert_eq!(result, 5);
    }
}
