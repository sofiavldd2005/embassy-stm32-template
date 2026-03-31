#![no_std]
#![no_main]

{% if use_defmt %}
use defmt_rtt as _;
use defmt::info;
{% else %}
// Provide a basic panic handler if defmt isn't used
use panic_halt as _;
{% endif %}

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    
    {% if use_defmt %}
    info!("Initialized {{ mcu }} successfully!");
    {% endif %}
    
    loop {}
}
