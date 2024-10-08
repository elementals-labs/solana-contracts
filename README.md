# Elementals - Solana Smart Contract

Welcome to the **Elementals** repository! This is a Solana-based set of smart contracts developed during the [Solana Radar Global Hackathon](https://www.colosseum.org/radar), constituting the core of the "Elementals" game. In this turn-based strategy game, players command teams of elementals in battles, each with unique abilities and strategies. The contracts here handle the initialization, player registration, game mechanics, and turn execution.

<p align="center">
  <img src="https://github.com/user-attachments/assets/076b1553-6fcb-42b8-a145-aced96b0f377" alt="A Zephyra and a Torchy battle each other in the forest arena" />
</p>


## Table of Contents

- [Overview](#overview)
- [Game Flow](#game-flow)
- [Contract Structure](#contract-structure)
- [Instructions](#instructions)
- [Events](#events)
- [How to Contribute](#how-to-contribute)
- [License](#license)

## Overview

**Elementals** is a turn-based strategy game where players select teams of elementals and battle in pairs. This repository contains the smart contracts deployed on the Solana blockchain, which handle the core game mechanics such as queue management, player registration, and game logic execution.

The game involves the following steps:
1. Players register for a match.
2. Once two players are registered, the game initializes.
3. Players take turns executing moves, and the game progresses based on the moves made.

## Game Flow

1. **Queue Initialization**: A queue is created to collect players until a match can be formed.
2. **Player Registration**: Players join the queue, selecting their team of elementals.
3. **Game Creation**: Once two players are registered, the game starts and both players' teams are initialized.
4. **Game Play**: Players alternate turns, executing their moves until the game reaches a conclusion.

## Contract Structure

The game is implemented in Rust using the Solana **Anchor framework**, with the following main components:

- **enums**: Defines enumerations used across the game, such as game statuses, elemental types, etc.
- **events**: Handles the logging of events like player registration, game creation, and actions within the game.
- **game_accounts**: Manages the Solana accounts associated with each player and game instance.
- **movements**: Defines possible player actions during a game, including various elemental abilities and statuses.

### Key Functions

1. **initialize_queue**: Initializes the queue that will hold players waiting for a game.
2. **register_to_play**: Registers a player to join the queue and waits for another player to start a game.
3. **play_game**: Allows players to make a move and progresses the game based on the input provided by the player.

## Instructions

### 1. **initialize_queue**

Initializes a queue where players are registered before starting a game.

```rust
pub fn initialize_queue(ctx: Context<InitializeQueue>, name: String) -> Result<()>
```

### 2. **register_to_play**

Registers a player for the next available game. Once two players are registered, the game starts automatically.

```rust
pub fn register_to_play(ctx: Context<RegisterPlayer>, team: ElementalTeamInput) -> Result<()>
```

### 3. **play_game**

Executes a player’s move in the game. The game progresses turn by turn based on the moves made by each player.

```rust
pub fn play_game(ctx: Context<Playing>, play: UserAction) -> Result<()>
```

## Events

The following events are emitted during gameplay:

- **PlayerRegistered**: Emitted when a player registers for a game.
- **GameCreated**: Emitted when a new game instance is created.
- **UserPlayRegistered**: Emitted when a player makes a move.
- **GameUpdated**: Emitted when the game status is updated (e.g., game completion).

## How to Contribute

Feel free to contribute to the Elementals project! You can help by:
- Reporting bugs.
- Suggesting new features
- Suggesting code improvements.
