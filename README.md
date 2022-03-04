# BROUTUS

> Game prototype

The soon-extraordinary (first) game by Nutshell

## Getting started

First, install [rustup](https://rustup.rs/).

Next install rust 1.58.0.
```sh
rustup install 1.58.0
```

Next follow [bevy's setup instructions](https://bevyengine.org/learn/book/getting-started/setup/).

## Gameplay

- Start a game
    - Select 3 warriors to create a team
    - Warriors are avalable through a predefined list
    - (later) draft system - alternate pick / ban
- Figth in arena 3vs3
    - Turn based combat
    - Grid based deplacement
    - Automatic selection of the current warrior based on automatic turn ordering system
    - Movement points
    - Action points
    - Warrior has a dedicated set of actions

## Prototyping
Create a team:
1. Show warriors list
2. Select 3 of them
3. Click Fight!

Fight :
1. Spawn a warrior 
2. Move the warrior to a location
3. Make the location follow a path (orthogonal)
4. Make a tile below the mouse highlight
5. Hilight a path between two tiles
