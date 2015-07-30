// metadata([topic_name])
// produce(acks, timeout, [(topic_name, [(partition, message_set_size, message_set)])])
// fetch(replica_id, max_wait_time, min_bytes, [(topic_name, [(partition, fetch_offset, max_bytes)])])
// offset(replica_id, [(topic_name, [(partition, time, max_number_offsets)])]
//
// Get/set offsets for consumer:
//   consumer_metadata(consumer_group: String)
//   commit_offset
//   fetch_offset(consumer_group, [(topic_name, [partition])])
//
// Design: client deals with those awful structs, Producer and Consumer provide
// nicer APIs on top of that.

pub mod protocol;

// let client = kafka::Client::new(vec!["localhost:9091"]);
//
// client.send_metadata_all(); // alias for send_metadata(&[])
// client.send_metadata(&["topic1", "topic2"]);
//
// let message = ("my-topic", 0, b"My awesome message!")
// client.produce(1, 100, vec![message])
