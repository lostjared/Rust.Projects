
pub mod res_ex {

    pub fn extract_res(input: &str) -> (u32, u32) {
        let pos = input.find('x').unwrap();
        let left = &input[..pos];
        let right = &input[pos+1..];
        (left.parse().unwrap(), right.parse().unwrap())
    }

    pub struct Res {
        width: u32,
        height: u32,
    }

    pub fn extract_resolution(input: &str) -> Res {
        let x = extract_res(input);
        Res {
            width: x.0,
            height: x.1,
        }
    }
}