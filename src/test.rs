#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::node::Nodes;

    const QRIS: &str = "00020101021126710019ID.CO.CIMBNIAGA.WWW011878728356757817222102150002186871651250303UMI51450015ID.OR.QRNPG.WWW0215ID81275673266770303UMI5204599953033605802ID5914AABBCCD*6714516006KEDIRI61054423462120708123456786304097D";
    #[test]
    fn verify_crc16_and_dumps_qris() -> Result<(), Box<dyn Error>>{
        let nodes  = Nodes::from_str(QRIS)?;
        assert!(nodes.verify());
        assert!(nodes.dumps() == QRIS);
        Ok(())
    }
    #[test]
    fn set_mechant_name()-> Result<(), Box<dyn Error>>{
        let name = String::from("KStore");
        let name_cmp = name.clone();
        let mut nodes  = Nodes::from_str(QRIS)?;
        nodes.set_merchant_name(name);
        nodes.rewrite_crc16();
        let result_content = nodes.dumps();
        let result_parse = Nodes::from_str(&result_content)?;
        assert!(result_parse.get_merchant_name().unwrap() == name_cmp);
        Ok(())
    }
    #[test]
    fn set_merchant_city()-> Result<(), Box<dyn Error>>{
        let name = String::from("Jakarta");
        let name_cmp = name.clone();
        let mut nodes  = Nodes::from_str(QRIS)?;
        nodes.set_merchant_city(name);
        nodes.rewrite_crc16();
        let result_content = nodes.dumps();
        let result_parse = Nodes::from_str(&result_content)?;
        assert!(result_parse.get_merchant_city().unwrap() == name_cmp);
        Ok(())
    }
    #[test]
    fn set_postal_code()-> Result<(), Box<dyn Error>>{
        let name = String::from("87162");
        let name_cmp = name.clone();
        let mut nodes  = Nodes::from_str(QRIS)?;
        nodes.set_postal_code(name);
        nodes.rewrite_crc16();
        let result_content = nodes.dumps();
        let result_parse = Nodes::from_str(&result_content)?;
        assert!(result_parse.get_postal_code().unwrap() == name_cmp);
        Ok(())
    }
}
