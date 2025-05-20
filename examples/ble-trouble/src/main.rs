#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use defmt::{info, warn, unwrap};
use embassy_executor::Spawner;
use embassy_futures::select::select;
use microbit_bsp::{ble::MultiprotocolServiceLayer, Config, Microbit};
use trouble_host::prelude::*;

/// Size of L2CAP packets (ATT MTU is this - 4)
const L2CAP_MTU: usize = 251;

/// Max number of connections
const CONNECTIONS_MAX: usize = 1;

/// Max number of L2CAP channels.
const L2CAP_CHANNELS_MAX: usize = 2; // Signal + att

// GATT Server definition
#[gatt_server]
struct Server {
    battery_service: BatteryService,
}

// Battery service
#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, read, value = "Battery Level")]
    #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify)]
    level: u8,
}

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::new(Config::default());
    let (sdc, mpsl) = board
        .ble
        .init(board.timer0, board.rng)
        .expect("BLE Stack failed to initialize");
    spawner.must_spawn(mpsl_task(mpsl));

    run(sdc).await;
}

pub async fn run<C>(controller: C)
where
    C: Controller,
{
    // Using a fixed "random" address can be useful for testing. In real scenarios, one would
    // use e.g. the MAC 6 byte array as the address (how to get that varies by the platform).
    let address = Address::random([0x42, 0x6A, 0xE3, 0x1E, 0x83, 0xE7]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU> = HostResources::new();
    let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral, runner, ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "Trouble",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .expect("Failed to create GATT server");

    let app_task = async {
        loop {
            match advertise("Trouble Example", &mut peripheral, &server).await {
                Ok(conn) => {
                    connection_task(&server, &conn).await;
                }
                Err(e) => {
                    let e = defmt::Debug2Format(&e);
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    };
    select(ble_task(runner), app_task).await;
}

/// This is a background task that is required to run forever alongside any other BLE tasks.
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) -> Result<(), BleHostError<C::Error>> {
    runner.run().await
}



/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, 'b, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
    server: &'b Server<'_>,
) -> Result<GattConnection<'a, 'b>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn connection_task(server: &Server<'_>, conn: &GattConnection<'_, '_>) {
    let level = server.battery_service.level;
    unwrap!(level.set(server, &42));
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event {
                // Server processing emits
                Ok(event) => {
                    if let Ok(reply) = event.accept() {
                        reply.send().await;
                    }
                }
                Err(e) => warn!("[gatt] error processing event: {:?}", e),
            }
            _ => (),
        }
    }
    info!("[gatt] task finished");
}
