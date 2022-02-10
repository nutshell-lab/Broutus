# BROUTUS

The soon-extraordinary game by Nutshell

# Gameplay

- Create a team
    - Player has teams
    - Team has characters
    - Team has a maximum customization points limit
    - Character has stats
        - Strength (damages)
        - Agility (dodge)
        - Accuracy (aim)
        - Constitution (health)
        - Luck (critical strikes)
    - Character has items
    - Character has limited items slots
        - Head
        - Left arm
        - Right arm
        - Upper body
        - Lower body
        - Back
        - Pocket 1
        - Pocket 2
    - Item has a type
        - Helmet
        - Weapon (1h)
        - Weapon (2h)
        - Shield
        - Armor
        - Pants
    - Item provides stats bonuses
    - Item provides actions
    - Item has a wost in customization points
- Figth in arena
    - Turn based
    - Grid based deplacement
    - Automatic selection of the current character based on a stat
    - Move or Act
    - Movement points
    - Action points
    - Differents actions
    - Items changes character stats and actions
    - Character has base actions
    - Character has actions given by his items
    - Combined stuff provide more actions (item1 + item2 = action3 unlocked)

# Prototyping
Create a team:
1. Add a basic HUD
2. Show a list of items
3. Display some text-properties of an item

Fight :
1. Spawn a character 
2. Move the character to a location
3. Make the location follow a path (orthogonal)
4. Make a tile below the mouse highlight
5. Hilight a path between two tiles
