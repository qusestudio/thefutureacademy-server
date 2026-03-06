use std::io::Error;
use std::sync::Arc;
use actix_web::web;

use crate::domains::learning::topics::models::topic::{Topic, TopicViewedEvent};
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use crate::domains::learning::topics::repo::topic_repo::TopicRepository;


pub struct TopicsService {
    pub repo: Arc<dyn TopicRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl TopicsService {
    pub async fn get_topic(
        &self,
        user_id: &str,
        topic_id: &str,
    ) -> Result<Topic, Error> {
        match self.repo.db_get_topic(topic_id).await {
            Ok(topic) => {
                match topic {
                    Some(topic) => {
                        // emit event here.
                        let topic_viewed = TopicViewedEvent::new(user_id, topic.title.as_str());
                        if let Err(e) = self.event_bus.send(Event::TopicViewed(topic_viewed)) {
                            log::error!("Failed to send topic.viewed event: {}", e);
                        };
                        Ok(topic)
                    }
                    None => Err(Error::other(
                        "Topic not found",
                    )),
                }
            }
            Err(error) => Err(Error::other(error.to_string())),
        }
    }
}