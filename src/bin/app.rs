#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use ravenscar_app as _;

use ravenscar_app::Mono;

use ravenscar_app::*;
use constants::constants::*;
use system_overhead::SystemOverhead;

#[rtic::app(
    device = stm32f3xx_hal::pac,
    
    dispatchers = [SPI1,ADC3,TIM3],

    peripherals = true
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
    fn init(_cx: init::Context) -> (Shared, Local) {
        defmt::info!("System init");

        Mono::start(36_000_000);

        let activation_log = activation_log::ActivationLog::new();
        let (p, c) = make_channel!(u32, { REQUEST_BUFFER_CAPACITY as usize });
        let (writer,reader) = make_channel!(bool, 1);
        regular_producer::spawn(writer.clone()).unwrap();
        on_call_producer::spawn().unwrap();
        activation_log_reader::spawn(reader).unwrap();
        force_interrupt_handler::spawn().unwrap();

        (
            Shared {
                activation_manager : activation_manager::ActivationManager::new(),
                activation_log : activation_log,
            },
            Local {
                p,
                c,
                aux : auxiliary::Auxiliary::new()
            }
        )
    }

    // Regular producer, a periodic task that executes a workload
    #[task(local = [aux,p], shared = [&activation_manager], priority = 7)]
    async fn regular_producer(cx: regular_producer::Context,mut writer: Sender<'static, bool, 1>) {

        let mut next_time = Mono::now();

        let mut work = production_workload::ProductionWorkload::new();

        let mut tracker = SystemOverhead::new();

        Mono::delay_until(cx.shared.activation_manager.get_activation_time()).await;

        loop {
            let period = next_time + REGULAR_PRODUCER_PERIOD.millis();
            let deadline = next_time + REGULAR_PRODUCER_DEADLINE.millis();
            next_time = period;
            tracker.start_tracking();
            tracker.start_sub_program();
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
            tracker.show_exec_results("Regular");
            let slack = deadline - Mono::now();
            defmt::info!("Slack time for Regular Producer: {} ms", slack.to_millis());
            Mono::delay_until(period).await
        }
    
    }

    // On Call producer, a sporadic task that is activated by the regular producer through a channel
    #[task(local = [c], shared = [&activation_manager], priority = 5)]
    async fn on_call_producer(cx: on_call_producer::Context) {
        let actv_time = cx.shared.activation_manager.get_activation_time();

        let mut tracker = SystemOverhead::new();

        let mut work = production_workload::ProductionWorkload::new();

        Mono::delay_until(actv_time).await;
        loop {
            tracker.start_tracking();

            if let Ok(w) = cx.local.c.recv().await{
                let deadline = Mono::now() + 800.millis();
                tracker.start_sub_program();
                work.small_whetstone(w);
                defmt::info!("Slack of on call producer: {} ms", (deadline - Mono::now()).to_millis());
                    
            }
            tracker.show_exec_results("On call");
            defmt::info!("End of sporadic execution.");
        }
        
    }

    #[task(shared = [&activation_manager,activation_log],priority = 3)]
    async fn activation_log_reader(mut cx: activation_log_reader::Context, mut reader : Receiver<'static, bool, 1>){
        
        let actv_time = cx.shared.activation_manager.get_activation_time();

        let mut tracker = SystemOverhead::new();

        let mut work = production_workload::ProductionWorkload::new();
        
        Mono::delay_until(actv_time).await;
        loop {
            // The signal feature is behind an alpha so I resorted to a channel based wake up communication :P
            tracker.start_tracking();

            reader.recv().await.unwrap();

            let deadline = Mono::now() + 1000.millis();
            tracker.start_sub_program();
            work.small_whetstone(ACTIVATION_LOG_READER_WORKLOAD);
            cx.shared.activation_log.lock(|log|{
                let (_count, _time) = log.read();
            });
            
            tracker.show_exec_results("Log reader");
            let slack = deadline - Mono::now();
            defmt::info!("Slack time for Activation Log Reader: {} ms", slack.to_millis());
            defmt::info!("End of parameterless sporadic activation.");
            
        }
    }

    #[task(binds = EXTI0 , shared = [&activation_manager,activation_log], priority = 11)]
    fn external_event_server(mut cx: external_event_server::Context) {
        let mut tracker = SystemOverhead::new();
        tracker.start_tracking();
        tracker.start_sub_program();
        
        let deadline = Mono::now() + 100.millis();
        
        cx.shared.activation_log.lock(|log|{
            log.write(Mono::now());
        });

        tracker.show_exec_results("External server");
        let slack = deadline - Mono::now();
        defmt::info!("Slack time for External Event Server: {} ms", slack.to_millis());
    }

    #[task(priority = 0)]
    async fn force_interrupt_handler(_: force_interrupt_handler::Context){
        let mut next_time = Mono::now();

        loop {
            next_time = next_time + INTERRUPT_PERIOD.millis();
            rtic::pend(interrupt::EXTI0);
            defmt::warn!("Interrupt submitted.");
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
