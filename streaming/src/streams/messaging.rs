use crate::error::Error;
use crate::message::Message;
use crate::streams::stream::Stream;

impl Stream {
    pub fn get_messages(
        &mut self,
        topic_id: u32,
        partition_id: u32,
        offset: u64,
        count: u32,
    ) -> Result<Vec<&Message>, Error> {
        let topic = self.topics.get_mut(&topic_id);
        if topic.is_none() {
            return Err(Error::TopicNotFound(topic_id));
        }

        topic.unwrap().get_messages(partition_id, offset, count)
    }

    pub async fn append_messages(
        &mut self,
        topic_id: u32,
        partition_id: u32,
        message: Message,
    ) -> Result<(), Error> {
        let topic = self.topics.get_mut(&topic_id);
        if topic.is_none() {
            return Err(Error::TopicNotFound(topic_id));
        }

        let topic = topic.unwrap();
        topic.append_messages(partition_id, message).await?;
        Ok(())
    }
}