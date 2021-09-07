use std::fmt::Debug;

pub struct Judge<T> {
    assert_type: AssertType,
    src_val: T,
}

pub enum AssertType {
    Eq,
    Ne,
}

impl<T> Judge<T>
where
    T: Debug + Eq,
{
    pub fn new(assert_type: AssertType, src_val: T) -> Self {
        return Judge {
            assert_type,
            src_val,
        };
    }

    pub fn assert(&self, target_val: T) {
        match self.assert_type {
            AssertType::Eq => {
                assert_eq!(self.src_val, target_val);
            }
            AssertType::Ne => todo!(),
        }
    }
}
