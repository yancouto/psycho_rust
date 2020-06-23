local F = Formation
local LE = LevelEvent
local SB = BallEnemy.Simple
local DB = BallEnemy.Double

local s = 7
local r = 25

local top_bottom = {HorizontalLineSide.Top, HorizontalLineSide.Bottom}
local left_right = {VerticalLineSide.Left, VerticalLineSide.Right}

LE.CustomSpawn {
    indicator_duration = 3,
    formation = F.Single {
        enemy = SB,
        pos = {-r, HEIGHT / 2},
        radius = r * 1.25,
        speed = {s / 2, 0},
    }
}

LE.WaitUntilNoEnemies()
LE.Wait(0.25)

LE.Spawn(F.Single { enemy = SB, radius = r, pos = {WIDTH / 2, -r}, speed = {0, s}})
LE.Spawn(F.Single { enemy = SB, radius = r, pos = {WIDTH / 2, HEIGHT + r}, speed = {0, -s}})

LE.Wait(1.25)

LE.Spawn(F.Circle { enemies = {SB}, enemy_radius = r, speed = s, amount = 4 })

LE.WaitUntilNoEnemies()

LE.Spawn(F.VerticalLine {
    enemies = {SB},
    side = VerticalLineSide.Left,
    speed = s,
    amount = 8,
    radius = r,
    placement = VerticalLinePlacement.Distribute {
        margin = 10,
    }
})

LE.Wait(2)

for _, side in ipairs(top_bottom) do
    LE.Spawn(F.HorizontalLine {
        enemies = {SB},
        side = side,
        speed = s,
        amount = 12,
        radius = r,
        placement = HorizontalLinePlacement.Distribute {
            margin = 10,
        }
    })
    LE.Wait(.5)
end

LE.Wait(1.5)
LE.CustomSpawn {
    indicator_duration = 0.5,
    formation = F.Circle {
        starting_angle = math.pi / 4,
        enemies = {SB},
        amount = 4,
        enemy_radius = r,
        speed = s * 1.25,
    }
}

LE.WaitUntilNoEnemies()

for i = 1, 5 do
    LE.CustomSpawn {
        indicator_duration = 0.5,
        formation = F.VerticalLine {
            enemies = {SB},
            amount = 15,
            side = VerticalLineSide.Left,
            speed = s * (i == 5 and 1.25 or 1),
            placement = VerticalLinePlacement.V {
                margin = 10,
                spacing = 10 + i * 10,
            }
        }
    }
    LE.Wait(0.35)
end

LE.WaitUntilNoEnemies()
LE.Wait(1)


for _, side in ipairs(top_bottom) do
    for _, placement in ipairs { HorizontalLinePlacement.FromLeft, HorizontalLinePlacement.FromRight } do
        LE.Spawn(F.HorizontalLine {
            enemies = {SB},
            side = side,
            amount = 8,
            speed = s,
            radius = r,
            placement = placement {
                margin = 10,
                spacing = 20,
            }
        })
    end
end

LE.Wait(1.5)

for _, side in ipairs(top_bottom) do
    LE.Spawn(F.HorizontalLine {
        enemies = {SB},
        side = side,
        amount = 7,
        speed = s,
        radius = r,
        placement = HorizontalLinePlacement.Distribute {
            margin = WIDTH * .3,
        }
    })
end

LE.Wait(0.5)

for _, side in ipairs(left_right) do
    LE.Spawn(F.VerticalLine {
        enemies = {SB},
        side = side,
        amount = 10,
        speed = s,
        radius = r,
        placement = VerticalLinePlacement.Distribute {
            margin = WIDTH * .15,
        }
    })
end

LE.WaitUntilNoEnemies()

LE.CustomSpawn {
    indicator_duration = 2,
    formation = F.Single {
        enemy = DB,
        radius = r * 1.25,
        pos = {WIDTH / 2, HEIGHT + r},
        speed = {0, -s / 2},
    }
}

LE.Wait(3)
LE.WaitUntilNoEnemies()

LE.Spawn(F.Multiple {
    enemies = {DB},
    amount = 45,
    speed = {-s, 0},
    pos = {WIDTH + r, HEIGHT / 2},
    radius = r,
})
LE.Wait(3)
LE.Spawn(F.HorizontalLine {
    enemies = {DB},
    amount = 17,
    side = HorizontalLineSide.Top,
    speed = s,
    radius = r,
    placement = HorizontalLinePlacement.Distribute { margin = 10 }
})
LE.Wait(1.5)
LE.Spawn(F.VerticalLine {
    enemies = {DB},
    amount = 16,
    side = VerticalLineSide.Left,
    speed = s,
    radius = r,
    placement = VerticalLinePlacement.Distribute { margin = 10 }
})
LE.WaitUntilNoEnemies()