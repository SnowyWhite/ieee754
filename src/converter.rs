#[derive(Debug, Default, Clone, Copy)]
pub struct Ieee754 {
    pub input: f32,
    pub decimal_place: f32,
    pub pre_decimal_place: u32,
    pub exponent: usize,
}

impl Ieee754 {
    pub fn new(input: f32) -> Self {
        Ieee754 {
            input: input,
            ..Default::default()
        }
    }

    pub fn convert_predecimalplace_to_binary(&mut self, bias: Option<usize>) -> String {
        match bias {
            Some(b) => format!("{:b}", b),
            None => {
                self.pre_decimal_place = self.input as u32;
                format!("{:b}", self.pre_decimal_place)
            }
        }
    }

    pub fn convert_decimalplace_to_binary(&mut self) -> String {
        let mut str_bin: String = String::new();
        let mut decplace: f32 = self.input - (self.input as u32) as f32;
        self.decimal_place = decplace;
        let mut tmp: f32;

        while str_bin.len() < 23 {
            decplace = decplace * 2.0;
            tmp = decplace;

            if tmp >= 1.0 {
                str_bin.push_str("1");
                let i: u32 = decplace as u32;
                let f: f32 = tmp - i as f32;
                decplace = f;
            } else {
                str_bin.push_str("0");
            }
        }

        str_bin
    }

    pub fn normalize(&mut self, mut input: String) -> String {
        let size = input.find(".");

        match size {
            Some(s) => self.exponent = s - 1,
            None => panic!("Invalid input"),
        }

        let mut result: String = String::new();

        result.reserve(input.capacity());
        input = input.replace(".", "");
        input = input.split_off(1);
        input = "1.".to_string() + &input;

        if input.len() < 23 {
            let add = 23 - input.len();
            for _ in 0..add {
                input.push_str("0");
            }
        }
        input
    }

    pub fn get_bias(&mut self) -> String {
        let bias: usize = self.exponent + 127;
        let ret: String = self.convert_predecimalplace_to_binary(Option::Some(bias));
        let mut add_zero: String = String::new();

        if ret.len() < 8 {
            let add: usize = 8 - ret.len();

            for _ in 0..add {
                add_zero.push_str("0");
            }

            add_zero + &ret
        } else {
            ret
        }
    }

    pub fn get_ieee754(&mut self, normalized_binary: String, bias: String) -> String {
        let fixed: String = normalized_binary[2..].to_string();
        let mut ieee: String = "0".to_string() + &bias + &fixed;

        if ieee.len() > 32 {
            ieee.truncate(23);
        }

        while ieee.len() < 32 {
            ieee.push_str("0");
        }

        ieee
    }
}
