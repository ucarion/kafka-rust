struct Request<T> {
    size: i32,
    api_key: i16,
    api_version: i16,
    correlation_id: i32,
    client_id: String,

    body: T
}

mod metadata {
    struct MetadataRequest {
        topics: Vec<String>
    }
}

mod produce {
    use protocol::MessageSet;

    struct ProduceRequest {
        required_acks: i16,
        timeout: i32,
        messages: Vec<TopicMessages>
    }

    struct TopicMessages {
        topic: String,
        messages: Vec<PartitionMessages>
    }

    struct PartitionMessages {
        partition: i32,
        message_set_size: i32,
        message_set: MessageSet
    }
}

mod fetch {
    struct FetchRequest {
        replica_id: i32,
        max_wait_time: i32,
        min_bytes: i32,
        offsets: Vec<TopicOffsets>
    }

    struct TopicOffsets {
        topic: String,
        offsets: Vec<PartitionOffsets>
    }

    struct PartitionOffsets {
        partition: i32,
        fetch_offset: i64,
        max_bytes: i32
    }
}

mod offset {
    struct OffsetRequest {
        replica_id: i32,
        offsets: Vec<TopicOffsets>
    }

    struct TopicOffsets {
        topic: String,
        offsets: Vec<PartitionOffsets>
    }

    struct PartitionOffsets {
        partition: i32,
        time: OffsetTime,
        max_offsets: i32
    }

    enum OffsetTime {
        Latest,
        Earliest,

        // all messages made in the last N ms
        Since(i32)
    }
}

mod consumer_metadata {
    struct ConsumerMetadataRequest {

    }
}

// struct OffsetFetchRequest {
//
// }
