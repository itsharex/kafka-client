use std::{collections::HashMap, io::{BufRead, Cursor}};
use byteorder::{BigEndian, ReadBytesExt};
use rdkafka::{Offset, TopicPartitionList};
use std::str;

pub fn read_str<'a>(rdr: &'a mut Cursor<&[u8]>) -> Result<&'a str, Box<dyn std::error::Error>> {
    let len = (rdr.read_i16::<BigEndian>())? as usize;
    let pos = rdr.position() as usize;
    let slice = str::from_utf8(&rdr.get_ref()[pos..(pos + len)])?;
    rdr.consume(len);
    Ok(slice)
}
pub type TopicOffsetsMap = HashMap<String, Vec<(i32, i64)>>;
pub fn from_topic_partition_list_to_map(tpl: TopicPartitionList) -> TopicOffsetsMap {
    let entry = tpl.to_topic_map().into_iter()
        .filter_map(|((top, part), offset)| match offset {
            Offset::Offset(val) => Some((top, part, val)),
            _ => None
        })
        .collect::<Vec<(String, i32, i64)>>();

    let mut out: HashMap<String, Vec<(i32, i64)>> = HashMap::new();
    for (topic, partition, offset) in entry {
        if out.contains_key(topic.as_str()) {
            out.get_mut(topic.as_str()).unwrap().push((partition, offset));
        } else {
            out.insert(topic.to_string(), vec![(partition, offset)]);
        }
    }
    out
}
