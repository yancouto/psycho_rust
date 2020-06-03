local F = Formations
local LM = LevelManager

for i = 1, 3 do
    -- Wait for 1s before doing anything else
    LM.wait(1)

    -- Spawn a single ball enemy
    LM.spawn(F.Single {
        -- Type of enemy to spawn - Simple means one ball that dies with one shot
        enemy = BallEnemy.Simple,
        -- Initial position of the ball
        pos = vec2(-20, HEIGHT/2),
        -- Ball speed
        speed = vec2(10, 0),
        -- Ball radius (default is 20)
        radius = 10 * i,
    })
end

-- Wait until all enemies leave screen or die
LM.wait_until_no_enemies()

-- Vertical line of ball enemies
LM.spawn(F.VerticalLine {
    -- Type of enemies
    enemies = {BallEnemy.Simple},
    -- Whether the line spawns left or right
    side = VerticalLineSide.Left,
    -- Horizontal speed of each ball (Default 15). Always positive!
    speed = 10,
    -- How many enemies are spawned (Must be 1<amount<100)
    amount = 15,
    -- Radius of each ball (default 20)
    radius = 10,
    -- How enemies are placed on the line
    -- Distribute means enemies are evenly distributed from top to bottom
    placement = VerticalLinePlacement.Distribute {
        -- Margin is the distance of the top and bottom enemies to the edge of the screen (default 0)
        -- If 0, top enemy will be touching the screen
        margin = 10
    },
})

LM.wait(2)

LM.spawn(F.VerticalLine {
    enemies = {BallEnemy.Simple},
    side = VerticalLineSide.Right,
    amount = 10,
    placement = VerticalLinePlacement.Distribute {},
})

LM.wait_until_no_enemies()
LM.spawn(F.VerticalLine {
    enemies = {BallEnemy.Simple},
    side = VerticalLineSide.Right,
    amount = 4,
    -- Enemies start placed from top to bottom
    placement = VerticalLinePlacement.FromTop {
        -- Margin between the first enemy and the top (default 0)
        margin = 10,
        -- Spacing between two consecutive enemies
        spacing = 10,
    },
})

LM.spawn(F.VerticalLine {
    enemies = {BallEnemy.Simple},
    side = VerticalLineSide.Right,
    amount = 4,
    -- Analogous to FromTop
    placement = VerticalLinePlacement.FromBottom {
        margin = 10,
        spacing = 10,
    },
})

--[[
LM.spawn(F.horizontal_line {
    enemies = {BallEnemy.Simple},
    side = F.HorizontalLineSide.Top,
    amount = 11,
    placement = F.HorizontalLinePlacement.Distribute {},
})
]]