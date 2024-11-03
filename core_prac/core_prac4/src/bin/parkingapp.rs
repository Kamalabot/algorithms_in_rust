fn main() {
    println!("Parking app");
    let spt = SpotType::Car;
}

#[derive(PartialEq, Clone, Debug)]
enum SpotType {
    MotorCycle,
    Car,
    Large,
}

#[derive(Debug, Clone)]
struct ParkingSpot {
    spot_type: SpotType,
    is_taken: bool,
}

impl ParkingSpot {
    fn new(spot_type: SpotType) -> Self {
        Self {
            spot_type,
            is_taken: false,
        }
    }

    fn park(&mut self) -> bool {
        if !self.is_taken {
            self.is_taken = true;
            true
        } else {
            false
        }
    }

    fn leave(&mut self) {
        self.is_taken = false;
    }
}

enum Vehicle {
    MotorCycle,
    Car,
    Van,
}

struct ParkingLot {
    mc_spots: Vec<ParkingSpot>,
    car_spots: Vec<ParkingSpot>,
    large_spots: Vec<ParkingSpot>,
}

impl ParkingLot {
    fn new(mc_spots: usize, car_spots: usize, large_spots: usize) -> Self {
        let mc_spots = vec![ParkingSpot::new(SpotType::MotorCycle); mc_spots];
        let car_spots = vec![ParkingSpot::new(SpotType::Car); car_spots];
        let large_spots = vec![ParkingSpot::new(SpotType::Large); large_spots];
        Self {
            mc_spots,
            car_spots,
            large_spots,
        }
    }

    fn remaining_spots(&self) -> usize {
        self.mc_spots.iter().filter(|s| s.is_taken).count()
            + self.car_spots.iter().filter(|s| s.is_taken).count()
            + self.large_spots.iter().filter(|s| s.is_taken).count()
    }

    fn is_full(&self) -> bool {
        self.mc_spots.iter().all(|s| s.is_taken)
            && self.car_spots.iter().all(|s| s.is_taken)
            && self.large_spots.iter().all(|s| s.is_taken)
    }
    fn is_empty(&self) -> bool {
        self.mc_spots.iter().all(|s| !s.is_taken)
            && self.car_spots.iter().all(|s| !s.is_taken)
            && self.large_spots.iter().all(|s| !s.is_taken)
    }

    fn park_in_spot(&mut self, spots: &mut Vec<ParkingSpot>) -> bool {
        for spot in spots.iter_mut() {
            if spot.park() {
                return true;
            }
        }
        false
    }
    fn park_vehicle(&mut self, vehicle: Vehicle) -> bool {
        match vehicle {
            Vehicle::MotorCycle => {
                self.park_in_spot(&mut self.mc_spots)
                    || self.park_in_spot(&mut self.car_spots)
                    || self.park_in_spot(&mut self.large_spots)
            }
            Vehicle::Car => {
                self.park_in_spot(&mut self.car_spots) || self.park_in_spot(&mut self.large_spots)
            }
            Vehicle::Van => self.park_in_spot(&mut self.large_spots),
        }
    }
}
