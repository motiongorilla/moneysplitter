// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

const TAXED: f32 = 0.40;
const NEEDS: f32 = 0.50;
const WANTS: f32 = 0.30;
const SAVINGS: f32 = 0.20;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let amount: f32 = string.trim().parse().unwrap();
            let taxed: f32 = amount * TAXED;
            let needs: f32 = taxed * NEEDS;
            let wants: f32 = taxed * WANTS;
            let savings: f32 = taxed * SAVINGS;
            let result: String = format!(
                "After taxes: {:.2}\nNeeds: {:.2}\nWants: {:.2}\nSavings: {:.2}",
                taxed, needs, wants, savings
            );
            ui.set_results(result.into());
        }
    });

    ui.run()
}
