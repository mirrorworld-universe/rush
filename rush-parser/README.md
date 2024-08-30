# rush-parser

## Sample Blueprint

```toml
[world]
name = "Sonic's World"
entities = ["player"]
regions = ["farm", "house"]

[player] # (e.g. [<entity_name>])
name = "string"
x = 0
y = 0
w = 0
h = 0
speed = 0

[instances.farm]
player = [
  # speed = 0, taken default value from [player] table
	{ name = "npc", x = 0, y = 0, w = 0, h = 0 }
]

[instances.house]
player = [
	{ name = "npc", x = 0, y = 0, w = 0, h = 0, speed = 50 }
]
```
