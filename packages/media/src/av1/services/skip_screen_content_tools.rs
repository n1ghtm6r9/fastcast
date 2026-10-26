use crate::avc::BitReader;

pub fn skip_screen_content_tools(reader: &mut BitReader) -> Option<()> {
    let seq_choose_screen_content_tools = reader.read_bit()? == 1;
    let screen_content_tools_used = seq_choose_screen_content_tools || reader.read_bit()? == 1;

    if screen_content_tools_used {
        let seq_choose_integer_mv = reader.read_bit()? == 1;

        if !seq_choose_integer_mv {
            let _seq_force_integer_mv = reader.read_bit()?;
        }
    }

    Some(())
}
