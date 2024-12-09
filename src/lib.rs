pub mod utils;
pub mod node;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::node::Nodes;


    #[test]
    fn verify_crc16_and_dumps_qris() -> Result<(), Box<dyn Error>>{
        let content = String::from("00020101021126710019ID.CO.CIMBNIAGA.WWW011878728356757817222102150002186871651250303UMI51450015ID.OR.QRNPG.WWW0215ID81275673266770303UMI5204599953033605802ID5914AABBCCD*6714516006KEDIRI61054423462120708123456786304097D");
        let d  = Nodes::from_str(&content)?;
        assert!(d.verify());
        assert!(d.dumps() == content);
        Ok(())
    }
}
