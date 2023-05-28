pub mod catgfx {

    use byteorder::{LittleEndian, ReadBytesExt};
    use std::io::prelude::*;
    use std::io::Cursor;
    use std::io::Read;
    use std::io::Write;

    /// Graphics Item
    pub struct GfxItem {
        pub index: u32,
        pub solid: u32,
        pub obj: u32,
        pub name: String,
        pub data: Vec<u8>,
    }

    /// Graphics Table
    pub struct GfxTable {
        pub items: Vec<GfxItem>,
    }

    /// implementation of GfxItem
    impl GfxItem {
        /// create new item function
        pub fn new(i: u32, s: u32, o: u32, n: &str, d: Vec<u8>) -> Self {
            GfxItem {
                index: i,
                solid: s,
                obj: o,
                name: n.to_string(),
                data: d,
            }
        }
    }

    /// implemtnation of GfxTable
    impl GfxTable {
        /// create new table
        pub fn new() -> Self {
            GfxTable { items: Vec::new() }
        }
        /// add Item to table
        pub fn add_item(&mut self, item: GfxItem) {
            self.items.push(item);
        }
        /// clear table
        pub fn clear(&mut self) {
            self.items.clear();
        }
    }
    /// concat graphics
    pub fn cat_gfx(input: &str, cfg_file: &str) -> std::io::Result<()> {
        println!("catgfx: concat {} with {}", input, cfg_file);
        let f = std::fs::File::open(cfg_file)?;
        let r = std::io::BufReader::new(f);
        let mut out_f = std::fs::File::create(input)?;
        let header: u32 = 0x421;
        out_f.write_all(&header.to_le_bytes())?;
        let mut cur_index: u32 = 0;
        for input in r.lines() {
            match input {
                Ok(input_str) => {
                    let pos = input_str.find('#');
                    if pos == None {
                        let mut values: Vec<u32> = Vec::new();
                        let pos = input_str.find(':').unwrap();
                        let right = &input_str[pos + 1..input_str.len()];
                        let left = &input_str[0..pos];
                        values.push(left.parse().unwrap());
                        let pos = right.find(':').unwrap();
                        let left = &right[0..pos];
                        values.push(left.parse().unwrap());
                        let right = &right[pos + 1..right.len()];

                        let rpos = right.rfind('/');
                        let filename = if rpos != None {
                            let rpos = rpos.unwrap();
                            &right[rpos + 1..]
                        } else {
                            right
                        };
                        let name_len = filename.len() as u32;
                        out_f.write_all(&name_len.to_le_bytes())?;
                        out_f.write_all(filename.as_bytes())?;
                        out_f.write_all(&cur_index.to_le_bytes())?;
                        out_f.write_all(&values[0].to_le_bytes())?;
                        out_f.write_all(&values[1].to_le_bytes())?;
                        let mut file_value = std::fs::File::open(&right)?;
                        let mut buf: Vec<u8> = Vec::new();
                        file_value.read_to_end(&mut buf)?;
                        let file_size = buf.len() as u32;
                        out_f.write_all(&file_size.to_le_bytes())?;
                        out_f.write_all(buf.as_slice())?;
                        println!(
                            "Wrote: {} -> {}:{} [{}/{}]",
                            right, values[0], values[1], cur_index, file_size
                        );
                    } else {
                        let value = &input_str[1..input_str.len()];
                        cur_index = value.parse().unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Ok(())
    }
    /// build gfx table
    pub fn build_gfx(input: &str, table: &mut GfxTable) -> std::io::Result<()> {
        let mut f = std::fs::File::open(input)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;
        let dlen = data.len() as u64;
        let mut reader = Cursor::new(data);
        let header = reader.read_u32::<LittleEndian>()?;
        if header != 0x421 {
            panic!("Error invalid file type");
        }
        while reader.position() < dlen {
            let len: u32 = reader.read_u32::<LittleEndian>()?;
            if len > 0 {
                let mut index = 0;
                let mut s = String::new();
                while index < len {
                    let b = reader.read_u8()?;
                    s.push(b as char);
                    index += 1;
                }
                let file_index: u32 = reader.read_u32::<LittleEndian>()?;
                let file_solid: u32 = reader.read_u32::<LittleEndian>()?;
                let file_obj: u32 = reader.read_u32::<LittleEndian>()?;
                let file_len: u32 = reader.read_u32::<LittleEndian>()?;
                println!(
                    "{} [ index: {} solid: {} obj: {} len: {} ]",
                    s, file_index, file_solid, file_obj, file_len
                );
                let mut index: i64 = 0;
                let mut out_buffer: Vec<u8> = Vec::new();
                while index < file_len as i64 {
                    let b = reader.read_u8()?;
                    index += 1;
                    out_buffer.push(b);
                }
                table.add_item(GfxItem::new(
                    file_index, file_solid, file_obj, &s, out_buffer,
                ));
            }
        }
        Ok(())
    }

    /// extract graphics
    pub fn extract_gfx(input: &str, output_dir: &str) -> std::io::Result<()> {
        println!("catgfx: extract {} to {}", input, output_dir);
        let mut f = std::fs::File::open(input)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;
        let dlen = data.len() as u64;
        let mut reader = Cursor::new(data);
        let header = reader.read_u32::<LittleEndian>()?;
        if header != 0x421 {
            panic!("Error invalid file type");
        }
        while reader.position() < dlen {
            let len: u32 = reader.read_u32::<LittleEndian>()?;
            if len > 0 {
                let mut index = 0;
                let mut s = String::new();
                while index < len {
                    let b = reader.read_u8()?;
                    s.push(b as char);
                    index += 1;
                }
                let file_index: u32 = reader.read_u32::<LittleEndian>()?;
                let file_solid: u32 = reader.read_u32::<LittleEndian>()?;
                let file_obj: u32 = reader.read_u32::<LittleEndian>()?;
                let file_len: u32 = reader.read_u32::<LittleEndian>()?;
                println!(
                    "{} [ index: {} solid: {} obj: {} len: {} ]",
                    s, file_index, file_solid, file_obj, file_len
                );
                let mut index: i64 = 0;
                let _ = std::fs::create_dir(&output_dir);
                let path = format!("{}/{}", output_dir, s);
                let mut out_file = std::fs::File::create(path)?;
                let mut out_buffer: Vec<u8> = Vec::new();
                while index < file_len as i64 {
                    let b = reader.read_u8()?;
                    index += 1;
                    out_buffer.push(b);
                }
                out_file.write_all(out_buffer.as_slice())?;
            }
        }
        Ok(())
    }

    /// list graphics in file
    pub fn list_gfx(input: &str) -> std::io::Result<()> {
        println!("catgfx: list {}", input);
        let mut f = std::fs::File::open(input)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;
        let dlen = data.len() as u64;
        let mut reader = Cursor::new(data);
        let header = reader.read_u32::<LittleEndian>()?;
        if header != 0x421 {
            panic!("Error invalid file type");
        }
        while reader.position() < dlen {
            let len: u32 = reader.read_u32::<LittleEndian>()?;
            if len > 0 {
                let mut index = 0;
                let mut s = String::new();
                while index < len {
                    let b = reader.read_u8()?;
                    s.push(b as char);
                    index += 1;
                }
                let file_index: u32 = reader.read_u32::<LittleEndian>()?;
                let file_solid: u32 = reader.read_u32::<LittleEndian>()?;
                let file_obj: u32 = reader.read_u32::<LittleEndian>()?;
                let file_len: u32 = reader.read_u32::<LittleEndian>()?;
                println!(
                    "{} [ index: {} solid: {} obj: {} len: {} ]",
                    s, file_index, file_solid, file_obj, file_len
                );
                reader.seek(std::io::SeekFrom::Current(file_len as i64))?;
            }
        }
        Ok(())
    }
}
