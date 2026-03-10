use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::domains::allocations::models::allocation::InstructorAllocatedEvent;
use crate::domains::billing::checkouts::models::checkouts_events::CheckoutCompletedEvent;
use crate::domains::billing::payments::models::payment_events::PaymentCompletedEvent;
use crate::domains::enrollments::models::enrollment::StudentEnrolledEvent;
use crate::domains::learning::lessons::models::lesson::{LessonCreatedEvent, LessonOpenedEvent};
use crate::domains::learning::subjects::models::subject::SubjectViewedEvent;
use crate::domains::learning::topics::models::topic::TopicViewedEvent;
use crate::infrastructure::channel::events_channel_checker::EventMessage;
use crate::domains::users::instructors::models::instructor::InstructorRegisteredEvent;
use crate::domains::users::instructors::models::instructor_profile::InstructorProfileCreatedEvent;
use crate::domains::users::students::models::student::StudentRegisteredEvent;
use crate::domains::users::students::models::student_profile::StudentProfileCreatedEvent;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    HealthCheck(EventMessage),
    // Users & Profiles
    StudentRegistered(StudentRegisteredEvent),
    StudentProfileCreated(StudentProfileCreatedEvent),
    InstructorRegistered(InstructorRegisteredEvent),
    InstructorProfileCreated(InstructorProfileCreatedEvent),
    // Classroom
    StudentEnrolled(StudentEnrolledEvent),
    InstructorAllocated(InstructorAllocatedEvent),
    // Learning
    SubjectViewed(SubjectViewedEvent),
    TopicViewed(TopicViewedEvent),
    LessonCreated(LessonCreatedEvent),
    LessonOpened(LessonOpenedEvent),
    // Payments
    PaymentCompleted(PaymentCompletedEvent),
    PaymentFailed,
    // Subscriptions
    SubscriptionActivated,
    SubscriptionExpired,
    // Checkout Events
    CheckoutCompleted(CheckoutCompletedEvent)
}

impl Event {
    pub fn to_string(&self) -> String {
        match self {
            Event::HealthCheck(_) => {
                String::from("health.check")
            }
            // Users and profiles
            Event::StudentRegistered(_) => {
                String::from("student.registered")
            }
            Event::StudentProfileCreated(_) => {
                String::from("student_profile.created")
            }
            Event::InstructorRegistered(_) => {
                String::from("instructor.registered")
            }
            Event::InstructorProfileCreated(_) => {
                String::from("instructor_profile.created")
            }
            // Classroom
            Event::StudentEnrolled(_) => {
                String::from("student.enrolled")
            }
            Event::InstructorAllocated(_) => {
                String::from("instructor.allocated")
            }
            // Learning
            Event::SubjectViewed(_) => {
                String::from("subject.viewed")
            }
            Event::TopicViewed(_) => {
                String::from("topic.viewed")
            }
            Event::LessonCreated(_) => {
                String::from("lesson.created")
            }
            Event::LessonOpened(_) => {
                String::from("lesson.opened")
            }
            
            // billing
            Event::PaymentCompleted(_) => {
                String::from("payment.completed")
            }
            Event::PaymentFailed => {
                String::from("payment.failed")
            }
            Event::SubscriptionActivated => {
                String::from("subscription.activated")
            }
            Event::SubscriptionExpired => {
                String::from("subscription.expired")
            }
            Event::CheckoutCompleted(_) => {
                String::from("checkout.completed")
            }
        }
    }
}
 
pub fn init_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}
