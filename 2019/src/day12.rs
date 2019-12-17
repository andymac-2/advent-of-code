fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / (gcd(a, b))
}

#[must_use]
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32, i32);
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[derive(Copy, Clone, Debug)]
struct Planet {
    s: Point,
    v: Point,
}
impl Planet {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Planet {
            s: Point(x, y, z),
            v: Point(0, 0, 0),
        }
    }
    fn get_acceleration(&self, others: &[Planet]) -> Point {
        let mut accel = Point(0, 0, 0);
        for other in others {
            accel.0 += (other.s.0 - self.s.0).signum();
            accel.1 += (other.s.1 - self.s.1).signum();
            accel.2 += (other.s.2 - self.s.2).signum();
        }
        accel
    }
    fn apply_acceleration(&mut self, accel: Point) {
        self.v += accel;
        self.s += self.v;
    }
    fn get_energy(&self) -> i32 {
        let potential = self.s.0.abs() + self.s.1.abs() + self.s.2.abs();
        let kinetic = self.v.0.abs() + self.v.1.abs() + self.v.2.abs();
        potential * kinetic
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PlanetD {
    s: i32,
    v: i32,
}
impl PlanetD {
    fn new(s: i32) -> Self {
        PlanetD { s, v: 0 }
    }
    fn get_acceleration(self, others: &[PlanetD]) -> i32 {
        let mut accel = 0;
        for other in others {
            accel += (other.s - self.s).signum();
        }
        accel
    }
    fn apply_acceleration(&mut self, accel: i32) {
        self.v += accel;
        self.s += self.v;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlanetDGroup([PlanetD; 4]);
impl PlanetDGroup {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        PlanetDGroup([
            PlanetD::new(a),
            PlanetD::new(b),
            PlanetD::new(c),
            PlanetD::new(d),
        ])
    }
    fn cycle_len(&self) -> u64 {
        let mut cycle = 0;
        let mut current = *self;

        loop {
            let accelerations = [
                current.0[0].get_acceleration(&current.0),
                current.0[1].get_acceleration(&current.0),
                current.0[2].get_acceleration(&current.0),
                current.0[3].get_acceleration(&current.0),
            ];

            for (i, accel) in accelerations.iter().enumerate() {
                current.0[i].apply_acceleration(*accel);
            }

            cycle += 1;

            if self == &current {
                return cycle;
            }
        }
    }
}

pub fn part1() -> i32 {
    let mut planets = [
        Planet::new(1, -4, 3),
        Planet::new(-14, 9, -4),
        Planet::new(-4, -6, 7),
        Planet::new(6, -9, -11),
    ];

    for _ in 0..1000 {
        let accelerations: Vec<_> = planets
            .iter()
            .map(|planet| planet.get_acceleration(&planets))
            .collect();

        for (i, accel) in accelerations.iter().enumerate() {
            planets[i].apply_acceleration(*accel);
        }
    }

    planets.iter().map(|planet| planet.get_energy()).sum()
}

// short example
// <x=-1, y=0, z=2>
// <x=2, y=-10, z=-7>
// <x=4, y=-8, z=8>
// <x=3, y=5, z=-1>

// puzzle input
// <x=1, y=-4, z=3>
// <x=-14, y=9, z=-4>
// <x=-4, y=-6, z=7>
// <x=6, y=-9, z=-11>

pub fn part2() -> u64 {
    let cycle_x = PlanetDGroup::new(1, -14, -4, 6).cycle_len();
    let cycle_y = PlanetDGroup::new(-4, 9, -6, -9).cycle_len();
    let cycle_z = PlanetDGroup::new(3, -4, 7, -11).cycle_len();

    lcm(cycle_x, lcm(cycle_y, cycle_z))
}

pub fn start() {
    println!("Program Output: {:?}", part2());
}
