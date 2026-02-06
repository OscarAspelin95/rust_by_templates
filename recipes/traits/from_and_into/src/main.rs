use crate::schema::{Celsius, Farenheit};
mod schema;

fn main() {
    let temp_f = Farenheit::from(32.0);
    let temp_c = Celsius::from(temp_f);

    assert_eq!(0.0, temp_c.into());
}
