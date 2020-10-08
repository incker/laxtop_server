/// Wrapper to make requests and responses always objects (instead arrays/text)
#[derive(Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    data: T,
}

impl<T> DataWrapper<T> {
    pub fn new(data: T) -> Self {
        DataWrapper { data }
    }

    pub fn data(self) -> T {
        self.data
    }
}
