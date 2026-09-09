//! Small-deflection cantilever sizing, not a fatigue certification.
//! N, mm, MPa. The load acts at the centre of the finger pad, before the hook.
use std::sync::OnceLock;

pub const MODULUS: f64 = 3100.;
pub const RELEASE: f64 = 0.85;
pub const TARGET_FORCE: f64 = 4.;
pub const BRIDGE: f64 = 1.8;
pub const FINGER_OFFSET: f64 = 4.4;

#[derive(Clone, Debug)]
pub struct Spring {
    pub length: f64,
    pub width: f64,
    pub root_thickness: f64,
    pub tip_thickness: f64,
    pub force: f64,
    pub strain: f64,
}

impl Spring {
    pub fn thickness(&self, x: f64) -> f64 {
        self.root_thickness
            + (self.tip_thickness - self.root_thickness) * (x / self.length).clamp(0., 1.)
    }

    // Virtual work: displacement at y per unit force at a.
    // Integral (a-x)(y-x)/(E I(x)) dx from zero to min(a,y).
    fn compliance(&self, y: f64) -> f64 {
        let a = self.length - FINGER_OFFSET;
        let end = y.max(0.).min(a);
        let dx = end / 96.;
        (0..96)
            .map(|i| {
                let x = (i as f64 + 0.5) * dx;
                let inertia = self.width * self.thickness(x).powi(3) / 12.;
                (a - x) * (y - x) / (MODULUS * inertia) * dx
            })
            .sum()
    }

    pub fn deflection(&self, y: f64) -> f64 {
        self.force * self.compliance(y)
    }

    fn evaluate(length: f64, width: f64, root_thickness: f64, tip_thickness: f64) -> Self {
        let mut s = Self {
            length,
            width,
            root_thickness,
            tip_thickness,
            force: 0.,
            strain: 0.,
        };
        s.force = RELEASE / s.compliance(length);
        let a = length - FINGER_OFFSET;
        s.strain = (0..=96)
            .map(|i| {
                let x = a * i as f64 / 96.;
                6. * s.force * (a - x) / (MODULUS * width * s.thickness(x).powi(2))
            })
            .fold(0., f64::max);
        s
    }
}

pub fn choose(requested: [f64; 3], gap: f64) -> (Spring, [f64; 3]) {
    static CANDIDATES: OnceLock<Vec<Spring>> = OnceLock::new();
    let candidates = CANDIDATES.get_or_init(|| {
        let mut values = vec![];
        for length in 18..=32 {
            for width in (8..=18).step_by(2) {
                for layers in 8..=14 {
                    let root = layers as f64 * 0.2;
                    let tip = ((root * 0.65 / 0.2).round() * 0.2).max(1.2);
                    let s = Spring::evaluate(length as f64, width as f64, root, tip);
                    // Deliberately conservative design bounds for a prototype;
                    // not PLA fatigue limits inferred from a tensile datasheet.
                    let stop_strain = s.strain * (s.deflection(s.length + 2.8) + 0.15)
                        / s.deflection(s.length - BRIDGE - 0.6);
                    if (3.6..=4.4).contains(&s.force) && s.strain <= 0.006 && stop_strain <= 0.008 {
                        values.push(s);
                    }
                }
            }
        }
        assert!(!values.is_empty());
        values
    });
    let cavity = |s: &Spring| {
        [
            requested[0].max(s.width + 2. * (0.8 + 2. * gap) + 2. * gap + 1.6),
            requested[1].max(s.length + 3. - BRIDGE - gap),
            requested[2],
        ]
    };
    let score = |s: &Spring| {
        let inner = cavity(s);
        let extra_area = inner[0] * inner[1] - requested[0] * requested[1];
        let beam_volume = s.width * s.length * (s.root_thickness + s.tip_thickness) / 2.;
        0.02 * extra_area
            + ((s.force - TARGET_FORCE) / 0.4).powi(2)
            + 0.15 * (s.strain / 0.006).powi(2)
            + 0.2 * beam_volume / 800.
    };
    let best = candidates
        .iter()
        .min_by(|a, b| score(a).total_cmp(&score(b)))
        .unwrap()
        .clone();
    let inner = cavity(&best);
    (best, inner)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn integration_matches_uniform_beam_closed_form() {
        let s = Spring::evaluate(26., 12., 2., 2.);
        let a = s.length - FINGER_OFFSET;
        let inertia = s.width * 2_f64.powi(3) / 12.;
        let exact = RELEASE * 6. * MODULUS * inertia / (a * a * (3. * s.length - a));
        assert!((s.force / exact - 1.).abs() < 0.0001);
        assert!((s.deflection(s.length) - RELEASE).abs() < 1e-10);
    }
    #[test]
    fn sized_for_finger_force_and_strain_instead_of_box_scale() {
        for size in [
            [10.6; 3],
            [20.6, 30.6, 10.6],
            [70.6, 100.6, 30.6],
            [220.6, 240.6, 200.6],
        ] {
            for gap in [0.15, 0.3, 0.6] {
                let (s, inner) = choose(size, gap);
                assert!((3.6..=4.4).contains(&s.force));
                assert!(s.strain <= 0.006);
                assert!(inner.iter().zip(size).all(|(a, b)| *a >= b));
                let bridge_deflection = s.deflection(s.length - BRIDGE - gap);
                assert!(
                    s.strain * (s.deflection(s.length + 2.8) + 0.15) / bridge_deflection <= 0.008
                );
            }
        }
    }
}
