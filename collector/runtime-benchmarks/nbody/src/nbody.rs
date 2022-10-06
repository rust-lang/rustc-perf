type Number = f64;

const G: Number = 6.67e-11;
const TIMESTEP: Number = 0.25;

struct Position {
    x: Number,
    y: Number,
    z: Number,
}
struct Velocity {
    dx: Number,
    dy: Number,
    dz: Number,
}
struct Force {
    fx: Number,
    fy: Number,
    fz: Number,
}
struct Acceleration {
    ax: Number,
    ay: Number,
    az: Number,
}

/// The main structure of this program.
/// The state for a body is split across
/// a position vector,
/// a velocities vector,
/// and a mass vector.
///
/// Originally, this was implemented as an array of structures—now
/// it's a structure of arrays.
/// This was both for testing optimizations and for minute practice with EC(S).
pub struct BodyStates {
    poss: Vec<Position>,
    vels: Vec<Velocity>,
    masses: Vec<Number>,
}

fn dist_squared(dx: Number, dy: Number, dz: Number) -> Number {
    (dx * dx) + (dy * dy) + (dz * dz)
}

fn force_d(mass1: Number, mass2: Number, distance_squared: Number) -> Number {
    (G * mass1 * mass2) / distance_squared
}

/// Given the position and mass of one body,
/// calculate the force acting on it from all of the other bodies.
fn forces_for_body<'a, I>(p: &Position, m: Number, reference: I) -> Force
where
    I: IntoIterator<Item = (&'a Position, &'a Number)>,
{
    reference
        .into_iter()
        .map(|(ref otherpos, &othermass)| {
            let dx = p.x - otherpos.x;
            let dy = p.y - otherpos.y;
            let dz = p.z - otherpos.z;

            let d = dist_squared(dx, dy, dz);
            let f = force_d(m, othermass, d);

            Force {
                fx: (f * dx) / d,
                fy: (f * dy) / d,
                fz: (f * dz) / d,
            }
        })
        .reduce(|acc: Force, f: Force| Force {
            fx: acc.fx + f.fx,
            fy: acc.fy + f.fy,
            fz: acc.fz + f.fz,
        })
        .unwrap()
}

/// Calculate the accelerations for these bodies—used to update the bodies' velocities.
fn accelerations(bs: &BodyStates) -> Vec<Acceleration> {
    bs.poss
        .iter()
        .zip(bs.masses.iter())
        .map(|(ref p, &m)| {
            let reference = bs.poss.iter().zip(bs.masses.iter());

            // calculate forces over all other things
            let Force { fx, fy, fz } = forces_for_body(p, m, reference);
            Acceleration {
                ax: fx / m,
                ay: fy / m,
                az: fz / m,
            }
        })
        .collect()
}

/// Returns a new `Position` from a `Position` moving at a certain `Velocity`.
fn move_position(p: &Position, v: &Velocity) -> Position {
    Position {
        x: p.x + v.dx * TIMESTEP,
        y: p.y + v.dy * TIMESTEP,
        z: p.z + v.dz * TIMESTEP,
    }
}

/// Returns a new `Velocity` from a `Velocity` accelerating at a certain `Acceleration`.
fn update_velocity(v: &Velocity, a: &Acceleration) -> Velocity {
    Velocity {
        dx: v.dx + a.ax * TIMESTEP,
        dy: v.dy + a.ay * TIMESTEP,
        dz: v.dz + a.az * TIMESTEP,
    }
}

/// Computes the next `BodyStates`.
pub fn compute_forces(bs: BodyStates) -> BodyStates {
    let accs = accelerations(&bs);
    BodyStates {
        poss: bs
            .poss
            .iter()
            .zip(bs.vels.iter())
            .map(|(p, v)| move_position(p, v))
            .collect(),
        vels: bs
            .vels
            .iter()
            .zip(accs.iter())
            .map(|(v, a)| update_velocity(v, a))
            .collect(),
        masses: bs.masses,
    }
}

/// Simple function to create a lot of bodies.
/// Thank you, Larkins, for letting me use these umbers.
pub fn init(count: usize) -> BodyStates {
    let range: Vec<Number> = (0..count).map(|i| i as Number).collect();
    let ret = BodyStates {
        poss: range
            .iter()
            .map(|i| Position {
                x: 100. * (*i * 0.1),
                y: 200. * (*i * 0.1),
                z: 300. * (*i * 0.1),
            })
            .collect(),
        vels: range
            .iter()
            .map(|i| Velocity {
                dx: 400. + *i,
                dy: 500. + *i,
                dz: 600. + *i,
            })
            .collect(),
        masses: range.iter().map(|i| 10e6 * (*i + 100.2)).collect(),
    };
    ret
}
