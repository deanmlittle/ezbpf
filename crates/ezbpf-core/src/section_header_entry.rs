use std::{fmt::Debug, io::Cursor};

use serde::{ser::Error, Deserialize, Serialize, Serializer};
use serde_json::{error, Value};

use crate::{cursor::ELFCursor, errors::EZBpfError, instructions::Ix};

#[derive(Debug, Clone, Deserialize)]
pub struct SectionHeaderEntry {
    pub label: String,
    pub offset: usize,
    pub data: Vec<u8>,
}

impl SectionHeaderEntry {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn to_ixs(&self) -> Result<Vec<Ix>, EZBpfError> {
        if self.data.len() % 8 != 0 {
            return Err(EZBpfError::InvalidDataLength);
        }
        let mut ixs: Vec<Ix> = vec![];
        if self.data.len() >= 8 {
            let mut c = Cursor::new(self.data.as_slice());
            while let Ok(ix) = c.read_ix() {
                ixs.push(ix)
            }
        }
        Ok(ixs)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Serialize for SectionHeaderEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut json_value = serde_json::Map::new();

        json_value.insert("label".to_string(), Value::String(self.label.clone()));
        json_value.insert("offset".to_string(), Value::Number(self.offset.into()));

        if self.label.as_str() == ".text\0" {
            json_value.insert(
                "ixs".to_string(),
                Value::Array(
                    self.to_ixs()
                        .map_err(|_| error::Error::custom("Invalid IX"))
                        .unwrap()
                        .into_iter()
                        .map(|ix| {
                            Value::String(
                                ix.to_asm()
                                    .map_err(|_| error::Error::custom("Invalid IX"))
                                    .unwrap(),
                            )
                        })
                        .collect::<Vec<Value>>(),
                ),
            );
        };

        if let Ok(utf8) = String::from_utf8(self.data.clone()) {
            json_value.insert("utf8".to_string(), Value::String(utf8));
        }

        json_value.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::{instructions::Ix, opcodes::OpCode, section_header_entry::SectionHeaderEntry};

    #[test]
    fn serialize_e2e() {
        let data = vec![
            0x18, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let h = SectionHeaderEntry {
            label: ".text\0".to_string(),
            offset: 128,
            data: data.clone(),
        };

        let ixs = vec![
            Ix {
                op: OpCode::Lddw,
                dst: 1,
                src: 0,
                off: 0,
                imm: 0,
            },
            Ix {
                op: OpCode::Exit,
                dst: 0,
                src: 0,
                off: 0,
                imm: 0,
            },
        ];
        assert_eq!(ixs, h.to_ixs().unwrap());

        assert_eq!(
            data,
            h.to_ixs()
                .expect("Invalid IX")
                .into_iter()
                .flat_map(|i| i.to_bytes())
                .collect::<Vec<u8>>()
        )
    }
}
