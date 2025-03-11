use gtest::{Program, System};
use wordle_game_session_io::*;

const COUNT_ATTEMPTS: u32 = 5;
const DELAY_TIMEOUT: u32 = 1;
const USER: u64 = 42;

fn get_program(system: &System) -> Program {
    system.init_logger();
    let session = Program::current(system);
    let wordle = Program::from_file(
        system,
        "../target/wasm32-unknown-unknown/release/wordle_game.opt.wasm",
    );

    assert_eq!(session.id(), 1.into());
    assert_eq!(wordle.id(), 2.into());
    system.mint_to(USER, 100000000000000000);
    wordle.send_bytes(USER, b"");
    system.run_next_block();
    session.send(
        USER,
        WordleInit {
            wordle_address: wordle.id(),
            count_attempts: COUNT_ATTEMPTS,
            delay_timeout: DELAY_TIMEOUT,
        },
    );
    system.run_next_block();
    let state: WordleState = session.read_state(b"").expect("Failed to read state");
    assert_eq!(state.count_attemps, COUNT_ATTEMPTS);
    assert_eq!(state.status, WordleStatus::Init);

    session
}

#[test]
fn handle_start_game() {
    let system = System::new();
    let program = get_program(&system);

    program.send(USER, WordleAction::StartGame);
    system.run_next_block();
    let state: WordleState = program.read_state(b"").expect("Failed to read state");
    assert_eq!(state.status, WordleStatus::GameStarted);
}

#[test]
fn handle_you_are_win() {
    let system = System::new();
    let program = get_program(&system);
    let _ = program.send(USER, WordleAction::StartGame);
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("house".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("human".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("horse".to_string()));
    system.run_next_block();
    let state: WordleState = program.read_state(b"").expect("Failed to read state");
    assert_eq!(
        state.status,
        WordleStatus::GameOver(WordlePlayerStatus::Win)
    );
}

#[test]
fn handle_you_are_loose() {
    let system = System::new();
    let program = get_program(&system);
    let _ = program.send(USER, WordleAction::StartGame);
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    program.send(USER, WordleAction::CheckWord("hhhhh".to_string()));
    system.run_next_block();
    let state: WordleState = program.read_state(b"").expect("Failed to read state");
    assert_eq!(
        state.status,
        WordleStatus::GameOver(WordlePlayerStatus::Loose)
    );
}
