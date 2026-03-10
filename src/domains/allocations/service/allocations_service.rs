use std::io::Error;
use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::allocations::models::allocation::{Allocation, AllocationNew, InstructorAllocatedEvent, TeachingAllocation};
use crate::domains::allocations::repo::allocation_repo::AllocationRepository;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};

pub struct AllocationsService {
    pub repo: Arc<dyn AllocationRepository + Send + Sync>,
    pub event_bus: Data<EventBus>,
}

impl AllocationsService {
    pub async fn allocate_instructor(&self, new_allocation: AllocationNew) -> Result<Allocation, Error> {
        match self.repo.db_set_allocation(new_allocation.clone()).await {
            Ok(allocation) => {
                match allocation {
                    Some(allocation) => {
                        let instructor_allocated = InstructorAllocatedEvent::new(new_allocation.instructor_id.as_str());
                        let event = Event::InstructorAllocated(instructor_allocated);
                        if let Err(e) = self.event_bus.send(event) {
                            log::error!("Failed to send instructor.allocated event: {}", e);
                        };
                        Ok(allocation)
                    }
                    None => Err(Error::other("Allocation failed or not returned.")),
                }
            }
            Err(error) => Err(Error::other(error.to_string()))
        }
    }

    pub async fn get_all_allocations(&self) -> Result<Vec<Allocation>, Error> {
        match self.repo.db_get_all_allocations().await {
            Ok(allocations) => {
                log::info!("Retrieving all allocations...");
                Ok(allocations)
            },
            Err(error) => {
                log::error!("Failed to retrieve all allocations... {}", error.to_string());
                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_instructor_allocations(&self, instructor_id: &str) -> Result<Vec<TeachingAllocation>, Error> {
        match self.repo.db_get_instructor_allocations(instructor_id.to_string()).await {
            Ok(allocations) => {
                log::info!("Retrieving instructor allocations...");
               Ok(allocations)
            }
            Err(e) => {
                Err(Error::other(format!("Failed to retrieve instructor allocations: {}", e)))
            }
        }
    }
}