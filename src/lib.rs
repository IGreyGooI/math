pub mod angle;

#[cfg(test)]
mod tests {
    use crate::angle::{a32, a64};
    #[test]
    fn convert_test() {
        assert_eq!(2 + 2, 4);
        let single: a32 = a32::new(0x40000000);
        let double: a64 = a64::new(0x4000000000000000);
        let x: a32 = a32::from(double);
        let y: a32 = double.into();
        assert_eq!(single, x);
        assert_eq!(single, y);
    }
    #[test]
    fn add_test(){
        let single1: a32 = a32::new(0x40000000);
        let single2: a32 = a32::new(0x40000000);
        assert_eq!(single1 + single2, a32::new(0x80000000));
    }
    #[test]
    fn add_overload_test(){
        let single1: a32 = a32::new(0xffffffff);
        let single2: a32 = a32::new(0x40000000);
        assert_eq!(single1 + single2, a32::new(0x3fffffff));
    }
    #[test]
    fn sub_test(){
        let single1: a32 = a32::new(0x80000000);
        let single2: a32 = a32::new(0x40000000);
        assert_eq!(single1 - single2, a32::new(0x40000000));
    }
    #[test]
    fn sub_overload_test(){
        let single1: a32 = a32::new(0x4fffffff);
        let single2: a32 = a32::new(0xf0000000);
        assert_eq!(single1 - single2, a32::new(0x5fffffff));
    }
}
