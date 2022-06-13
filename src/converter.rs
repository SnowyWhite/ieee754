
/// This struct stores data that is needed to do the conversion
#[derive(Debug, Default, Clone, Copy)]
pub struct Ieee754 {
    /// This is the float number that will get converted
    pub input: f32,
    /// This value represents the decimal places, i.e. for 13.37 it would be 0.37
    pub decimal_place: f32,
    /// This value represents the digits before the decimal point, i.e. for 13.37 it would be 13
    pub pre_decimal_place: u32,
    pub exponent: usize,
    pub bias: usize
}

impl Ieee754 {
    /// Returns a new instance with the provided input value 
    /// and any member else with its default value initialized 
    /// 
    /// # Arguments
    /// * `input` - A float number that should be converted
    /// 
    /// # Examples
    /// 
    /// ```
    /// let ieee = Ieee754::new(13.37);
    /// ```
    pub fn new(input: f32) -> Self {
        Ieee754 {
            input,
            ..Default::default()
        }
    }

    /// Convert the digits before the decimal point to a string which represents the binary format of it
    /// 
    /// # Arguments
    /// * `bias` - This method is used twice, once without an argument (and thus returning the **Ieee754::input** in binary format) 
    /// and once with the bias (yes ik that it's ugly af)
    /// 
    ///  # Examples
    /// 
    /// ```
    /// let ieee = Ieee754::new(13.37);
    /// let binary_str = ieee.convert_pre_decimalplace_to_binary_string(None);
    /// let bias_str = ieee.convert_pre_decimalplace_to_binary_string(Option::Some(127));
    /// 
    /// assert_eq!(binary_str, "1101");
    /// assert_eq!(bias_str, "1111111");
    /// ```
    pub fn convert_pre_decimalplace_to_binary_string(&mut self, bias: Option<usize>) -> String {
    
        match bias {
            Some(b) => format!("{:b}", b),
            None => {
                self.pre_decimal_place = self.input as u32;
                format!("{:b}", self.pre_decimal_place)
            }
        }
    }

    
    pub fn convert_decimalplace_to_binary_string(&mut self) -> String {
        let mut str_bin: String = String::new();
        let mut decplace: f32 = self.input - (self.input as u32) as f32;
        self.decimal_place = decplace;
        let mut tmp: f32;

        while str_bin.len() < 23 {
            decplace *= 2.0;
            tmp = decplace;

            if tmp >= 1.0 {
                str_bin.push('1');
                let i: u32 = decplace as u32;
                let f: f32 = tmp - i as f32;
                decplace = f;
            } else {
                str_bin.push('0');
            }
        }

        str_bin
    }

    pub fn normalize(&mut self, mut input: String) -> String {
        let size = input.find('.');

        match size {
            Some(s) => self.exponent = s - 1,
            None => panic!("Invalid input"),
        }

        let mut result: String = String::new();

        result.reserve(input.capacity());
        input = input.replace('.', "");
        input = input.split_off(1);
        input = "1.".to_string() + &input;

        if input.len() < 23 {
            let add = 23 - input.len();
            for _ in 0..add {
                input.push('0');
            }
        }
        input
    }

    pub fn get_bias(&mut self) -> String {
        let bias: usize = self.exponent + 127;
        let ret: String = self.convert_pre_decimalplace_to_binary_string(Option::Some(bias));
        let mut add_zero: String = String::new();

        if ret.len() < 8 {
            let add: usize = 8 - ret.len();

            for _ in 0..add {
                add_zero.push('0');
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
            ieee.push('0');
        }

        ieee
    }
}
