use std::io::{stdout, Write};
use chrono::{Datelike, Timelike};
use crossterm::{cursor, event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

enum DateInputKind {
    Hour,
    Minute,

    Day,
    Month,
    Year,
}

struct UserDateInput {
    hour: u32,
    minute: u32,

    day: u32,
    month: u32,
    year: i32,
}

fn mutate_user_input (user_date_input: &mut UserDateInput, input_kind: &DateInputKind, diff: i32) {
    match input_kind {
        DateInputKind::Hour => user_date_input.hour = user_date_input.hour.saturating_add_signed(diff),
        DateInputKind::Minute => user_date_input.minute = user_date_input.minute.saturating_add_signed(diff),

        DateInputKind::Day => user_date_input.day = user_date_input.day.saturating_add_signed(diff),
        DateInputKind::Month => user_date_input.month = user_date_input.month.saturating_add_signed(diff),
        DateInputKind::Year => user_date_input.year = user_date_input.year.saturating_add(diff),
    }

}

fn main() {
    let mut user_input = UserDateInput {
        hour: chrono::offset::Local::now().hour(),
        minute: chrono::offset::Local::now().minute(),

        day: chrono::offset::Local::now().day(),
        month: chrono::offset::Local::now().month(),
        year: chrono::offset::Local::now().year()
    };

    let mut current_selected_input: DateInputKind = DateInputKind::Hour; 

    enable_raw_mode().unwrap();

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    loop {

        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        execute!(stdout(), Print("Please select a new date".to_string())).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();

        execute!(
            // Idea: implement printing pethod for user_input ? 
            stdout(),
            SetForegroundColor(Color::Red),

            Print(user_input.hour),
            Print(":"),
            Print(user_input.minute),

            Print("    "),

            Print(user_input.day),

            Print("/"),
            Print(user_input.month),

            Print("/"),
            Print(user_input.year),

            ResetColor
        ).unwrap();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => mutate_user_input(&mut user_input, &current_selected_input, 1),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => mutate_user_input(&mut user_input, &current_selected_input, -1),

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => match current_selected_input {
                // Find better data structures ? Tuples ? Can I dynamically access tuples ?
                DateInputKind::Hour => current_selected_input = DateInputKind::Minute,
                DateInputKind::Minute => current_selected_input = DateInputKind::Day,

                DateInputKind::Day => current_selected_input = DateInputKind::Month,
                DateInputKind::Month => current_selected_input = DateInputKind::Year,
                DateInputKind::Year => ()
            }

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => match current_selected_input {
                DateInputKind::Hour => (),
                DateInputKind::Minute => current_selected_input = DateInputKind::Hour,

                DateInputKind::Day => current_selected_input = DateInputKind::Minute,
                DateInputKind::Month => current_selected_input = DateInputKind::Day,
                DateInputKind::Year => current_selected_input = DateInputKind::Month,
            }

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => break,

            _ => ()
            
        }
    }

    

    disable_raw_mode().unwrap();

    // TODO: Pipe me!

    // stdout().wr





}
