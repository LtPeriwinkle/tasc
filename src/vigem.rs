use crate::parse::Tas;
use crate::TasError;
use std::time::Instant;
use vigem_client::{Client, TargetId, XButtons, Xbox360Wired};

impl Tas {
    pub fn run_tas(&mut self, dbg: bool) -> Result<(), TasError> {
        println!("Connecting to ViGEm...");
        let client = Client::connect().map_err(|e| TasError::Vigem { e: e.to_string() })?;
        let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
        target
            .plugin()
            .map_err(|e| TasError::Vigem { e: e.to_string() })?;
        println!("Connected!");
        let start = Instant::now();
        let mut on: u16 = 0;
        let mut gamepad = vigem_client::XGamepad::default();
        let (mut lx, mut ly) = (0, 0);
        let (mut rx, mut ry) = (0, 0);
        let (mut lt, mut rt) = (0, 0);
        for line in &self.lines {
            if dbg {
                println!("Sleeping for {} frames.", line.delay.as_nanos() / 16666666);
            }
            std::thread::sleep(line.delay);
            let on_without_triggers = !(line.on & 0xC00); // mask the bits for triggers, 0x400 and 0x800
            let on_without_off = on_without_triggers & !line.off; // turn off bits for things that are not meant to be on
            on |= on_without_off;
            gamepad.buttons = XButtons::from(on);
            println!("{:?}", on);
            if let Some(s) = line.lstick {
                lx = s.x;
                ly = s.y;
            }
            gamepad.thumb_lx = lx;
            gamepad.thumb_ly = ly;
            if let Some(s) = line.rstick {
                rx = s.x;
                ry = s.y;
            }
            gamepad.thumb_rx = rx;
            gamepad.thumb_ry = ry;
            let triggers = on >> 10;
            if triggers & 1 == 0 {
                lt = 255;
            }
            if (triggers >> 1) & 1 == 0 {
                rt = 255;
            }
            let t_off = line.off >> 10;
            if t_off & 1 == 0 {
                lt = 0;
            }
            if (t_off >> 1) & 1 == 0 {
                rt = 0;
            }
            gamepad.left_trigger = lt;
            gamepad.right_trigger = rt;
        }
        println!("Ran tas in {} ms", start.elapsed().as_millis());
        Ok(())
    }
}
