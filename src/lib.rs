type Floor = i32;
type Person = i32;

struct Elevator {
    position: Floor,
    floors_served: Vec<Floor>, 
    in_service: bool
}

struct Building {
    elevators: Vec<Elevator>,
    passengers: Vec<Passenger>
}

struct Passenger {
    uuid: Person,
    location: Floor,
    destination: Floor
}

impl Building {
    /// Creates a new building with 0 elevators and 0 passengers.
    fn new() -> Building {
        let elevators: Vec<Elevator> = Vec::new();
        let passengers: Vec<Passenger> = Vec::new();
        
        Building {elevators, passengers}
    }

    /// Adds new elevators to the building.
    fn add_elevator(mut building: Building, elevator: Elevator) -> Building {
        building.elevators.push(elevator);
        building
    }

    /// Adds new passengers to the building.
    fn add_passenger(mut building: Building, passenger: Passenger) -> Building {
        building.passengers.push(passenger);
        building
    }
}

impl Elevator {
    /// Creates a new, inactive elevator at the default floor
    /// for the building.
    fn new(floors_served: Vec<Floor>) -> Elevator {
        Elevator { position: 0, 
                   floors_served: floors_served,
                   in_service: false }
    }
    
    // Puts the elevator in service.
    fn activate(mut elevator: Elevator) -> Elevator {
        elevator.in_service = true;
        elevator
    }

    // Removes the elevator from service.
    fn deactivate(mut elevator: Elevator) -> Elevator {
        elevator.in_service = false;
        elevator
    }

}
