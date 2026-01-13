pub fn honesty(faults: u64, audits: u64) -> f64 {
    if audits == 0 {
        return 1.0;
    }

    let h = 1.0 - (faults as f64 / audits as f64);
    if h < 0.0 { 0.0 } else { h }
}

pub fn age_metric(epochs_active: u64) -> f64 {
    ((1 + epochs_active) as f64).ln()
}

pub fn loyalty(honesty: f64, age: f64, max_age: f64) -> f64 {
    let norm_age = if max_age == 0.0 { 0.0 } else { age / max_age };
    let l = 0.7 * honesty + 0.3 * norm_age;
    if l > 1.0 { 1.0 } else { l }
}
