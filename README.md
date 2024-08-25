# guessing-game

A classic number guessing game implemented in Rust. The game generates a random number between 1 and 100 (inclusive) and challenges the player to guess it.

## How to Play

1. The game will generate a secret number between 1 and 100.
2. You'll be prompted to enter your guess.
3. After each guess, you'll be told if your guess was too high, too low, or correct.
4. Keep guessing until you find the correct number!

## Running the Project Locally

To run this project on your local machine:

1. Ensure you have Rust and Cargo installed. If not, [install Rust](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```
    git clone https://github.com/AmulGaurav/guessing-game.git
    cd guessing-game
   ```
3. Build and run the project using Cargo:
   ```
    cargo run
   ```
   Alternatively, you can build the project and then run the binary separately:
   ```
    cargo build
    ./target/debug/guessing_game
   ```

## Contributing

Contributions to improve the game are welcome! Here's how you can contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix:
   ```
    git checkout -b feature/your-feature-name
   ```
3. Make your changes and commit them with a descriptive commit message.
4. Push your changes to your fork:
   ```
    git push origin feature/your-feature-name
   ```
5. Create a pull request from your fork to the main repository.

Please ensure your code follows Rust best practices and includes appropriate comments.

## License

This project is licensed under the [MIT License](LICENSE).
