local F = Formation
local LE = LevelEvent

-- Changes the default indicator duration. By default it is 1.
LE.SetDefaultIndicatorDuration(0.5)

for i = 1, 3 do
    -- Wait for 1s before doing anything else
    LE.Wait(1)

    -- Spawn a single ball enemy
    LE.Spawn(F.Single {
        -- Type of enemy to Spawn - Simple means one ball that dies with one shot
        enemy = BallEnemy.Simple,
        -- Initial position of the ball
        pos = {-20, HEIGHT/2},
        -- Ball speed
        speed = {10, 0},
        -- Ball radius (default is 20)
        radius = 10 * i,
    })
end

-- Wait until all enemies leave screen or die
LE.WaitUntilNoEnemies()

-- This spawns an enemy using custom configurations
LE.CustomSpawn {
    -- How long will the indicator show on screen. Has a default.
    indicator_duration = 6,
    -- Whether this spawning should instead point at the player.
    -- Speeds directions are disregarded and only the length of the speed vector counts.
    -- Default is false.
    follow_player = true,
    formation = F.Single {
        enemy = BallEnemy.Simple,
        pos = {WIDTH / 2, -20},
        speed = {0, 10},
    }
}

LE.WaitUntilNoEnemies()

LE.SetDefaultIndicatorDuration(1.)

for i = 1, 3 do
    -- Spawn multiple balls in a line
    LE.Spawn(F.Multiple {
        enemies = {BallEnemy.Simple},
        -- Amount of balls spawned
        amount = 30,
        -- Spacing between adjacent balls (default is 5)
        spacing = 10,
        -- Initial position of the ball
        pos = {WIDTH + 20, HEIGHT / 4 * i},
        -- Balls speed (and direction)
        speed = {-10, 0},
        -- Radius of all balls (default is 20)
        radius = nil,
    })
end

for i = 1, 3 do
    LE.Spawn(F.VerticalLine {
        -- Type of enemies
        enemies = {BallEnemy.Simple, BallEnemy.Double},
        -- Whether the V spawns left or right
        side = VerticalLineSide.Left,
        -- Horizontal speed of each ball (Default 15). Always positive!
        speed = 10,
        -- How many enemies are Spawned (Must be at most 255, and ODD!)
        amount = 15,
        -- Radius of each ball (default 20)
        radius = 25,
        -- V means balls are spawned like this, centered on the screen
        --  *
        --    *
        --      *
        --    *
        --  *
        placement = VerticalLinePlacement.V {
            -- Margin is the distance of the top and bottom enemies to the edge of the screen (default 0)
            -- If 0, top enemy will be touching the screen
            margin = 10,
            -- How far apart adjacent balls are in the X axis.
            spacing = 10,
        },
    })
    LE.Wait(1.5)
end

-- Vertical line of ball enemies
LE.Spawn(F.VerticalLine {
    -- Type of enemies
    enemies = {BallEnemy.Simple},
    -- Whether the line Spawns left or right
    side = VerticalLineSide.Left,
    -- Horizontal speed of each ball (Default 15). Always positive!
    speed = 10,
    -- How many enemies are Spawned (Must be at most 255)
    amount = 15,
    -- Radius of each ball (default 20)
    radius = 25,
    -- How enemies are placed on the line
    -- Distribute means enemies are evenly distributed from top to bottom
    placement = VerticalLinePlacement.Distribute {
        -- Margin is the distance of the top and bottom enemies to the edge of the screen (default 0)
        -- If 0, top enemy will be touching the screen
        margin = 10
    },
})

LE.Wait(2)

LE.Spawn(F.VerticalLine {
    enemies = {BallEnemy.Simple},
    side = VerticalLineSide.Right,
    amount = 10,
    placement = VerticalLinePlacement.Distribute {},
})

LE.WaitUntilNoEnemies()
LE.Spawn(F.VerticalLine {
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

LE.Spawn(F.VerticalLine {
    enemies = {BallEnemy.Simple},
    side = VerticalLineSide.Right,
    amount = 4,
    -- Analogous to FromTop
    placement = VerticalLinePlacement.FromBottom {
        margin = 10,
        spacing = 10,
    },
})

LE.Wait(2)

-- Analogous to VerticalLine
LE.Spawn(F.HorizontalLine {
    enemies = {BallEnemy.Simple},
    side = HorizontalLineSide.Top,
    amount = 11,
    placement = HorizontalLinePlacement.Distribute {},
})

LE.WaitUntilNoEnemies()

LE.Spawn(F.Circle {
    enemies = {BallEnemy.Simple},
    amount = 12,
    -- Enemy radius (default is 20)
    enemy_radius = 19,
    -- Speed of each ball (Default 15). Always positive!
    speed = 10,
    -- Radius of the formation. Defaults to half the screen diagonal + enemy_radius,
    -- so the enemies will be just outside the screen.
    formation_radius = nil,
    -- Center of the formation. If you specify this you *must* specify formation_radius
    -- Defaults to the center of the screen.
    formation_center = nil,
})

LE.WaitUntilNoEnemies()
LE.Wait(0.5)

for i = 1, 2 do
    LE.Spawn(F.Circle {
        enemies = {BallEnemy.Simple},
        amount = 10,
        formation_center = {WIDTH / 3 * i, HEIGHT / 2},
        formation_radius = (WIDTH ^ 2 + HEIGHT ^ 2) ^ 0.5,
    })
end

-- Create a spiral, that is, a series of enemies created in a circle
-- with increasing radius
LE.Spawn(F.Spiral {
    enemies = {BallEnemy.Simple},
    -- How many enemies should be in a full circunference
    -- This means the angle between two adjacent circles in this formation
    -- is 2 * pi / amount_in_circle
    amount_in_circle = 30,
    -- Total amount of enemies. May be smaller or larger than amount_in_circle
    amount = 50,
    -- "Radial" spacing between enemies. That means the distance from an enemy to
    -- the center of the formation will increase by this value for each circle
    spacing = 30,
    -- Enemy speed (defaults to 10)
    speed = 10,
    -- Enemy radius
    radius = 20,
})
-- Use small values of amount_in_circle like this to create a quick spiral around psycho


LE.WaitUntilNoEnemies()

-- Or larger values to create a slowly drifting line
LE.Spawn(F.Spiral {
    enemies = {BallEnemy.Simple},
    amount_in_circle = 150,
    amount = 40,
    spacing = 60,
    speed = 10,
    radius = 20,
})

LE.WaitUntilNoEnemies()