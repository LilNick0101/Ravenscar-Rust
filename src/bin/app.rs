#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use ravenscar_app as _; // global logger + panicking-behavior + memory layout

use ravenscar_app::Mono;

use ravenscar_app::*;
use constants::constants::*;
use task_tracker::TaskTracker;

// TODO(7) Configure the `rtic::app` macro
#[rtic::app(
    // TODO: Replace `some_hal::pac` with the path to the PAC
    device = stm32f3xx_hal::pac,
    // TODO: Replace the `FreeInterrupt1, ...` with free interrupt vectors if software tasks are used
    // You can usually find the names of the interrupt vectors in the some_hal::pac::interrupt enum.
    dispatchers = [SPI1,ADC3,TIM2]
)]
mod app {

    use rtic_monotonics::fugit::ExtU32;
    use rtic_sync::{channel::{Receiver, Sender}, make_channel};
    use rtic_time::Monotonic;
    use stm32f3xx_hal::interrupt;
    

    use super::*;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
        activation_manager : activation_manager::ActivationManager,
        activation_log : activation_log::ActivationLog
    }

    // Local resources go here
    #[local]
    struct Local {
        p: Sender<'static, u32, {REQUEST_BUFFER_CAPACITY as usize}>,
        c: Receiver<'static, u32, { REQUEST_BUFFER_CAPACITY as usize }>,
        aux : auxiliary::Auxiliary
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        Mono::start(cx.core.SYST,36_000_000);
        // TODO setup monotonic if used
        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = 
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);
        let (p, c) = make_channel!(u32, { REQUEST_BUFFER_CAPACITY as usize });
        let (writer,reader) = make_channel!(bool, 1);
        regular_producer::spawn(writer.clone()).unwrap();
        on_call_producer::spawn().unwrap();
        activation_log_reader::spawn(reader).unwrap();
        force_interrupt_handler::spawn().unwrap();

        (
            Shared {
                // Initialization of shared resources go here
                activation_manager : activation_manager::ActivationManager::new(),
                activation_log : activation_log::ActivationLog::new(),
            },
            Local {
                // Initialization of local resources go here
                p,
                c,
                aux : auxiliary::Auxiliary::new()
            },
        )
    }

    // Regular producer, a periodic task that executes a workload
    #[task(local = [aux,p], shared = [activation_manager], priority = 7)]
    async fn regular_producer(mut cx: regular_producer::Context,mut writer: Sender<'static, bool, 1>) {

        let mut next_time = Mono::now();

        let mut work = production_workload::ProductionWorkload::new();

        let mut tracker = TaskTracker::new();

        Mono::delay_until(cx.shared.activation_manager.lock(
            |actv| actv.activation_cyclic(next_time)
        )).await;

        loop {
            next_time = next_time + REGULAR_PRODUCER_PERIOD.millis();

            tracker.start_tracking();

            work.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

            if cx.local.aux.due_activation(2){
                if let Err(_) = cx.local.p.try_send(ON_CALL_PRODUCER_WORKLOAD) {
                    defmt::error!("Failed sporadic activation.")
                }
            };

            if cx.local.aux.check_due(){
                writer.try_send(true).unwrap_or_default();
            };
            
            defmt::info!("End of cyclic execution.");

            tracker.end_tracking("regular");
            
            Mono::delay_until(next_time).await;
        }
    
    }

    // On Call producer, a sporadic task that is activated by the regular producer through a channel
    #[task(local = [c], shared = [activation_manager], priority = 5)]
    async fn on_call_producer(mut cx: on_call_producer::Context) {
        let actv_time = cx.shared.activation_manager.lock(|actv|{
            actv.activation_sporaic()
        });

        let mut work = production_workload::ProductionWorkload::new();

        Mono::delay_until(actv_time).await;
        loop {
            if let Ok(w) = cx.local.c.recv().await{
                work.small_whetstone(w);
                defmt::info!("End of sporadic execution.");
            }
        }
        
    }

    #[task(shared = [activation_manager,activation_log],priority = 3)]
    async fn activation_log_reader(mut cx: activation_log_reader::Context, mut reader : Receiver<'static, bool, 1>){
        
        let actv_time = cx.shared.activation_manager.lock(|actv|{
            actv.activation_sporaic()
        });
        
        let mut work = production_workload::ProductionWorkload::new();
        
        Mono::delay_until(actv_time).await;
        loop {
            // The signal feature is behind an alpha so I resorted to a channel based wake up communication :P
            reader.recv().await.unwrap();
            work.small_whetstone(ACTIVATION_LOG_READER_WORKLOAD);
            cx.shared.activation_log.lock(|log|{
                let (_count, _time) = log.read();
            });
            defmt::info!("End of parameterless sporadic activation.");
        }
    }

    #[task(binds = EXTI0 , shared = [activation_manager,activation_log], priority = 11)]
    fn external_event_server(mut cx: external_event_server::Context) {
        cx.shared.activation_log.lock(|log|{
            log.write(Mono::now());
        });

        defmt::info!("External event server has written in the activation log.");
    }

    #[task(priority = 0)]
    async fn force_interrupt_handler(_: force_interrupt_handler::Context){
        let mut next_time = Mono::now();

        loop {
            next_time = next_time + (REGULAR_PRODUCER_PERIOD * 5).millis();
            rtic::pend(interrupt::EXTI0);
            Mono::delay_until(next_time).await;
        }
        
    }
    /*
    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }
    */
}
