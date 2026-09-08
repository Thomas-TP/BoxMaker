use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub service: String,
    pub category: String,
    pub cents: u32,
    pub counter_cents: u32,
    pub tracking: bool,
    pub delay: String,
}

/// Millimetres, grams, integer centimes. Rates verified 2026-09-07.
/// See docs/POSTAL.md for scope, sources and excluded optional surcharges.
pub fn quotes(mut dims: [f64; 3], weight: f64, online: bool) -> Vec<Quote> {
    if !weight.is_finite() || weight <= 0. || dims.iter().any(|v| !v.is_finite() || *v <= 0.) {
        return vec![];
    }
    dims.sort_by(|a, b| b.total_cmp(a));
    let [l, w, h] = dims;
    let weight = weight.ceil();
    let mut out = vec![];
    let mut add = |service: &str,
                   category: &str,
                   cents: u32,
                   counter_cents: u32,
                   tracking: bool,
                   delay: &str| {
        out.push(Quote {
            service: service.into(),
            category: category.into(),
            cents,
            counter_cents,
            tracking,
            delay: delay.into(),
        });
    };
    let min_letter = l >= 140. && w >= 90.;
    if min_letter && l <= 250. && w <= 176. && h <= 50. && weight <= 500. {
        let extra = if h > 20. { 200 } else { 0 };
        let (b, a, category) = if weight <= 100. {
            (100, 120, "Lettre standard B5")
        } else {
            (140, 170, "Midilettre B5")
        };
        add(
            "Courrier B",
            category,
            b + extra,
            b + extra,
            false,
            "Jusqu’à 3 jours ouvrables",
        );
        add(
            "Courrier A",
            category,
            a + extra,
            a + extra,
            false,
            "Jour ouvrable suivant",
        );
        add(
            "Courrier A Plus",
            "B5 · prêt à l’envoi",
            290 + extra,
            390 + extra,
            true,
            "Jour ouvrable suivant",
        );
        add(
            "Recommandé",
            "B5 · prêt à l’envoi",
            580,
            680,
            true,
            "Contre signature",
        );
    }
    if min_letter && l <= 353. && w <= 250. && h <= 20. && weight <= 1000. {
        add(
            "Courrier B",
            "Grande lettre B4",
            200,
            200,
            false,
            "Jusqu’à 3 jours ouvrables",
        );
        add(
            "Courrier A",
            "Grande lettre B4",
            250,
            250,
            false,
            "Jour ouvrable suivant",
        );
        add(
            "Courrier A Plus",
            "B4 · prêt à l’envoi",
            470,
            570,
            true,
            "Jour ouvrable suivant",
        );
        add(
            "Recommandé",
            "B4 · prêt à l’envoi",
            580,
            680,
            true,
            "Contre signature",
        );
    }
    let standard = l <= 1000. && w <= 600. && h <= 600. && weight <= 30000.;
    let bulky = !standard
        && l + 2. * w + 2. * h <= 4000.
        && ((l <= 2000. && weight <= 30000.) || (l <= 2500. && weight <= 10000.));
    if standard || bulky {
        let (eco, prio, express, category) = if bulky {
            (3100, 3250, 3800, "Encombrant")
        } else if weight <= 2000. {
            (900, 1050, 1700, "Colis ≤ 2 kg")
        } else if weight <= 10000. {
            (1200, 1350, 2300, "Colis ≤ 10 kg")
        } else {
            (2100, 2250, 2900, "Colis ≤ 30 kg")
        };
        let discount = if online { 150 } else { 0 };
        add(
            "PostPac Economy",
            category,
            eco - discount,
            eco,
            true,
            "2 jours ouvrables",
        );
        add(
            "PostPac Priority",
            category,
            prio - discount,
            prio,
            true,
            "Jour ouvrable suivant",
        );
        add(
            "Swiss-Express Lune",
            category,
            express,
            express,
            true,
            "Avant 9 h le jour suivant",
        );
    }
    out.sort_by_key(|q| q.cents);
    // Keep the cheapest eligible format per service, e.g. a thin B5 is also B4 eligible.
    let mut services = std::collections::HashSet::new();
    out.retain(|q| services.insert(q.service.clone()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn price(d: [f64; 3], w: f64, s: &str) -> Option<u32> {
        quotes(d, w, true)
            .iter()
            .find(|q| q.service == s)
            .map(|q| q.cents)
    }
    #[test]
    fn weight_boundaries() {
        for (g, expected) in [
            (100., 100),
            (100.01, 140),
            (500., 140),
            (500.01, 200),
            (1000., 200),
        ] {
            assert_eq!(price([250., 176., 20.], g, "Courrier B"), Some(expected));
        }
        for (g, expected) in [
            (2000., 750),
            (2000.01, 1050),
            (10000., 1050),
            (10000.01, 1950),
            (30000., 1950),
        ] {
            assert_eq!(
                price([400., 300., 100.], g, "PostPac Economy"),
                Some(expected)
            );
        }
        assert_eq!(price([400., 300., 100.], 30000.01, "PostPac Economy"), None);
    }
    #[test]
    fn dimensions_and_rotation() {
        assert_eq!(price([20., 176., 250.], 100., "Courrier B"), Some(100));
        assert_eq!(price([250., 176., 20.01], 100., "Courrier B"), Some(300));
        assert_eq!(price([250., 176., 50.], 500., "Courrier B"), Some(340));
        assert_eq!(price([250., 176., 50.01], 100., "Courrier B"), None);
        assert_eq!(price([353., 250., 20.01], 100., "Courrier B"), None);
        assert_eq!(price([139., 90., 10.], 100., "Courrier B"), None);
        assert_eq!(
            price([1000., 600., 600.], 2000., "PostPac Economy"),
            Some(750)
        );
        assert_eq!(
            price([1000.01, 600., 600.], 2000., "PostPac Economy"),
            Some(2950)
        );
    }
    #[test]
    fn bulky_boundaries() {
        assert_eq!(
            price([2000., 500., 500.], 30000., "PostPac Economy"),
            Some(2950)
        );
        assert_eq!(
            price([2000.01, 500., 500.], 30000., "PostPac Economy"),
            None
        );
        assert_eq!(
            price([2500., 375., 375.], 10000., "PostPac Economy"),
            Some(2950)
        );
        assert_eq!(
            price([2500., 375.01, 375.], 10000., "PostPac Economy"),
            None
        );
        assert_eq!(
            price([2500., 375., 375.], 10000.01, "PostPac Economy"),
            None
        );
    }
    #[test]
    fn discount_only_for_parcels() {
        let q = quotes([400., 300., 100.], 100., false);
        assert_eq!(q[0].cents, 900);
        assert_eq!(
            price([400., 300., 100.], 100., "Swiss-Express Lune"),
            Some(1700)
        );
    }
}
